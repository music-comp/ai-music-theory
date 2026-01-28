//! Response types for graph query MCP tools.
//!
//! This module defines the response structures for all graph query tools.
//! All types implement Serialize and Deserialize for JSON encoding.
//!
//! ## Response Types
//!
//! - **RelatedConceptsResponse** - Related concepts with filtering
//! - **ConceptPathResponse** - Shortest path between concepts
//! - **PrerequisitesResponse** - Prerequisites in learning order
//! - **NeighborhoodResponse** - Local subgraph around concept
//! - **DependentsResponse** - Concepts that depend on this
//! - **CentralConceptsResponse** - Most connected concepts
//! - **ConceptSourcesResponse** - Sources covering a concept
//! - **ConceptVariantsResponse** - Source-specific variants
//! - **BridgeConceptsResponse** - Cross-category connectors
//! - **SourceCoverageResponse** - What a source covers

use serde::{Deserialize, Serialize};

// ============================================================================
// Related Concepts
// ============================================================================

/// Response for get_related_concepts tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedConceptsResponse {
    /// ID of the queried concept
    pub concept_id: String,
    /// Depth parameter used
    pub depth: u32,
    /// Total number of related concepts found
    pub total: u32,
    /// List of related concepts
    pub related: Vec<RelatedConcept>,
}

/// A related concept with relationship metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedConcept {
    /// Concept ID
    pub id: String,
    /// Concept title
    pub title: String,
    /// Concept category
    pub category: String,
    /// Relationship type (Prerequisite, RelatesTo, etc.)
    pub relationship: String,
    /// Direction: "incoming" or "outgoing"
    pub direction: String,
    /// Edge weight (0.0-1.0)
    pub weight: f32,
    /// Distance from starting concept (in hops)
    pub distance: u32,
}

// ============================================================================
// Concept Path
// ============================================================================

/// Response for find_concept_path tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConceptPathResponse {
    /// Starting concept ID
    pub from: String,
    /// Target concept ID
    pub to: String,
    /// Whether a path was found
    pub found: bool,
    /// Path length (number of edges, 0 if not found)
    pub path_length: u32,
    /// Nodes in the path (empty if not found)
    pub nodes: Vec<PathNode>,
    /// Edges in the path (empty if not found)
    pub edges: Vec<PathEdge>,
}

/// A node in a path.
#[derive(Debug, Serialize, Deserialize)]
pub struct PathNode {
    /// Node ID
    pub id: String,
    /// Node title
    pub title: String,
    /// Node category
    pub category: String,
}

/// An edge in a path.
#[derive(Debug, Serialize, Deserialize)]
pub struct PathEdge {
    /// Source node ID
    pub from: String,
    /// Target node ID
    pub to: String,
    /// Relationship type
    pub relationship: String,
}

// ============================================================================
// Prerequisites
// ============================================================================

/// Response for get_prerequisites tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct PrerequisitesResponse {
    /// ID of the queried concept
    pub concept_id: String,
    /// Title of the queried concept
    pub concept_title: String,
    /// Total number of prerequisites found
    pub total: u32,
    /// List of prerequisites with metadata
    pub prerequisites: Vec<PrerequisiteConcept>,
    /// IDs in recommended learning order (sorted topologically)
    pub learning_order: Vec<String>,
}

/// A prerequisite concept.
#[derive(Debug, Serialize, Deserialize)]
pub struct PrerequisiteConcept {
    /// Concept ID
    pub id: String,
    /// Concept title
    pub title: String,
    /// Concept category
    pub category: String,
    /// Depth in prerequisite chain (0 = no prerequisites)
    pub depth: u32,
}

// ============================================================================
// Neighborhood
// ============================================================================

/// Response for get_concept_neighborhood tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct NeighborhoodResponse {
    /// Center concept ID
    pub center: String,
    /// Radius parameter used
    pub radius: u32,
    /// Number of nodes in neighborhood
    pub node_count: u32,
    /// Number of edges in neighborhood
    pub edge_count: u32,
    /// Nodes in the neighborhood
    pub nodes: Vec<NeighborhoodNode>,
    /// Edges in the neighborhood
    pub edges: Vec<NeighborhoodEdge>,
}

/// A node in a neighborhood.
#[derive(Debug, Serialize, Deserialize)]
pub struct NeighborhoodNode {
    /// Node ID
    pub id: String,
    /// Node title
    pub title: String,
    /// Node category
    pub category: String,
    /// Distance from center (in hops)
    pub distance: u32,
}

/// An edge in a neighborhood.
#[derive(Debug, Serialize, Deserialize)]
pub struct NeighborhoodEdge {
    /// Source node ID
    pub from: String,
    /// Target node ID
    pub to: String,
    /// Relationship type
    pub relationship: String,
}

// ============================================================================
// Dependents
// ============================================================================

/// Response for get_dependents tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct DependentsResponse {
    /// ID of the queried concept
    pub concept_id: String,
    /// Title of the queried concept
    pub concept_title: String,
    /// Total number of dependents found
    pub total: u32,
    /// List of dependent concepts
    pub dependents: Vec<DependentConcept>,
}

/// A dependent concept.
#[derive(Debug, Serialize, Deserialize)]
pub struct DependentConcept {
    /// Concept ID
    pub id: String,
    /// Concept title
    pub title: String,
    /// Concept category
    pub category: String,
    /// Depth in dependency chain
    pub depth: u32,
}

// ============================================================================
// Central Concepts
// ============================================================================

/// Response for get_central_concepts tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct CentralConceptsResponse {
    /// Category filter used (if any)
    pub category: Option<String>,
    /// Total number of concepts found
    pub total: u32,
    /// List of central concepts sorted by connections
    pub concepts: Vec<CentralConcept>,
}

/// A central concept.
#[derive(Debug, Serialize, Deserialize)]
pub struct CentralConcept {
    /// Concept ID
    pub id: String,
    /// Concept title
    pub title: String,
    /// Concept category
    pub category: String,
    /// Number of connections (degree)
    pub connections: u32,
}

// ============================================================================
// Concept Sources
// ============================================================================

/// Response for get_concept_sources tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConceptSourcesResponse {
    /// ID of the queried concept
    pub concept_id: String,
    /// Title of the queried concept
    pub concept_title: String,
    /// Total number of sources found
    pub total: u32,
    /// List of sources covering this concept
    pub sources: Vec<SourceCoverage>,
}

/// Source coverage information.
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceCoverage {
    /// Source ID
    pub source_id: String,
    /// Source title
    pub source_title: String,
    /// Source author
    pub source_author: String,
    /// Relationship type ("Introduces" or "Covers")
    pub relationship: String,
}

// ============================================================================
// Concept Variants
// ============================================================================

/// Response for get_concept_variants tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConceptVariantsResponse {
    /// Canonical concept ID
    pub canonical_id: String,
    /// Canonical concept title
    pub canonical_title: String,
    /// Total number of variants found
    pub total: u32,
    /// List of source-specific variants
    pub variants: Vec<ConceptVariant>,
}

/// A source-specific concept variant.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConceptVariant {
    /// Variant concept ID
    pub id: String,
    /// Variant title
    pub title: String,
    /// Source ID where this variant appears
    pub source_id: String,
    /// Source title
    pub source_title: String,
}

// ============================================================================
// Bridge Concepts
// ============================================================================

/// Response for find_bridge_concepts tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct BridgeConceptsResponse {
    /// First category
    pub category_a: String,
    /// Second category
    pub category_b: String,
    /// Total number of bridges found
    pub total: u32,
    /// List of bridge concepts sorted by score
    pub bridges: Vec<BridgeConcept>,
}

/// A bridge concept connecting two categories.
#[derive(Debug, Serialize, Deserialize)]
pub struct BridgeConcept {
    /// Concept ID
    pub id: String,
    /// Concept title
    pub title: String,
    /// Concept category
    pub category: String,
    /// Number of connections to category A
    pub connections_to_a: u32,
    /// Number of connections to category B
    pub connections_to_b: u32,
    /// Bridge score: sqrt(connections_a * connections_b)
    pub bridge_score: f32,
}

// ============================================================================
// Source Coverage
// ============================================================================

/// Response for get_source_coverage tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceCoverageResponse {
    /// Source ID
    pub source_id: String,
    /// Source title
    pub source_title: String,
    /// Source author
    pub source_author: String,
    /// Total concepts covered
    pub total_concepts: u32,
    /// Number of concepts introduced
    pub introduces_count: u32,
    /// Number of concepts covered
    pub covers_count: u32,
    /// Concepts introduced by this source
    pub introduces: Vec<ConceptBrief>,
    /// Concepts covered by this source
    pub covers: Vec<ConceptBrief>,
}

/// Brief concept information.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConceptBrief {
    /// Concept ID
    pub id: String,
    /// Concept title
    pub title: String,
    /// Concept category
    pub category: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_related_concepts_response_serialization() {
        let response = RelatedConceptsResponse {
            concept_id: "test-concept".to_string(),
            depth: 2,
            total: 3,
            related: vec![RelatedConcept {
                id: "related-1".to_string(),
                title: "Related Concept".to_string(),
                category: "harmony".to_string(),
                relationship: "Prerequisite".to_string(),
                direction: "incoming".to_string(),
                weight: 1.0,
                distance: 1,
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"concept_id\":\"test-concept\""));
        assert!(json.contains("\"depth\":2"));
        assert!(json.contains("\"total\":3"));
    }

    #[test]
    fn test_concept_path_response_serialization() {
        let response = ConceptPathResponse {
            from: "interval".to_string(),
            to: "seventh-chord".to_string(),
            found: true,
            path_length: 2,
            nodes: vec![
                PathNode {
                    id: "interval".to_string(),
                    title: "Interval".to_string(),
                    category: "fundamentals".to_string(),
                },
                PathNode {
                    id: "seventh-chord".to_string(),
                    title: "Seventh Chord".to_string(),
                    category: "harmony".to_string(),
                },
            ],
            edges: vec![PathEdge {
                from: "interval".to_string(),
                to: "seventh-chord".to_string(),
                relationship: "Prerequisite".to_string(),
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"found\":true"));
        assert!(json.contains("\"path_length\":2"));
    }

    #[test]
    fn test_prerequisites_response_serialization() {
        let response = PrerequisitesResponse {
            concept_id: "seventh-chord".to_string(),
            concept_title: "Seventh Chord".to_string(),
            total: 2,
            prerequisites: vec![PrerequisiteConcept {
                id: "interval".to_string(),
                title: "Interval".to_string(),
                category: "fundamentals".to_string(),
                depth: 0,
            }],
            learning_order: vec!["interval".to_string(), "triad".to_string()],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"concept_id\":\"seventh-chord\""));
        assert!(json.contains("\"total\":2"));
        assert!(json.contains("\"learning_order\""));
    }

    #[test]
    fn test_neighborhood_response_serialization() {
        let response = NeighborhoodResponse {
            center: "triad".to_string(),
            radius: 2,
            node_count: 3,
            edge_count: 2,
            nodes: vec![NeighborhoodNode {
                id: "interval".to_string(),
                title: "Interval".to_string(),
                category: "fundamentals".to_string(),
                distance: 1,
            }],
            edges: vec![NeighborhoodEdge {
                from: "interval".to_string(),
                to: "triad".to_string(),
                relationship: "Prerequisite".to_string(),
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"center\":\"triad\""));
        assert!(json.contains("\"node_count\":3"));
        assert!(json.contains("\"edge_count\":2"));
    }

    #[test]
    fn test_dependents_response_serialization() {
        let response = DependentsResponse {
            concept_id: "interval".to_string(),
            concept_title: "Interval".to_string(),
            total: 2,
            dependents: vec![DependentConcept {
                id: "triad".to_string(),
                title: "Triad".to_string(),
                category: "harmony".to_string(),
                depth: 1,
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"concept_id\":\"interval\""));
        assert!(json.contains("\"total\":2"));
    }

    #[test]
    fn test_central_concepts_response_serialization() {
        let response = CentralConceptsResponse {
            category: Some("harmony".to_string()),
            total: 5,
            concepts: vec![CentralConcept {
                id: "triad".to_string(),
                title: "Triad".to_string(),
                category: "harmony".to_string(),
                connections: 10,
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"category\":\"harmony\""));
        assert!(json.contains("\"total\":5"));
        assert!(json.contains("\"connections\":10"));
    }

    #[test]
    fn test_concept_sources_response_serialization() {
        let response = ConceptSourcesResponse {
            concept_id: "triad".to_string(),
            concept_title: "Triad".to_string(),
            total: 2,
            sources: vec![SourceCoverage {
                source_id: "test-source".to_string(),
                source_title: "Test Source".to_string(),
                source_author: "Test Author".to_string(),
                relationship: "Introduces".to_string(),
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"concept_id\":\"triad\""));
        assert!(json.contains("\"relationship\":\"Introduces\""));
    }

    #[test]
    fn test_concept_variants_response_serialization() {
        let response = ConceptVariantsResponse {
            canonical_id: "suspension".to_string(),
            canonical_title: "Suspension".to_string(),
            total: 2,
            variants: vec![ConceptVariant {
                id: "suspension@lewin".to_string(),
                title: "Suspension (Lewin)".to_string(),
                source_id: "lewin-gmit".to_string(),
                source_title: "Generalized Musical Intervals".to_string(),
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"canonical_id\":\"suspension\""));
        assert!(json.contains("\"total\":2"));
    }

    #[test]
    fn test_bridge_concepts_response_serialization() {
        let response = BridgeConceptsResponse {
            category_a: "harmony".to_string(),
            category_b: "counterpoint".to_string(),
            total: 3,
            bridges: vec![BridgeConcept {
                id: "triad".to_string(),
                title: "Triad".to_string(),
                category: "harmony".to_string(),
                connections_to_a: 5,
                connections_to_b: 3,
                bridge_score: 3.872,
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"category_a\":\"harmony\""));
        assert!(json.contains("\"category_b\":\"counterpoint\""));
        assert!(json.contains("\"bridge_score\""));
    }

    #[test]
    fn test_source_coverage_response_serialization() {
        let response = SourceCoverageResponse {
            source_id: "test-source".to_string(),
            source_title: "Test Source".to_string(),
            source_author: "Test Author".to_string(),
            total_concepts: 10,
            introduces_count: 5,
            covers_count: 5,
            introduces: vec![ConceptBrief {
                id: "triad".to_string(),
                title: "Triad".to_string(),
                category: "harmony".to_string(),
            }],
            covers: vec![ConceptBrief {
                id: "interval".to_string(),
                title: "Interval".to_string(),
                category: "fundamentals".to_string(),
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"source_id\":\"test-source\""));
        assert!(json.contains("\"introduces_count\":5"));
        assert!(json.contains("\"covers_count\":5"));
    }
}
