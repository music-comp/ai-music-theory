//! Core data structures for the concept graph.
//!
//! This module defines the types used to represent nodes, edges, and relationships
//! in the music theory concept graph.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

// ============================================================
// NODE TYPES
// ============================================================

/// A node in the concept graph.
#[derive(Clone, Debug, PartialEq, SerdeSerialize, SerdeDeserialize)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize)]
#[serde(tag = "type")]
pub enum Node {
    /// A concept node (canonical or source-specific)
    Concept(ConceptNode),
    /// A source node (book, paper, textbook)
    Source(SourceNode),
}

/// A concept node representing a music theory concept.
#[derive(Clone, Debug, PartialEq, SerdeSerialize, SerdeDeserialize)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize)]
pub struct ConceptNode {
    /// Unique identifier (e.g., "suspension" or "suspension@lewin")
    pub id: String,

    /// Human-readable title
    pub title: String,

    /// Category (harmony, counterpoint, form, etc.)
    pub category: String,

    /// Source that introduced/defines this concept
    pub source_id: String,

    /// If this is a source-specific variant, the canonical concept ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_id: Option<String>,

    /// Is this a canonical (harmonized) concept?
    #[serde(default)]
    pub is_canonical: bool,
}

/// A source node representing a book, paper, or textbook.
#[derive(Clone, Debug, PartialEq, SerdeSerialize, SerdeDeserialize)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize)]
pub struct SourceNode {
    /// Unique identifier (e.g., "oxford-lewin-gmit")
    pub id: String,

    /// Full title
    pub title: String,

    /// Author(s)
    pub author: String,

    /// Publication year
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,

    /// Is this source converted/indexed?
    #[serde(default)]
    pub is_converted: bool,
}

// ============================================================
// EDGE TYPES
// ============================================================

/// An edge in the concept graph representing a relationship between nodes.
#[derive(Clone, Debug, PartialEq, SerdeSerialize, SerdeDeserialize)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize)]
pub struct Edge {
    /// Source node ID
    pub from: String,

    /// Target node ID
    pub to: String,

    /// Type of relationship
    pub relationship: Relationship,

    /// Strength/relevance (0.0 - 1.0)
    #[serde(default = "default_weight")]
    pub weight: f32,

    /// How was this edge created?
    #[serde(default)]
    pub origin: EdgeOrigin,
}

/// Default weight for edges.
fn default_weight() -> f32 {
    1.0
}

/// Types of relationships between nodes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, SerdeSerialize, SerdeDeserialize)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize)]
#[serde(rename_all = "snake_case")]
pub enum Relationship {
    // Concept-to-Concept relationships
    /// General conceptual relationship
    RelatesTo,
    /// Must understand A before B (A is prerequisite for B)
    Prerequisite,
    /// B builds on / extends A
    Extends,
    /// A and B are the same concept (different names or source variants)
    SameAs,

    // Source-to-Concept relationships
    /// Source introduces/defines this concept
    Introduces,
    /// Source covers/discusses this concept
    Covers,

    // Source-to-Source relationships
    /// Source A cites Source B
    Cites,
}

/// How an edge was created.
#[derive(Clone, Debug, Default, PartialEq, Eq, SerdeSerialize, SerdeDeserialize)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeOrigin {
    /// Extracted from concept card "Related Concepts" section
    #[default]
    Extracted,
    /// Human-curated (manual_edges.json)
    Manual,
    /// Auto-inferred (future feature)
    Inferred,
}

// ============================================================
// GRAPH DATA (for serialization)
// ============================================================

/// Complete graph data for serialization.
#[derive(Clone, Debug, SerdeSerialize, SerdeDeserialize)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize)]
pub struct GraphData {
    /// Schema version
    pub version: String,

    /// All nodes
    pub nodes: Vec<Node>,

    /// All edges
    pub edges: Vec<Edge>,

    /// Build metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<GraphMetadata>,
}

/// Metadata about the graph build process.
#[derive(Clone, Debug, SerdeSerialize, SerdeDeserialize)]
#[derive(Archive, RkyvSerialize, RkyvDeserialize)]
pub struct GraphMetadata {
    /// ISO 8601 timestamp of when the graph was built
    pub built_at: String,

    /// Number of source concept cards processed
    pub source_cards_count: u32,

    /// Build duration in milliseconds
    pub build_duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_enum_serialization() {
        let concept = Node::Concept(ConceptNode {
            id: "suspension".to_string(),
            title: "Suspension".to_string(),
            category: "harmony".to_string(),
            source_id: "open-music-theory".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let json = serde_json::to_string(&concept).unwrap();
        assert!(json.contains("\"type\":\"Concept\""));
        assert!(json.contains("suspension"));

        let deserialized: Node = serde_json::from_str(&json).unwrap();
        assert_eq!(concept, deserialized);
    }

    #[test]
    fn test_source_node_serialization() {
        let source = Node::Source(SourceNode {
            id: "open-music-theory".to_string(),
            title: "Open Music Theory".to_string(),
            author: "Various".to_string(),
            year: Some(2022),
            is_converted: true,
        });

        let json = serde_json::to_string(&source).unwrap();
        assert!(json.contains("\"type\":\"Source\""));
        assert!(json.contains("Open Music Theory"));

        let deserialized: Node = serde_json::from_str(&json).unwrap();
        assert_eq!(source, deserialized);
    }

    #[test]
    fn test_edge_serialization() {
        let edge = Edge {
            from: "fourth-species".to_string(),
            to: "suspension".to_string(),
            relationship: Relationship::Prerequisite,
            weight: 1.0,
            origin: EdgeOrigin::Extracted,
        };

        let json = serde_json::to_string(&edge).unwrap();
        assert!(json.contains("fourth-species"));
        assert!(json.contains("suspension"));
        assert!(json.contains("prerequisite"));

        let deserialized: Edge = serde_json::from_str(&json).unwrap();
        assert_eq!(edge, deserialized);
    }

    #[test]
    fn test_relationship_serialization() {
        let rel = Relationship::Prerequisite;
        let json = serde_json::to_string(&rel).unwrap();
        assert_eq!(json, "\"prerequisite\"");

        let deserialized: Relationship = serde_json::from_str(&json).unwrap();
        assert_eq!(rel, deserialized);
    }

    #[test]
    fn test_edge_origin_default() {
        let origin = EdgeOrigin::default();
        assert_eq!(origin, EdgeOrigin::Extracted);
    }

    #[test]
    fn test_edge_default_weight() {
        assert_eq!(default_weight(), 1.0);
    }

    #[test]
    fn test_graph_data_serialization() {
        let concept_node = Node::Concept(ConceptNode {
            id: "test-concept".to_string(),
            title: "Test Concept".to_string(),
            category: "test".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let source_node = Node::Source(SourceNode {
            id: "test-source".to_string(),
            title: "Test Source".to_string(),
            author: "Test Author".to_string(),
            year: Some(2024),
            is_converted: false,
        });

        let edge = Edge {
            from: "test-source".to_string(),
            to: "test-concept".to_string(),
            relationship: Relationship::Introduces,
            weight: 1.0,
            origin: EdgeOrigin::Extracted,
        };

        let graph = GraphData {
            version: "1.0".to_string(),
            nodes: vec![concept_node, source_node],
            edges: vec![edge],
            metadata: Some(GraphMetadata {
                built_at: "2024-01-01T00:00:00Z".to_string(),
                source_cards_count: 1,
                build_duration_ms: 100,
            }),
        };

        let json = serde_json::to_string_pretty(&graph).unwrap();
        assert!(json.contains("\"version\": \"1.0\""));  // Pretty print adds space after colon
        assert!(json.contains("test-concept"));
        assert!(json.contains("test-source"));

        let deserialized: GraphData = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.version, "1.0");
        assert_eq!(deserialized.nodes.len(), 2);
        assert_eq!(deserialized.edges.len(), 1);
    }

    #[test]
    fn test_concept_node_with_canonical_id() {
        let node = ConceptNode {
            id: "suspension@tymoczko".to_string(),
            title: "Suspension".to_string(),
            category: "voice-leading".to_string(),
            source_id: "tymoczko-geometry".to_string(),
            canonical_id: Some("suspension".to_string()),
            is_canonical: false,
        };

        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("canonical_id"));
        assert!(json.contains("\"suspension\""));

        let deserialized: ConceptNode = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.canonical_id, Some("suspension".to_string()));
        assert!(!deserialized.is_canonical);
    }

    #[test]
    fn test_source_node_without_year() {
        let node = SourceNode {
            id: "test".to_string(),
            title: "Test".to_string(),
            author: "Test".to_string(),
            year: None,
            is_converted: false,
        };

        let json = serde_json::to_string(&node).unwrap();
        // year should be omitted when None
        assert!(!json.contains("\"year\""));

        let deserialized: SourceNode = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.year, None);
    }

    #[test]
    fn test_all_relationship_variants() {
        let relationships = vec![
            Relationship::RelatesTo,
            Relationship::Prerequisite,
            Relationship::Extends,
            Relationship::SameAs,
            Relationship::Introduces,
            Relationship::Covers,
            Relationship::Cites,
        ];

        for rel in relationships {
            let json = serde_json::to_string(&rel).unwrap();
            let deserialized: Relationship = serde_json::from_str(&json).unwrap();
            assert_eq!(rel, deserialized);
        }
    }

    #[test]
    fn test_all_edge_origin_variants() {
        let origins = vec![
            EdgeOrigin::Extracted,
            EdgeOrigin::Manual,
            EdgeOrigin::Inferred,
        ];

        for origin in origins {
            let json = serde_json::to_string(&origin).unwrap();
            let deserialized: EdgeOrigin = serde_json::from_str(&json).unwrap();
            assert_eq!(origin, deserialized);
        }
    }

    #[test]
    fn test_edge_with_custom_weight() {
        let edge = Edge {
            from: "a".to_string(),
            to: "b".to_string(),
            relationship: Relationship::RelatesTo,
            weight: 0.7,
            origin: EdgeOrigin::Extracted,
        };

        let json = serde_json::to_string(&edge).unwrap();
        assert!(json.contains("0.7"));

        let deserialized: Edge = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.weight, 0.7);
    }
}
