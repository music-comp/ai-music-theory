//! Application state management.
//!
//! This module provides AppState for managing the search backend state,
//! including FTS readiness tracking and dynamic backend switching.
//! Service lifecycle is tracked via `fabryk::core::ServiceHandle`.

use std::sync::Arc;

#[cfg(feature = "fts")]
use std::path::Path;

#[cfg(feature = "fts")]
use std::sync::RwLock as StdRwLock;

#[cfg(any(feature = "graph", feature = "vector"))]
use std::sync::RwLock;

#[cfg(any(feature = "fts", feature = "graph", feature = "vector"))]
use fabryk::core::{ServiceHandle, ServiceState};

use crate::config::Config;
use crate::error::Result;
use crate::search::SearchBackend;
use crate::search::SimpleSearch;

#[cfg(feature = "fts")]
use crate::search::{build_index, is_index_fresh, TantivySearch};

/// Type alias for FTS backend initialization return type.
#[cfg(feature = "fts")]
type FtsBackendInit = (Arc<StdRwLock<Option<Arc<TantivySearch>>>>, ServiceHandle);

/// Shared application state.
///
/// Manages search backends and FTS readiness. Cloneable for sharing across
/// request handlers via Arc-wrapped internals.
///
/// Service lifecycle is tracked via [`ServiceHandle`] instances rather than
/// raw `AtomicBool` / enum state. Each service handle provides state
/// observation, subscription, and audit-trail capabilities.
#[derive(Clone)]
pub struct AppState {
    /// Configuration
    pub config: Config,

    /// Simple search backend (always available)
    simple_backend: Arc<SimpleSearch>,

    /// FTS backend (optional, may be None initially)
    #[cfg(feature = "fts")]
    fts_backend: Arc<StdRwLock<Option<Arc<TantivySearch>>>>,

    /// FTS service lifecycle handle
    #[cfg(feature = "fts")]
    pub fts_service: ServiceHandle,

    /// Graph service lifecycle handle
    #[cfg(feature = "graph")]
    pub graph_service: ServiceHandle,

    /// Graph data (populated when `graph_service` reaches `Ready`)
    #[cfg(feature = "graph")]
    pub graph_data: Arc<RwLock<Option<crate::graph::LoadedGraph>>>,

    /// Vector service lifecycle handle
    #[cfg(feature = "vector")]
    pub vector_service: ServiceHandle,

    /// Vector backend (populated when `vector_service` reaches `Ready`)
    #[cfg(feature = "vector")]
    pub vector_backend: Arc<RwLock<Option<Arc<dyn fabryk::vector::VectorBackend>>>>,
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
        let (fts_backend, fts_service) = initialize_fts_backend(&config)?;

        Ok(Self {
            config,
            simple_backend,
            #[cfg(feature = "fts")]
            fts_backend,
            #[cfg(feature = "fts")]
            fts_service,
            #[cfg(feature = "graph")]
            graph_service: ServiceHandle::new("graph"),
            #[cfg(feature = "graph")]
            graph_data: Arc::new(RwLock::new(None)),
            #[cfg(feature = "vector")]
            vector_service: ServiceHandle::new("vector"),
            #[cfg(feature = "vector")]
            vector_backend: Arc::new(RwLock::new(None)),
        })
    }

    /// Get the currently active search backend.
    ///
    /// Returns FTS backend if ready, otherwise simple backend.
    /// This allows transparent failover during FTS initialization.
    ///
    /// # Returns
    ///
    /// Returns Arc-wrapped `fabryk::fts::SearchBackend` for shared ownership
    /// across requests.
    pub fn search_backend(&self) -> Arc<dyn SearchBackend + Send + Sync> {
        #[cfg(feature = "fts")]
        if self.fts_service.state().is_ready() {
            if let Ok(guard) = self.fts_backend.read() {
                if let Some(ref backend) = *guard {
                    return Arc::clone(backend) as Arc<dyn SearchBackend + Send + Sync>;
                }
            }
        }

        Arc::clone(&self.simple_backend) as Arc<dyn SearchBackend + Send + Sync>
    }

    /// Get the name of the currently active backend.
    ///
    /// # Returns
    ///
    /// Returns "tantivy" if FTS is ready, otherwise "simple".
    pub fn active_backend_name(&self) -> &'static str {
        #[cfg(feature = "fts")]
        if self.fts_service.state().is_ready() {
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
        self.fts_service.state().is_ready()
    }

    /// Mark FTS as ready or not ready (internal use).
    ///
    /// Used by background indexing to signal when FTS becomes available.
    #[cfg(feature = "fts")]
    pub(crate) fn set_fts_ready(&self, ready: bool) {
        if ready {
            self.fts_service.set_state(ServiceState::Ready);
        } else {
            self.fts_service.set_state(ServiceState::Stopped);
        }
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

    /// Acquire the loaded graph data, returning an appropriate error if the
    /// graph service is not in the `Ready` state.
    ///
    /// Callers receive a `RwLockReadGuard` that borrows `Option<LoadedGraph>`.
    /// When the service is `Ready` the `Option` is guaranteed to be `Some`.
    ///
    /// # Errors
    ///
    /// Returns an error describing the current service state (not loaded,
    /// loading, or failed).
    #[cfg(feature = "graph")]
    pub fn require_graph(
        &self,
    ) -> Result<std::sync::RwLockReadGuard<'_, Option<crate::graph::LoadedGraph>>> {
        let svc_state = self.graph_service.state();
        if !svc_state.is_ready() {
            return match svc_state {
                ServiceState::Starting => Err(crate::error::Error::not_found_msg(
                    "Graph is currently loading",
                )),
                ServiceState::Failed(msg) => Err(crate::error::Error::config(format!(
                    "Graph failed to load: {msg}"
                ))),
                _ => Err(crate::error::Error::not_found_msg("Graph not loaded yet")),
            };
        }
        let guard = self.graph_data.read().unwrap();
        Ok(guard)
    }

    /// Check if the vector index is ready for queries.
    ///
    /// # Returns
    ///
    /// Returns true if the vector service is in the `Ready` state.
    #[cfg(feature = "vector")]
    pub fn is_vector_ready(&self) -> bool {
        self.vector_service.state().is_ready()
    }

    /// Mark the vector service as ready or stopped.
    ///
    /// Used by background vector building to signal when the index
    /// becomes available.
    #[cfg(feature = "vector")]
    pub(crate) fn set_vector_ready(&self, ready: bool) {
        if ready {
            self.vector_service.set_state(ServiceState::Ready);
        } else {
            self.vector_service.set_state(ServiceState::Stopped);
        }
    }

    /// Update the vector backend after background index building completes.
    ///
    /// # Arguments
    ///
    /// * `backend` - The newly built vector backend
    ///
    /// # Errors
    ///
    /// Returns `Err` if the write lock cannot be acquired.
    #[cfg(feature = "vector")]
    pub(crate) fn update_vector_backend(
        &self,
        backend: Arc<dyn fabryk::vector::VectorBackend>,
    ) -> Result<()> {
        let mut guard = self.vector_backend.write().map_err(|_| {
            crate::error::Error::config("Failed to acquire vector write lock".to_string())
        })?;
        *guard = Some(backend);
        Ok(())
    }

    /// Acquire a read-guard on the vector backend, returning an appropriate
    /// error if the vector service is not in the `Ready` state.
    ///
    /// # Errors
    ///
    /// Returns an error describing the current service state (not built,
    /// building, or failed).
    #[cfg(feature = "vector")]
    pub fn require_vector(
        &self,
    ) -> Result<std::sync::RwLockReadGuard<'_, Option<Arc<dyn fabryk::vector::VectorBackend>>>>
    {
        let svc_state = self.vector_service.state();
        if !svc_state.is_ready() {
            return match svc_state {
                ServiceState::Starting => Err(crate::error::Error::not_found_msg(
                    "Vector index is currently building",
                )),
                ServiceState::Failed(msg) => Err(crate::error::Error::config(format!(
                    "Vector index failed to build: {msg}"
                ))),
                _ => Err(crate::error::Error::not_found_msg(
                    "Vector index not built yet",
                )),
            };
        }
        let guard = self.vector_backend.read().unwrap();
        Ok(guard)
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
/// Returns tuple of (fts_backend, fts_service) where backend may be None
/// if index needs to be built.
#[cfg(feature = "fts")]
fn initialize_fts_backend(config: &Config) -> Result<FtsBackendInit> {
    let fts_service = ServiceHandle::new("fts");

    if config.search.backend == "tantivy" {
        let fabryk_config = crate::search::to_fabryk_search_config(&config.search);

        // Try to load existing index
        match TantivySearch::new(&fabryk_config) {
            Ok(backend) => {
                // Index exists and is loadable
                log::info!("Loaded existing FTS index from disk");
                fts_service.set_state(ServiceState::Ready);
                Ok((
                    Arc::new(StdRwLock::new(Some(Arc::new(backend)))),
                    fts_service,
                ))
            }
            Err(e) => {
                // Index doesn't exist yet - will build in background (Phase 3)
                log::debug!("FTS index not found: {} (will build if configured)", e);
                Ok((Arc::new(StdRwLock::new(None)), fts_service))
            }
        }
    } else {
        // FTS not configured
        Ok((Arc::new(StdRwLock::new(None)), fts_service))
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

/// Initialize graph backend and start async loading.
///
/// # Arguments
///
/// * `state` - Shared application state
///
/// # Errors
///
/// Returns `Err` if data path resolution fails.
#[cfg(feature = "graph")]
pub async fn initialize_graph(state: &Arc<AppState>) -> Result<()> {
    let data_dir = state.config.paths.base_path()?.join("data");
    let graph_path = data_dir.join("graphs").join("concept_graph.json");

    if !graph_path.exists() {
        log::info!(
            "Concept graph not found at {}. Run `music-theory-mcp graph build` to create it.",
            graph_path.display()
        );
        return Ok(());
    }

    log::info!("Starting async graph load");
    start_graph_loading(Arc::clone(state), data_dir);
    Ok(())
}

/// Start async graph loading task.
#[cfg(feature = "graph")]
fn start_graph_loading(state: Arc<AppState>, data_dir: std::path::PathBuf) {
    tokio::spawn(async move {
        // Update state to Starting
        state.graph_service.set_state(ServiceState::Starting);

        log::info!("Loading concept graph");

        match crate::graph::load_concept_graph(&data_dir).await {
            Ok(loaded) => {
                log::info!(
                    "Concept graph loaded: {} nodes, {} edges ({} concepts, {} sources)",
                    loaded.stats.node_count,
                    loaded.stats.edge_count,
                    loaded.stats.concept_count,
                    loaded.stats.source_count
                );

                // Store graph data and mark service ready
                {
                    let mut guard = state.graph_data.write().unwrap();
                    *guard = Some(loaded);
                }
                state.graph_service.set_state(ServiceState::Ready);
            }
            Err(e) => {
                log::error!("Failed to load concept graph: {}", e);
                state
                    .graph_service
                    .set_state(ServiceState::Failed(e.to_string()));
            }
        }
    });
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
                let fabryk_config = crate::search::to_fabryk_search_config(&state.config.search);
                match TantivySearch::new(&fabryk_config) {
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
            }
            Err(e) => {
                log::warn!("Background indexing failed (graceful degradation): {}", e);
                // Mark FTS as ready anyway — the simple backend fallback will
                // handle searches. This ensures the server doesn't permanently
                // report FTS as unavailable when content paths are missing.
                state.set_fts_ready(true);
                log::info!("FTS marked ready with simple backend fallback");
            }
        }
    });
}

/// Build FTS index for the given state (module-level wrapper).
#[cfg(feature = "fts")]
async fn build_fts_index_for_state(state: &AppState) -> Result<crate::search::IndexStats> {
    build_index(&state.config).await
}

/// Initialize the vector search backend and start background index building.
///
/// Resolves the cache directory from the configured base path and spawns
/// an async task that discovers content, embeds it, and populates the
/// in-memory vector backend.
///
/// # Arguments
///
/// * `state` - Shared application state
///
/// # Errors
///
/// Returns `Err` if the base path cannot be resolved.
#[cfg(feature = "vector")]
pub async fn initialize_vector(state: &Arc<AppState>) -> Result<()> {
    let base = state.config.paths.base_path()?;
    let cache_dir = base.join(".cache").join("vector");

    log::info!("Starting background vector index build");
    start_vector_building(Arc::clone(state), cache_dir);
    Ok(())
}

/// Spawn an async task that builds the vector index and stores the result.
#[cfg(feature = "vector")]
fn start_vector_building(state: Arc<AppState>, cache_dir: std::path::PathBuf) {
    tokio::spawn(async move {
        state.vector_service.set_state(ServiceState::Starting);

        match build_vector_index(&state.config, &cache_dir).await {
            Ok(backend) => {
                use fabryk::vector::VectorBackend;
                let doc_count = backend.document_count().unwrap_or(0);
                let backend_arc: Arc<dyn fabryk::vector::VectorBackend> = Arc::new(backend);
                if state.update_vector_backend(backend_arc).is_ok() {
                    state.set_vector_ready(true);
                    log::info!(documents = doc_count; "Vector index ready");
                } else {
                    log::error!("Failed to store vector backend");
                    state
                        .vector_service
                        .set_state(ServiceState::Failed("Failed to store backend".to_string()));
                }
            }
            Err(e) => {
                log::warn!("Vector index build failed (graceful degradation): {}", e);
                state
                    .vector_service
                    .set_state(ServiceState::Failed(e.to_string()));
            }
        }
    });
}

/// Build the vector index from all configured content directories.
///
/// Iterates over concept cards, source documents, unified concepts, and
/// guides directories. The first directory that exists seeds the backend;
/// subsequent directories append into it. Returns an error only when no
/// content directories exist at all.
#[cfg(feature = "vector")]
async fn build_vector_index(
    config: &Config,
    cache_dir: &std::path::Path,
) -> std::result::Result<
    fabryk::vector::SimpleVectorBackend,
    Box<dyn std::error::Error + Send + Sync>,
> {
    use crate::extractors::MusicTheoryVectorExtractor;
    use fabryk::vector::builder::ErrorHandling;
    use fabryk::vector::{EmbeddingProvider, FastEmbedProvider, VectorIndexBuilder};

    // Ensure cache directory exists
    tokio::fs::create_dir_all(cache_dir).await?;
    let cache_file = cache_dir.join("vector-cache.json");

    // Create embedding provider
    let provider: Arc<dyn EmbeddingProvider> =
        Arc::new(FastEmbedProvider::new("bge-small-en-v1.5", None)?);

    // Resolve content directories
    let base = config.paths.base_path().map_err(|e| e.to_string())?;
    let content_dirs = [
        (base.join(&config.paths.concept_cards), "concept_cards"),
        (base.join(&config.paths.sources_md), "sources_md"),
        (
            base.join(&config.paths.concepts_unified),
            "concepts_unified",
        ),
        (base.join(&config.paths.guides), "guides"),
    ];

    let mut backend: Option<fabryk::vector::SimpleVectorBackend> = None;
    let mut total_docs = 0usize;

    for (content_path, label) in &content_dirs {
        if !content_path.exists() {
            log::debug!("Vector: skipping {} (not found)", label);
            continue;
        }

        let extractor = MusicTheoryVectorExtractor::new();
        let builder = VectorIndexBuilder::new(extractor)
            .with_content_path(content_path)
            .with_embedding_provider(Arc::clone(&provider))
            .with_error_handling(ErrorHandling::Skip);

        match &mut backend {
            None => {
                // First directory: build with cache
                let b = builder.with_cache_path(&cache_file);
                let (new_backend, stats) = b.build().await?;
                log::info!(
                    "Vector: indexed {} from {} ({} errors)",
                    stats.documents_indexed,
                    label,
                    stats.errors.len()
                );
                total_docs += stats.documents_indexed;
                backend = Some(new_backend);
            }
            Some(ref mut existing) => {
                // Subsequent directories: append
                let stats = builder.build_append(existing).await?;
                log::info!(
                    "Vector: indexed {} from {} ({} errors)",
                    stats.documents_indexed,
                    label,
                    stats.errors.len()
                );
                total_docs += stats.documents_indexed;
            }
        }
    }

    match backend {
        Some(b) => {
            log::info!("Vector index complete: {} total documents", total_docs);
            Ok(b)
        }
        None => Err("No content directories found for vector indexing".into()),
    }
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
    #[cfg(feature = "vector")]
    async fn test_vector_service_initial_state() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        assert_eq!(state.vector_service.state(), ServiceState::Stopped);
        assert_eq!(state.vector_service.name(), "vector");
        assert!(state.vector_backend.read().unwrap().is_none());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_appstate_fts_not_ready_initially() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut config = test_config("tantivy");
        // Use temp directory to ensure no existing index
        config.search.index_path = temp_dir
            .path()
            .join(".tantivy-index")
            .to_string_lossy()
            .to_string();

        let state = AppState::new(config).await.expect("Failed to create state");
        // Without existing index, FTS should not be ready
        assert!(!state.is_fts_ready());
        assert_eq!(state.fts_service.state(), ServiceState::Stopped);
        assert_eq!(state.active_backend_name(), "simple");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_set_fts_ready() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut config = test_config("tantivy");
        // Use temp directory to ensure no existing index
        config.search.index_path = temp_dir
            .path()
            .join(".tantivy-index")
            .to_string_lossy()
            .to_string();

        let state = AppState::new(config).await.expect("Failed to create state");

        assert!(!state.is_fts_ready());

        state.set_fts_ready(true);
        assert!(state.is_fts_ready());
        assert_eq!(state.fts_service.state(), ServiceState::Ready);

        state.set_fts_ready(false);
        assert!(!state.is_fts_ready());
        assert_eq!(state.fts_service.state(), ServiceState::Stopped);
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
        let fabryk_config = crate::search::to_fabryk_search_config(&config.search);
        let backend = TantivySearch::new(&fabryk_config).expect("Failed to load index");

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
        let fabryk_config = crate::search::to_fabryk_search_config(&config.search);
        let backend = TantivySearch::new(&fabryk_config).expect("Failed to load index");
        state.update_fts_backend(backend).expect("Failed to update");
        state.set_fts_ready(true);

        // Now search_backend should return FTS backend
        let _backend = state.search_backend();
        assert_eq!(state.active_backend_name(), "tantivy");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_active_backend_name_with_fts() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut config = test_config("tantivy");
        // Use temp directory to ensure no existing index
        config.search.index_path = temp_dir
            .path()
            .join(".tantivy-index")
            .to_string_lossy()
            .to_string();

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
        let metadata = fabryk::fts::IndexMetadata::new("old-hash".to_string(), 1);
        metadata.save(&index_path).expect("Failed to save metadata");

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
        std::fs::write(index_path.join("fabryk-fts-metadata.json"), "invalid json")
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

        let (backend, fts_service) = result.unwrap();
        assert!(backend.read().unwrap().is_none());
        assert!(!fts_service.state().is_ready());
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

        let (backend, fts_service) = result.unwrap();
        assert!(backend.read().unwrap().is_none());
        assert!(!fts_service.state().is_ready());
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
        // Override other paths to point to temp directory (v0.3.0: prevent indexing real content)
        config.paths.sources_md = temp_dir
            .path()
            .join("sources-md")
            .to_string_lossy()
            .to_string();
        config.paths.concepts_unified = temp_dir
            .path()
            .join("concepts-unified")
            .to_string_lossy()
            .to_string();
        config.paths.guides = temp_dir.path().join("guides").to_string_lossy().to_string();

        let state = AppState::new(config).await.expect("Failed to create state");

        let result = build_fts_index_for_state(&state).await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        // v0.3.0: Check total indexed and per-type counts
        assert_eq!(stats.indexed, 1);
        assert_eq!(stats.concept_cards, 1);
        assert_eq!(stats.source_chapters, 0);
        assert_eq!(stats.unified_concepts, 0);
        assert_eq!(stats.guides, 0);
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
        // Set other paths to temp dir to avoid indexing real project files
        config.paths.sources_md = temp_dir
            .path()
            .join("sources-md")
            .to_string_lossy()
            .to_string();
        config.paths.concepts_unified = temp_dir
            .path()
            .join("concepts-unified")
            .to_string_lossy()
            .to_string();
        config.paths.guides = temp_dir.path().join("guides").to_string_lossy().to_string();

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
        // Set other paths to temp dir to avoid indexing real project files
        config.paths.sources_md = temp_dir
            .path()
            .join("sources-md")
            .to_string_lossy()
            .to_string();
        config.paths.concepts_unified = temp_dir
            .path()
            .join("concepts-unified")
            .to_string_lossy()
            .to_string();
        config.paths.guides = temp_dir.path().join("guides").to_string_lossy().to_string();

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
    async fn test_start_background_indexing_empty_directory() {
        use std::env;
        use tempfile::TempDir;
        use tokio::time::{sleep, Duration};

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        let mut config = test_config("tantivy");
        config.search.index_path = index_path.to_string_lossy().to_string();
        // Use nonexistent concept cards path - graceful degradation should handle this
        let nonexistent_path =
            env::temp_dir().join(format!("nonexistent-cards-{}", std::process::id()));
        config.paths.concept_cards = nonexistent_path.to_string_lossy().to_string();

        let state = Arc::new(AppState::new(config).await.expect("Failed to create state"));

        // Verify initial state
        assert!(!state.is_fts_ready());

        // Manually call start_background_indexing (will succeed with 0 documents due to graceful degradation)
        start_background_indexing(Arc::clone(&state));

        // Poll for FTS to become ready (with timeout for CI environments)
        let mut attempts = 0;
        while !state.is_fts_ready() && attempts < 40 {
            sleep(Duration::from_millis(100)).await;
            attempts += 1;
        }

        // Should become ready even with no documents (graceful degradation feature)
        assert!(
            state.is_fts_ready(),
            "FTS should be ready after indexing empty directories (waited {} ms)",
            attempts * 100
        );
        assert_eq!(state.active_backend_name(), "tantivy");
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

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_fts_service_handle_name() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        assert_eq!(state.fts_service.name(), "fts");
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_graph_service_initial_state() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        assert_eq!(state.graph_service.state(), ServiceState::Stopped);
        assert_eq!(state.graph_service.name(), "graph");
        assert!(state.graph_data.read().unwrap().is_none());
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_require_graph_not_loaded() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        let result = state.require_graph();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not loaded"));
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_require_graph_loading() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state.graph_service.set_state(ServiceState::Starting);
        let result = state.require_graph();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("loading"));
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_require_graph_failed() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state
            .graph_service
            .set_state(ServiceState::Failed("disk full".to_string()));
        let result = state.require_graph();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("disk full"));
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_require_graph_ready_returns_guard() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state.graph_service.set_state(ServiceState::Ready);
        let result = state.require_graph();
        assert!(result.is_ok());
        // Guard should hold None since no graph data was actually loaded
        let guard = result.unwrap();
        assert!(guard.is_none());
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_initialize_graph_no_graph_file() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut config = test_config("simple");
        // Point base path to temp dir which has no graph file
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let state = Arc::new(AppState::new(config).await.expect("Failed to create state"));

        // Should succeed without starting any loading
        let result = initialize_graph(&state).await;
        assert!(result.is_ok());
        // Graph service should still be stopped since no graph file exists
        assert_eq!(state.graph_service.state(), ServiceState::Stopped);
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_initialize_graph_with_invalid_graph_file() {
        use std::fs;
        use tempfile::TempDir;
        use tokio::time::{sleep, Duration};

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let data_dir = temp_dir.path().join("data");
        let graphs_dir = data_dir.join("graphs");
        fs::create_dir_all(&graphs_dir).expect("Failed to create graphs dir");

        // Write an invalid graph file to trigger the error path in start_graph_loading
        fs::write(graphs_dir.join("concept_graph.json"), "invalid json")
            .expect("Failed to write graph file");

        let mut config = test_config("simple");
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let state = Arc::new(AppState::new(config).await.expect("Failed to create state"));

        let result = initialize_graph(&state).await;
        assert!(result.is_ok());

        // Wait for async graph loading to complete (or fail)
        let mut attempts = 0;
        while state.graph_service.state() == ServiceState::Stopped && attempts < 20 {
            sleep(Duration::from_millis(50)).await;
            attempts += 1;
        }
        // Allow additional time for the failure to be recorded
        sleep(Duration::from_millis(200)).await;

        // The graph service should be in Failed state due to invalid JSON
        let svc_state = state.graph_service.state();
        assert!(
            matches!(svc_state, ServiceState::Failed(_)),
            "Expected Failed state, got {:?}",
            svc_state
        );
    }

    #[tokio::test]
    #[cfg(feature = "vector")]
    async fn test_is_vector_ready() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        assert!(!state.is_vector_ready());
        state.set_vector_ready(true);
        assert!(state.is_vector_ready());
        state.set_vector_ready(false);
        assert!(!state.is_vector_ready());
    }

    #[tokio::test]
    #[cfg(feature = "vector")]
    async fn test_require_vector_not_ready() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        let result = state.require_vector();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("not built"));
    }

    #[tokio::test]
    #[cfg(feature = "vector")]
    async fn test_require_vector_loading() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state.vector_service.set_state(ServiceState::Starting);
        let result = state.require_vector();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("building"));
    }

    #[tokio::test]
    #[cfg(feature = "vector")]
    async fn test_require_vector_failed() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state
            .vector_service
            .set_state(ServiceState::Failed("out of memory".to_string()));
        let result = state.require_vector();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("out of memory"));
    }

    #[tokio::test]
    #[cfg(feature = "vector")]
    async fn test_require_vector_ready_returns_guard() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state.set_vector_ready(true);
        let result = state.require_vector();
        assert!(result.is_ok());
        // Guard should hold None since no vector backend was actually loaded
        let guard = result.unwrap();
        assert!(guard.is_none());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_appstate_new_tantivy_with_existing_index() {
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
        config.paths.sources_md = temp_dir
            .path()
            .join("sources-md")
            .to_string_lossy()
            .to_string();
        config.paths.concepts_unified = temp_dir
            .path()
            .join("concepts-unified")
            .to_string_lossy()
            .to_string();
        config.paths.guides = temp_dir.path().join("guides").to_string_lossy().to_string();

        // Build index first
        crate::search::build_index(&config)
            .await
            .expect("Failed to build index");

        // Now create AppState - should detect and load existing index
        let state = AppState::new(config).await.expect("Failed to create state");
        assert!(state.is_fts_ready());
        assert_eq!(state.active_backend_name(), "tantivy");

        // search_backend should return the FTS backend
        let _backend = state.search_backend();
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_initialize_fts_backend_with_existing_index() {
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
        config.paths.sources_md = temp_dir
            .path()
            .join("sources-md")
            .to_string_lossy()
            .to_string();
        config.paths.concepts_unified = temp_dir
            .path()
            .join("concepts-unified")
            .to_string_lossy()
            .to_string();
        config.paths.guides = temp_dir.path().join("guides").to_string_lossy().to_string();

        // Build index first
        crate::search::build_index(&config)
            .await
            .expect("Failed to build index");

        // initialize_fts_backend should load the existing index
        let (backend, fts_service) =
            initialize_fts_backend(&config).expect("Failed to init backend");
        assert!(fts_service.state().is_ready());
        assert!(backend.read().unwrap().is_some());
    }

    #[tokio::test]
    async fn test_appstate_config_accessible() {
        let config = test_config("simple");
        let expected_name = config.server.name.clone();
        let state = AppState::new(config).await.expect("Failed to create state");
        assert_eq!(state.config.server.name, expected_name);
    }

    #[tokio::test]
    async fn test_appstate_simple_backend_multiple_calls() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");

        // Calling search_backend multiple times should always work
        let _b1 = state.search_backend();
        let _b2 = state.search_backend();
        let _b3 = state.search_backend();
        assert_eq!(state.active_backend_name(), "simple");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_set_fts_ready_toggle_multiple_times() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut config = test_config("tantivy");
        config.search.index_path = temp_dir
            .path()
            .join(".tantivy-index")
            .to_string_lossy()
            .to_string();

        let state = AppState::new(config).await.expect("Failed to create state");

        // Toggle multiple times to exercise both branches repeatedly
        for _ in 0..3 {
            state.set_fts_ready(true);
            assert!(state.is_fts_ready());
            assert_eq!(state.active_backend_name(), "tantivy");

            state.set_fts_ready(false);
            assert!(!state.is_fts_ready());
            assert_eq!(state.active_backend_name(), "simple");
        }
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_fts_service_state_transitions() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut config = test_config("tantivy");
        config.search.index_path = temp_dir
            .path()
            .join(".tantivy-index")
            .to_string_lossy()
            .to_string();

        let state = AppState::new(config).await.expect("Failed to create state");

        // Initial state should be Stopped
        assert_eq!(state.fts_service.state(), ServiceState::Stopped);

        // Transition to Starting
        state.fts_service.set_state(ServiceState::Starting);
        assert!(!state.is_fts_ready());
        assert_eq!(state.active_backend_name(), "simple");

        // Transition to Ready
        state.set_fts_ready(true);
        assert!(state.is_fts_ready());

        // Transition to Failed
        state
            .fts_service
            .set_state(ServiceState::Failed("test error".to_string()));
        assert!(!state.is_fts_ready());
        assert_eq!(state.active_backend_name(), "simple");
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_graph_service_state_transitions() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");

        // Stopped -> Starting -> Ready -> Failed -> Stopped
        assert_eq!(state.graph_service.state(), ServiceState::Stopped);

        state.graph_service.set_state(ServiceState::Starting);
        let result = state.require_graph();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("loading"));

        state.graph_service.set_state(ServiceState::Ready);
        let result = state.require_graph();
        assert!(result.is_ok());

        state
            .graph_service
            .set_state(ServiceState::Failed("test failure".to_string()));
        let result = state.require_graph();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("test failure"));

        state.graph_service.set_state(ServiceState::Stopped);
        let result = state.require_graph();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not loaded"));
    }

    #[tokio::test]
    #[cfg(feature = "vector")]
    async fn test_vector_service_state_transitions() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");

        // Stopped -> Starting -> Ready -> Failed -> Stopped
        assert!(!state.is_vector_ready());

        state.vector_service.set_state(ServiceState::Starting);
        let result = state.require_vector();
        let err = result.err().expect("should be err when Starting");
        assert!(err.to_string().contains("building"));

        state.set_vector_ready(true);
        assert!(state.is_vector_ready());
        let result = state.require_vector();
        assert!(result.is_ok());

        state
            .vector_service
            .set_state(ServiceState::Failed("oom".to_string()));
        assert!(!state.is_vector_ready());
        let result = state.require_vector();
        let err = result.err().expect("should be err when Failed");
        assert!(err.to_string().contains("oom"));

        state.set_vector_ready(false);
        assert!(!state.is_vector_ready());
        let result = state.require_vector();
        let err = result.err().expect("should be err when not built");
        assert!(err.to_string().contains("not built"));
    }
}
