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
    /// Part number
    pub part: Option<u32>,
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
        match serde_yaml::from_str::<Frontmatter>(yaml_text) {
            Ok(fm) => Some(fm),
            Err(e) => {
                // Log warning but don't fail - treat as no frontmatter
                log::warn!("Failed to parse frontmatter: {}", e);
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
        assert_eq!(fm.part, Some(1));
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
}
