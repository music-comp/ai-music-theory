//! Metadata extraction from concept card files.
//!
//! This module provides centralized metadata extraction with a clear precedence:
//! 1. Frontmatter (primary source)
//! 2. Markdown structure (heading, paragraphs)
//! 3. Filesystem (directory structure, filename)

use crate::error::Result;
use crate::markdown::{extract_first_heading, extract_frontmatter};
use crate::util::files::read_file;
use std::path::Path;

/// Content type classification for universal search (v0.3.0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    /// Per-source concept extractions (concept_cards/)
    ConceptCard,
    /// Converted source chapters (sources_md/)
    SourceChapter,
    /// Synthesized cross-source concepts (concepts_unified/)
    UnifiedConcept,
    /// AI-optimized topic guides (guides/)
    Guide,
}

impl ContentType {
    /// Get the string identifier for this content type.
    #[cfg(feature = "fts")]
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::ConceptCard => "concept_card",
            ContentType::SourceChapter => "source_chapter",
            ContentType::UnifiedConcept => "unified_concept",
            ContentType::Guide => "guide",
        }
    }
}

/// Universal metadata for all content types (v0.3.0).
///
/// This struct provides a unified interface for metadata extraction across
/// all 4 content types: concept_cards, sources_md, concepts_unified, guides.
#[derive(Debug, Clone)]
pub struct UniversalMetadata {
    /// Document ID (derived from filename or frontmatter)
    pub id: String,
    /// Document title
    pub title: String,
    /// Thematic category
    pub category: String,
    /// Content type (read in FTS code, written always)
    pub content_type: ContentType,
    /// Source text name (optional)
    pub source: Option<String>,
    /// Chapter or section reference
    pub chapter: Option<String>,
    /// Fine-grained location (page numbers, section numbers)
    pub section: Option<String>,
    /// Part number or name (e.g., "1", "V. Chromaticism", "Workbook")
    pub part: Option<String>,
    /// Description or preview text
    pub description: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Author name
    pub author: Option<String>,
    /// Publication or modification date
    pub date: Option<String>,

    // V3 concept card fields
    /// Machine-readable identifier matching the filename
    pub slug: Option<String>,
    /// Finer classification within category
    pub subcategory: Option<String>,
    /// Prerequisite depth: foundational, intermediate, advanced
    pub tier: Option<String>,
    /// Normalized source directory name
    pub source_slug: Option<String>,
    /// Extraction quality: high, medium, low
    pub extraction_confidence: Option<String>,
    /// Alternative names, abbreviations
    pub aliases: Vec<String>,
    /// Concept slugs that must be understood first
    pub prerequisites: Vec<String>,
    /// Concept slugs this builds upon
    pub extends: Vec<String>,
    /// Associated concept slugs (non-hierarchical)
    pub related: Vec<String>,
    /// Commonly confused concept slugs
    pub contrasts_with: Vec<String>,
    /// Competency questions this card answers
    pub answers_questions: Vec<String>,
    /// Chapter number (integer)
    pub chapter_number: Option<i32>,
    /// PDF page number
    pub pdf_page: Option<i32>,
    /// Multi-author field
    pub authors: Option<String>,
}

/// Extract metadata from any content type (v0.3.0).
///
/// This is the main entry point for metadata extraction. It dispatches to
/// type-specific extractors based on the content_type parameter.
///
/// # Arguments
///
/// * `base_path` - Base directory for the content type
/// * `file_path` - Full path to the content file
/// * `content_type` - Type of content being extracted
///
/// # Returns
///
/// Returns `UniversalMetadata` with all available fields populated.
///
/// # Errors
///
/// Returns `Err` if the file cannot be read or parsed.
pub async fn extract_metadata(
    base_path: &Path,
    file_path: &Path,
    content_type: ContentType,
) -> Result<UniversalMetadata> {
    match content_type {
        ContentType::ConceptCard => extract_concept_card_metadata(base_path, file_path).await,
        ContentType::SourceChapter => extract_source_chapter_metadata(base_path, file_path).await,
        ContentType::UnifiedConcept => extract_unified_concept_metadata(base_path, file_path).await,
        ContentType::Guide => extract_guide_metadata(base_path, file_path).await,
    }
}

/// Extract metadata from a concept card (concept_cards/).
async fn extract_concept_card_metadata(
    base_path: &Path,
    file_path: &Path,
) -> Result<UniversalMetadata> {
    let content = read_file(file_path).await?;
    let (frontmatter, body) = extract_frontmatter(&content)?;

    let id = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    let title = frontmatter
        .as_ref()
        .and_then(|fm| fm.title.clone())
        .or_else(|| frontmatter.as_ref().and_then(|fm| fm.concept.clone()))
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| id.replace(['-', '_'], " "));

    let category = frontmatter
        .as_ref()
        .and_then(|fm| fm.category.clone())
        .unwrap_or_else(|| extract_category_from_path(base_path, file_path));

    let fm = frontmatter.unwrap_or_default();

    Ok(UniversalMetadata {
        id,
        title,
        category,
        content_type: ContentType::ConceptCard,
        source: fm.source,
        chapter: fm.chapter,
        section: None, // Concept cards don't have section information
        part: fm.part,
        description: fm.description,
        tags: fm.tags,
        author: fm.author,
        date: fm.date,
        slug: fm.slug,
        subcategory: fm.subcategory,
        tier: fm.tier,
        source_slug: fm.source_slug,
        extraction_confidence: fm.extraction_confidence,
        aliases: fm.aliases,
        prerequisites: fm.prerequisites,
        extends: fm.extends,
        related: fm.related,
        contrasts_with: fm.contrasts_with,
        answers_questions: fm.answers_questions,
        chapter_number: fm.chapter_number,
        pdf_page: fm.pdf_page,
        authors: fm.authors,
    })
}

/// Extract metadata from a source chapter (sources_md/).
async fn extract_source_chapter_metadata(
    base_path: &Path,
    file_path: &Path,
) -> Result<UniversalMetadata> {
    let content = read_file(file_path).await?;
    let (frontmatter, body) = extract_frontmatter(&content)?;

    let id = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    let title = frontmatter
        .as_ref()
        .and_then(|fm| fm.title.clone())
        .or_else(|| frontmatter.as_ref().and_then(|fm| fm.chapter.clone()))
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| id.replace(['-', '_'], " "));

    // For sources, derive category from first directory component or use frontmatter
    let category = frontmatter
        .as_ref()
        .and_then(|fm| fm.category.clone())
        .unwrap_or_else(|| extract_category_from_path(base_path, file_path));

    let fm = frontmatter.unwrap_or_default();

    Ok(UniversalMetadata {
        id,
        title,
        category,
        content_type: ContentType::SourceChapter,
        source: fm.source,
        chapter: fm.chapter,
        section: fm.section, // Section info (e.g., "pp. 23-28")
        part: fm.part,
        description: fm.description,
        tags: fm.tags,
        author: fm.author,
        date: fm.date,
        slug: fm.slug,
        subcategory: fm.subcategory,
        tier: fm.tier,
        source_slug: fm.source_slug,
        extraction_confidence: fm.extraction_confidence,
        aliases: fm.aliases,
        prerequisites: fm.prerequisites,
        extends: fm.extends,
        related: fm.related,
        contrasts_with: fm.contrasts_with,
        answers_questions: fm.answers_questions,
        chapter_number: fm.chapter_number,
        pdf_page: fm.pdf_page,
        authors: fm.authors,
    })
}

/// Extract metadata from a unified concept (concepts_unified/).
async fn extract_unified_concept_metadata(
    base_path: &Path,
    file_path: &Path,
) -> Result<UniversalMetadata> {
    let content = read_file(file_path).await?;
    let (frontmatter, body) = extract_frontmatter(&content)?;

    let id = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    let title = frontmatter
        .as_ref()
        .and_then(|fm| fm.title.clone())
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| id.replace(['-', '_'], " "));

    let category = frontmatter
        .as_ref()
        .and_then(|fm| fm.category.clone())
        .unwrap_or_else(|| extract_category_from_path(base_path, file_path));

    let fm = frontmatter.unwrap_or_default();

    // For unified concepts, source might be a comma-separated list
    let source = fm.source;

    Ok(UniversalMetadata {
        id,
        title,
        category,
        content_type: ContentType::UnifiedConcept,
        source,
        chapter: fm.chapter,
        section: None, // Unified concepts are synthetic, no section
        part: fm.part,
        description: fm.description,
        tags: fm.tags,
        author: fm.author,
        date: fm.date,
        slug: fm.slug,
        subcategory: fm.subcategory,
        tier: fm.tier,
        source_slug: fm.source_slug,
        extraction_confidence: fm.extraction_confidence,
        aliases: fm.aliases,
        prerequisites: fm.prerequisites,
        extends: fm.extends,
        related: fm.related,
        contrasts_with: fm.contrasts_with,
        answers_questions: fm.answers_questions,
        chapter_number: fm.chapter_number,
        pdf_page: fm.pdf_page,
        authors: fm.authors,
    })
}

/// Extract metadata from a guide (guides/).
async fn extract_guide_metadata(base_path: &Path, file_path: &Path) -> Result<UniversalMetadata> {
    let content = read_file(file_path).await?;
    let (frontmatter, body) = extract_frontmatter(&content)?;

    let id = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    let title = frontmatter
        .as_ref()
        .and_then(|fm| fm.title.clone())
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| id.replace(['-', '_'], " "));

    let category = frontmatter
        .as_ref()
        .and_then(|fm| fm.category.clone())
        .unwrap_or_else(|| extract_category_from_path(base_path, file_path));

    let fm = frontmatter.unwrap_or_default();

    Ok(UniversalMetadata {
        id,
        title,
        category,
        content_type: ContentType::Guide,
        source: fm.source,
        chapter: fm.chapter,
        section: fm.section, // Section references (e.g., "Section 2.3")
        part: fm.part,
        description: fm.description,
        tags: fm.tags,
        author: fm.author,
        date: fm.date,
        slug: fm.slug,
        subcategory: fm.subcategory,
        tier: fm.tier,
        source_slug: fm.source_slug,
        extraction_confidence: fm.extraction_confidence,
        aliases: fm.aliases,
        prerequisites: fm.prerequisites,
        extends: fm.extends,
        related: fm.related,
        contrasts_with: fm.contrasts_with,
        answers_questions: fm.answers_questions,
        chapter_number: fm.chapter_number,
        pdf_page: fm.pdf_page,
        authors: fm.authors,
    })
}

/// Complete metadata for a concept card.
///
/// This struct captures all metadata fields from a concept card file,
/// providing a single source of truth for concept information.
#[derive(Debug, Clone)]
pub struct ConceptMetadata {
    /// Concept ID (derived from filename)
    pub id: String,
    /// Concept title
    pub title: String,
    /// Thematic category (e.g., "fundamentals", "harmony", "voice-leading")
    pub category: String,
    /// Source text name (e.g., "Open Music Theory")
    pub source: Option<String>,
    /// Chapter or section reference
    pub chapter: Option<String>,
    /// Part number or name (e.g., "1", "V. Chromaticism", "Workbook")
    pub part: Option<String>,
    /// Description or preview text
    pub description: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Author name
    pub author: Option<String>,
    /// Publication or modification date
    pub date: Option<String>,
}

/// Extract all metadata from a concept card file.
///
/// This function is maintained for backward compatibility and wraps the new
/// universal metadata extractor.
///
/// # Arguments
///
/// * `base_path` - Base directory for concept cards (used for relative path calculation)
/// * `file_path` - Full path to the concept card file
///
/// # Returns
///
/// Returns `ConceptMetadata` with all available fields populated.
///
/// # Errors
///
/// Returns `Err` if the file cannot be read or parsed.
pub async fn extract_concept_metadata(
    base_path: &Path,
    file_path: &Path,
) -> Result<ConceptMetadata> {
    let universal = extract_metadata(base_path, file_path, ContentType::ConceptCard).await?;

    Ok(ConceptMetadata {
        id: universal.id,
        title: universal.title,
        category: universal.category,
        source: universal.source,
        chapter: universal.chapter,
        part: universal.part,
        description: universal.description,
        tags: universal.tags,
        author: universal.author,
        date: universal.date,
    })
}

/// Extract category from directory structure (fallback).
///
/// Takes the first directory component after the base path.
/// For example, `/base/harmony/triads.md` → `"harmony"`
///
/// # Arguments
///
/// * `base` - Base directory for concept cards
/// * `file_path` - Full path to the concept card file
///
/// # Returns
///
/// Returns the category name, or `"uncategorized"` if no directory structure exists.
fn extract_category_from_path(base: &Path, file_path: &Path) -> String {
    file_path
        .parent()
        .and_then(|parent| parent.strip_prefix(base).ok())
        .and_then(|relative| relative.components().next())
        .and_then(|component| component.as_os_str().to_str())
        .unwrap_or("uncategorized")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use tokio::fs;

    #[test]
    fn test_extract_category_from_path_single_level() {
        let base = PathBuf::from("/concepts");
        let file_path = PathBuf::from("/concepts/harmony/triads.md");
        let category = extract_category_from_path(&base, &file_path);
        assert_eq!(category, "harmony");
    }

    #[test]
    fn test_extract_category_from_path_nested() {
        let base = PathBuf::from("/concepts");
        let file_path = PathBuf::from("/concepts/harmony/advanced/neo-riemannian.md");
        let category = extract_category_from_path(&base, &file_path);
        assert_eq!(category, "harmony");
    }

    #[test]
    fn test_extract_category_from_path_root_level() {
        let base = PathBuf::from("/concepts");
        let file_path = PathBuf::from("/concepts/readme.md");
        let category = extract_category_from_path(&base, &file_path);
        assert_eq!(category, "uncategorized");
    }

    #[test]
    fn test_extract_category_from_path_no_parent() {
        let base = PathBuf::from("/concepts");
        let file_path = PathBuf::from("file.md");
        let category = extract_category_from_path(&base, &file_path);
        assert_eq!(category, "uncategorized");
    }

    #[tokio::test]
    async fn test_extract_concept_metadata_complete_frontmatter() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let harmony_dir = base_path.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        let file_path = harmony_dir.join("accidental.md");
        let content = r#"---
title: "Accidental"
description: "Musical symbol that alters pitch"
category: "fundamentals"
concept: "Accidental"
source: "Open Music Theory"
chapter: "Half Steps, Whole Steps, and Accidentals"
part: 1
tags: ["pitch", "notation"]
author: "Test Author"
date: "2024-01-01"
---
# Accidental

Content here"#;
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_concept_metadata(base_path, &file_path)
            .await
            .unwrap();

        assert_eq!(meta.id, "accidental");
        assert_eq!(meta.title, "Accidental");
        assert_eq!(meta.category, "fundamentals"); // From frontmatter, not directory
        assert_eq!(meta.source, Some("Open Music Theory".to_string()));
        assert_eq!(
            meta.chapter,
            Some("Half Steps, Whole Steps, and Accidentals".to_string())
        );
        assert_eq!(meta.part, Some("1".to_string()));
        assert_eq!(
            meta.description,
            Some("Musical symbol that alters pitch".to_string())
        );
        assert_eq!(meta.tags, vec!["pitch", "notation"]);
        assert_eq!(meta.author, Some("Test Author".to_string()));
        assert_eq!(meta.date, Some("2024-01-01".to_string()));
    }

    #[tokio::test]
    async fn test_extract_concept_metadata_partial_frontmatter() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let harmony_dir = base_path.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        let file_path = harmony_dir.join("triad.md");
        let content = r#"---
title: "Triad Basics"
category: "harmony"
---
# Triad

Content"#;
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_concept_metadata(base_path, &file_path)
            .await
            .unwrap();

        assert_eq!(meta.id, "triad");
        assert_eq!(meta.title, "Triad Basics");
        assert_eq!(meta.category, "harmony");
        assert_eq!(meta.source, None);
        assert_eq!(meta.chapter, None);
        assert_eq!(meta.part, None);
    }

    #[tokio::test]
    async fn test_extract_concept_metadata_no_frontmatter() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let harmony_dir = base_path.join("voice-leading");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        let file_path = harmony_dir.join("parallel-fifths.md");
        let content = "# Parallel Fifths\n\nA voice leading principle.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_concept_metadata(base_path, &file_path)
            .await
            .unwrap();

        assert_eq!(meta.id, "parallel-fifths");
        assert_eq!(meta.title, "Parallel Fifths"); // From heading
        assert_eq!(meta.category, "voice-leading"); // From directory
        assert_eq!(meta.source, None);
        assert_eq!(meta.description, None);
        assert!(meta.tags.is_empty());
    }

    #[tokio::test]
    async fn test_extract_concept_metadata_title_precedence() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("test");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("test-concept.md");

        // Test 1: Frontmatter title takes precedence
        let content = r#"---
title: "Frontmatter Title"
concept: "Concept Name"
---
# Heading Title

Content"#;
        fs::write(&file_path, content).await.unwrap();
        let meta = extract_concept_metadata(base_path, &file_path)
            .await
            .unwrap();
        assert_eq!(meta.title, "Frontmatter Title");

        // Test 2: Concept field used if title missing
        let content = r#"---
concept: "Concept Name"
---
# Heading Title

Content"#;
        fs::write(&file_path, content).await.unwrap();
        let meta = extract_concept_metadata(base_path, &file_path)
            .await
            .unwrap();
        assert_eq!(meta.title, "Concept Name");

        // Test 3: Heading used if no frontmatter title/concept
        let content = "# Heading Title\n\nContent";
        fs::write(&file_path, content).await.unwrap();
        let meta = extract_concept_metadata(base_path, &file_path)
            .await
            .unwrap();
        assert_eq!(meta.title, "Heading Title");

        // Test 4: Filename used as last resort
        let content = "Just plain content without heading";
        fs::write(&file_path, content).await.unwrap();
        let meta = extract_concept_metadata(base_path, &file_path)
            .await
            .unwrap();
        assert_eq!(meta.title, "test concept"); // From filename with dashes replaced
    }

    #[tokio::test]
    async fn test_extract_concept_metadata_category_precedence() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let harmony_dir = base_path.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        let file_path = harmony_dir.join("test.md");

        // Test 1: Frontmatter category takes precedence over directory
        let content = r#"---
title: "Test"
category: "fundamentals"
---
Content"#;
        fs::write(&file_path, content).await.unwrap();
        let meta = extract_concept_metadata(base_path, &file_path)
            .await
            .unwrap();
        assert_eq!(meta.category, "fundamentals"); // From frontmatter, not "harmony"

        // Test 2: Directory used if no frontmatter category
        let content = "# Test\n\nContent";
        fs::write(&file_path, content).await.unwrap();
        let meta = extract_concept_metadata(base_path, &file_path)
            .await
            .unwrap();
        assert_eq!(meta.category, "harmony"); // From directory
    }

    #[tokio::test]
    async fn test_extract_concept_metadata_id_from_filename() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("test");
        fs::create_dir_all(&dir).await.unwrap();

        // Test various filename formats
        let test_cases = vec![
            ("simple.md", "simple"),
            ("with-dashes.md", "with-dashes"),
            ("with_underscores.md", "with_underscores"),
            ("MixedCase.md", "MixedCase"),
        ];

        for (filename, expected_id) in test_cases {
            let file_path = dir.join(filename);
            fs::write(&file_path, "# Test").await.unwrap();
            let meta = extract_concept_metadata(base_path, &file_path)
                .await
                .unwrap();
            assert_eq!(meta.id, expected_id);
        }
    }

    // --- ContentType::as_str tests ---

    #[test]
    #[cfg(feature = "fts")]
    fn test_content_type_as_str_concept_card() {
        assert_eq!(ContentType::ConceptCard.as_str(), "concept_card");
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_content_type_as_str_source_chapter() {
        assert_eq!(ContentType::SourceChapter.as_str(), "source_chapter");
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_content_type_as_str_unified_concept() {
        assert_eq!(ContentType::UnifiedConcept.as_str(), "unified_concept");
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_content_type_as_str_guide() {
        assert_eq!(ContentType::Guide.as_str(), "guide");
    }

    // --- ContentType trait tests ---

    #[test]
    fn test_content_type_debug() {
        let ct = ContentType::ConceptCard;
        let debug_str = format!("{:?}", ct);
        assert_eq!(debug_str, "ConceptCard");
    }

    #[test]
    fn test_content_type_clone() {
        let ct = ContentType::Guide;
        let cloned = ct;
        assert_eq!(ct, cloned);
    }

    #[test]
    fn test_content_type_eq() {
        assert_eq!(ContentType::ConceptCard, ContentType::ConceptCard);
        assert_ne!(ContentType::ConceptCard, ContentType::Guide);
        assert_ne!(ContentType::SourceChapter, ContentType::UnifiedConcept);
    }

    // --- extract_metadata dispatcher tests ---

    #[tokio::test]
    async fn test_extract_metadata_dispatches_concept_card() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("harmony");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("triads.md");
        let content = "---\ntitle: \"Triads\"\n---\n# Triads\n\nContent.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::ConceptCard)
            .await
            .unwrap();
        assert_eq!(meta.content_type, ContentType::ConceptCard);
        assert_eq!(meta.title, "Triads");
    }

    #[tokio::test]
    async fn test_extract_metadata_dispatches_source_chapter() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("theory-book");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("ch01.md");
        let content = "---\ntitle: \"Chapter 1\"\n---\n# Chapter 1\n\nContent.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::SourceChapter)
            .await
            .unwrap();
        assert_eq!(meta.content_type, ContentType::SourceChapter);
        assert_eq!(meta.title, "Chapter 1");
    }

    #[tokio::test]
    async fn test_extract_metadata_dispatches_unified_concept() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("harmony");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("intervals.md");
        let content = "---\ntitle: \"Intervals\"\n---\n# Intervals\n\nContent.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::UnifiedConcept)
            .await
            .unwrap();
        assert_eq!(meta.content_type, ContentType::UnifiedConcept);
        assert_eq!(meta.title, "Intervals");
    }

    #[tokio::test]
    async fn test_extract_metadata_dispatches_guide() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("beginner");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("getting-started.md");
        let content = "---\ntitle: \"Getting Started\"\n---\n# Getting Started\n\nContent.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::Guide)
            .await
            .unwrap();
        assert_eq!(meta.content_type, ContentType::Guide);
        assert_eq!(meta.title, "Getting Started");
    }

    // --- Source chapter metadata tests ---

    #[tokio::test]
    async fn test_extract_source_chapter_metadata_complete() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("music-theory-book");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("chapter-03.md");
        let content = r#"---
title: "Scales and Modes"
category: "fundamentals"
source: "Tonal Harmony"
chapter: "Chapter 3: Scales and Keys"
section: "pp. 45-62"
part: "1"
description: "An introduction to major and minor scales"
tags: ["scales", "modes", "keys"]
author: "Stefan Kostka"
date: "2023-06-15"
---
# Scales and Modes

Detailed content about scales.
"#;
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::SourceChapter)
            .await
            .unwrap();

        assert_eq!(meta.id, "chapter-03");
        assert_eq!(meta.title, "Scales and Modes");
        assert_eq!(meta.category, "fundamentals");
        assert_eq!(meta.content_type, ContentType::SourceChapter);
        assert_eq!(meta.source, Some("Tonal Harmony".to_string()));
        assert_eq!(meta.chapter, Some("Chapter 3: Scales and Keys".to_string()));
        assert_eq!(meta.section, Some("pp. 45-62".to_string()));
        assert_eq!(meta.part, Some("1".to_string()));
        assert_eq!(
            meta.description,
            Some("An introduction to major and minor scales".to_string())
        );
        assert_eq!(meta.tags, vec!["scales", "modes", "keys"]);
        assert_eq!(meta.author, Some("Stefan Kostka".to_string()));
        assert_eq!(meta.date, Some("2023-06-15".to_string()));
    }

    #[tokio::test]
    async fn test_extract_source_chapter_metadata_title_from_chapter_field() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("book");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("ch05.md");
        let content = r#"---
chapter: "Chapter 5: Counterpoint"
---
# Heading Title

Content."#;
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::SourceChapter)
            .await
            .unwrap();

        // Title should come from `chapter` field when `title` is absent
        assert_eq!(meta.title, "Chapter 5: Counterpoint");
    }

    #[tokio::test]
    async fn test_extract_source_chapter_metadata_title_from_heading() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("book");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("ch07.md");
        let content = "# Seventh Chords\n\nContent about seventh chords.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::SourceChapter)
            .await
            .unwrap();

        assert_eq!(meta.title, "Seventh Chords");
        assert_eq!(meta.category, "book"); // from directory
    }

    #[tokio::test]
    async fn test_extract_source_chapter_metadata_title_from_filename() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("book");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("voice-leading_basics.md");
        let content = "Just some content without heading or frontmatter.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::SourceChapter)
            .await
            .unwrap();

        assert_eq!(meta.id, "voice-leading_basics");
        assert_eq!(meta.title, "voice leading basics");
    }

    // --- Unified concept metadata tests ---

    #[tokio::test]
    async fn test_extract_unified_concept_metadata_complete() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("harmony");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("cadences.md");
        let content = r#"---
title: "Cadences"
category: "harmony"
source: "Tonal Harmony, Open Music Theory"
chapter: "Harmonic Progressions"
part: "2"
description: "Unified concept covering authentic, half, and deceptive cadences"
tags: ["cadences", "harmony", "progressions"]
author: "AI Synthesizer"
date: "2024-03-01"
---
# Cadences

Synthesized content about cadences from multiple sources.
"#;
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::UnifiedConcept)
            .await
            .unwrap();

        assert_eq!(meta.id, "cadences");
        assert_eq!(meta.title, "Cadences");
        assert_eq!(meta.category, "harmony");
        assert_eq!(meta.content_type, ContentType::UnifiedConcept);
        assert_eq!(
            meta.source,
            Some("Tonal Harmony, Open Music Theory".to_string())
        );
        assert_eq!(meta.chapter, Some("Harmonic Progressions".to_string()));
        assert_eq!(meta.section, None); // Unified concepts have no section
        assert_eq!(meta.part, Some("2".to_string()));
        assert_eq!(
            meta.description,
            Some("Unified concept covering authentic, half, and deceptive cadences".to_string())
        );
        assert_eq!(meta.tags, vec!["cadences", "harmony", "progressions"]);
        assert_eq!(meta.author, Some("AI Synthesizer".to_string()));
        assert_eq!(meta.date, Some("2024-03-01".to_string()));
    }

    #[tokio::test]
    async fn test_extract_unified_concept_metadata_title_from_heading() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("rhythm");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("syncopation.md");
        let content = "# Syncopation\n\nA unified concept about syncopation.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::UnifiedConcept)
            .await
            .unwrap();

        assert_eq!(meta.title, "Syncopation");
        assert_eq!(meta.category, "rhythm");
    }

    #[tokio::test]
    async fn test_extract_unified_concept_metadata_title_from_filename() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("form");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("sonata-allegro_form.md");
        let content = "Content without heading or frontmatter.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::UnifiedConcept)
            .await
            .unwrap();

        assert_eq!(meta.id, "sonata-allegro_form");
        assert_eq!(meta.title, "sonata allegro form");
    }

    // --- Guide metadata tests ---

    #[tokio::test]
    async fn test_extract_guide_metadata_complete() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("beginner");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("introduction-to-harmony.md");
        let content = r#"---
title: "Introduction to Harmony"
category: "beginner-guides"
source: "AI Generated"
chapter: "Getting Started"
section: "Section 1.1"
part: "1"
description: "A beginner-friendly guide to harmonic concepts"
tags: ["beginner", "harmony", "guide"]
author: "Music Theory AI"
date: "2024-02-15"
---
# Introduction to Harmony

Guide content about harmony basics.
"#;
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::Guide)
            .await
            .unwrap();

        assert_eq!(meta.id, "introduction-to-harmony");
        assert_eq!(meta.title, "Introduction to Harmony");
        assert_eq!(meta.category, "beginner-guides");
        assert_eq!(meta.content_type, ContentType::Guide);
        assert_eq!(meta.source, Some("AI Generated".to_string()));
        assert_eq!(meta.chapter, Some("Getting Started".to_string()));
        assert_eq!(meta.section, Some("Section 1.1".to_string()));
        assert_eq!(meta.part, Some("1".to_string()));
        assert_eq!(
            meta.description,
            Some("A beginner-friendly guide to harmonic concepts".to_string())
        );
        assert_eq!(meta.tags, vec!["beginner", "harmony", "guide"]);
        assert_eq!(meta.author, Some("Music Theory AI".to_string()));
        assert_eq!(meta.date, Some("2024-02-15".to_string()));
    }

    #[tokio::test]
    async fn test_extract_guide_metadata_title_from_heading() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("advanced");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("neo-riemannian-theory.md");
        let content = "# Neo-Riemannian Theory\n\nAdvanced guide content.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::Guide)
            .await
            .unwrap();

        assert_eq!(meta.title, "Neo-Riemannian Theory");
        assert_eq!(meta.category, "advanced");
    }

    #[tokio::test]
    async fn test_extract_guide_metadata_title_from_filename() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("intermediate");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("chord-progressions_guide.md");
        let content = "Plain content without heading or frontmatter.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::Guide)
            .await
            .unwrap();

        assert_eq!(meta.id, "chord-progressions_guide");
        assert_eq!(meta.title, "chord progressions guide");
    }

    #[tokio::test]
    async fn test_extract_guide_metadata_no_frontmatter_minimal() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("tips");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("practice-tips.md");
        let content = "# Practice Tips\n\nSome tips for practice.";
        fs::write(&file_path, content).await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::Guide)
            .await
            .unwrap();

        assert_eq!(meta.id, "practice-tips");
        assert_eq!(meta.title, "Practice Tips");
        assert_eq!(meta.category, "tips");
        assert_eq!(meta.source, None);
        assert_eq!(meta.chapter, None);
        assert_eq!(meta.section, None);
        assert_eq!(meta.part, None);
        assert_eq!(meta.description, None);
        assert!(meta.tags.is_empty());
        assert_eq!(meta.author, None);
        assert_eq!(meta.date, None);
    }

    // --- Error path tests ---

    #[tokio::test]
    async fn test_extract_metadata_nonexistent_file_returns_error() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let file_path = base_path.join("nonexistent.md");

        let result = extract_metadata(base_path, &file_path, ContentType::ConceptCard).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_extract_source_chapter_nonexistent_file_returns_error() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let file_path = base_path.join("nonexistent.md");

        let result = extract_metadata(base_path, &file_path, ContentType::SourceChapter).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_extract_unified_concept_nonexistent_file_returns_error() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let file_path = base_path.join("nonexistent.md");

        let result = extract_metadata(base_path, &file_path, ContentType::UnifiedConcept).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_extract_guide_nonexistent_file_returns_error() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let file_path = base_path.join("nonexistent.md");

        let result = extract_metadata(base_path, &file_path, ContentType::Guide).await;
        assert!(result.is_err());
    }

    // --- UniversalMetadata and ConceptMetadata derive trait tests ---

    #[tokio::test]
    async fn test_universal_metadata_debug_and_clone() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("test");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("test.md");
        fs::write(&file_path, "# Test\n\nContent").await.unwrap();

        let meta = extract_metadata(base_path, &file_path, ContentType::ConceptCard)
            .await
            .unwrap();

        // Test Debug
        let debug_str = format!("{:?}", meta);
        assert!(debug_str.contains("UniversalMetadata"));
        assert!(debug_str.contains("Test"));

        // Test Clone
        let cloned = meta.clone();
        assert_eq!(cloned.id, meta.id);
        assert_eq!(cloned.title, meta.title);
        assert_eq!(cloned.content_type, meta.content_type);
    }

    #[tokio::test]
    async fn test_concept_metadata_debug_and_clone() {
        let temp = TempDir::new().unwrap();
        let base_path = temp.path();
        let dir = base_path.join("test");
        fs::create_dir_all(&dir).await.unwrap();

        let file_path = dir.join("test.md");
        fs::write(&file_path, "# Test\n\nContent").await.unwrap();

        let meta = extract_concept_metadata(base_path, &file_path)
            .await
            .unwrap();

        // Test Debug
        let debug_str = format!("{:?}", meta);
        assert!(debug_str.contains("ConceptMetadata"));

        // Test Clone
        let cloned = meta.clone();
        assert_eq!(cloned.id, meta.id);
        assert_eq!(cloned.title, meta.title);
    }

    // --- Category from path edge cases ---

    #[test]
    fn test_extract_category_from_path_same_base_and_parent() {
        let base = PathBuf::from("/data");
        let file_path = PathBuf::from("/data/file.md");
        let category = extract_category_from_path(&base, &file_path);
        assert_eq!(category, "uncategorized");
    }

    #[test]
    fn test_extract_category_from_path_deeply_nested() {
        let base = PathBuf::from("/data");
        let file_path = PathBuf::from("/data/a/b/c/d/e/file.md");
        let category = extract_category_from_path(&base, &file_path);
        // Should return only the first component
        assert_eq!(category, "a");
    }
}
