//! Index building for Tantivy full-text search.
//!
//! This module provides functionality to build a complete Tantivy index
//! from all concept card files in the concept_cards directory.

use std::path::Path;
use std::time::SystemTime;

use crate::config::Config;
use crate::error::Result;
use crate::metadata::{extract_concept_metadata, extract_metadata, ContentType};
use crate::search::{
    compute_content_hash, save_metadata, IndexMetadata, Indexer, SearchDocument, SCHEMA_VERSION,
};
use crate::util::files::{exists, find_all_files, FindOptions};

/// Statistics from index building.
/// Will be used when search backends are implemented (Phase 3+).
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct IndexStats {
    /// Total number of files found across all content types
    pub files_found: usize,
    /// Total number of documents successfully indexed
    pub indexed: usize,
    /// Total number of errors encountered
    pub errors: usize,
    /// Per-type statistics (v0.3.0)
    pub concept_cards: usize,
    pub source_chapters: usize,
    pub unified_concepts: usize,
    pub guides: usize,
}

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

/// Build a complete Tantivy index from all content types (v0.3.0).
///
/// This function:
/// 1. Clears any existing index
/// 2. Scans all 4 content type directories (concept_cards, sources_md, concepts_unified, guides)
/// 3. Extracts metadata and content from each file
/// 4. Adds each document to the Tantivy index
/// 5. Commits the index
///
/// Missing directories are gracefully handled (logged but don't cause failure).
/// Errors for individual files are logged but don't stop the build process.
/// Will be used when search backends are implemented (Phase 3+).
///
/// # Arguments
///
/// * `config` - Server configuration
///
/// # Returns
///
/// Returns `Ok(IndexStats)` with statistics about the build process, including
/// per-type document counts.
///
/// # Errors
///
/// Returns `Err` if:
/// - Index cannot be created or opened
/// - Index commit fails
#[allow(dead_code)]
pub async fn build_index(config: &Config) -> Result<IndexStats> {
    let index_path = config.search.index_path()?;

    log::info!("Building Tantivy index at: {}", index_path.display());

    // Create indexer
    let mut indexer = Indexer::new(&index_path)?;

    // Clear existing index
    log::info!("Clearing existing index...");
    indexer.clear()?;

    let mut stats = IndexStats::default();

    // Index all 4 content types
    stats += index_content_type(
        &mut indexer,
        &config.paths.concept_cards_path()?,
        ContentType::ConceptCard,
    )
    .await?;

    stats += index_content_type(
        &mut indexer,
        &config.paths.sources_md_path()?,
        ContentType::SourceChapter,
    )
    .await?;

    stats += index_content_type(
        &mut indexer,
        &config.paths.concepts_unified_path()?,
        ContentType::UnifiedConcept,
    )
    .await?;

    stats += index_content_type(
        &mut indexer,
        &config.paths.guides_path()?,
        ContentType::Guide,
    )
    .await?;

    // Commit
    log::info!("Committing index...");
    indexer.commit()?;

    // Save metadata for freshness tracking
    let content_hash = compute_content_hash(config).await?;
    let metadata = IndexMetadata {
        schema_version: SCHEMA_VERSION,
        doc_count: stats.indexed,
        last_indexed: SystemTime::now(),
        content_hash,
        concept_cards: stats.concept_cards,
        source_chapters: stats.source_chapters,
        unified_concepts: stats.unified_concepts,
        guides: stats.guides,
    };
    save_metadata(&index_path, &metadata).await?;

    log::info!(
        "Index build complete: {} docs indexed ({} concept_cards, {} sources, {} unified, {} guides), {} errors",
        stats.indexed,
        stats.concept_cards,
        stats.source_chapters,
        stats.unified_concepts,
        stats.guides,
        stats.errors
    );

    Ok(stats)
}

/// Index all files of a specific content type (v0.3.0).
///
/// This helper function handles a single content type directory, with graceful
/// degradation if the directory doesn't exist.
///
/// # Arguments
///
/// * `indexer` - Mutable reference to the Tantivy indexer
/// * `base_path` - Base directory for this content type
/// * `content_type` - Type of content being indexed
///
/// # Returns
///
/// Returns `IndexStats` for this content type (never fails, returns empty stats
/// if directory doesn't exist).
///
/// # Errors
///
/// Returns `Err` only if file operations fail unexpectedly.
async fn index_content_type(
    indexer: &mut Indexer,
    base_path: &Path,
    content_type: ContentType,
) -> Result<IndexStats> {
    // Gracefully handle missing directories
    if !exists(base_path).await {
        log::info!(
            "{:?} directory not found at {}, skipping",
            content_type,
            base_path.display()
        );
        return Ok(IndexStats::default());
    }

    log::info!(
        "Indexing {:?} files from {}...",
        content_type,
        base_path.display()
    );

    let files = find_all_files(base_path, FindOptions::markdown()).await?;
    let files_found = files.len();

    if files_found == 0 {
        log::info!("{:?} directory is empty", content_type);
        return Ok(IndexStats::default());
    }

    log::info!("Found {} {:?} files", files_found, content_type);

    let mut stats = IndexStats {
        files_found,
        ..Default::default()
    };

    for file_info in files {
        let path = &file_info.path;

        match extract_metadata(base_path, path, content_type).await {
            Ok(meta) => match SearchDocument::from_universal_metadata(meta, path).await {
                Ok(doc) => {
                    if let Err(e) = indexer.add_document(&doc) {
                        log::warn!("Failed to index {}: {}", path.display(), e);
                        stats.errors += 1;
                    } else {
                        stats.indexed += 1;
                        // Update per-type counter
                        match content_type {
                            ContentType::ConceptCard => stats.concept_cards += 1,
                            ContentType::SourceChapter => stats.source_chapters += 1,
                            ContentType::UnifiedConcept => stats.unified_concepts += 1,
                            ContentType::Guide => stats.guides += 1,
                        }

                        if stats.indexed.is_multiple_of(50) {
                            log::info!("Indexed {} documents...", stats.indexed);
                        }
                    }
                }
                Err(e) => {
                    log::warn!(
                        "Failed to create SearchDocument for {}: {}",
                        path.display(),
                        e
                    );
                    stats.errors += 1;
                }
            },
            Err(e) => {
                log::warn!("Failed to extract metadata from {}: {}", path.display(), e);
                stats.errors += 1;
            }
        }
    }

    log::info!(
        "Indexed {} {:?} documents ({} errors)",
        match content_type {
            ContentType::ConceptCard => stats.concept_cards,
            ContentType::SourceChapter => stats.source_chapters,
            ContentType::UnifiedConcept => stats.unified_concepts,
            ContentType::Guide => stats.guides,
        },
        content_type,
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
pub async fn build_index_at(index_path: &Path, concept_cards_path: &Path) -> Result<IndexStats> {
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
        concept_cards: indexed, // build_index_at only indexes concept cards
        source_chapters: 0,
        unified_concepts: 0,
        guides: 0,
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
        assert_eq!(stats.concept_cards, 0);
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
        assert_eq!(stats.concept_cards, 1);
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
        assert_eq!(stats.concept_cards, 3);
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
        assert_eq!(stats.concept_cards, 2);
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
            concept_cards: 50,
            source_chapters: 30,
            unified_concepts: 10,
            guides: 5,
        };

        // Verify fields are accessible
        assert_eq!(stats.files_found, 100);
        assert_eq!(stats.indexed, 95);
        assert_eq!(stats.errors, 5);
        assert_eq!(stats.concept_cards, 50);
        assert_eq!(stats.source_chapters, 30);
        assert_eq!(stats.unified_concepts, 10);
        assert_eq!(stats.guides, 5);
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
        assert_eq!(cloned.errors, stats.errors);
        assert_eq!(cloned.concept_cards, stats.concept_cards);
        assert_eq!(cloned.source_chapters, stats.source_chapters);
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
}
