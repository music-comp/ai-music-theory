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
    let index_path = crate::config::resolve_index_path(&config.search)?;

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

    #[test]
    fn test_simple_search_new() {
        let config = crate::config::default_search_config();
        let backend = SimpleSearch::with_default_extractor(&config);
        assert_eq!(backend.name(), "simple");
    }

    #[tokio::test]
    async fn test_simple_search_empty_directory() {
        let config = crate::config::default_search_config();
        let backend = SimpleSearch::with_default_extractor(&config);

        let params = SearchParams {
            query: "test".to_string(),
            limit: Some(10),
            ..Default::default()
        };

        let result = backend.search(params).await;
        assert!(result.is_ok());
    }
}
