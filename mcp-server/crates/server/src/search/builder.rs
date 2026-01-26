//! Index building for Tantivy full-text search.
//!
//! This module provides functionality to build a complete Tantivy index
//! from all concept card files in the concept_cards directory.

#![cfg(feature = "fts")]

use std::path::Path;
use std::time::SystemTime;

use crate::config::Config;
use crate::error::Result;
use crate::metadata::extract_concept_metadata;
use crate::search::{compute_content_hash, save_metadata, IndexMetadata, Indexer, SearchDocument};
use crate::util::files::{find_all_files, FindOptions};

/// Statistics from index building.
/// Will be used when search backends are implemented (Phase 3+).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IndexStats {
    /// Number of files found
    pub files_found: usize,
    /// Number of documents successfully indexed
    pub indexed: usize,
    /// Number of errors encountered
    pub errors: usize,
}

/// Build a complete Tantivy index from all concept cards.
///
/// This function:
/// 1. Clears any existing index
/// 2. Scans the concept_cards directory for all markdown files
/// 3. Extracts metadata and content from each file
/// 4. Adds each document to the Tantivy index
/// 5. Commits the index
///
/// Errors for individual files are logged but don't stop the build process.
/// Will be used when search backends are implemented (Phase 3+).
///
/// # Arguments
///
/// * `config` - Server configuration
///
/// # Returns
///
/// Returns `Ok(IndexStats)` with statistics about the build process.
///
/// # Errors
///
/// Returns `Err` if:
/// - Index cannot be created or opened
/// - Concept cards directory doesn't exist
/// - Index commit fails
#[allow(dead_code)]
pub async fn build_index(config: &Config) -> Result<IndexStats> {
    let index_path = config.search.index_path()?;
    let concept_cards_path = config.paths.concept_cards_path()?;

    log::info!("Building Tantivy index at: {}", index_path.display());

    // Create indexer
    let mut indexer = Indexer::new(&index_path)?;

    // Clear existing index
    log::info!("Clearing existing index...");
    indexer.clear()?;

    // Find all markdown files
    let files = find_all_files(&concept_cards_path, FindOptions::markdown()).await?;
    let files_found = files.len();
    log::info!("Found {} concept card files", files_found);

    let mut indexed = 0;
    let mut errors = 0;

    // Index each file
    for file_info in files {
        let path = &file_info.path;

        match extract_concept_metadata(&concept_cards_path, path).await {
            Ok(meta) => {
                match SearchDocument::from_metadata(meta, path).await {
                    Ok(doc) => {
                        if let Err(e) = indexer.add_document(&doc) {
                            log::warn!("Failed to index {}: {}", path.display(), e);
                            errors += 1;
                        } else {
                            indexed += 1;
                            if indexed % 50 == 0 {
                                log::info!("Indexed {} documents...", indexed);
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to create SearchDocument for {}: {}",
                            path.display(),
                            e
                        );
                        errors += 1;
                    }
                }
            }
            Err(e) => {
                log::warn!(
                    "Failed to extract metadata from {}: {}",
                    path.display(),
                    e
                );
                errors += 1;
            }
        }
    }

    // Commit
    log::info!("Committing index...");
    indexer.commit()?;

    // Save metadata for freshness tracking
    let content_hash = compute_content_hash(config).await?;
    let metadata = IndexMetadata {
        doc_count: indexed,
        last_indexed: SystemTime::now(),
        content_hash,
    };
    save_metadata(&index_path, &metadata).await?;

    let stats = IndexStats {
        files_found,
        indexed,
        errors,
    };

    log::info!(
        "Index build complete: {} docs indexed, {} errors",
        stats.indexed,
        stats.errors
    );

    Ok(stats)
}

/// Build index at a specific path (for testing).
///
/// This is similar to `build_index()` but allows specifying custom
/// index and concept card paths directly.
///
/// # Arguments
///
/// * `index_path` - Path to the Tantivy index directory
/// * `concept_cards_path` - Path to the concept cards directory
///
/// # Returns
///
/// Returns `Ok(IndexStats)` with statistics about the build process.
///
/// # Errors
///
/// Returns `Err` if index operations or file scanning fails.
#[allow(dead_code)] // Used in tests
pub async fn build_index_at(
    index_path: &Path,
    concept_cards_path: &Path,
) -> Result<IndexStats> {
    log::info!("Building Tantivy index at: {}", index_path.display());

    let mut indexer = Indexer::new(index_path)?;
    indexer.clear()?;

    let files = find_all_files(concept_cards_path, FindOptions::markdown()).await?;
    let files_found = files.len();
    log::info!("Found {} concept card files", files_found);

    let mut indexed = 0;
    let mut errors = 0;

    for file_info in files {
        let path = &file_info.path;

        match extract_concept_metadata(concept_cards_path, path).await {
            Ok(meta) => match SearchDocument::from_metadata(meta, path).await {
                Ok(doc) => {
                    if let Err(e) = indexer.add_document(&doc) {
                        log::warn!("Failed to index {}: {}", path.display(), e);
                        errors += 1;
                    } else {
                        indexed += 1;
                    }
                }
                Err(e) => {
                    log::warn!("Failed to create SearchDocument: {}", e);
                    errors += 1;
                }
            },
            Err(e) => {
                log::warn!("Failed to extract metadata: {}", e);
                errors += 1;
            }
        }
    }

    indexer.commit()?;

    let stats = IndexStats {
        files_found,
        indexed,
        errors,
    };

    log::info!(
        "Index build complete: {} docs indexed, {} errors",
        stats.indexed,
        stats.errors
    );

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_concept_card(dir: &Path, id: &str, title: &str) -> std::io::Result<()> {
        let content = format!(
            r#"---
id: {}
title: {}
category: harmony
---

# {}

This is a test concept card about {}.
"#,
            id, title, title, title
        );

        let file_path = dir.join(format!("{}.md", id));
        fs::write(file_path, content)
    }

    #[tokio::test]
    async fn test_build_index_at_empty_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("index");
        let concepts_path = temp_dir.path().join("concepts");
        fs::create_dir(&concepts_path).expect("Failed to create concepts dir");

        let result = build_index_at(&index_path, &concepts_path).await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_eq!(stats.files_found, 0);
        assert_eq!(stats.indexed, 0);
        assert_eq!(stats.errors, 0);
    }

    #[tokio::test]
    async fn test_build_index_at_single_document() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("index");
        let concepts_path = temp_dir.path().join("concepts");
        fs::create_dir(&concepts_path).expect("Failed to create concepts dir");

        create_test_concept_card(&concepts_path, "test-harmony", "Triads")
            .expect("Failed to create test file");

        let result = build_index_at(&index_path, &concepts_path).await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_eq!(stats.files_found, 1);
        assert_eq!(stats.indexed, 1);
        assert_eq!(stats.errors, 0);
    }

    #[tokio::test]
    async fn test_build_index_at_multiple_documents() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("index");
        let concepts_path = temp_dir.path().join("concepts");
        fs::create_dir(&concepts_path).expect("Failed to create concepts dir");

        create_test_concept_card(&concepts_path, "harmony-1", "Triads")
            .expect("Failed to create test file");
        create_test_concept_card(&concepts_path, "harmony-2", "Seventh Chords")
            .expect("Failed to create test file");
        create_test_concept_card(&concepts_path, "rhythm-1", "Meter")
            .expect("Failed to create test file");

        let result = build_index_at(&index_path, &concepts_path).await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_eq!(stats.files_found, 3);
        assert_eq!(stats.indexed, 3);
        assert_eq!(stats.errors, 0);
    }

    #[tokio::test]
    async fn test_build_index_at_with_invalid_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("index");
        let concepts_path = temp_dir.path().join("concepts");
        fs::create_dir(&concepts_path).expect("Failed to create concepts dir");

        // Create valid file
        create_test_concept_card(&concepts_path, "valid", "Valid Concept")
            .expect("Failed to create test file");

        // Create invalid file (no frontmatter, will use fallback extraction)
        let invalid_path = concepts_path.join("invalid.md");
        fs::write(invalid_path, "# No Frontmatter\n\nJust content.")
            .expect("Failed to create invalid file");

        let result = build_index_at(&index_path, &concepts_path).await;
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_eq!(stats.files_found, 2);
        // Both should be indexed (invalid falls back to filename/heading extraction)
        assert_eq!(stats.indexed, 2);
        assert_eq!(stats.errors, 0);
    }

    #[tokio::test]
    async fn test_build_index_at_rebuild_clears_old() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("index");
        let concepts_path = temp_dir.path().join("concepts");
        fs::create_dir(&concepts_path).expect("Failed to create concepts dir");

        // Build initial index
        create_test_concept_card(&concepts_path, "doc1", "First")
            .expect("Failed to create test file");

        let result1 = build_index_at(&index_path, &concepts_path).await;
        assert!(result1.is_ok());
        let stats1 = result1.unwrap();
        assert_eq!(stats1.indexed, 1);

        // Add another document and rebuild
        create_test_concept_card(&concepts_path, "doc2", "Second")
            .expect("Failed to create test file");

        let result2 = build_index_at(&index_path, &concepts_path).await;
        assert!(result2.is_ok());
        let stats2 = result2.unwrap();
        assert_eq!(stats2.indexed, 2); // Should have both documents
    }

    #[tokio::test]
    async fn test_index_stats_serialization() {
        let stats = IndexStats {
            files_found: 100,
            indexed: 95,
            errors: 5,
        };

        // Verify fields are accessible
        assert_eq!(stats.files_found, 100);
        assert_eq!(stats.indexed, 95);
        assert_eq!(stats.errors, 5);
    }

    #[test]
    fn test_index_stats_clone() {
        let stats = IndexStats {
            files_found: 10,
            indexed: 10,
            errors: 0,
        };

        let cloned = stats.clone();
        assert_eq!(cloned.files_found, stats.files_found);
        assert_eq!(cloned.indexed, stats.indexed);
        assert_eq!(cloned.errors, stats.errors);
    }
}
