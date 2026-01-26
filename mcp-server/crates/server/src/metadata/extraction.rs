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
    /// Part number
    pub part: Option<u32>,
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
/// This function follows a clear precedence strategy:
/// 1. Read frontmatter (primary source for all fields)
/// 2. Fallback to markdown structure (heading for title)
/// 3. Derive from filesystem (directory for category, filename for ID)
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
    let content = read_file(file_path).await?;
    let (frontmatter, body) = extract_frontmatter(&content)?;

    // Extract ID from filename
    let id = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Title: frontmatter.title OR frontmatter.concept OR heading OR filename
    let title = frontmatter
        .as_ref()
        .and_then(|fm| fm.title.clone())
        .or_else(|| frontmatter.as_ref().and_then(|fm| fm.concept.clone()))
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| id.replace(['-', '_'], " "));

    // Category: frontmatter.category OR directory structure
    let category = frontmatter
        .as_ref()
        .and_then(|fm| fm.category.clone())
        .unwrap_or_else(|| extract_category_from_path(base_path, file_path));

    // Source: frontmatter.source
    let source = frontmatter.as_ref().and_then(|fm| fm.source.clone());

    // Other fields from frontmatter
    let fm = frontmatter.unwrap_or_default();

    Ok(ConceptMetadata {
        id,
        title,
        category,
        source,
        chapter: fm.chapter,
        part: fm.part,
        description: fm.description,
        tags: fm.tags,
        author: fm.author,
        date: fm.date,
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
        assert_eq!(meta.part, Some(1));
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
}
