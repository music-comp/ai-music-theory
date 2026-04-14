//! Application state management.
//!
//! This module provides AppState for managing the search backend state,
//! including FTS readiness tracking and dynamic backend switching.
//! Service lifecycle is tracked via `fabryk::core::ServiceHandle`.

use std::sync::Arc;

#[cfg(feature = "fts")]
use std::path::Path;

#[cfg(any(feature = "fts", feature = "graph", feature = "vector"))]
use fabryk::core::{BackendSlot, ServiceState};

use crate::config::Config;
use crate::error::Result;
use crate::search::SearchBackend;
use crate::search::SimpleSearch;

#[cfg(feature = "fts")]
use crate::search::{build_index, is_index_fresh, TantivySearch};

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

    /// FTS backend slot (service lifecycle + backend storage)
    #[cfg(feature = "fts")]
    pub fts: BackendSlot<Arc<TantivySearch>>,

    /// Graph backend slot (service lifecycle + graph data storage)
    #[cfg(feature = "graph")]
    pub graph: BackendSlot<crate::graph::LoadedGraph>,

    /// Shared graph data for fabryk `GraphTools` (tokio RwLock).
    ///
    /// This is a separate reference to the graph data that uses `tokio::sync::RwLock`
    /// (required by `GraphTools::with_shared`). It is updated in lockstep with
    /// `graph` whenever the graph is loaded or rebuilt.
    #[cfg(feature = "graph")]
    pub shared_graph: Arc<tokio::sync::RwLock<crate::graph::GraphData>>,

    /// Vector backend slot (service lifecycle + vector backend storage)
    #[cfg(feature = "vector")]
    pub vector: BackendSlot<Arc<dyn fabryk::vector::VectorBackend>>,

    /// Shared vector slot for fabryk `SemanticSearchTools` (tokio RwLock).
    ///
    /// This mirrors `vector` but uses `tokio::sync::RwLock` as required
    /// by [`fabryk_mcp::semantic::VectorSlot`]. It is updated in lockstep with
    /// `vector` whenever the vector index finishes building.
    #[cfg(feature = "vector")]
    pub vector_slot: fabryk_mcp::semantic::VectorSlot,
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
        let mut search_config = crate::search::to_fabryk_search_config(&config.search)?;
        // Set content_path so SimpleSearch knows where to scan
        if let Ok(cards_path) = config.paths.concept_cards_path() {
            search_config.content_path = Some(cards_path.to_string_lossy().into_owned());
        }
        let simple_backend = Arc::new(SimpleSearch::with_default_extractor(&search_config));

        Ok(Self {
            config,
            simple_backend,
            #[cfg(feature = "fts")]
            fts: BackendSlot::new("fts"),
            #[cfg(feature = "graph")]
            graph: BackendSlot::new("graph"),
            #[cfg(feature = "graph")]
            shared_graph: Arc::new(tokio::sync::RwLock::new(crate::graph::GraphData::new())),
            #[cfg(feature = "vector")]
            vector: BackendSlot::new("vector"),
            #[cfg(feature = "vector")]
            vector_slot: Arc::new(tokio::sync::RwLock::new(None)),
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
        if self.fts.is_ready() {
            if let Ok(guard) = self.fts.inner().read() {
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
        if self.fts.is_ready() {
            return "tantivy";
        }
        "simple"
    }

}

/// Spawn an async task that loads or rebuilds the FTS index.
///
/// Follows the same async pattern as graph and vector initialization:
/// transitions through `Starting → Ready` (or `Failed`/`Degraded`).
#[cfg(feature = "fts")]
fn start_fts_loading(state: Arc<AppState>, needs_rebuild: bool) {
    tokio::spawn(async move {
        state.fts.service().set_state(ServiceState::Starting);

        if needs_rebuild {
            // Build index from scratch
            match build_fts_index_for_state(&state).await {
                Ok(stats) => {
                    log::info!(
                        indexed = stats.indexed,
                        errors = stats.errors;
                        "FTS index built"
                    );
                }
                Err(e) => {
                    log::warn!("FTS indexing failed (graceful degradation): {}", e);
                    state
                        .fts
                        .service()
                        .set_state(ServiceState::Degraded(format!("simple fallback: {e}")));
                    return;
                }
            }
        }

        // Load index from disk (freshly built or pre-existing)
        let fabryk_config = match crate::search::to_fabryk_search_config(&state.config.search) {
            Ok(c) => c,
            Err(e) => {
                state.fts.service().set_state(ServiceState::Failed(format!(
                    "Failed to resolve FTS config: {e}"
                )));
                return;
            }
        };
        match TantivySearch::new(&fabryk_config) {
            Ok(backend) => {
                if state.fts.set(Arc::new(backend)).is_ok() {
                    state.fts.service().set_state(ServiceState::Ready);
                } else {
                    state.fts.service().set_state(ServiceState::Failed(
                        "Failed to store FTS backend".to_string(),
                    ));
                }
            }
            Err(e) => {
                state.fts.service().set_state(ServiceState::Failed(format!(
                    "Failed to load FTS index: {e}"
                )));
            }
        }
    });
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
    let needs_rebuild = !index_exists_and_fresh(&index_path, &state.config).await?;

    start_fts_loading(Arc::clone(state), needs_rebuild);
    Ok(())
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
        log::info!("Concept graph not found, building from source cards");
        let (graph_data, build_stats) = crate::graph::build_graph(&state.config).await?;
        log::info!(
            "Graph built: {} nodes, {} edges ({} files processed)",
            graph_data.node_count(),
            graph_data.edge_count(),
            build_stats.files_processed
        );

        let graphs_dir = data_dir.join("graphs");
        tokio::fs::create_dir_all(&graphs_dir)
            .await
            .map_err(|e| crate::error::Error::io_with_path(e, &graphs_dir))?;

        fabryk::graph::save_graph(&graph_data, &graph_path, None)
            .map_err(|e| crate::error::Error::operation(format!("Failed to save graph: {}", e)))?;
        log::info!("Graph saved to {}", graph_path.display());
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
        state.graph.service().set_state(ServiceState::Starting);

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

                // Update the shared tokio::sync::RwLock<GraphData> for GraphTools
                {
                    let mut shared = state.shared_graph.write().await;
                    *shared = loaded.data.clone();
                }

                // Store graph data and mark service ready
                if state.graph.set(loaded).is_ok() {
                    state.graph.service().set_state(ServiceState::Ready);
                } else {
                    state.graph.service().set_state(ServiceState::Failed(
                        "Failed to store graph data".to_string(),
                    ));
                }
            }
            Err(e) => {
                log::error!("Failed to load concept graph: {}", e);
                state
                    .graph
                    .service()
                    .set_state(ServiceState::Failed(e.to_string()));
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

    let cache_file = cache_dir.join("vector-cache.json");
    if cache_file.exists() {
        log::info!("Starting background vector index build (using cached embeddings)");
    } else {
        log::info!(
            "Starting background vector index build (no cache — this may take a while). \
             Tip: run `music-theory-mcp cache download vector` to use pre-built embeddings."
        );
    }
    start_vector_building(Arc::clone(state), cache_dir);
    Ok(())
}

/// Spawn an async task that builds the vector index and stores the result.
#[cfg(feature = "vector")]
fn start_vector_building(state: Arc<AppState>, cache_dir: std::path::PathBuf) {
    tokio::spawn(async move {
        state.vector.service().set_state(ServiceState::Starting);

        match build_vector_index(&state.config, &cache_dir).await {
            Ok(backend) => {
                use fabryk::vector::VectorBackend;
                let doc_count = backend.document_count().unwrap_or(0);
                let backend_arc: Arc<dyn fabryk::vector::VectorBackend> = Arc::new(backend);
                if state.vector.set(Arc::clone(&backend_arc)).is_ok() {
                    // Also update the tokio-based vector slot used by SemanticSearchTools.
                    if let Ok(mut slot) = state.vector_slot.try_write() {
                        *slot = Some(backend_arc);
                    } else {
                        log::warn!("Could not acquire vector_slot write lock; SemanticSearchTools may not see the new backend immediately");
                    }
                    state.vector.service().set_state(ServiceState::Ready);
                    log::info!(documents = doc_count; "Vector index ready");
                } else {
                    log::error!("Failed to store vector backend");
                    state
                        .vector
                        .service()
                        .set_state(ServiceState::Failed("Failed to store backend".to_string()));
                }
            }
            Err(e) => {
                log::warn!("Vector index build failed (graceful degradation): {}", e);
                state
                    .vector
                    .service()
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
    use fabryk::vector::builder::ErrorHandling;
    use fabryk::vector::ConceptCardVectorExtractor;
    use fabryk::vector::{EmbeddingProvider, FastEmbedProvider, VectorIndexBuilder};

    // Ensure cache directory exists
    tokio::fs::create_dir_all(cache_dir).await?;
    let cache_file = cache_dir.join("vector-cache.json");

    // Create embedding provider using config values
    let provider: Arc<dyn EmbeddingProvider> = Arc::new(FastEmbedProvider::new(
        &config.lancedb.embedding_model,
        config.lancedb.embedding_cache_dir.as_deref(),
    )?);

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

        let extractor = ConceptCardVectorExtractor::new();
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
            lancedb: crate::config::LanceDbConfig::default(),
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
        assert_eq!(state.vector.service().state(), ServiceState::Stopped);
        assert_eq!(state.vector.service().name(), "vector");
        assert!(state.vector.inner().read().unwrap().is_none());
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
        assert!(!state.fts.is_ready());
        assert_eq!(state.fts.service().state(), ServiceState::Stopped);
        assert_eq!(state.active_backend_name(), "simple");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_fts_service_ready_via_handle() {
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

        assert!(!state.fts.is_ready());

        state.fts.service().set_state(ServiceState::Ready);
        assert!(state.fts.is_ready());
        assert_eq!(state.fts.service().state(), ServiceState::Ready);

        state.fts.service().set_state(ServiceState::Stopped);
        assert!(!state.fts.is_ready());
        assert_eq!(state.fts.service().state(), ServiceState::Stopped);
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
        let fabryk_config = crate::search::to_fabryk_search_config(&config.search)
            .expect("Failed to resolve FTS config");
        let backend = TantivySearch::new(&fabryk_config).expect("Failed to load index");

        // Update backend
        let result = state.fts.set(Arc::new(backend));
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
        let fabryk_config = crate::search::to_fabryk_search_config(&config.search)
            .expect("Failed to resolve FTS config");
        let backend = TantivySearch::new(&fabryk_config).expect("Failed to load index");
        state.fts.set(Arc::new(backend)).expect("Failed to update");
        state.fts.service().set_state(ServiceState::Ready);

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
        state.fts.service().set_state(ServiceState::Ready);
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
    async fn test_new_state_fts_not_ready_simple_backend() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        assert!(!state.fts.is_ready());
        assert_eq!(state.fts.service().state(), ServiceState::Stopped);
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_new_state_fts_not_ready_no_index() {
        use std::env;

        let mut config = test_config("tantivy");
        let nonexistent_path = env::temp_dir().join(format!("nonexistent-{}", std::process::id()));
        config.search.index_path = nonexistent_path.to_string_lossy().to_string();

        let state = AppState::new(config).await.expect("Failed to create state");
        assert!(!state.fts.is_ready());
        assert_eq!(state.fts.service().state(), ServiceState::Stopped);
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

        // Build index first
        crate::search::build_index(&config)
            .await
            .expect("Failed to build index");

        // Create state (starts with empty FTS state)
        let state = Arc::new(AppState::new(config).await.expect("Failed to create state"));

        // Initialize FTS - should detect fresh index and start async loading
        let result = initialize_fts(&state).await;
        assert!(result.is_ok());

        // Poll for FTS to become ready (async loading)
        let mut attempts = 0;
        while !state.fts.is_ready() && attempts < 40 {
            sleep(Duration::from_millis(100)).await;
            attempts += 1;
        }

        assert!(
            state.fts.is_ready(),
            "FTS should be ready after loading fresh index (waited {} ms)",
            attempts * 100
        );
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
    async fn test_start_fts_loading_with_rebuild() {
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

        // Manually call start_fts_loading with rebuild
        start_fts_loading(Arc::clone(&state), true);

        // Poll for FTS to become ready
        let mut attempts = 0;
        while !state.fts.is_ready() && attempts < 40 {
            sleep(Duration::from_millis(100)).await;
            attempts += 1;
        }

        // Check if FTS became ready
        assert!(
            state.fts.is_ready(),
            "FTS should be ready after rebuild (waited {} ms)",
            attempts * 100
        );
        assert_eq!(state.active_backend_name(), "tantivy");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_start_fts_loading_empty_directory() {
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
        assert!(!state.fts.is_ready());

        // Call start_fts_loading with rebuild (will succeed with 0 documents due to graceful degradation)
        start_fts_loading(Arc::clone(&state), true);

        // Poll for FTS to become ready (with timeout for CI environments)
        let mut attempts = 0;
        while !state.fts.is_ready() && attempts < 40 {
            sleep(Duration::from_millis(100)).await;
            attempts += 1;
        }

        // Should become ready even with no documents (graceful degradation feature)
        assert!(
            state.fts.is_ready(),
            "FTS should be ready after indexing empty directories (waited {} ms)",
            attempts * 100
        );
        assert_eq!(state.active_backend_name(), "tantivy");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_search_backend_read_lock_fallback() {
        // This tests the fallback path when fts.inner().read() succeeds but backend is None
        let config = test_config("tantivy");
        let state = AppState::new(config).await.expect("Failed to create state");

        // Set ready but don't actually set a backend
        state.fts.service().set_state(ServiceState::Ready);

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
        assert_eq!(state.fts.service().name(), "fts");
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_graph_service_initial_state() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        assert_eq!(state.graph.service().state(), ServiceState::Stopped);
        assert_eq!(state.graph.service().name(), "graph");
        assert!(state.graph.inner().read().unwrap().is_none());
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_require_graph_not_loaded() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        let result = state.graph.require();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not initialized"));
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_require_graph_loading() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state.graph.service().set_state(ServiceState::Starting);
        let result = state.graph.require();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("currently initializing"));
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_require_graph_failed() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state
            .graph
            .service()
            .set_state(ServiceState::Failed("disk full".to_string()));
        let result = state.graph.require();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("disk full"));
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_require_graph_ready_returns_guard() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state.graph.service().set_state(ServiceState::Ready);
        let result = state.graph.require();
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
        assert_eq!(state.graph.service().state(), ServiceState::Stopped);
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
        while state.graph.service().state() == ServiceState::Stopped && attempts < 20 {
            sleep(Duration::from_millis(50)).await;
            attempts += 1;
        }
        // Allow additional time for the failure to be recorded
        sleep(Duration::from_millis(200)).await;

        // The graph service should be in Failed state due to invalid JSON
        let svc_state = state.graph.service().state();
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
        assert!(!state.vector.is_ready());
        state.vector.service().set_state(ServiceState::Ready);
        assert!(state.vector.is_ready());
        state.vector.service().set_state(ServiceState::Stopped);
        assert!(!state.vector.is_ready());
    }

    #[tokio::test]
    #[cfg(feature = "vector")]
    async fn test_require_vector_not_ready() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        let result = state.vector.require();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("not initialized"));
    }

    #[tokio::test]
    #[cfg(feature = "vector")]
    async fn test_require_vector_loading() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state.vector.service().set_state(ServiceState::Starting);
        let result = state.vector.require();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("currently initializing"));
    }

    #[tokio::test]
    #[cfg(feature = "vector")]
    async fn test_require_vector_failed() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state
            .vector
            .service()
            .set_state(ServiceState::Failed("out of memory".to_string()));
        let result = state.vector.require();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert!(err.to_string().contains("out of memory"));
    }

    #[tokio::test]
    #[cfg(feature = "vector")]
    async fn test_require_vector_ready_returns_guard() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");
        state.vector.service().set_state(ServiceState::Ready);
        let result = state.vector.require();
        assert!(result.is_ok());
        // Guard should hold None since no vector backend was actually loaded
        let guard = result.unwrap();
        assert!(guard.is_none());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_initialize_fts_loads_existing_index() {
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

        // Create AppState - now starts with empty FTS state
        let state = Arc::new(AppState::new(config).await.expect("Failed to create state"));

        // Initialize FTS - should detect fresh index and load it asynchronously
        initialize_fts(&state)
            .await
            .expect("Failed to initialize FTS");

        // Poll for FTS to become ready
        let mut attempts = 0;
        while !state.fts.is_ready() && attempts < 40 {
            sleep(Duration::from_millis(100)).await;
            attempts += 1;
        }

        assert!(
            state.fts.is_ready(),
            "FTS should be ready after loading existing index (waited {} ms)",
            attempts * 100
        );
        assert_eq!(state.active_backend_name(), "tantivy");

        // search_backend should return the FTS backend
        let _backend = state.search_backend();
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_start_fts_loading_with_existing_index() {
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

        // Create state and start FTS loading (no rebuild needed since index exists)
        let state = Arc::new(AppState::new(config).await.expect("Failed to create state"));
        start_fts_loading(Arc::clone(&state), false);

        // Poll for FTS to become ready
        let mut attempts = 0;
        while !state.fts.is_ready() && attempts < 40 {
            sleep(Duration::from_millis(100)).await;
            attempts += 1;
        }

        assert!(
            state.fts.is_ready(),
            "FTS should be ready after loading existing index (waited {} ms)",
            attempts * 100
        );
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
    async fn test_fts_service_toggle_multiple_times() {
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
            state.fts.service().set_state(ServiceState::Ready);
            assert!(state.fts.is_ready());
            assert_eq!(state.active_backend_name(), "tantivy");

            state.fts.service().set_state(ServiceState::Stopped);
            assert!(!state.fts.is_ready());
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
        assert_eq!(state.fts.service().state(), ServiceState::Stopped);

        // Transition to Starting
        state.fts.service().set_state(ServiceState::Starting);
        assert!(!state.fts.is_ready());
        assert_eq!(state.active_backend_name(), "simple");

        // Transition to Ready
        state.fts.service().set_state(ServiceState::Ready);
        assert!(state.fts.is_ready());

        // Transition to Failed
        state
            .fts
            .service()
            .set_state(ServiceState::Failed("test error".to_string()));
        assert!(!state.fts.is_ready());
        assert_eq!(state.active_backend_name(), "simple");
    }

    #[tokio::test]
    #[cfg(feature = "graph")]
    async fn test_graph_service_state_transitions() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");

        // Stopped -> Starting -> Ready -> Failed -> Stopped
        assert_eq!(state.graph.service().state(), ServiceState::Stopped);

        state.graph.service().set_state(ServiceState::Starting);
        let result = state.graph.require();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("currently initializing"));

        state.graph.service().set_state(ServiceState::Ready);
        let result = state.graph.require();
        assert!(result.is_ok());

        state
            .graph
            .service()
            .set_state(ServiceState::Failed("test failure".to_string()));
        let result = state.graph.require();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("test failure"));

        state.graph.service().set_state(ServiceState::Stopped);
        let result = state.graph.require();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not initialized"));
    }

    #[tokio::test]
    #[cfg(feature = "vector")]
    async fn test_vector_service_state_transitions() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");

        // Stopped -> Starting -> Ready -> Failed -> Stopped
        assert!(!state.vector.is_ready());

        state.vector.service().set_state(ServiceState::Starting);
        let result = state.vector.require();
        let err = result.err().expect("should be err when Starting");
        assert!(err.to_string().contains("currently initializing"));

        state.vector.service().set_state(ServiceState::Ready);
        assert!(state.vector.is_ready());
        let result = state.vector.require();
        assert!(result.is_ok());

        state
            .vector
            .service()
            .set_state(ServiceState::Failed("oom".to_string()));
        assert!(!state.vector.is_ready());
        let result = state.vector.require();
        let err = result.err().expect("should be err when Failed");
        assert!(err.to_string().contains("oom"));

        state.vector.service().set_state(ServiceState::Stopped);
        assert!(!state.vector.is_ready());
        let result = state.vector.require();
        let err = result.err().expect("should be err when not built");
        assert!(err.to_string().contains("not initialized"));
    }
}
