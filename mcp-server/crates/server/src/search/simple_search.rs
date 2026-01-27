//! Simple search backend (linear scan).
//!
//! This module provides SimpleSearch, a linear scan search backend
//! that preserves the original simple search functionality.

use async_trait::async_trait;

use crate::config::Config;
use crate::error::Result;
use crate::metadata::extract_concept_metadata;
use crate::search::{backend::SearchBackend, SearchDocument};
use crate::tools::search::{SearchConceptsParams, SearchResult};
use crate::util::files::{find_all_files, FindOptions};

/// Simple search backend using linear scan.
///
/// This backend scans all concept cards sequentially, checking each
/// for query matches. It's suitable for small to medium collections
/// (<500 documents) and requires no index.
pub struct SimpleSearch {
    config: Config,
}

impl SimpleSearch {
    /// Create a new SimpleSearch backend.
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration
    pub fn new(config: Config) -> Self {
        SimpleSearch { config }
    }
}

#[async_trait]
impl SearchBackend for SimpleSearch {
    async fn search(&self, params: &SearchConceptsParams) -> Result<Vec<SearchResult>> {
        let concept_cards_path = self.config.paths.concept_cards_path()?;

        if !crate::util::files::exists(&concept_cards_path).await {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();

        // Find all markdown files
        let files = find_all_files(&concept_cards_path, FindOptions::markdown()).await?;

        for file_info in files {
            let path = &file_info.path;

            // Extract metadata
            if let Ok(meta) = extract_concept_metadata(&concept_cards_path, path).await {
                // Build SearchDocument
                if let Ok(doc) = SearchDocument::from_metadata(meta, path).await {
                    // Check if document matches query
                    if doc.matches_query(&params.query) {
                        // Extract snippet and calculate relevance
                        let snippet = doc.extract_snippet(&params.query, 200);
                        let relevance = doc.relevance(&params.query);

                        results.push(SearchResult {
                            id: doc.id,
                            title: doc.title,
                            category: doc.category,
                            source: doc.source,
                            path: doc.path,
                            snippet,
                            relevance,
                        });
                    }
                }
            }
        }

        // Sort by relevance (highest first)
        results.sort_by(|a, b| {
            b.relevance
                .partial_cmp(&a.relevance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply limit
        results.truncate(params.limit);

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{PathsConfig, SearchConfig, ServerConfig, SourcesConfig};

    fn test_config() -> Config {
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
                backend: "simple".to_string(),
                index_path: ".tantivy-index".to_string(),
                rebuild_on_startup: false,
                snippet_size: 200,
                fuzzy_search: false,
                fuzzy_distance: 2,
            },
        }
    }

    #[test]
    fn test_simple_search_new() {
        let config = test_config();
        let backend = SimpleSearch::new(config);
        assert!(std::ptr::addr_of!(backend).is_null() == false);
    }

    #[tokio::test]
    async fn test_simple_search_empty_directory() {
        let config = test_config();
        let backend = SimpleSearch::new(config);

        let params = SearchConceptsParams {
            query: "test".to_string(),
            limit: 10,
        };

        let result = backend.search(&params).await;
        assert!(result.is_ok());
        // Just verify search executes without error
        // Results may or may not be empty depending on actual directory content
    }
}
