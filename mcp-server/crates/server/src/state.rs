//! Application state management.
//!
//! This module provides AppState for managing the search backend state,
//! including FTS readiness tracking and dynamic backend switching.

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

use crate::config::Config;
use crate::error::Result;
use crate::search::backend::SearchBackend;
use crate::search::SimpleSearch;

#[cfg(feature = "fts")]
use crate::search::{build_index, is_index_fresh, TantivySearch};

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
fn initialize_fts_backend(
    config: &Config,
) -> Result<(Arc<RwLock<Option<Arc<TantivySearch>>>>, Arc<AtomicBool>)> {
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
        log::debug!("FTS not configured (backend={})", state.config.search.backend);
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
async fn build_fts_index_for_state(
    state: &AppState,
) -> Result<crate::search::IndexStats> {
    build_index(&state.config).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{LoggingConfig, PathsConfig, SearchConfig, ServerConfig, SourcesConfig};

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
            logging: LoggingConfig {
                level: "info".to_string(),
            },
            search: SearchConfig {
                backend: backend.to_string(),
                index_path: ".tantivy-index-test".to_string(),
                rebuild_on_startup: false,
                snippet_size: 200,
                fuzzy_search: false,
                fuzzy_distance: 2,
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
}
