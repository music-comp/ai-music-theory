//! Search functionality delegating to fabryk-fts.
//!
//! This module provides a thin wrapper around `fabryk::fts` types, adapting
//! them to the project's existing API surface. It replaces the original 11-file
//! search module with delegation to the fabryk framework.
//!
//! # Architecture
//!
//! - **`SearchBackend` trait**: Re-exported from `fabryk::fts::SearchBackend`.
//!   Uses `SearchParams` / `SearchResults` instead of the old
//!   `SearchConceptsParams` / `Vec<SearchResult>` contract.
//! - **`SimpleSearch`**: Project-specific implementation that scans concept card
//!   files and uses `fabryk::fts::SearchDocument` methods for matching.
//! - **`TantivySearch`**: Re-exported from fabryk when the `fts` feature is
//!   enabled.
//! - **`build_index`** / **`is_index_fresh`**: Wrapper functions that bridge
//!   the project's `Config` to fabryk's `IndexBuilder` / `IndexMetadata`.
//! - **`IndexStats`** / **`IndexMetadata`**: Adapter types providing
//!   backward-compatible field names on top of fabryk's types.

mod simple_search;

// ============================================================================
// Re-exports from fabryk
// ============================================================================

// The SearchBackend trait (async fn search(SearchParams) -> Result<SearchResults>)
pub use fabryk::fts::SearchBackend;

// Types needed by tools/search.rs, state.rs, and sources.rs
pub use fabryk::fts::SearchParams;

// Re-export our SimpleSearch
pub use simple_search::SimpleSearch;

// FTS-specific re-exports (only when feature enabled)
#[cfg(feature = "fts")]
pub use fabryk::fts::TantivySearch;

// ============================================================================
// Config conversion
// ============================================================================

/// Convert the project's `SearchConfig` to fabryk's `SearchConfig`.
///
/// Maps field names and handles type differences:
/// - `query_mode`: `MinimumMatch(f32)` -> `MinimumMatch` (drops the f32)
/// - `index_path`: `String` -> `Option<String>`
/// - `snippet_size` -> `snippet_length`
/// - `stopword_allowlist` -> `allowlist`
#[cfg(feature = "fts")]
pub fn to_fabryk_search_config(
    config: &crate::config::SearchConfig,
) -> crate::error::Result<fabryk::fts::SearchConfig> {
    let query_mode = match config.query_mode {
        crate::config::QueryMode::Smart => fabryk::fts::QueryMode::Smart,
        crate::config::QueryMode::And => fabryk::fts::QueryMode::And,
        crate::config::QueryMode::Or => fabryk::fts::QueryMode::Or,
        crate::config::QueryMode::MinimumMatch(_) => fabryk::fts::QueryMode::MinimumMatch,
    };

    let index_path = config.index_path()?;

    Ok(fabryk::fts::SearchConfig {
        backend: config.backend.clone(),
        index_path: Some(index_path.to_string_lossy().into_owned()),
        content_path: None, // set per-invocation
        query_mode,
        fuzzy_enabled: config.fuzzy_search,
        fuzzy_distance: config.fuzzy_distance,
        stopwords_enabled: config.enable_stopwords,
        custom_stopwords: config.custom_stopwords.clone(),
        allowlist: config.stopword_allowlist.clone(),
        default_limit: 10,
        snippet_length: config.snippet_size,
    })
}

/// Convert the project's `QueryMode` to fabryk's `QueryMode`.
pub fn to_fabryk_query_mode(mode: &crate::config::QueryMode) -> fabryk::fts::QueryMode {
    match mode {
        crate::config::QueryMode::Smart => fabryk::fts::QueryMode::Smart,
        crate::config::QueryMode::And => fabryk::fts::QueryMode::And,
        crate::config::QueryMode::Or => fabryk::fts::QueryMode::Or,
        crate::config::QueryMode::MinimumMatch(_) => fabryk::fts::QueryMode::MinimumMatch,
    }
}

// ============================================================================
// IndexStats adapter (backward-compatible field names)
// ============================================================================

/// Statistics from index building.
///
/// Provides backward-compatible field names on top of fabryk's `IndexStats`.
/// The project uses `files_found`, `indexed`, `errors` while fabryk uses
/// `files_processed`, `documents_indexed`, `errors`.
#[cfg(feature = "fts")]
#[derive(Debug, Clone, Default)]
pub struct IndexStats {
    /// Total number of files found across all content types.
    pub files_found: usize,
    /// Total number of documents successfully indexed.
    pub indexed: usize,
    /// Total number of errors encountered.
    pub errors: usize,
    /// Per-type statistics.
    pub concept_cards: usize,
    pub source_chapters: usize,
    pub unified_concepts: usize,
    pub guides: usize,
}

#[cfg(feature = "fts")]
impl From<fabryk::fts::IndexStats> for IndexStats {
    fn from(fabryk_stats: fabryk::fts::IndexStats) -> Self {
        Self {
            files_found: fabryk_stats.files_processed,
            indexed: fabryk_stats.documents_indexed,
            errors: fabryk_stats.errors,
            // fabryk's IndexStats doesn't track per-type counts;
            // all indexed documents are counted as concept_cards for now.
            concept_cards: fabryk_stats.documents_indexed,
            source_chapters: 0,
            unified_concepts: 0,
            guides: 0,
        }
    }
}

#[cfg(feature = "fts")]
impl std::ops::AddAssign for IndexStats {
    fn add_assign(&mut self, other: Self) {
        self.files_found += other.files_found;
        self.indexed += other.indexed;
        self.errors += other.errors;
        self.concept_cards += other.concept_cards;
        self.source_chapters += other.source_chapters;
        self.unified_concepts += other.unified_concepts;
        self.guides += other.guides;
    }
}

// ============================================================================
// IndexMetadata adapter (backward-compatible field names)
// ============================================================================

/// Index metadata with backward-compatible field names.
///
/// The project uses `doc_count` and `last_indexed: SystemTime` while fabryk
/// uses `document_count` and `indexed_at: String` (ISO 8601).
#[cfg(feature = "fts")]
pub struct IndexMetadata {
    inner: fabryk::fts::IndexMetadata,
}

#[cfg(feature = "fts")]
impl IndexMetadata {
    /// Load metadata from the index directory.
    ///
    /// Returns `Ok(None)` if the metadata file doesn't exist.
    pub fn load(index_path: &std::path::Path) -> crate::error::Result<Option<Self>> {
        match fabryk::fts::IndexMetadata::load(index_path) {
            Ok(Some(inner)) => Ok(Some(Self { inner })),
            Ok(None) => Ok(None),
            Err(e) => Err(crate::error::Error::operation(e.to_string())),
        }
    }

    /// Get the document count.
    pub fn doc_count(&self) -> usize {
        self.inner.document_count
    }

    /// Get the indexed-at timestamp as a formatted string.
    pub fn indexed_at_display(&self) -> String {
        self.inner.indexed_at.clone()
    }

    /// Get the schema version.
    pub fn schema_version(&self) -> u32 {
        self.inner.schema_version
    }

    /// Get the content hash.
    pub fn content_hash(&self) -> &str {
        &self.inner.content_hash
    }
}

// ============================================================================
// FTS wrapper functions
// ============================================================================

/// Build a full-text search index from all content types.
///
/// Uses fabryk's `IndexBuilder` with fabryk's `ConceptCardDocumentExtractor`.
/// Indexes content from all 4 content directories: concept_cards, sources_md,
/// concepts_unified, and guides. Missing directories are silently skipped.
///
/// # Errors
///
/// Returns `Err` if index path resolution fails or all content directories are missing.
#[cfg(feature = "fts")]
pub async fn build_index(config: &crate::config::Config) -> crate::error::Result<IndexStats> {
    let index_path = config.search.index_path()?;

    log::info!("Building FTS index at: {}", index_path.display());

    // Collect all content directories that exist.
    let content_dirs: Vec<(std::path::PathBuf, &str)> = [
        (config.paths.concept_cards_path(), "concept_cards"),
        (config.paths.sources_md_path(), "source_chapters"),
        (config.paths.concepts_unified_path(), "unified_concepts"),
        (config.paths.guides_path(), "guides"),
    ]
    .into_iter()
    .filter_map(|(result, label)| {
        result.ok().and_then(|path| {
            if path.exists() {
                Some((path, label))
            } else {
                log::debug!(
                    "Skipping missing content dir for {}: {}",
                    label,
                    path.display()
                );
                None
            }
        })
    })
    .collect();

    if content_dirs.is_empty() {
        return Err(crate::error::Error::operation(
            "No content directories found to index".to_string(),
        ));
    }

    let mut total_stats = IndexStats::default();
    let mut first = true;

    for (content_path, label) in &content_dirs {
        let extractor = fabryk::fts::ConceptCardDocumentExtractor::new();
        let builder = fabryk::fts::IndexBuilder::new()
            .with_extractor(Box::new(extractor))
            .force_rebuild();

        let result = if first {
            builder.build(content_path, &index_path).await
        } else {
            builder.build_append(content_path, &index_path).await
        };

        match result {
            Ok(fabryk_stats) => {
                let partial = IndexStats::from(fabryk_stats);
                // Update per-type counts based on the directory label.
                let mut typed_stats = IndexStats {
                    files_found: partial.files_found,
                    indexed: partial.indexed,
                    errors: partial.errors,
                    ..Default::default()
                };
                match *label {
                    "concept_cards" => typed_stats.concept_cards = partial.indexed,
                    "source_chapters" => typed_stats.source_chapters = partial.indexed,
                    "unified_concepts" => typed_stats.unified_concepts = partial.indexed,
                    "guides" => typed_stats.guides = partial.indexed,
                    _ => typed_stats.concept_cards = partial.indexed,
                }
                total_stats += typed_stats;
                first = false;
                log::info!(
                    "Indexed {} from {}: {} docs",
                    label,
                    content_path.display(),
                    partial.indexed
                );
            }
            Err(e) => {
                log::warn!(
                    "Failed to index {} from {}: {}",
                    label,
                    content_path.display(),
                    e
                );
                total_stats.errors += 1;
            }
        }
    }

    log::info!(
        "Index build complete: {} docs indexed, {} errors",
        total_stats.indexed,
        total_stats.errors
    );

    Ok(total_stats)
}

/// Check if the index is fresh (content hasn't changed).
///
/// Delegates to fabryk's `IndexMetadata::is_fresh()` using the concept_cards
/// path as the content directory to check.
///
/// # Errors
///
/// Returns `Err` if path resolution or freshness check fails.
#[cfg(feature = "fts")]
pub async fn is_index_fresh(
    index_path: &std::path::Path,
    config: &crate::config::Config,
) -> crate::error::Result<bool> {
    let concept_cards_path = config.paths.concept_cards_path()?;

    fabryk::fts::is_index_fresh(index_path, &concept_cards_path)
        .await
        .map_err(|e| crate::error::Error::operation(e.to_string()))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(all(test, feature = "fts"))]
mod tests {
    use super::*;
    use crate::config::{PathsConfig, SearchConfig, ServerConfig, SourcesConfig};

    fn test_search_config() -> crate::config::SearchConfig {
        SearchConfig {
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
            stopword_allowlist: vec!["I".to_string(), "V".to_string(), "do".to_string()],
            field_boost_title: 3.0,
            field_boost_description: 2.0,
            field_boost_content: 1.0,
        }
    }

    fn test_config(backend: &str) -> crate::config::Config {
        crate::config::Config {
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
                ..test_search_config()
            },
            lancedb: crate::config::LanceDbConfig::default(),
        }
    }

    #[test]
    fn test_to_fabryk_search_config_smart() {
        let config = test_search_config();
        let fabryk_config = to_fabryk_search_config(&config).expect("config resolution failed");

        assert_eq!(fabryk_config.backend, "simple");
        // index_path is resolved to an absolute path
        let index_path = fabryk_config.index_path.expect("index_path should be Some");
        assert!(index_path.ends_with(".tantivy-index"));
        assert_eq!(fabryk_config.query_mode, fabryk::fts::QueryMode::Smart);
        assert!(!fabryk_config.fuzzy_enabled);
        assert_eq!(fabryk_config.fuzzy_distance, 2);
        assert!(fabryk_config.stopwords_enabled);
        assert_eq!(fabryk_config.snippet_length, 200);
        // Allowlist should carry through
        assert!(fabryk_config.allowlist.contains(&"I".to_string()));
        assert!(fabryk_config.allowlist.contains(&"V".to_string()));
        assert!(fabryk_config.allowlist.contains(&"do".to_string()));
    }

    #[test]
    fn test_to_fabryk_search_config_and() {
        let mut config = test_search_config();
        config.query_mode = crate::config::QueryMode::And;
        let fabryk_config = to_fabryk_search_config(&config).expect("config resolution failed");
        assert_eq!(fabryk_config.query_mode, fabryk::fts::QueryMode::And);
    }

    #[test]
    fn test_to_fabryk_search_config_or() {
        let mut config = test_search_config();
        config.query_mode = crate::config::QueryMode::Or;
        let fabryk_config = to_fabryk_search_config(&config).expect("config resolution failed");
        assert_eq!(fabryk_config.query_mode, fabryk::fts::QueryMode::Or);
    }

    #[test]
    fn test_to_fabryk_search_config_minimum_match() {
        let mut config = test_search_config();
        config.query_mode = crate::config::QueryMode::MinimumMatch(0.6);
        let fabryk_config = to_fabryk_search_config(&config).expect("config resolution failed");
        assert_eq!(
            fabryk_config.query_mode,
            fabryk::fts::QueryMode::MinimumMatch
        );
    }

    #[test]
    fn test_to_fabryk_query_mode() {
        assert_eq!(
            to_fabryk_query_mode(&crate::config::QueryMode::Smart),
            fabryk::fts::QueryMode::Smart
        );
        assert_eq!(
            to_fabryk_query_mode(&crate::config::QueryMode::And),
            fabryk::fts::QueryMode::And
        );
        assert_eq!(
            to_fabryk_query_mode(&crate::config::QueryMode::Or),
            fabryk::fts::QueryMode::Or
        );
        assert_eq!(
            to_fabryk_query_mode(&crate::config::QueryMode::MinimumMatch(0.5)),
            fabryk::fts::QueryMode::MinimumMatch
        );
    }

    #[test]
    fn test_index_stats_default() {
        let stats = IndexStats::default();
        assert_eq!(stats.files_found, 0);
        assert_eq!(stats.indexed, 0);
        assert_eq!(stats.errors, 0);
        assert_eq!(stats.concept_cards, 0);
    }

    #[test]
    fn test_index_stats_add_assign() {
        let mut stats1 = IndexStats {
            files_found: 10,
            indexed: 8,
            errors: 2,
            concept_cards: 8,
            source_chapters: 0,
            unified_concepts: 0,
            guides: 0,
        };

        let stats2 = IndexStats {
            files_found: 5,
            indexed: 4,
            errors: 1,
            concept_cards: 0,
            source_chapters: 4,
            unified_concepts: 0,
            guides: 0,
        };

        stats1 += stats2;

        assert_eq!(stats1.files_found, 15);
        assert_eq!(stats1.indexed, 12);
        assert_eq!(stats1.errors, 3);
        assert_eq!(stats1.concept_cards, 8);
        assert_eq!(stats1.source_chapters, 4);
    }

    #[test]
    fn test_index_stats_clone() {
        let stats = IndexStats {
            files_found: 10,
            indexed: 10,
            errors: 0,
            concept_cards: 5,
            source_chapters: 3,
            unified_concepts: 1,
            guides: 1,
        };

        let cloned = stats.clone();
        assert_eq!(cloned.files_found, stats.files_found);
        assert_eq!(cloned.indexed, stats.indexed);
    }

    #[cfg(feature = "fts")]
    #[test]
    fn test_index_stats_from_fabryk() {
        let fabryk_stats = fabryk::fts::IndexStats {
            documents_indexed: 42,
            files_processed: 50,
            files_skipped: 3,
            errors: 5,
            bytes_processed: 1024,
            content_hash: "abc123".to_string(),
            label_counts: std::collections::HashMap::new(),
        };

        let stats = IndexStats::from(fabryk_stats);
        assert_eq!(stats.indexed, 42);
        assert_eq!(stats.files_found, 50);
        assert_eq!(stats.errors, 5);
        assert_eq!(stats.concept_cards, 42);
    }

    #[test]
    fn test_simple_search_new() {
        let config = test_config("simple");
        let backend = SimpleSearch::new(config);
        // Verify it was created without panic
        assert_eq!(backend.name(), "simple");
    }

    #[tokio::test]
    async fn test_simple_search_empty_directory() {
        let config = test_config("simple");
        let backend = SimpleSearch::new(config);

        let params = SearchParams {
            query: "test".to_string(),
            limit: Some(10),
            ..Default::default()
        };

        let result = backend.search(params).await;
        assert!(result.is_ok());
    }
}
