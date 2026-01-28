//! Search document representation for full-text search.
//!
//! SearchDocument is the search-ready representation of a concept card.
//! It's used by both the current simple search and future Tantivy indexing.

use crate::error::Result;
use crate::markdown::strip_frontmatter;
use crate::metadata::{ConceptMetadata, UniversalMetadata};
use crate::util::files::read_file;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A search-ready document containing all searchable fields.
///
/// This struct represents a concept card in a format optimized for searching.
/// Fields are organized by their role in search:
/// - Identity fields (id, path) for retrieval
/// - Metadata fields (category, source, tags) for filtering/faceting
/// - Full-text fields (title, description, content) for relevance matching
/// - Structured fields (chapter, part, author, date) for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchDocument {
    // Identity
    pub id: String,
    pub path: String,

    // Metadata (facets for filtering)
    pub category: String,
    pub source: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,

    // Full-text fields (with implicit weights for relevance)
    /// Title field (search weight: 3.0 - title matches are most important)
    pub title: String,
    /// Description field (search weight: 2.0 - description is secondary)
    pub description: String,
    /// Content field (search weight: 1.0 - body content is baseline)
    pub content: String,

    // Structured metadata (stored but not full-text indexed)
    pub chapter: Option<String>,
    pub part: Option<u32>,
    pub author: Option<String>,
    pub date: Option<String>,

    // Content type discrimination (v0.3.0)
    pub content_type: String, // "concept_card" | "source_chapter" | "unified_concept" | "guide"
    pub section: Option<String>, // Fine-grained location: "pp. 23-28" or "Section 2.3"
}

impl SearchDocument {
    /// Create a SearchDocument from ConceptMetadata and file content.
    ///
    /// # Arguments
    ///
    /// * `meta` - Extracted concept metadata
    /// * `file_path` - Path to the concept card file
    ///
    /// # Returns
    ///
    /// Returns a `SearchDocument` ready for searching.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the file cannot be read.
    pub async fn from_metadata(meta: ConceptMetadata, file_path: &Path) -> Result<Self> {
        // Read full file content
        let full_content = read_file(file_path).await?;

        // Strip frontmatter from content (we already have metadata)
        let content = strip_frontmatter(&full_content);

        Ok(SearchDocument {
            id: meta.id,
            path: file_path.to_string_lossy().to_string(),
            category: meta.category,
            source: meta.source.clone(),
            tags: meta.tags.clone(),
            title: meta.title,
            description: meta.description.unwrap_or_default(),
            content: content.to_string(),
            chapter: meta.chapter,
            part: meta.part,
            author: meta.author,
            date: meta.date,
            content_type: "concept_card".to_string(), // v0.3.0: backward compatible default
            section: None,                            // v0.3.0: no section info for concept cards
        })
    }

    /// Create a SearchDocument from UniversalMetadata (v0.3.0).
    ///
    /// # Arguments
    ///
    /// * `meta` - Extracted universal metadata
    /// * `file_path` - Path to the content file
    ///
    /// # Returns
    ///
    /// Returns a `SearchDocument` ready for searching.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the file cannot be read.
    pub async fn from_universal_metadata(
        meta: UniversalMetadata,
        file_path: &Path,
    ) -> Result<Self> {
        // Read full file content
        let full_content = read_file(file_path).await?;

        // Strip frontmatter from content (we already have metadata)
        let content = strip_frontmatter(&full_content);

        Ok(SearchDocument {
            id: meta.id,
            path: file_path.to_string_lossy().to_string(),
            category: meta.category,
            source: meta.source.clone(),
            tags: meta.tags.clone(),
            title: meta.title,
            description: meta.description.unwrap_or_default(),
            content: content.to_string(),
            chapter: meta.chapter,
            part: meta.part,
            author: meta.author,
            date: meta.date,
            content_type: meta.content_type.as_str().to_string(),
            section: meta.section,
        })
    }

    /// Check if this document matches a query (case-insensitive).
    ///
    /// # Arguments
    ///
    /// * `query` - Search query string
    ///
    /// # Returns
    ///
    /// Returns `true` if the query matches any full-text field.
    pub fn matches_query(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        let title_lower = self.title.to_lowercase();
        let desc_lower = self.description.to_lowercase();
        let content_lower = self.content.to_lowercase();

        title_lower.contains(&query_lower)
            || desc_lower.contains(&query_lower)
            || content_lower.contains(&query_lower)
    }

    /// Calculate relevance score for a query.
    ///
    /// Uses weighted field matching:
    /// - Title matches: 3.0x weight
    /// - Description matches: 2.0x weight
    /// - Content matches: 1.0x weight
    /// - Exact word matches: bonus 2.0x
    ///
    /// Score is normalized by document length.
    ///
    /// # Arguments
    ///
    /// * `query` - Search query string
    ///
    /// # Returns
    ///
    /// Returns a relevance score (higher = more relevant).
    pub fn relevance(&self, query: &str) -> f32 {
        let query_lower = query.to_lowercase();
        let mut score = 0.0;

        // Title matches (weight: 3.0)
        let title_lower = self.title.to_lowercase();
        if title_lower.contains(&query_lower) {
            score += 10.0 * 3.0;
        }
        let title_occurrences = title_lower.matches(&query_lower).count() as f32;
        score += title_occurrences * 3.0;

        // Description matches (weight: 2.0)
        let desc_lower = self.description.to_lowercase();
        let desc_occurrences = desc_lower.matches(&query_lower).count() as f32;
        score += desc_occurrences * 2.0;

        // Content matches (weight: 1.0)
        let content_lower = self.content.to_lowercase();
        let content_occurrences = content_lower.matches(&query_lower).count() as f32;
        score += content_occurrences;

        // Exact word match bonus (across all fields)
        let all_text = format!("{} {} {}", title_lower, desc_lower, content_lower);
        let words: Vec<&str> = all_text.split_whitespace().collect();
        let exact_matches = words.iter().filter(|&&w| w == query_lower).count() as f32;
        score += exact_matches * 2.0;

        // Normalize by content length
        let total_len = (self.title.len() + self.description.len() + self.content.len()) as f32;
        if total_len > 0.0 {
            score = score * 1000.0 / total_len;
        }

        score
    }

    /// Extract a snippet around the first query match.
    ///
    /// Searches for the query in description first (more relevant), then content.
    /// Returns a snippet with context around the match.
    ///
    /// # Arguments
    ///
    /// * `query` - Search query string
    /// * `context_chars` - Number of characters of context to include
    ///
    /// # Returns
    ///
    /// Returns a snippet string with "..." indicating truncation.
    pub fn extract_snippet(&self, query: &str, context_chars: usize) -> String {
        // Try to find match in description first (more relevant)
        if let Some(snippet) = Self::find_snippet(&self.description, query, context_chars) {
            return snippet;
        }

        // Fall back to content
        if let Some(snippet) = Self::find_snippet(&self.content, query, context_chars) {
            return snippet;
        }

        // No match found - return first part of description or content
        if !self.description.is_empty() {
            self.description.chars().take(context_chars).collect()
        } else {
            self.content.chars().take(context_chars).collect()
        }
    }

    /// Find a snippet around a query match in text.
    fn find_snippet(text: &str, query: &str, context_chars: usize) -> Option<String> {
        let text_lower = text.to_lowercase();
        let query_lower = query.to_lowercase();

        if let Some(pos) = text_lower.find(&query_lower) {
            // Find character boundaries for safe slicing
            let mut start_byte = pos.saturating_sub(context_chars / 2);
            let mut end_byte = (pos + query.len() + context_chars / 2).min(text.len());

            // Adjust start to character boundary
            while start_byte > 0 && !text.is_char_boundary(start_byte) {
                start_byte -= 1;
            }

            // Adjust end to character boundary
            while end_byte < text.len() && !text.is_char_boundary(end_byte) {
                end_byte += 1;
            }

            let mut snippet = String::with_capacity(end_byte - start_byte + 6);

            if start_byte > 0 {
                snippet.push_str("...");
            }

            let slice = &text[start_byte..end_byte];
            for ch in slice.chars() {
                if ch == '\n' {
                    snippet.push(' ');
                } else {
                    snippet.push(ch);
                }
            }

            if end_byte < text.len() {
                snippet.push_str("...");
            }

            Some(snippet.trim().to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_doc() -> SearchDocument {
        SearchDocument {
            id: "test-doc".to_string(),
            path: "/test.md".to_string(),
            category: "harmony".to_string(),
            source: Some("Test Source".to_string()),
            tags: vec!["test".to_string()],
            title: "Test Document".to_string(),
            description: "A test document about harmony".to_string(),
            content: "This document discusses harmonic concepts in detail.".to_string(),
            chapter: None,
            part: None,
            author: None,
            date: None,
            content_type: "concept_card".to_string(),
            section: None,
        }
    }

    #[test]
    fn test_matches_query_title() {
        let doc = sample_doc();
        assert!(doc.matches_query("document"));
        assert!(doc.matches_query("DOCUMENT"));
    }

    #[test]
    fn test_matches_query_description() {
        let doc = sample_doc();
        assert!(doc.matches_query("harmony"));
    }

    #[test]
    fn test_matches_query_content() {
        let doc = sample_doc();
        assert!(doc.matches_query("harmonic"));
    }

    #[test]
    fn test_matches_query_no_match() {
        let doc = sample_doc();
        assert!(!doc.matches_query("nonexistent"));
    }

    #[test]
    fn test_relevance_title_boost() {
        let doc = sample_doc();
        let score = doc.relevance("document");
        assert!(score > 10.0); // Title match should give high score
    }

    #[test]
    fn test_relevance_content_match() {
        let doc = sample_doc();
        let score = doc.relevance("harmonic");
        assert!(score > 0.0);
    }

    #[test]
    fn test_relevance_multiple_occurrences() {
        let mut doc = sample_doc();
        doc.content = "harmony harmony harmony in music".to_string();
        let score = doc.relevance("harmony");
        assert!(score > 0.0);
    }

    #[test]
    fn test_relevance_exact_word_match_bonus() {
        let doc = sample_doc();
        let score = doc.relevance("harmony");
        // Should get exact word match bonus from description
        assert!(score > 0.0);
    }

    #[test]
    fn test_relevance_no_match() {
        let doc = sample_doc();
        let score = doc.relevance("nonexistent");
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_extract_snippet_from_description() {
        let doc = sample_doc();
        let snippet = doc.extract_snippet("harmony", 20);
        assert!(snippet.contains("harmony"));
    }

    #[test]
    fn test_extract_snippet_from_content() {
        let doc = sample_doc();
        let snippet = doc.extract_snippet("discusses", 30);
        assert!(snippet.contains("discusses"));
    }

    #[test]
    fn test_extract_snippet_no_match() {
        let doc = sample_doc();
        let snippet = doc.extract_snippet("nonexistent", 50);
        // Should return first part of description
        assert!(snippet.contains("test"));
    }

    #[test]
    fn test_extract_snippet_with_ellipsis() {
        let mut doc = sample_doc();
        doc.description = String::new(); // Clear description so it checks content
        doc.content = format!("{} harmony {}", "x".repeat(200), "y".repeat(200));
        let snippet = doc.extract_snippet("harmony", 50);
        assert!(snippet.contains("harmony"));
        // Should have ellipsis at start or end (or both)
        assert!(snippet.starts_with("...") || snippet.ends_with("..."));
    }

    #[test]
    fn test_extract_snippet_newline_replacement() {
        let mut doc = sample_doc();
        doc.description = "First line\nSecond line with harmony\nThird line".to_string();
        let snippet = doc.extract_snippet("harmony", 100);
        assert!(!snippet.contains('\n')); // Newlines should be replaced
        assert!(snippet.contains("harmony"));
    }

    #[test]
    fn test_find_snippet_at_start() {
        let text = "harmony is important in music";
        let snippet = SearchDocument::find_snippet(text, "harmony", 50);
        assert!(snippet.is_some());
        let snippet = snippet.unwrap();
        assert!(snippet.starts_with("harmony"));
        assert!(!snippet.starts_with("..."));
    }

    #[test]
    fn test_find_snippet_at_end() {
        let text = format!("{} harmony", "x".repeat(200));
        let snippet = SearchDocument::find_snippet(&text, "harmony", 50);
        assert!(snippet.is_some());
        let snippet = snippet.unwrap();
        assert!(snippet.ends_with("harmony"));
        assert!(!snippet.ends_with("..."));
    }

    #[test]
    fn test_find_snippet_in_middle() {
        let text = format!("{} harmony {}", "x".repeat(100), "y".repeat(100));
        let snippet = SearchDocument::find_snippet(&text, "harmony", 50);
        assert!(snippet.is_some());
        let snippet = snippet.unwrap();
        assert!(snippet.contains("harmony"));
        assert!(snippet.starts_with("..."));
        assert!(snippet.ends_with("..."));
    }

    #[test]
    fn test_find_snippet_no_match() {
        let text = "This text has no matching term";
        let snippet = SearchDocument::find_snippet(text, "nonexistent", 50);
        assert!(snippet.is_none());
    }

    #[test]
    fn test_search_document_serialization() {
        let doc = sample_doc();
        let json = serde_json::to_string(&doc).expect("Should serialize");
        assert!(json.contains("test-doc"));
        assert!(json.contains("Test Document"));
        assert!(json.contains("harmony"));
    }

    #[test]
    fn test_search_document_deserialization() {
        let json = r#"{
            "id": "test",
            "path": "/test.md",
            "category": "harmony",
            "source": null,
            "tags": [],
            "title": "Test",
            "description": "Test description",
            "content": "Test content",
            "chapter": null,
            "part": null,
            "author": null,
            "date": null,
            "content_type": "concept_card",
            "section": null
        }"#;
        let doc: SearchDocument = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(doc.id, "test");
        assert_eq!(doc.title, "Test");
        assert_eq!(doc.category, "harmony");
        assert_eq!(doc.content_type, "concept_card");
        assert_eq!(doc.section, None);
    }
}
