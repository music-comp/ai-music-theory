//! Search backend abstraction.
//!
//! This module provides the SearchBackend trait and factory function
//! for creating search backends based on configuration.

use async_trait::async_trait;

use crate::config::Config;
use crate::error::Result;
use crate::tools::search::{SearchConceptsParams, SearchResult};

/// Abstract search backend trait.
///
/// Implementations provide different search strategies:
/// - SimpleSearch: Linear scan (O(n)), suitable for <500 documents
/// - TantivySearch: Full-text index (O(log n)), recommended for 500+ documents
#[async_trait]
pub trait SearchBackend: Send + Sync {
    /// Execute a search query.
    ///
    /// # Arguments
    ///
    /// * `params` - Search parameters (query string and limit)
    ///
    /// # Returns
    ///
    /// Returns a vector of SearchResults sorted by relevance.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the search fails.
    async fn search(&self, params: &SearchConceptsParams) -> Result<Vec<SearchResult>>;
}

/// Create a search backend based on configuration.
///
/// Reads `config.search.backend` to determine which backend to use:
/// - "simple": SimpleSearch (linear scan, no index needed)
/// - "tantivy": TantivySearch (full-text index, requires existing index)
///
/// # Arguments
///
/// * `config` - Server configuration
///
/// # Returns
///
/// Returns a boxed SearchBackend ready to execute searches.
///
/// # Errors
///
/// Returns `Err` if:
/// - Backend type is unknown
/// - Backend initialization fails (e.g., Tantivy index doesn't exist)
#[allow(dead_code)]
pub async fn create_search_backend(config: &Config) -> Result<Box<dyn SearchBackend>> {
    match config.search.backend.as_str() {
        #[cfg(feature = "fts")]
        "tantivy" => {
            use crate::search::TantivySearch;

            let index_path = config.search.index_path()?;
            let backend = TantivySearch::new(&index_path, config.search.clone())?;
            Ok(Box::new(backend))
        }
        _ => {
            // Default to simple search (includes "simple" and any unknown backends)
            use crate::search::SimpleSearch;

            let backend = SimpleSearch::new(config.clone());
            Ok(Box::new(backend))
        }
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
                index_path: ".tantivy-index".to_string(),
                rebuild_on_startup: false,
                snippet_size: 200,
                fuzzy_search: false,
                fuzzy_distance: 2,
            },
        }
    }

    #[tokio::test]
    async fn test_create_search_backend_simple() {
        let config = test_config("simple");
        let result = create_search_backend(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_search_backend_default() {
        let config = test_config("unknown");
        let result = create_search_backend(&config).await;
        // Should fall back to simple
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "fts")]
    async fn test_create_search_backend_tantivy_no_index() {
        let config = test_config("tantivy");
        let result = create_search_backend(&config).await;
        // Should fail because index doesn't exist
        assert!(result.is_err());
    }
}
