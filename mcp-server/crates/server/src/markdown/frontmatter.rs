//! YAML frontmatter parsing for markdown files.

use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Standard frontmatter structure for markdown files.
///
/// This represents the YAML metadata block at the beginning of markdown files,
/// delimited by `---` markers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Frontmatter {
    /// Document title
    pub title: Option<String>,
    /// Document description
    pub description: Option<String>,
    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
    /// Author name
    pub author: Option<String>,
    /// Publication or modification date
    pub date: Option<String>,

    // Music theory specific fields
    /// Canonical concept name
    pub concept: Option<String>,
    /// Thematic category (fundamentals, harmony, voice-leading, etc.)
    pub category: Option<String>,
    /// Source text name (e.g., "Open Music Theory")
    pub source: Option<String>,
    /// Chapter or section reference
    pub chapter: Option<String>,
    /// Section or page range (v0.3.0) - e.g., "pp. 23-28" or "Section 2.3"
    pub section: Option<String>,
    /// Part number or name (e.g., "1", "V. Chromaticism", "Workbook")
    pub part: Option<String>,

    // V3 concept card fields
    /// Machine-readable identifier matching the filename (without .md)
    pub slug: Option<String>,
    /// Finer classification within category
    pub subcategory: Option<String>,
    /// Prerequisite depth: foundational, intermediate, advanced
    pub tier: Option<String>,
    /// Normalized source directory name
    pub source_slug: Option<String>,
    /// Extraction quality: high, medium, low
    pub extraction_confidence: Option<String>,
    /// Alternative names, abbreviations, historical names
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Concept slugs that must be understood first
    #[serde(default)]
    pub prerequisites: Vec<String>,
    /// Concept slugs this builds upon
    #[serde(default)]
    pub extends: Vec<String>,
    /// Associated concept slugs (non-hierarchical)
    #[serde(default)]
    pub related: Vec<String>,
    /// Commonly confused concept slugs
    #[serde(default)]
    pub contrasts_with: Vec<String>,
    /// Competency questions this card answers
    #[serde(default)]
    pub answers_questions: Vec<String>,
    /// Chapter number (integer)
    pub chapter_number: Option<i32>,
    /// PDF page number
    pub pdf_page: Option<i32>,
    /// Multi-author field (v3 uses "authors" plural)
    pub authors: Option<String>,
}

/// Extract YAML frontmatter from markdown content.
///
/// Frontmatter is expected to be at the beginning of the file, delimited by `---`:
///
/// ```markdown
/// ---
/// title: "Example"
/// description: "An example document"
/// ---
/// # Content here
/// ```
///
/// # Arguments
///
/// * `content` - The full markdown file content
///
/// # Returns
///
/// Returns a tuple of `(Option<Frontmatter>, &str)` where:
/// - The first element is `Some(frontmatter)` if valid YAML frontmatter was found, or `None`
/// - The second element is the remaining markdown content after the frontmatter
///
/// # Errors
///
/// Returns `Err` if there is an I/O or parsing error that cannot be recovered from.
/// Invalid YAML in the frontmatter is logged as a warning and treated as if no frontmatter exists.
pub fn extract_frontmatter(content: &str) -> Result<(Option<Frontmatter>, &str)> {
    extract_frontmatter_inner(content, None)
}

/// Extract YAML frontmatter, logging the file path on parse failure.
pub fn extract_frontmatter_with_path<'a>(
    content: &'a str,
    path: &std::path::Path,
) -> Result<(Option<Frontmatter>, &'a str)> {
    extract_frontmatter_inner(content, Some(path))
}

fn extract_frontmatter_inner<'a>(
    content: &'a str,
    path: Option<&std::path::Path>,
) -> Result<(Option<Frontmatter>, &'a str)> {
    // Check if content starts with ---
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return Ok((None, content));
    }

    // Find frontmatter boundaries
    // splitn(3, "---") gives us:
    // [0] = empty (before first ---)
    // [1] = YAML content
    // [2] = remaining markdown content
    let parts: Vec<&str> = trimmed.splitn(3, "---").collect();

    if parts.len() < 3 {
        // No valid frontmatter (missing closing ---)
        return Ok((None, content));
    }

    let yaml_text = parts[1].trim();
    let body = parts[2];

    // Parse YAML
    let frontmatter = if yaml_text.is_empty() {
        None
    } else {
        match yaml_serde::from_str::<Frontmatter>(yaml_text) {
            Ok(fm) => Some(fm),
            Err(e) => {
                // Log warning but don't fail - treat as no frontmatter
                if let Some(p) = path {
                    log::warn!("Failed to parse frontmatter in {}: {}", p.display(), e);
                } else {
                    log::warn!("Failed to parse frontmatter: {}", e);
                }
                None
            }
        }
    };

    Ok((frontmatter, body))
}

/// Extract just the body content (everything after frontmatter).
///
/// # Arguments
///
/// * `content` - The full markdown file content
///
/// # Returns
///
/// Returns the markdown content without the frontmatter block.
pub fn strip_frontmatter(content: &str) -> &str {
    match extract_frontmatter(content) {
        Ok((_, body)) => body,
        Err(_) => content,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_frontmatter_valid() {
        let content = r#"---
title: "Test Title"
description: "Test Description"
tags: ["tag1", "tag2"]
---
# Heading

Content here"#;

        let (fm, body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_some());
        let fm = fm.unwrap();
        assert_eq!(fm.title, Some("Test Title".to_string()));
        assert_eq!(fm.description, Some("Test Description".to_string()));
        assert_eq!(fm.tags, vec!["tag1", "tag2"]);
        assert!(body.contains("# Heading"));
        assert!(body.contains("Content here"));
    }

    #[test]
    fn test_extract_frontmatter_no_frontmatter() {
        let content = "# Just a heading\n\nSome content";
        let (fm, body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_none());
        assert_eq!(body, content);
    }

    #[test]
    fn test_extract_frontmatter_invalid_yaml() {
        let content = "---\ninvalid: yaml: structure: bad\n---\nBody";
        let (fm, body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_none()); // Gracefully handles invalid YAML
        assert!(body.contains("Body"));
    }

    #[test]
    fn test_extract_frontmatter_empty() {
        let content = "---\n---\nContent";
        let (fm, body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_none());
        assert!(body.contains("Content"));
    }

    #[test]
    fn test_extract_frontmatter_missing_closing() {
        let content = "---\ntitle: Test\nNo closing delimiter\n# Heading";
        let (fm, body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_none());
        assert_eq!(body, content);
    }

    #[test]
    fn test_extract_frontmatter_partial_fields() {
        let content = "---\ntitle: \"Only Title\"\n---\nBody text";
        let (fm, body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_some());
        let fm = fm.unwrap();
        assert_eq!(fm.title, Some("Only Title".to_string()));
        assert_eq!(fm.description, None);
        assert!(fm.tags.is_empty());
        assert!(body.contains("Body text"));
    }

    #[test]
    fn test_strip_frontmatter_with_frontmatter() {
        let content = "---\ntitle: Test\n---\nBody";
        let body = strip_frontmatter(content);
        assert!(body.contains("Body"));
        assert!(!body.contains("---"));
        assert!(!body.contains("title:"));
    }

    #[test]
    fn test_strip_frontmatter_without_frontmatter() {
        let content = "# Heading\nContent";
        let body = strip_frontmatter(content);
        assert_eq!(body, content);
    }

    #[test]
    fn test_frontmatter_default() {
        let fm = Frontmatter::default();
        assert_eq!(fm.title, None);
        assert_eq!(fm.description, None);
        assert!(fm.tags.is_empty());
        assert_eq!(fm.author, None);
        assert_eq!(fm.date, None);
        assert_eq!(fm.concept, None);
        assert_eq!(fm.category, None);
        assert_eq!(fm.source, None);
        assert_eq!(fm.chapter, None);
        assert_eq!(fm.part, None);
        // v3 fields
        assert_eq!(fm.slug, None);
        assert_eq!(fm.subcategory, None);
        assert_eq!(fm.tier, None);
        assert_eq!(fm.source_slug, None);
        assert_eq!(fm.extraction_confidence, None);
        assert!(fm.aliases.is_empty());
        assert!(fm.prerequisites.is_empty());
        assert!(fm.extends.is_empty());
        assert!(fm.related.is_empty());
        assert!(fm.contrasts_with.is_empty());
        assert!(fm.answers_questions.is_empty());
        assert_eq!(fm.chapter_number, None);
        assert_eq!(fm.pdf_page, None);
        assert_eq!(fm.authors, None);
    }

    #[test]
    fn test_extract_frontmatter_music_theory_fields() {
        let content = r#"---
title: "Accidental"
description: "Musical symbol that alters pitch"
category: "fundamentals"
concept: "Accidental"
source: "Open Music Theory"
chapter: "Half Steps, Whole Steps, and Accidentals"
part: 1
tags: ["pitch", "notation"]
---
# Accidental

Content here"#;

        let (fm, body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_some());
        let fm = fm.unwrap();
        assert_eq!(fm.title, Some("Accidental".to_string()));
        assert_eq!(fm.category, Some("fundamentals".to_string()));
        assert_eq!(fm.concept, Some("Accidental".to_string()));
        assert_eq!(fm.source, Some("Open Music Theory".to_string()));
        assert_eq!(
            fm.chapter,
            Some("Half Steps, Whole Steps, and Accidentals".to_string())
        );
        assert_eq!(fm.part, Some("1".to_string()));
        assert!(body.contains("# Accidental"));
    }

    #[test]
    fn test_extract_frontmatter_partial_music_theory_fields() {
        let content = r#"---
title: "Test"
category: "harmony"
---
Body"#;

        let (fm, _body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_some());
        let fm = fm.unwrap();
        assert_eq!(fm.title, Some("Test".to_string()));
        assert_eq!(fm.category, Some("harmony".to_string()));
        assert_eq!(fm.concept, None);
        assert_eq!(fm.source, None);
        assert_eq!(fm.chapter, None);
        assert_eq!(fm.part, None);
    }

    #[test]
    fn test_extract_frontmatter_v3_all_fields() {
        let content = r#"---
title: "Acoustic Consonance"
description: "The phenomenon of two or more tones sounding stable together"
concept: "Acoustic Consonance"
category: "acoustics"
source: "Geometry of Music"
chapter: "Consonance and Dissonance"
section: "2.1"
part: "I"
slug: "acoustic-consonance"
subcategory: "consonance-dissonance"
tier: "foundational"
source_slug: "geometry-of-music"
extraction_confidence: "high"
aliases:
  - "sensory consonance"
  - "tonal consonance"
prerequisites:
  - "harmonic-series"
  - "frequency-ratio"
extends:
  - "interval-quality"
related:
  - "roughness"
  - "beating"
contrasts_with:
  - "musical-consonance"
  - "acoustic-dissonance"
answers_questions:
  - "What makes two tones sound consonant?"
  - "How does frequency ratio affect consonance?"
chapter_number: 2
pdf_page: 45
authors: "Dmitri Tymoczko"
tags: ["acoustics", "consonance"]
---
# Acoustic Consonance

Content here"#;

        let (fm, body) = extract_frontmatter(content).unwrap();
        assert!(fm.is_some());
        let fm = fm.unwrap();
        // Existing fields
        assert_eq!(fm.title, Some("Acoustic Consonance".to_string()));
        assert_eq!(fm.category, Some("acoustics".to_string()));
        assert_eq!(fm.source, Some("Geometry of Music".to_string()));
        assert_eq!(fm.chapter, Some("Consonance and Dissonance".to_string()));
        assert_eq!(fm.section, Some("2.1".to_string()));
        assert_eq!(fm.part, Some("I".to_string()));
        // V3 fields
        assert_eq!(fm.slug, Some("acoustic-consonance".to_string()));
        assert_eq!(fm.subcategory, Some("consonance-dissonance".to_string()));
        assert_eq!(fm.tier, Some("foundational".to_string()));
        assert_eq!(fm.source_slug, Some("geometry-of-music".to_string()));
        assert_eq!(fm.extraction_confidence, Some("high".to_string()));
        assert_eq!(fm.aliases, vec!["sensory consonance", "tonal consonance"]);
        assert_eq!(fm.prerequisites, vec!["harmonic-series", "frequency-ratio"]);
        assert_eq!(fm.extends, vec!["interval-quality"]);
        assert_eq!(fm.related, vec!["roughness", "beating"]);
        assert_eq!(
            fm.contrasts_with,
            vec!["musical-consonance", "acoustic-dissonance"]
        );
        assert_eq!(
            fm.answers_questions,
            vec![
                "What makes two tones sound consonant?",
                "How does frequency ratio affect consonance?"
            ]
        );
        assert_eq!(fm.chapter_number, Some(2));
        assert_eq!(fm.pdf_page, Some(45));
        assert_eq!(fm.authors, Some("Dmitri Tymoczko".to_string()));
        assert!(body.contains("# Acoustic Consonance"));
    }

    #[test]
    fn test_extract_frontmatter_v2_card_graceful_defaults() {
        // A v2 card with no v3 fields should parse fine with defaults
        let content = r#"---
title: "Accidental"
description: "Musical symbol that alters pitch"
category: "fundamentals"
concept: "Accidental"
source: "Open Music Theory"
chapter: "Half Steps"
tags: ["pitch"]
---
Body"#;

        let (fm, _body) = extract_frontmatter(content).unwrap();
        let fm = fm.unwrap();
        assert_eq!(fm.title, Some("Accidental".to_string()));
        // All v3 fields should be None/empty
        assert_eq!(fm.slug, None);
        assert_eq!(fm.subcategory, None);
        assert_eq!(fm.tier, None);
        assert_eq!(fm.source_slug, None);
        assert_eq!(fm.extraction_confidence, None);
        assert!(fm.aliases.is_empty());
        assert!(fm.prerequisites.is_empty());
        assert!(fm.extends.is_empty());
        assert!(fm.related.is_empty());
        assert!(fm.contrasts_with.is_empty());
        assert!(fm.answers_questions.is_empty());
        assert_eq!(fm.chapter_number, None);
        assert_eq!(fm.pdf_page, None);
        assert_eq!(fm.authors, None);
    }

    #[test]
    fn test_extract_frontmatter_v3_integer_fields() {
        let content = "---\ntitle: \"Test\"\nchapter_number: 15\npdf_page: 203\n---\nBody";
        let (fm, _) = extract_frontmatter(content).unwrap();
        let fm = fm.unwrap();
        assert_eq!(fm.chapter_number, Some(15));
        assert_eq!(fm.pdf_page, Some(203));
    }

    #[test]
    fn test_extract_frontmatter_v3_empty_arrays() {
        let content = "---\ntitle: \"Test\"\naliases: []\nprerequisites: []\n---\nBody";
        let (fm, _) = extract_frontmatter(content).unwrap();
        let fm = fm.unwrap();
        assert!(fm.aliases.is_empty());
        assert!(fm.prerequisites.is_empty());
    }
}
