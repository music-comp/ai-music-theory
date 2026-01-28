//! Application state management.
//!
//! This module provides AppState for managing the search backend state,
//! including FTS readiness tracking and dynamic backend switching.

use std::sync::Arc;

#[cfg(feature = "fts")]
use std::path::Path;
#[cfg(feature = "fts")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(feature = "fts")]
use std::sync::RwLock;

use crate::config::Config;
use crate::error::Result;
use crate::search::backend::SearchBackend;
use crate::search::SimpleSearch;

#[cfg(feature = "fts")]
use crate::search::{build_index, is_index_fresh, TantivySearch};

/// Type alias for FTS backend initialization return type.
#[cfg(feature = "fts")]
type FtsBackendInit = (Arc<RwLock<Option<Arc<TantivySearch>>>>, Arc<AtomicBool>);

/// Shared application state.
///
/// Manages search backends and FTS readiness. Cloneable for sharing across
/// request handlers via Arc-wrapped internals.
#[derive(Clone)]
pub struct AppState {
    /// Configuration
    pub config: Config,

    /// Simple search backend (always available)
    simple_backend: Arc<SimpleSearch>,

    /// FTS backend (optional, may be None initially)
    #[cfg(feature = "fts")]
    fts_backend: Arc<RwLock<Option<Arc<TantivySearch>>>>,

    /// Whether FTS index is ready for queries
    #[cfg(feature = "fts")]
    fts_ready: Arc<AtomicBool>,
}

impl AppState {
    /// Create a new AppState.
    ///
    /// Initializes simple search backend (always available) and attempts
    /// to load FTS backend if configured and index exists.
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration
    ///
    /// # Returns
    ///
    /// Returns AppState ready for use.
    ///
    /// # Errors
    ///
    /// Returns `Err` if simple search initialization fails.
    pub async fn new(config: Config) -> Result<Self> {
        let simple_backend = Arc::new(SimpleSearch::new(config.clone()));

        #[cfg(feature = "fts")]
        let (fts_backend, fts_ready) = initialize_fts_backend(&config)?;

        Ok(Self {
            config,
            simple_backend,
            #[cfg(feature = "fts")]
            fts_backend,
            #[cfg(feature = "fts")]
            fts_ready,
        })
    }

    /// Get the currently active search backend.
    ///
    /// Returns FTS backend if ready, otherwise simple backend.
    /// This allows transparent failover during FTS initialization.
    ///
    /// # Returns
    ///
    /// Returns Arc-wrapped SearchBackend for shared ownership across requests.
    pub fn search_backend(&self) -> Arc<dyn SearchBackend> {
        #[cfg(feature = "fts")]
        if self.fts_ready.load(Ordering::Acquire) {
            if let Ok(guard) = self.fts_backend.read() {
                if let Some(ref backend) = *guard {
                    return Arc::clone(backend) as Arc<dyn SearchBackend>;
                }
            }
        }

        Arc::clone(&self.simple_backend) as Arc<dyn SearchBackend>
    }

    /// Get the name of the currently active backend.
    ///
    /// # Returns
    ///
    /// Returns "tantivy" if FTS is ready, otherwise "simple".
    pub fn active_backend_name(&self) -> &'static str {
        #[cfg(feature = "fts")]
        if self.fts_ready.load(Ordering::Acquire) {
            return "tantivy";
        }
        "simple"
    }

    /// Check if FTS is ready (feature-gated).
    ///
    /// # Returns
    ///
    /// Returns true if FTS backend is initialized and ready for queries.
    #[cfg(feature = "fts")]
    pub fn is_fts_ready(&self) -> bool {
        self.fts_ready.load(Ordering::Acquire)
    }

    /// Mark FTS as ready or not ready (internal use).
    ///
    /// Used by background indexing to signal when FTS becomes available.
    #[cfg(feature = "fts")]
    pub(crate) fn set_fts_ready(&self, ready: bool) {
        self.fts_ready.store(ready, Ordering::Release);
    }

    /// Update the FTS backend after background indexing completes.
    ///
    /// # Arguments
    ///
    /// * `backend` - The newly built TantivySearch backend
    ///
    /// # Errors
    ///
    /// Returns `Err` if the write lock cannot be acquired.
    #[cfg(feature = "fts")]
    pub(crate) fn update_fts_backend(&self, backend: TantivySearch) -> Result<()> {
        let mut guard = self
            .fts_backend
            .write()
            .map_err(|_| crate::error::Error::config("Failed to acquire write lock".to_string()))?;
        *guard = Some(Arc::new(backend));
        Ok(())
    }
}

/// Initialize FTS backend on startup (module-level function).
///
/// Attempts to load an existing Tantivy index if configured.
/// Returns None if index doesn't exist (will be built in background).
///
/// # Arguments
///
/// * `config` - Server configuration
///
/// # Returns
///
/// Returns tuple of (fts_backend, fts_ready) where backend may be None
/// if index needs to be built.
#[cfg(feature = "fts")]
fn initialize_fts_backend(config: &Config) -> Result<FtsBackendInit> {
    if config.search.backend == "tantivy" {
        let index_path = config.search.index_path()?;

        // Try to load existing index
        match TantivySearch::new(&index_path, config.search.clone()) {
            Ok(backend) => {
                // Index exists and is loadable
                log::info!("Loaded existing FTS index from disk");
                Ok((
                    Arc::new(RwLock::new(Some(Arc::new(backend)))),
                    Arc::new(AtomicBool::new(true)),
                ))
            }
            Err(e) => {
                // Index doesn't exist yet - will build in background (Phase 3)
                log::debug!("FTS index not found: {} (will build if configured)", e);
                Ok((
                    Arc::new(RwLock::new(None)),
                    Arc::new(AtomicBool::new(false)),
                ))
            }
        }
    } else {
        // FTS not configured
        Ok((
            Arc::new(RwLock::new(None)),
            Arc::new(AtomicBool::new(false)),
        ))
    }
}

/// Initialize FTS backend and start background indexing if needed.
///
/// This is the main entry point for FTS initialization. It:
/// 1. Checks if an index exists and is fresh
/// 2. If fresh, loads it immediately
/// 3. If not fresh or missing, starts background indexing
///
/// Server starts immediately - indexing happens asynchronously.
///
/// # Arguments
///
/// * `state` - Shared application state
///
/// # Errors
///
/// Returns `Err` if index path resolution fails.
#[cfg(feature = "fts")]
pub async fn initialize_fts(state: &Arc<AppState>) -> Result<()> {
    if state.config.search.backend != "tantivy" {
        log::debug!(
            "FTS not configured (backend={})",
            state.config.search.backend
        );
        return Ok(());
    }

    let index_path = state.config.search.index_path()?;

    // Check if index exists and is fresh
    if index_exists_and_fresh(&index_path, &state.config).await? {
        log::info!("FTS index found and is current");
        // Index already loaded during AppState::new()
        Ok(())
    } else {
        log::info!("FTS index needs building - starting background task");
        start_background_indexing(Arc::clone(state));
        Ok(())
    }
}

/// Check if index exists and is fresh (module-level helper).
#[cfg(feature = "fts")]
async fn index_exists_and_fresh(index_path: &Path, config: &Config) -> Result<bool> {
    if !index_path.exists() {
        log::debug!("Index path does not exist: {}", index_path.display());
        return Ok(false);
    }

    match is_index_fresh(index_path, config).await {
        Ok(fresh) => {
            if fresh {
                log::debug!("Index is fresh");
            } else {
                log::debug!("Index exists but is stale");
            }
            Ok(fresh)
        }
        Err(e) => {
            log::warn!("Error checking index freshness: {}", e);
            Ok(false)
        }
    }
}

/// Start background indexing task (module-level function).
///
/// Spawns a tokio task that builds the index asynchronously.
/// When complete, updates the AppState with the new backend and marks FTS ready.
#[cfg(feature = "fts")]
fn start_background_indexing(state: Arc<AppState>) {
    tokio::spawn(async move {
        log::info!("Background indexing started");

        match build_fts_index_for_state(&state).await {
            Ok(stats) => {
                log::info!(
                    indexed = stats.indexed,
                    errors = stats.errors;
                    "Background indexing complete"
                );

                // Load newly built index
                if let Ok(index_path) = state.config.search.index_path() {
                    match TantivySearch::new(&index_path, state.config.search.clone()) {
                        Ok(backend) => {
                            if state.update_fts_backend(backend).is_ok() {
                                state.set_fts_ready(true);
                                log::info!("FTS backend now active");
                            } else {
                                log::error!("Failed to update FTS backend");
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to load newly built index: {}", e);
                        }
                    }
                } else {
                    log::error!("Failed to resolve index path after build");
                }
            }
            Err(e) => {
                log::error!("Background indexing failed: {}", e);
                // Simple backend remains active
            }
        }
    });
}

/// Build FTS index for the given state (module-level wrapper).
#[cfg(feature = "fts")]
async fn build_fts_index_for_state(state: &AppState) -> Result<crate::search::IndexStats> {
    build_index(&state.config).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{PathsConfig, SearchConfig, ServerConfig, SourcesConfig};

    fn test_config(backend: &str) -> Config {
        Config {
            server: ServerConfig {
                name: "test".to_string(),
                version: "0.1.0".to_string(),
            },
            paths: PathsConfig {
                base: ".".to_string(),
                sources_md: "sources-md".to_string(),
                concept_cards: "concept-cards".to_string(),
                concepts_unified: "concepts-unified".to_string(),
                guides: "guides".to_string(),
                skill_docs: ".".to_string(),
            },
            sources: SourcesConfig::default(),
            logging: twyg::OptsBuilder::new()
                .level(twyg::LogLevel::Info)
                .coloured(true)
                .output(twyg::Output::Stderr)
                .report_caller(false)
                .build()
                .unwrap(),
            search: SearchConfig {
                backend: backend.to_string(),
                index_path: ".tantivy-index-test".to_string(),
                rebuild_on_startup: false,
                snippet_size: 200,
                fuzzy_search: false,
                fuzzy_distance: 2,
                query_mode: crate::config::QueryMode::Smart,
                minimum_match_percent: 0.6,
                enable_stopwords: true,
                custom_stopwords: vec![],
                stopword_allowlist: vec![],
                field_boost_title: 3.0,
                field_boost_description: 2.0,
                field_boost_content: 1.0,
            },
        }
    }

    #[tokio::test]
    async fn test_appstate_new_simple() {
        let config = test_config("simple");
        let result = AppState::new(config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_appstate_clone() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        let _cloned = state.clone();
        // Should not panic
    }

    #[tokio::test]
    async fn test_search_backend_returns_simple() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        let _backend = state.search_backend();
        // Should return simple backend (can't easily test type, but shouldn't panic)
        // Getting the backend is sufficient to verify it works
    }

    #[tokio::test]
    async fn test_active_backend_name_simple() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        assert_eq!(state.active_backend_name(), "simple");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_appstate_fts_not_ready_initially() {
        let config = test_config("tantivy");
        let state = AppState::new(config).await.expect("Failed to create state");
        // Without existing index, FTS should not be ready
        assert!(!state.is_fts_ready());
        assert_eq!(state.active_backend_name(), "simple");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_set_fts_ready() {
        let config = test_config("tantivy");
        let state = AppState::new(config).await.expect("Failed to create state");

        assert!(!state.is_fts_ready());

        state.set_fts_ready(true);
        assert!(state.is_fts_ready());

        state.set_fts_ready(false);
        assert!(!state.is_fts_ready());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_update_fts_backend() {
        use crate::search::TantivySearch;
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        // Build a minimal index for testing
        fs::create_dir_all(&index_path).expect("Failed to create index dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        // Create a test concept card
        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        // Build index
        let mut config = test_config("tantivy");
        config.search.index_path = index_path.to_string_lossy().to_string();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        crate::search::build_index(&config)
            .await
            .expect("Failed to build index");

        // Create state
        let state = AppState::new(config.clone())
            .await
            .expect("Failed to create state");

        // Load the index we just built
        let backend =
            TantivySearch::new(&index_path, config.search.clone()).expect("Failed to load index");

        // Update backend
        let result = state.update_fts_backend(backend);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_search_backend_with_fts_ready() {
        use crate::search::TantivySearch;
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        // Build a minimal index
        fs::create_dir_all(&index_path).expect("Failed to create index dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        let mut config = test_config("tantivy");
        config.search.index_path = index_path.to_string_lossy().to_string();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        crate::search::build_index(&config)
            .await
            .expect("Failed to build index");

        let state = AppState::new(config.clone())
            .await
            .expect("Failed to create state");

        // Load and set backend
        let backend =
            TantivySearch::new(&index_path, config.search.clone()).expect("Failed to load index");
        state.update_fts_backend(backend).expect("Failed to update");
        state.set_fts_ready(true);

        // Now search_backend should return FTS backend
        let _backend = state.search_backend();
        assert_eq!(state.active_backend_name(), "tantivy");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_active_backend_name_with_fts() {
        let config = test_config("tantivy");
        let state = AppState::new(config).await.expect("Failed to create state");

        // Initially simple
        assert_eq!(state.active_backend_name(), "simple");

        // After marking ready
        state.set_fts_ready(true);
        assert_eq!(state.active_backend_name(), "tantivy");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_initialize_fts_with_simple_backend() {
        let config = test_config("simple");
        let state = Arc::new(AppState::new(config).await.expect("Failed to create state"));

        // Should succeed without doing anything
        let result = initialize_fts(&state).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_initialize_fts_with_nonexistent_index() {
        use std::env;

        let mut config = test_config("tantivy");
        // Use guaranteed non-existent path
        let nonexistent_path = env::temp_dir().join(format!("nonexistent-{}", std::process::id()));
        config.search.index_path = nonexistent_path.to_string_lossy().to_string();

        let state = Arc::new(AppState::new(config).await.expect("Failed to create state"));

        // Should succeed and start background indexing (though it will fail)
        let result = initialize_fts(&state).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_index_exists_and_fresh_nonexistent() {
        use std::env;

        let nonexistent_path = env::temp_dir().join(format!("nonexistent-{}", std::process::id()));
        let config = test_config("tantivy");

        let result = index_exists_and_fresh(&nonexistent_path, &config).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_index_exists_and_fresh_stale() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");
        let concept_cards_path = temp_dir.path().join("concept-cards");

        fs::create_dir_all(&index_path).expect("Failed to create index dir");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        // Create old metadata to make index appear stale
        let metadata = crate::search::freshness::IndexMetadata {
            doc_count: 1,
            last_indexed: std::time::SystemTime::now(),
            content_hash: "old-hash".to_string(),
        };
        crate::search::freshness::save_metadata(&index_path, &metadata)
            .await
            .expect("Failed to save metadata");

        // Create a new file to make content hash different
        let card_content = r#"---
title: New Card
category: test
---

# New Card

New content.
"#;
        fs::write(concept_cards_path.join("new.md"), card_content).expect("Failed to write card");

        let mut config = test_config("tantivy");
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let result = index_exists_and_fresh(&index_path, &config).await;
        assert!(result.is_ok());
        // Should be false because content hash doesn't match
        assert!(!result.unwrap());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_index_exists_and_fresh_error_handling() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        // Create index dir but with invalid metadata
        std::fs::create_dir_all(&index_path).expect("Failed to create index dir");
        std::fs::write(index_path.join("metadata.json"), "invalid json")
            .expect("Failed to write invalid metadata");

        let config = test_config("tantivy");

        // Should handle error gracefully and return false
        let result = index_exists_and_fresh(&index_path, &config).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_initialize_fts_backend_with_simple() {
        let config = test_config("simple");
        let result = initialize_fts_backend(&config);
        assert!(result.is_ok());

        let (backend, ready) = result.unwrap();
        assert!(backend.read().unwrap().is_none());
        assert!(!ready.load(Ordering::Acquire));
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_initialize_fts_backend_no_index() {
        use std::env;

        let mut config = test_config("tantivy");
        let nonexistent_path = env::temp_dir().join(format!("nonexistent-{}", std::process::id()));
        config.search.index_path = nonexistent_path.to_string_lossy().to_string();

        let result = initialize_fts_backend(&config);
        assert!(result.is_ok());

        let (backend, ready) = result.unwrap();
        assert!(backend.read().unwrap().is_none());
        assert!(!ready.load(Ordering::Acquire));
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_build_fts_index_for_state() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");
        let concept_cards_path = temp_dir.path().join("concept-cards");

        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        let mut config = test_config("tantivy");
        config.search.index_path = index_path.to_string_lossy().to_string();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let state = AppState::new(config).await.expect("Failed to create state");

        let result = build_fts_index_for_state(&state).await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_eq!(stats.indexed, 1);
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_initialize_fts_with_fresh_index() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");
        let concept_cards_path = temp_dir.path().join("concept-cards");

        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        let mut config = test_config("tantivy");
        config.search.index_path = index_path.to_string_lossy().to_string();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        // Build index first
        crate::search::build_index(&config)
            .await
            .expect("Failed to build index");

        // Create state (should load existing index)
        let state = Arc::new(AppState::new(config).await.expect("Failed to create state"));

        // Initialize FTS - should detect fresh index and not start background indexing
        let result = initialize_fts(&state).await;
        assert!(result.is_ok());
        assert!(state.is_fts_ready());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_index_exists_and_fresh_with_fresh_index() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");
        let concept_cards_path = temp_dir.path().join("concept-cards");

        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        let mut config = test_config("tantivy");
        config.search.index_path = index_path.to_string_lossy().to_string();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        // Build index
        crate::search::build_index(&config)
            .await
            .expect("Failed to build index");

        // Check freshness - should be true
        let result = index_exists_and_fresh(&index_path, &config).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_start_background_indexing_success() {
        use std::fs;
        use tempfile::TempDir;
        use tokio::time::{sleep, Duration};

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");
        let concept_cards_path = temp_dir.path().join("concept-cards");

        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        let mut config = test_config("tantivy");
        config.search.index_path = index_path.to_string_lossy().to_string();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let state = Arc::new(AppState::new(config).await.expect("Failed to create state"));

        // Manually call start_background_indexing
        start_background_indexing(Arc::clone(&state));

        // Give it time to complete
        sleep(Duration::from_secs(2)).await;

        // Check if FTS became ready
        assert!(state.is_fts_ready());
        assert_eq!(state.active_backend_name(), "tantivy");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_start_background_indexing_failure() {
        use std::env;
        use tempfile::TempDir;
        use tokio::time::{sleep, Duration};

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        let mut config = test_config("tantivy");
        config.search.index_path = index_path.to_string_lossy().to_string();
        // Use nonexistent concept cards path to cause indexing failure
        let nonexistent_path =
            env::temp_dir().join(format!("nonexistent-cards-{}", std::process::id()));
        config.paths.concept_cards = nonexistent_path.to_string_lossy().to_string();

        let state = Arc::new(AppState::new(config).await.expect("Failed to create state"));

        // Verify initial state
        assert!(!state.is_fts_ready());

        // Manually call start_background_indexing (will fail due to missing concept cards)
        start_background_indexing(Arc::clone(&state));

        // Give it time to attempt and fail
        sleep(Duration::from_millis(500)).await;

        // Should remain not ready since indexing failed
        assert!(!state.is_fts_ready());
        assert_eq!(state.active_backend_name(), "simple");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_search_backend_read_lock_fallback() {
        // This tests the fallback path when fts_backend.read() succeeds but backend is None
        let config = test_config("tantivy");
        let state = AppState::new(config).await.expect("Failed to create state");

        // Set ready but don't actually set a backend
        state.set_fts_ready(true);

        // Should fall back to simple backend because backend is None
        let backend = state.search_backend();
        // Can't easily test the type, but it shouldn't panic
        drop(backend);
    }
}
