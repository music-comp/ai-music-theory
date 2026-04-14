//! Search functionality delegating to fabryk-fts.
//!
//! This module re-exports fabryk's search types and provides thin wrappers
//! that bridge the project's `Config` to fabryk's index building APIs.

// ============================================================================
// Re-exports from fabryk
// ============================================================================

pub use fabryk::fts::SearchBackend;
pub use fabryk::fts::SearchParams;
pub use fabryk::fts::SimpleSearch;

#[cfg(feature = "fts")]
pub use fabryk::fts::{IndexMetadata, IndexStats, TantivySearch};

// ============================================================================
// Config conversion (maps local field names to fabryk field names)
// ============================================================================

/// Convert the project's `SearchConfig` to fabryk's `SearchConfig`.
///
/// Maps field names between the project's config format and fabryk's:
/// - `index_path`: `String` -> `Option<String>` (with path expansion)
/// - `snippet_size` -> `snippet_length`
/// - `fuzzy_search` -> `fuzzy_enabled`
/// - `enable_stopwords` -> `stopwords_enabled`
/// - `stopword_allowlist` -> `allowlist`
///
/// QueryMode is now the same type (fabryk::fts::QueryMode) and needs no conversion.
#[cfg(feature = "fts")]
pub fn to_fabryk_search_config(
    config: &crate::config::SearchConfig,
) -> crate::error::Result<fabryk::fts::SearchConfig> {
    let index_path = config.index_path()?;

    Ok(fabryk::fts::SearchConfig {
        backend: config.backend.clone(),
        index_path: Some(index_path.to_string_lossy().into_owned()),
        content_path: None, // set per-invocation
        query_mode: config.query_mode,
        fuzzy_enabled: config.fuzzy_search,
        fuzzy_distance: config.fuzzy_distance,
        stopwords_enabled: config.enable_stopwords,
        custom_stopwords: config.custom_stopwords.clone(),
        allowlist: config.stopword_allowlist.clone(),
        default_limit: 10,
        snippet_length: config.snippet_size,
        rebuild_on_startup: config.rebuild_on_startup,
        minimum_match_percent: config.minimum_match_percent,
        field_boost_title: config.field_boost_title,
        field_boost_description: config.field_boost_description,
        field_boost_content: config.field_boost_content,
    })
}

// ============================================================================
// FTS wrapper functions
// ============================================================================

/// Build a full-text search index from all content types.
///
/// Delegates to fabryk's `build_index_multi` with content directories resolved
/// from the project's `Config`. Missing directories are silently skipped.
///
/// # Errors
///
/// Returns `Err` if index path resolution fails or all content directories
/// are missing.
#[cfg(feature = "fts")]
pub async fn build_index(
    config: &crate::config::Config,
) -> crate::error::Result<fabryk::fts::IndexStats> {
    let index_path = config.search.index_path()?;

    log::info!("Building FTS index at: {}", index_path.display());

    // Collect all content directories that exist.
    let content_dirs: Vec<(std::path::PathBuf, &str)> = [
        (config.paths.concept_cards_path(), "concept_cards"),
        (config.paths.sources_md_path(), "source_chapters"),
        #[cfg(feature = "fts")]
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

    let extractor = fabryk::fts::ConceptCardDocumentExtractor::new();
    let stats = fabryk::fts::build_index_multi(&content_dirs, &index_path, Box::new(extractor))
        .await
        .map_err(|e| crate::error::Error::operation(format!("Index build failed: {}", e)))?;

    log::info!(
        "Index build complete: {} docs indexed, {} errors",
        stats.documents_indexed,
        stats.errors
    );

    Ok(stats)
}

/// Check if the index is fresh (content hasn't changed).
///
/// Delegates to fabryk's `is_index_fresh` using the concept_cards path as
/// the content directory to check.
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
        let index_path = fabryk_config.index_path.expect("index_path should be Some");
        assert!(index_path.ends_with(".tantivy-index"));
        assert_eq!(fabryk_config.query_mode, fabryk::fts::QueryMode::Smart);
        assert!(!fabryk_config.fuzzy_enabled);
        assert_eq!(fabryk_config.fuzzy_distance, 2);
        assert!(fabryk_config.stopwords_enabled);
        assert_eq!(fabryk_config.snippet_length, 200);
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
        config.query_mode = crate::config::QueryMode::MinimumMatch;
        let fabryk_config = to_fabryk_search_config(&config).expect("config resolution failed");
        assert_eq!(
            fabryk_config.query_mode,
            fabryk::fts::QueryMode::MinimumMatch
        );
    }

    #[test]
    fn test_simple_search_new() {
        let config = test_config("simple");
        let search_config = to_fabryk_search_config(&config.search).unwrap();
        let backend = SimpleSearch::with_default_extractor(&search_config);
        assert_eq!(backend.name(), "simple");
    }

    #[tokio::test]
    async fn test_simple_search_empty_directory() {
        let config = test_config("simple");
        let search_config = to_fabryk_search_config(&config.search).unwrap();
        let backend = SimpleSearch::with_default_extractor(&search_config);

        let params = SearchParams {
            query: "test".to_string(),
            limit: Some(10),
            ..Default::default()
        };

        let result = backend.search(params).await;
        assert!(result.is_ok());
    }
}
