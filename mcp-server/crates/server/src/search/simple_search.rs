//! Simple search backend (linear scan) implementing fabryk's SearchBackend trait.
//!
//! This module provides `SimpleSearch`, a linear scan search backend that preserves
//! the original simple search functionality. It uses fabryk's `SearchDocument`
//! methods (`matches_query`, `relevance`, `extract_snippet`) for matching and
//! ranking, and returns results through fabryk's `SearchBackend` trait interface.

use async_trait::async_trait;

use fabryk::fts::{SearchBackend, SearchParams, SearchResult as FabrykSearchResult, SearchResults};

use crate::config::Config;
use crate::extractors::MusicTheoryDocumentExtractor;
use crate::util::files::{find_all_files, FindOptions};

/// Simple search backend using linear scan.
///
/// This backend scans all concept card files sequentially, parsing each with
/// `MusicTheoryDocumentExtractor` and using fabryk `SearchDocument` methods
/// for matching and relevance scoring.
///
/// It is suitable for small to medium collections (<500 documents) and
/// requires no index.
pub struct SimpleSearch {
    config: Config,
}

impl SimpleSearch {
    /// Create a new SimpleSearch backend.
    ///
    /// This is the fallback search backend for users who do not want to set up
    /// full-text search with Tantivy. It performs a linear scan of all concept cards.
    ///
    /// For better search quality with large collections, consider using the Tantivy
    /// backend by setting `backend = "tantivy"` in your configuration.
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
    async fn search(&self, params: SearchParams) -> fabryk::core::Result<SearchResults> {
        let concept_cards_path = self
            .config
            .paths
            .concept_cards_path()
            .map_err(|e| fabryk::core::Error::config(e.to_string()))?;

        if !crate::util::files::exists(&concept_cards_path).await {
            return Ok(SearchResults::empty(self.name()));
        }

        let extractor = MusicTheoryDocumentExtractor::new();
        let snippet_length = params.snippet_length.unwrap_or(200);
        let limit = params.limit.unwrap_or(10);

        let mut results = Vec::new();

        // Find all markdown files
        let files = find_all_files(&concept_cards_path, FindOptions::markdown())
            .await
            .map_err(|e| fabryk::core::Error::operation(e.to_string()))?;

        for file_info in files {
            let path = &file_info.path;

            // Read file content
            let content = match crate::util::files::read_file(path).await {
                Ok(c) => c,
                Err(_) => continue,
            };

            // Extract SearchDocument using the MusicTheoryDocumentExtractor
            let doc = match extractor.extract(path, &content) {
                Some(d) => d,
                None => continue,
            };

            // Check if document matches query (skip for wildcard queries)
            if !doc.matches_query(&params.query) {
                continue;
            }

            // Apply category filter
            if let Some(ref filter_category) = params.category {
                if !doc.matches_category(filter_category) {
                    continue;
                }
            }

            // Apply source filter
            if let Some(ref filter_source) = params.source {
                if !doc.matches_source(filter_source) {
                    continue;
                }
            }

            // Apply content_types filter
            if let Some(ref content_types) = params.content_types {
                let matches_any = content_types.iter().any(|ct| doc.matches_content_type(ct));
                if !matches_any {
                    continue;
                }
            }

            // Calculate relevance and extract snippet
            let relevance = doc.relevance(&params.query);
            let snippet = doc.extract_snippet(&params.query, snippet_length);

            results.push(FabrykSearchResult {
                id: doc.id.clone(),
                title: doc.title.clone(),
                description: doc.description.clone(),
                category: doc.category.clone(),
                source: doc.source.clone(),
                path: Some(doc.path.clone()),
                snippet,
                relevance,
                content_type: doc.content_type.clone(),
                section: doc.section.clone(),
                chapter: doc.chapter.clone(),
            });
        }

        // Sort by relevance (highest first)
        results.sort_by(|a, b| {
            b.relevance
                .partial_cmp(&a.relevance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply limit
        results.truncate(limit);

        let total = results.len();

        Ok(SearchResults {
            items: results,
            total,
            backend: self.name().to_string(),
        })
    }

    fn name(&self) -> &str {
        "simple"
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

    #[test]
    fn test_simple_search_new() {
        let config = test_config();
        let backend = SimpleSearch::new(config);
        assert_eq!(backend.name(), "simple");
    }

    #[tokio::test]
    async fn test_simple_search_empty_directory() {
        let config = test_config();
        let backend = SimpleSearch::new(config);

        let params = SearchParams {
            query: "test".to_string(),
            limit: Some(10),
            ..Default::default()
        };

        let result = backend.search(params).await;
        assert!(result.is_ok());
        // Just verify search executes without error
    }

    #[tokio::test]
    async fn test_simple_search_is_ready() {
        let config = test_config();
        let backend = SimpleSearch::new(config);
        assert!(backend.is_ready());
    }
}
