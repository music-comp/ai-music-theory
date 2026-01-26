//! Health and status reporting tool.
//!
//! This module provides the health tool for reporting server status,
//! active search backend, and FTS readiness.

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::state::AppState;

/// Health check response.
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Overall status ("ok")
    pub status: String,
    /// Backend status information
    pub backend: BackendStatus,
}

/// Search backend status information.
#[derive(Debug, Serialize, Deserialize)]
pub struct BackendStatus {
    /// Currently active backend ("simple" or "tantivy")
    pub active: String,

    /// Whether FTS feature is enabled
    #[cfg(feature = "fts")]
    pub fts_enabled: bool,

    /// Whether FTS backend is ready for queries
    #[cfg(feature = "fts")]
    pub fts_ready: bool,

    /// Index statistics (if FTS is ready)
    #[cfg(feature = "fts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_stats: Option<IndexStats>,
}

/// FTS index statistics.
#[cfg(feature = "fts")]
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexStats {
    /// Number of documents in the index
    pub doc_count: usize,
    /// When the index was last built
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_indexed: Option<String>,
}

/// Get server health and status information.
///
/// Returns information about the active search backend and FTS readiness.
/// If FTS is ready, includes index statistics.
///
/// # Arguments
///
/// * `state` - Application state
///
/// # Returns
///
/// Returns HealthResponse with current status.
///
/// # Errors
///
/// Returns `Err` if index statistics cannot be loaded.
pub async fn get_health(state: &AppState) -> Result<HealthResponse> {
    let active_backend = state.active_backend_name();

    #[cfg(feature = "fts")]
    let (fts_enabled, fts_ready, index_stats) = {
        let enabled = state.config.search.backend == "tantivy";
        let ready = state.is_fts_ready();
        let stats = if ready {
            load_index_stats(&state.config).await.ok()
        } else {
            None
        };
        (enabled, ready, stats)
    };

    Ok(HealthResponse {
        status: "ok".to_string(),
        backend: BackendStatus {
            active: active_backend.to_string(),
            #[cfg(feature = "fts")]
            fts_enabled,
            #[cfg(feature = "fts")]
            fts_ready,
            #[cfg(feature = "fts")]
            index_stats,
        },
    })
}

/// Load index statistics from metadata.
///
/// Reads the metadata.json file from the index directory and extracts
/// document count and last indexed time.
///
/// # Arguments
///
/// * `config` - Server configuration
///
/// # Returns
///
/// Returns IndexStats if metadata can be loaded.
///
/// # Errors
///
/// Returns `Err` if metadata cannot be read or parsed.
#[cfg(feature = "fts")]
async fn load_index_stats(config: &crate::config::Config) -> Result<IndexStats> {
    use crate::search::freshness::IndexMetadata;

    let index_path = config.search.index_path()?;
    let metadata_path = index_path.join("metadata.json");

    if let Ok(json) = tokio::fs::read_to_string(&metadata_path).await {
        if let Ok(metadata) = serde_json::from_str::<IndexMetadata>(&json) {
            return Ok(IndexStats {
                doc_count: metadata.doc_count,
                last_indexed: Some(format!("{:?}", metadata.last_indexed)),
            });
        }
    }

    // If metadata can't be loaded, return minimal stats
    Ok(IndexStats {
        doc_count: 0,
        last_indexed: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, LoggingConfig, PathsConfig, SearchConfig, ServerConfig, SourcesConfig};

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
    async fn test_get_health_simple_backend() {
        let config = test_config("simple");
        let state = AppState::new(config).await.expect("Failed to create state");

        let response = get_health(&state).await.expect("Health check failed");

        assert_eq!(response.status, "ok");
        assert_eq!(response.backend.active, "simple");
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_get_health_tantivy_not_ready() {
        let config = test_config("tantivy");
        let state = AppState::new(config).await.expect("Failed to create state");

        let response = get_health(&state).await.expect("Health check failed");

        assert_eq!(response.status, "ok");
        assert_eq!(response.backend.active, "simple"); // Not ready yet, using simple
        assert!(response.backend.fts_enabled);
        assert!(!response.backend.fts_ready);
        assert!(response.backend.index_stats.is_none());
    }

    #[tokio::test]
    async fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "ok".to_string(),
            backend: BackendStatus {
                active: "simple".to_string(),
                #[cfg(feature = "fts")]
                fts_enabled: false,
                #[cfg(feature = "fts")]
                fts_ready: false,
                #[cfg(feature = "fts")]
                index_stats: None,
            },
        };

        let json = serde_json::to_string(&response).expect("Serialization failed");
        assert!(json.contains("\"status\":\"ok\""));
        assert!(json.contains("\"active\":\"simple\""));
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_load_index_stats_no_metadata() {
        let config = test_config("tantivy");
        let result = load_index_stats(&config).await;

        // Should return Ok with empty stats (graceful fallback)
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert_eq!(stats.doc_count, 0);
        assert!(stats.last_indexed.is_none());
    }
}
