//! Extractor implementations for the fabryk content pipeline.
//!
//! This module provides domain-specific extractors that bridge existing music
//! theory content processing with fabryk's generic indexing infrastructure:
//!
//! - [`MusicTheoryDocumentExtractor`] — converts markdown concept cards into
//!   `fabryk::fts::SearchDocument` for full-text search indexing.
//! - [`MusicTheoryGraphExtractor`] — (feature `graph`) extracts nodes and
//!   edges from concept card frontmatter for the knowledge graph.
//! - [`MusicTheoryVectorExtractor`] — (feature `vector`) composes embedding
//!   text from concept card metadata for semantic search.
//!
//! These extractors wrap existing functions in `crate::markdown` and do not
//! replace them; the old code paths remain operational until the migration
//! switchover in a later milestone.

use std::path::Path;

use fabryk::fts::SearchDocument;

use crate::markdown::{extract_first_heading, extract_frontmatter, strip_frontmatter};

// ============================================================================
// DocumentExtractor (always available — used by both simple and FTS search)
// ============================================================================

/// Document extractor for music theory concept cards.
///
/// Parses YAML frontmatter and markdown headings from `.md` files and maps
/// them into `fabryk::fts::SearchDocument` instances suitable for full-text
/// search indexing.
///
/// The extraction logic is available as an inherent method (`extract`) so
/// it can be used with or without the `fts` feature. When the `fts` feature
/// is enabled, this struct also implements `fabryk::fts::DocumentExtractor`.
#[derive(Debug)]
pub struct MusicTheoryDocumentExtractor;

impl MusicTheoryDocumentExtractor {
    /// Create a new document extractor.
    pub fn new() -> Self {
        Self
    }

    /// Extract a `SearchDocument` from a markdown concept card.
    ///
    /// Returns `None` if the file path has no stem or frontmatter parsing
    /// fails irrecoverably.
    pub fn extract(&self, path: &Path, content: &str) -> Option<SearchDocument> {
        let id = path.file_stem()?.to_string_lossy().to_string();

        // Parse frontmatter (gracefully handle failures).
        let (frontmatter, body) = extract_frontmatter(content).ok()?;
        let fm = frontmatter.unwrap_or_default();

        // Title: prefer frontmatter, fall back to first heading, then file stem.
        let title = fm
            .title
            .clone()
            .or_else(|| {
                let stripped = strip_frontmatter(content);
                extract_first_heading(stripped).map(|(_, text)| text)
            })
            .unwrap_or_else(|| id.clone());

        // Category: prefer frontmatter, fall back to parent directory name.
        let category = fm.category.clone().unwrap_or_else(|| {
            path.parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("general")
                .to_string()
        });

        let mut builder = SearchDocument::builder()
            .id(&id)
            .path(path.to_string_lossy())
            .title(&title)
            .category(&category);

        if let Some(ref desc) = fm.description {
            builder = builder.description(desc);
        }
        if let Some(ref source) = fm.source {
            builder = builder.source(source);
        }
        if let Some(ref chapter) = fm.chapter {
            builder = builder.chapter(chapter);
        }
        if let Some(ref part) = fm.part {
            builder = builder.part(part);
        }
        if let Some(ref section) = fm.section {
            builder = builder.section(section);
        }
        // V3: merge aliases and answers_questions into tags for faceted search,
        // and append to content for full-text indexing.
        let mut tags = fm.tags.clone();
        if let Some(ref tier) = fm.tier {
            tags.push(format!("tier:{tier}"));
        }
        if let Some(ref subcat) = fm.subcategory {
            tags.push(format!("subcategory:{subcat}"));
        }
        if let Some(ref confidence) = fm.extraction_confidence {
            tags.push(format!("confidence:{confidence}"));
        }
        if !tags.is_empty() {
            builder = builder.tags(tags);
        }

        // V3: enrich content with aliases and competency questions for FTS.
        let mut enriched_content = body.trim().to_string();
        if !fm.aliases.is_empty() {
            enriched_content.push_str("\n\nAliases: ");
            enriched_content.push_str(&fm.aliases.join(", "));
        }
        if !fm.answers_questions.is_empty() {
            enriched_content.push_str("\n\nCompetency Questions: ");
            enriched_content.push_str(&fm.answers_questions.join(" | "));
        }
        builder = builder.content(&enriched_content);

        // Determine content type from the file path.
        let content_type = Self::detect_content_type(path);
        builder = builder.content_type(&content_type);

        Some(builder.build())
    }

    /// File extensions this extractor supports (without dots).
    pub fn supported_extensions(&self) -> &[&str] {
        &["md"]
    }

    /// Detect the content type from the file path.
    ///
    /// Looks at ancestor directory names to determine whether the file is a
    /// concept card, source chapter, unified concept, or guide.
    fn detect_content_type(path: &Path) -> String {
        // Walk ancestor components looking for known directory names.
        for component in path.components().rev() {
            if let std::path::Component::Normal(name) = component {
                let name = name.to_string_lossy();
                if name.starts_with("concept-cards") || name.starts_with("concept_cards") {
                    return "concept_card".to_string();
                }
                if name.starts_with("sources-md") || name.starts_with("sources_md") {
                    return "source_chapter".to_string();
                }
                if name.starts_with("concepts-unified") || name.starts_with("concepts_unified") {
                    return "unified_concept".to_string();
                }
                if name == "guides" {
                    return "guide".to_string();
                }
            }
        }
        // Default fallback
        "concept_card".to_string()
    }

    /// Check if this extractor supports the given file extension.
    pub fn supports_extension(&self, ext: &str) -> bool {
        self.supported_extensions()
            .iter()
            .any(|e| e.eq_ignore_ascii_case(ext))
    }
}

impl Default for MusicTheoryDocumentExtractor {
    fn default() -> Self {
        Self::new()
    }
}

// When the `fts` feature is enabled (which activates `fts-tantivy` in fabryk),
// implement the `DocumentExtractor` trait by delegating to the inherent methods.
#[cfg(feature = "fts")]
impl fabryk::fts::DocumentExtractor for MusicTheoryDocumentExtractor {
    fn extract(&self, path: &Path, content: &str) -> Option<SearchDocument> {
        // Delegate to the inherent method.
        MusicTheoryDocumentExtractor::extract(self, path, content)
    }

    fn supported_extensions(&self) -> &[&str] {
        MusicTheoryDocumentExtractor::supported_extensions(self)
    }
}

// ============================================================================
// GraphExtractor (feature-gated: graph)
// ============================================================================

#[cfg(feature = "graph")]
pub use graph_extractor::MusicTheoryGraphExtractor;

#[cfg(feature = "graph")]
mod graph_extractor {
    use std::path::Path;

    use fabryk::core::Result;
    use fabryk::graph::{Edge, GraphExtractor, Node, Relationship};
    use serde_json;

    /// Domain-specific node data extracted from a music theory concept card.
    #[derive(Clone, Debug)]
    pub struct MusicTheoryNodeData {
        /// Concept identifier.
        pub id: String,
        /// Human-readable title.
        pub title: String,
        /// Category (e.g., "harmony", "rhythm").
        pub category: Option<String>,
        /// Source text name, if any.
        pub source: Option<String>,
        // V3 metadata fields for graph node enrichment
        /// Prerequisite depth tier.
        pub tier: Option<String>,
        /// Finer classification within category.
        pub subcategory: Option<String>,
        /// Extraction quality.
        pub extraction_confidence: Option<String>,
        /// Alternative names.
        pub aliases: Vec<String>,
    }

    /// Domain-specific edge data extracted from concept card frontmatter.
    #[derive(Clone, Debug)]
    pub struct MusicTheoryEdgeData {
        /// IDs of prerequisite concepts.
        pub prerequisites: Vec<String>,
        /// IDs of related concepts (v2 field name).
        pub related_concepts: Vec<String>,
        /// Concept slugs this builds upon (v3).
        pub extends: Vec<String>,
        /// Commonly confused concept slugs (v3).
        pub contrasts_with: Vec<String>,
        /// Associated concept slugs (v3 field name).
        pub related: Vec<String>,
    }

    /// Graph extractor for music theory concept cards.
    ///
    /// Extracts nodes from frontmatter metadata and edges from the
    /// `prerequisites` and `related_concepts` arrays in YAML frontmatter.
    ///
    /// # Relationship Mapping
    ///
    /// The existing `crate::graph::types::Relationship` variants map to
    /// fabryk's `Relationship` enum as follows:
    ///
    /// | Local | Fabryk |
    /// |-------|--------|
    /// | `Prerequisite` | `Relationship::Prerequisite` |
    /// | `RelatesTo` | `Relationship::RelatesTo` |
    /// | `Extends` | `Relationship::Extends` |
    /// | `Introduces` | `Relationship::Introduces` |
    /// | `Covers` | `Relationship::Covers` |
    /// | `SameAs` | `Relationship::Custom("SameAs")` |
    pub struct MusicTheoryGraphExtractor;

    impl MusicTheoryGraphExtractor {
        /// Create a new graph extractor.
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for MusicTheoryGraphExtractor {
        fn default() -> Self {
            Self::new()
        }
    }

    impl GraphExtractor for MusicTheoryGraphExtractor {
        type NodeData = MusicTheoryNodeData;
        type EdgeData = MusicTheoryEdgeData;

        fn extract_node(
            &self,
            _base_path: &Path,
            file_path: &Path,
            frontmatter: &yaml_serde::Value,
            _content: &str,
        ) -> Result<Self::NodeData> {
            let id = fabryk::core::util::ids::id_from_path(file_path)
                .ok_or_else(|| fabryk::core::Error::parse("cannot derive ID from file path"))?;

            let title = frontmatter
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or(&id)
                .to_string();

            let category = frontmatter
                .get("category")
                .and_then(|v| v.as_str())
                .map(String::from);

            let source = frontmatter
                .get("source")
                .and_then(|v| v.as_str())
                .map(String::from);

            let tier = frontmatter
                .get("tier")
                .and_then(|v| v.as_str())
                .map(String::from);

            let subcategory = frontmatter
                .get("subcategory")
                .and_then(|v| v.as_str())
                .map(String::from);

            let extraction_confidence = frontmatter
                .get("extraction_confidence")
                .and_then(|v| v.as_str())
                .map(String::from);

            let aliases = extract_string_array(frontmatter, "aliases");

            Ok(MusicTheoryNodeData {
                id,
                title,
                category,
                source,
                tier,
                subcategory,
                extraction_confidence,
                aliases,
            })
        }

        fn extract_edges(
            &self,
            frontmatter: &yaml_serde::Value,
            _content: &str,
        ) -> Result<Option<Self::EdgeData>> {
            let prerequisites = extract_string_array(frontmatter, "prerequisites");
            let related_concepts = extract_string_array(frontmatter, "related_concepts");
            let extends = extract_string_array(frontmatter, "extends");
            let contrasts_with = extract_string_array(frontmatter, "contrasts_with");
            let related = extract_string_array(frontmatter, "related");

            let has_edges = !prerequisites.is_empty()
                || !related_concepts.is_empty()
                || !extends.is_empty()
                || !contrasts_with.is_empty()
                || !related.is_empty();

            if has_edges {
                Ok(Some(MusicTheoryEdgeData {
                    prerequisites,
                    related_concepts,
                    extends,
                    contrasts_with,
                    related,
                }))
            } else {
                Ok(None)
            }
        }

        fn to_graph_node(&self, node_data: &Self::NodeData) -> Node {
            let mut node = Node::new(&node_data.id, &node_data.title);
            if let Some(ref cat) = node_data.category {
                node = node.with_category(cat);
            }
            if let Some(ref source) = node_data.source {
                node = node.with_source(source);
            }
            // V3 metadata enrichment
            if let Some(ref tier) = node_data.tier {
                node = node.with_metadata("tier", serde_json::Value::String(tier.clone()));
            }
            if let Some(ref confidence) = node_data.extraction_confidence {
                node = node.with_metadata(
                    "extraction_confidence",
                    serde_json::Value::String(confidence.clone()),
                );
            }
            if let Some(ref subcat) = node_data.subcategory {
                node = node.with_metadata("subcategory", serde_json::Value::String(subcat.clone()));
            }
            if !node_data.aliases.is_empty() {
                node = node.with_metadata("aliases", serde_json::json!(node_data.aliases));
            }
            node
        }

        fn to_graph_edges(&self, from_id: &str, edge_data: &Self::EdgeData) -> Vec<Edge> {
            let mut edges = Vec::new();

            for prereq in &edge_data.prerequisites {
                edges.push(Edge::new(from_id, prereq, Relationship::Prerequisite));
            }

            // V2 field name (backward compat)
            for related in &edge_data.related_concepts {
                edges.push(Edge::new(from_id, related, Relationship::RelatesTo));
            }

            // V3 field names
            for ext in &edge_data.extends {
                edges.push(Edge::new(from_id, ext, Relationship::Extends));
            }
            for contrast in &edge_data.contrasts_with {
                edges.push(Edge::new(from_id, contrast, Relationship::ContrastsWith));
            }
            for rel in &edge_data.related {
                edges.push(Edge::new(from_id, rel, Relationship::RelatesTo));
            }

            edges
        }

        fn content_glob(&self) -> &str {
            "**/*.md"
        }

        fn name(&self) -> &str {
            "music-theory"
        }
    }

    /// Extract a `Vec<String>` from a YAML frontmatter sequence field.
    fn extract_string_array(frontmatter: &yaml_serde::Value, key: &str) -> Vec<String> {
        frontmatter
            .get(key)
            .and_then(|v| v.as_sequence())
            .map(|seq| {
                seq.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default()
    }
}

// ============================================================================
// VectorExtractor (feature-gated: vector)
// ============================================================================

#[cfg(feature = "vector")]
pub use vector_extractor::MusicTheoryVectorExtractor;

#[cfg(feature = "vector")]
mod vector_extractor {
    use std::path::Path;

    use fabryk::core::Result;
    use fabryk::vector::{VectorDocument, VectorExtractor};

    /// Vector extractor for music theory concept cards.
    ///
    /// Composes embedding text from concept card metadata in the format:
    ///
    /// ```text
    /// title | category | description | content body
    /// ```
    ///
    /// This ordering places the most semantically significant tokens
    /// (title, category) at the beginning of the text, giving them
    /// slightly higher influence in typical embedding models.
    pub struct MusicTheoryVectorExtractor;

    impl MusicTheoryVectorExtractor {
        /// Create a new vector extractor.
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for MusicTheoryVectorExtractor {
        fn default() -> Self {
            Self::new()
        }
    }

    impl VectorExtractor for MusicTheoryVectorExtractor {
        fn extract_document(
            &self,
            _base_path: &Path,
            file_path: &Path,
            frontmatter: &yaml_serde::Value,
            content: &str,
        ) -> Result<VectorDocument> {
            let id = fabryk::core::util::ids::id_from_path(file_path)
                .ok_or_else(|| fabryk::core::Error::parse("cannot derive ID from file path"))?;

            let title = frontmatter
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or(&id);

            let category = frontmatter.get("category").and_then(|v| v.as_str());

            let description = frontmatter.get("description").and_then(|v| v.as_str());

            let subcategory = frontmatter.get("subcategory").and_then(|v| v.as_str());

            let aliases: Vec<String> = frontmatter
                .get("aliases")
                .and_then(|v| v.as_sequence())
                .map(|seq| {
                    seq.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default();

            let answers_questions: Vec<String> = frontmatter
                .get("answers_questions")
                .and_then(|v| v.as_sequence())
                .map(|seq| {
                    seq.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default();

            // Compose embedding text: title | category | subcategory | description |
            // aliases | answers_questions | body
            let mut parts = vec![title.to_string()];
            if let Some(cat) = category {
                parts.push(cat.to_string());
            }
            if let Some(subcat) = subcategory {
                parts.push(subcat.to_string());
            }
            if let Some(desc) = description {
                parts.push(desc.to_string());
            }
            if !aliases.is_empty() {
                parts.push(aliases.join(", "));
            }
            if !answers_questions.is_empty() {
                parts.push(answers_questions.join(" | "));
            }
            parts.push(content.trim().to_string());

            let text = parts.join(" | ");

            let mut doc = VectorDocument::new(id, text);
            if let Some(cat) = category {
                doc = doc.with_category(cat);
            }
            if let Some(source) = frontmatter.get("source").and_then(|v| v.as_str()) {
                doc = doc.with_metadata("source", source);
            }
            if let Some(chapter) = frontmatter.get("chapter").and_then(|v| v.as_str()) {
                doc = doc.with_metadata("chapter", chapter);
            }

            Ok(doc)
        }

        fn content_glob(&self) -> &str {
            "**/*.md"
        }

        fn name(&self) -> &str {
            "music-theory"
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // -----------------------------------------------------------------------
    // MusicTheoryDocumentExtractor tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_document_extractor_supported_extensions() {
        let extractor = MusicTheoryDocumentExtractor::new();
        assert_eq!(extractor.supported_extensions(), &["md"]);
    }

    #[test]
    fn test_document_extractor_supports_extension() {
        let extractor = MusicTheoryDocumentExtractor::default();
        assert!(extractor.supports_extension("md"));
        assert!(extractor.supports_extension("MD"));
        assert!(!extractor.supports_extension("txt"));
        assert!(!extractor.supports_extension("pdf"));
    }

    #[test]
    fn test_document_extractor_full_frontmatter() {
        let extractor = MusicTheoryDocumentExtractor::new();
        let path = PathBuf::from("/content/concept-cards/harmony/major-triad.md");
        let content = r#"---
title: "Major Triad"
description: "A three-note chord built on major and minor thirds"
category: "harmony"
source: "Open Music Theory"
chapter: "Triads and Seventh Chords"
part: "II"
section: "pp. 45-52"
tags: ["chord", "fundamentals"]
---

# Major Triad

The major triad consists of a root, major third, and perfect fifth."#;

        let doc = extractor.extract(&path, content);
        assert!(doc.is_some());

        let doc = doc.unwrap();
        assert_eq!(doc.id, "major-triad");
        assert_eq!(doc.title, "Major Triad");
        assert_eq!(doc.category, "harmony");
        assert_eq!(
            doc.description,
            Some("A three-note chord built on major and minor thirds".to_string())
        );
        assert_eq!(doc.source, Some("Open Music Theory".to_string()));
        assert_eq!(doc.chapter, Some("Triads and Seventh Chords".to_string()));
        assert_eq!(doc.part, Some("II".to_string()));
        assert_eq!(doc.section, Some("pp. 45-52".to_string()));
        assert_eq!(doc.tags, vec!["chord", "fundamentals"]);
        assert_eq!(doc.content_type, Some("concept_card".to_string()));
        assert!(doc.content.contains("major triad consists of"));
    }

    #[test]
    fn test_document_extractor_minimal_frontmatter() {
        let extractor = MusicTheoryDocumentExtractor::new();
        let path = PathBuf::from("/content/harmony/intervals.md");
        let content = "---\ntitle: Intervals\n---\n\n# Intervals\n\nContent here.";

        let doc = extractor.extract(&path, content).unwrap();
        assert_eq!(doc.id, "intervals");
        assert_eq!(doc.title, "Intervals");
        // Falls back to parent dir for category
        assert_eq!(doc.category, "harmony");
        assert!(doc.description.is_none());
        assert!(doc.source.is_none());
    }

    #[test]
    fn test_document_extractor_no_frontmatter() {
        let extractor = MusicTheoryDocumentExtractor::new();
        let path = PathBuf::from("/content/rhythm/meter.md");
        let content = "# Meter\n\nMeter is the pattern of beats.";

        let doc = extractor.extract(&path, content).unwrap();
        assert_eq!(doc.id, "meter");
        // Falls back to heading for title
        assert_eq!(doc.title, "Meter");
        // Falls back to parent dir for category
        assert_eq!(doc.category, "rhythm");
    }

    #[test]
    fn test_document_extractor_no_title_no_heading() {
        let extractor = MusicTheoryDocumentExtractor::new();
        let path = PathBuf::from("/content/general/something.md");
        let content = "Just plain text with no heading or frontmatter.";

        let doc = extractor.extract(&path, content).unwrap();
        // Falls back to file stem for title
        assert_eq!(doc.title, "something");
        assert_eq!(doc.category, "general");
    }

    #[test]
    fn test_document_extractor_path_provides_id() {
        let extractor = MusicTheoryDocumentExtractor::new();
        let path = PathBuf::from("/deep/nested/path/my-concept.md");
        let content = "---\ntitle: My Concept\ncategory: test\n---\nBody.";

        let doc = extractor.extract(&path, content).unwrap();
        assert_eq!(doc.id, "my-concept");
        assert!(doc.path.contains("my-concept.md"));
    }

    #[test]
    fn test_document_extractor_empty_tags() {
        let extractor = MusicTheoryDocumentExtractor::new();
        let path = PathBuf::from("/content/test/doc.md");
        let content = "---\ntitle: Test\ncategory: test\n---\nBody.";

        let doc = extractor.extract(&path, content).unwrap();
        assert!(doc.tags.is_empty());
    }

    #[test]
    fn test_document_extractor_v3_enrichment() {
        let extractor = MusicTheoryDocumentExtractor::new();
        let path = PathBuf::from("/content/concept-cards/acoustics/consonance.md");
        let content = r#"---
title: "Acoustic Consonance"
category: "acoustics"
tier: "foundational"
subcategory: "consonance-dissonance"
extraction_confidence: "high"
aliases:
  - "sensory consonance"
  - "tonal consonance"
answers_questions:
  - "What makes two tones sound consonant?"
  - "How does frequency ratio affect consonance?"
tags: ["acoustics"]
---
The phenomenon of stability."#;

        let doc = extractor.extract(&path, content).unwrap();
        // V3 fields encoded into tags
        assert!(doc.tags.contains(&"tier:foundational".to_string()));
        assert!(doc
            .tags
            .contains(&"subcategory:consonance-dissonance".to_string()));
        assert!(doc.tags.contains(&"confidence:high".to_string()));
        assert!(doc.tags.contains(&"acoustics".to_string()));
        // V3 fields appended to content for FTS
        assert!(doc.content.contains("sensory consonance"));
        assert!(doc.content.contains("tonal consonance"));
        assert!(doc
            .content
            .contains("What makes two tones sound consonant?"));
        assert!(doc.content.contains("The phenomenon of stability."));
    }

    #[test]
    fn test_document_extractor_v3_no_enrichment_for_v2() {
        let extractor = MusicTheoryDocumentExtractor::new();
        let path = PathBuf::from("/content/concept-cards/harmony/triad.md");
        let content = "---\ntitle: Triad\ncategory: harmony\n---\nA three-note chord.";

        let doc = extractor.extract(&path, content).unwrap();
        // No v3 tags added
        assert!(doc.tags.is_empty());
        // Content is just the body
        assert_eq!(doc.content, "A three-note chord.");
    }

    // -----------------------------------------------------------------------
    // GraphExtractor tests (feature-gated)
    // -----------------------------------------------------------------------

    #[cfg(feature = "graph")]
    mod graph_tests {
        use super::super::graph_extractor::*;
        use fabryk::graph::{GraphExtractor, Relationship};
        use std::path::PathBuf;

        fn sample_frontmatter() -> yaml_serde::Value {
            yaml_serde::from_str(
                r#"
title: "Voice Leading"
category: "counterpoint"
source: "Open Music Theory"
prerequisites:
  - intervals
  - scales
related_concepts:
  - chord-voicing
  - part-writing
"#,
            )
            .unwrap()
        }

        #[test]
        fn test_graph_extractor_extract_node() {
            let extractor = MusicTheoryGraphExtractor::new();
            let base_path = PathBuf::from("/data/concepts");
            let file_path = PathBuf::from("/data/concepts/counterpoint/voice-leading.md");
            let fm = sample_frontmatter();

            let node_data = extractor
                .extract_node(&base_path, &file_path, &fm, "body content")
                .unwrap();

            assert_eq!(node_data.id, "voice-leading");
            assert_eq!(node_data.title, "Voice Leading");
            assert_eq!(node_data.category, Some("counterpoint".to_string()));
            assert_eq!(node_data.source, Some("Open Music Theory".to_string()));
        }

        #[test]
        fn test_graph_extractor_extract_node_minimal() {
            let extractor = MusicTheoryGraphExtractor::new();
            let base_path = PathBuf::from("/data");
            let file_path = PathBuf::from("/data/simple.md");
            let fm: yaml_serde::Value = yaml_serde::from_str("title: Simple").unwrap();

            let node_data = extractor
                .extract_node(&base_path, &file_path, &fm, "")
                .unwrap();

            assert_eq!(node_data.id, "simple");
            assert_eq!(node_data.title, "Simple");
            assert!(node_data.category.is_none());
            assert!(node_data.source.is_none());
        }

        #[test]
        fn test_graph_extractor_extract_node_no_title() {
            let extractor = MusicTheoryGraphExtractor::new();
            let base_path = PathBuf::from("/data");
            let file_path = PathBuf::from("/data/untitled-concept.md");
            let fm: yaml_serde::Value = yaml_serde::from_str("category: test").unwrap();

            let node_data = extractor
                .extract_node(&base_path, &file_path, &fm, "")
                .unwrap();

            // Falls back to file stem as title
            assert_eq!(node_data.title, "untitled-concept");
        }

        #[test]
        fn test_graph_extractor_extract_edges() {
            let extractor = MusicTheoryGraphExtractor::new();
            let fm = sample_frontmatter();

            let edge_data = extractor.extract_edges(&fm, "content").unwrap().unwrap();

            assert_eq!(edge_data.prerequisites, vec!["intervals", "scales"]);
            assert_eq!(
                edge_data.related_concepts,
                vec!["chord-voicing", "part-writing"]
            );
        }

        #[test]
        fn test_graph_extractor_extract_edges_none() {
            let extractor = MusicTheoryGraphExtractor::new();
            let fm: yaml_serde::Value = yaml_serde::from_str("title: Leaf").unwrap();

            let edge_data = extractor.extract_edges(&fm, "content").unwrap();
            assert!(edge_data.is_none());
        }

        #[test]
        fn test_graph_extractor_extract_edges_only_prerequisites() {
            let extractor = MusicTheoryGraphExtractor::new();
            let fm: yaml_serde::Value = yaml_serde::from_str(
                r#"
prerequisites:
  - pitch
"#,
            )
            .unwrap();

            let edge_data = extractor.extract_edges(&fm, "").unwrap().unwrap();
            assert_eq!(edge_data.prerequisites, vec!["pitch"]);
            assert!(edge_data.related_concepts.is_empty());
        }

        #[test]
        fn test_graph_extractor_to_graph_node() {
            let extractor = MusicTheoryGraphExtractor::default();
            let node_data = MusicTheoryNodeData {
                id: "test-id".to_string(),
                title: "Test Title".to_string(),
                category: Some("harmony".to_string()),
                source: Some("OMT".to_string()),
                tier: None,
                subcategory: None,
                extraction_confidence: None,
                aliases: vec![],
            };

            let node = extractor.to_graph_node(&node_data);
            assert_eq!(node.id, "test-id");
            assert_eq!(node.title, "Test Title");
            assert_eq!(node.category, Some("harmony".to_string()));
            assert_eq!(node.source_id, Some("OMT".to_string()));
        }

        #[test]
        fn test_graph_extractor_to_graph_node_no_optionals() {
            let extractor = MusicTheoryGraphExtractor::new();
            let node_data = MusicTheoryNodeData {
                id: "x".to_string(),
                title: "X".to_string(),
                category: None,
                source: None,
                tier: None,
                subcategory: None,
                extraction_confidence: None,
                aliases: vec![],
            };

            let node = extractor.to_graph_node(&node_data);
            assert!(node.category.is_none());
            assert!(node.source_id.is_none());
        }

        #[test]
        fn test_graph_extractor_to_graph_edges() {
            let extractor = MusicTheoryGraphExtractor::new();
            let edge_data = MusicTheoryEdgeData {
                prerequisites: vec!["a".to_string(), "b".to_string()],
                related_concepts: vec!["x".to_string()],
                extends: vec![],
                contrasts_with: vec![],
                related: vec![],
            };

            let edges = extractor.to_graph_edges("from-node", &edge_data);
            assert_eq!(edges.len(), 3);

            assert!(edges
                .iter()
                .any(|e| e.to == "a" && e.relationship == Relationship::Prerequisite));
            assert!(edges
                .iter()
                .any(|e| e.to == "b" && e.relationship == Relationship::Prerequisite));
            assert!(edges
                .iter()
                .any(|e| e.to == "x" && e.relationship == Relationship::RelatesTo));

            // All should have correct from_id
            assert!(edges.iter().all(|e| e.from == "from-node"));
        }

        #[test]
        fn test_graph_extractor_to_graph_edges_empty() {
            let extractor = MusicTheoryGraphExtractor::new();
            let edge_data = MusicTheoryEdgeData {
                prerequisites: vec![],
                related_concepts: vec![],
                extends: vec![],
                contrasts_with: vec![],
                related: vec![],
            };

            let edges = extractor.to_graph_edges("node", &edge_data);
            assert!(edges.is_empty());
        }

        #[test]
        fn test_graph_extractor_defaults() {
            let extractor = MusicTheoryGraphExtractor::new();
            assert_eq!(extractor.content_glob(), "**/*.md");
            assert_eq!(extractor.name(), "music-theory");
        }

        #[test]
        fn test_graph_extractor_v3_node_metadata() {
            let extractor = MusicTheoryGraphExtractor::new();
            let base_path = PathBuf::from("/data");
            let file_path = PathBuf::from("/data/test.md");
            let fm: yaml_serde::Value = yaml_serde::from_str(
                r#"
title: "Acoustic Consonance"
category: "acoustics"
tier: "foundational"
subcategory: "consonance-dissonance"
extraction_confidence: "high"
aliases:
  - "sensory consonance"
  - "tonal consonance"
"#,
            )
            .unwrap();

            let node_data = extractor
                .extract_node(&base_path, &file_path, &fm, "")
                .unwrap();

            assert_eq!(node_data.tier, Some("foundational".to_string()));
            assert_eq!(
                node_data.subcategory,
                Some("consonance-dissonance".to_string())
            );
            assert_eq!(node_data.extraction_confidence, Some("high".to_string()));
            assert_eq!(
                node_data.aliases,
                vec!["sensory consonance", "tonal consonance"]
            );

            // Verify metadata is set on the graph node
            let node = extractor.to_graph_node(&node_data);
            assert_eq!(
                node.metadata.get("tier").unwrap(),
                &serde_json::Value::String("foundational".to_string())
            );
            assert_eq!(
                node.metadata.get("subcategory").unwrap(),
                &serde_json::Value::String("consonance-dissonance".to_string())
            );
            assert_eq!(
                node.metadata.get("extraction_confidence").unwrap(),
                &serde_json::Value::String("high".to_string())
            );
            assert_eq!(
                node.metadata.get("aliases").unwrap(),
                &serde_json::json!(["sensory consonance", "tonal consonance"])
            );
        }

        #[test]
        fn test_graph_extractor_v3_edges() {
            let extractor = MusicTheoryGraphExtractor::new();
            let fm: yaml_serde::Value = yaml_serde::from_str(
                r#"
prerequisites:
  - harmonic-series
extends:
  - interval-quality
contrasts_with:
  - acoustic-dissonance
related:
  - roughness
"#,
            )
            .unwrap();

            let edge_data = extractor.extract_edges(&fm, "").unwrap().unwrap();
            assert_eq!(edge_data.prerequisites, vec!["harmonic-series"]);
            assert_eq!(edge_data.extends, vec!["interval-quality"]);
            assert_eq!(edge_data.contrasts_with, vec!["acoustic-dissonance"]);
            assert_eq!(edge_data.related, vec!["roughness"]);

            let edges = extractor.to_graph_edges("source-node", &edge_data);
            assert_eq!(edges.len(), 4);
            assert!(
                edges
                    .iter()
                    .any(|e| e.to == "harmonic-series"
                        && e.relationship == Relationship::Prerequisite)
            );
            assert!(edges
                .iter()
                .any(|e| e.to == "interval-quality" && e.relationship == Relationship::Extends));
            assert!(edges
                .iter()
                .any(|e| e.to == "acoustic-dissonance"
                    && e.relationship == Relationship::ContrastsWith));
            assert!(edges
                .iter()
                .any(|e| e.to == "roughness" && e.relationship == Relationship::RelatesTo));
        }
    }

    // -----------------------------------------------------------------------
    // VectorExtractor tests (feature-gated)
    // -----------------------------------------------------------------------

    #[cfg(feature = "vector")]
    mod vector_tests {
        use super::super::vector_extractor::*;
        use fabryk::vector::VectorExtractor;
        use std::path::PathBuf;

        fn sample_frontmatter() -> yaml_serde::Value {
            yaml_serde::from_str(
                r#"
title: "Major Scale"
category: "fundamentals"
description: "The most common scale in Western music"
source: "Open Music Theory"
chapter: "Scales and Scale Degrees"
"#,
            )
            .unwrap()
        }

        #[test]
        fn test_vector_extractor_full_metadata() {
            let extractor = MusicTheoryVectorExtractor::new();
            let base_path = PathBuf::from("/data/concepts");
            let file_path = PathBuf::from("/data/concepts/fundamentals/major-scale.md");
            let fm = sample_frontmatter();
            let body = "The major scale is a diatonic scale with seven notes.";

            let doc = extractor
                .extract_document(&base_path, &file_path, &fm, body)
                .unwrap();

            assert_eq!(doc.id, "major-scale");
            assert!(doc.text.contains("Major Scale"));
            assert!(doc.text.contains("fundamentals"));
            assert!(doc.text.contains("The most common scale"));
            assert!(doc.text.contains("diatonic scale"));
            assert_eq!(doc.category, Some("fundamentals".to_string()));
            assert_eq!(doc.metadata.get("source").unwrap(), "Open Music Theory");
            assert_eq!(
                doc.metadata.get("chapter").unwrap(),
                "Scales and Scale Degrees"
            );
        }

        #[test]
        fn test_vector_extractor_minimal_metadata() {
            let extractor = MusicTheoryVectorExtractor::default();
            let base_path = PathBuf::from("/data");
            let file_path = PathBuf::from("/data/simple.md");
            let fm: yaml_serde::Value = yaml_serde::from_str("title: Simple").unwrap();

            let doc = extractor
                .extract_document(&base_path, &file_path, &fm, "Content")
                .unwrap();

            assert_eq!(doc.id, "simple");
            assert_eq!(doc.text, "Simple | Content");
            assert!(doc.category.is_none());
            assert!(doc.metadata.is_empty());
        }

        #[test]
        fn test_vector_extractor_no_title() {
            let extractor = MusicTheoryVectorExtractor::new();
            let base_path = PathBuf::from("/data");
            let file_path = PathBuf::from("/data/no-title.md");
            let fm: yaml_serde::Value = yaml_serde::from_str("category: test").unwrap();

            let doc = extractor
                .extract_document(&base_path, &file_path, &fm, "Body text")
                .unwrap();

            // Falls back to file stem as title in text
            assert!(doc.text.contains("no-title"));
            assert!(doc.text.contains("Body text"));
            assert_eq!(doc.category, Some("test".to_string()));
        }

        #[test]
        fn test_vector_extractor_text_composition_order() {
            let extractor = MusicTheoryVectorExtractor::new();
            let base_path = PathBuf::from("/data");
            let file_path = PathBuf::from("/data/test.md");
            let fm: yaml_serde::Value = yaml_serde::from_str(
                r#"
title: "Title"
category: "Category"
description: "Description"
"#,
            )
            .unwrap();

            let doc = extractor
                .extract_document(&base_path, &file_path, &fm, "Body")
                .unwrap();

            // Should be: title | category | description | body
            assert_eq!(doc.text, "Title | Category | Description | Body");
        }

        #[test]
        fn test_vector_extractor_trims_body() {
            let extractor = MusicTheoryVectorExtractor::new();
            let base_path = PathBuf::from("/data");
            let file_path = PathBuf::from("/data/test.md");
            let fm: yaml_serde::Value = yaml_serde::from_str("title: T").unwrap();

            let doc = extractor
                .extract_document(&base_path, &file_path, &fm, "  \n  Body  \n  ")
                .unwrap();

            assert_eq!(doc.text, "T | Body");
        }

        #[test]
        fn test_vector_extractor_defaults() {
            let extractor = MusicTheoryVectorExtractor::new();
            assert_eq!(extractor.content_glob(), "**/*.md");
            assert_eq!(extractor.name(), "music-theory");
        }
    }
}
