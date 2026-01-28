//! Graph validation functions.
//!
//! This module provides validation checks for graph integrity:
//! - Orphan nodes (no edges)
//! - Self-loops
//! - Broken references

use petgraph::visit::EdgeRef;

use super::persistence::ConceptGraph;

/// Validation results.
#[derive(Clone, Debug)]
#[allow(dead_code)] // Public API
pub struct ValidationResults {
    /// List of validation issues found
    pub issues: Vec<String>,
    /// Count of orphan nodes (no edges)
    pub orphan_count: usize,
    /// Count of self-loops
    pub self_loop_count: usize,
}

impl ValidationResults {
    /// Check if the graph is valid (no issues).
    #[allow(dead_code)] // Public API, used in tools/graph.rs
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }
}

/// Validate graph integrity.
///
/// Checks for:
/// - Orphan nodes (nodes with no incoming or outgoing edges)
/// - Self-loops (edges from node to itself)
///
/// # Arguments
///
/// * `graph` - The concept graph to validate
///
/// # Returns
///
/// Returns ValidationResults with issues and counts.
#[allow(dead_code)] // Public API, used in tools/graph.rs
pub fn validate_graph(graph: &ConceptGraph) -> ValidationResults {
    let mut issues = Vec::new();
    let mut orphan_count = 0;
    let mut self_loop_count = 0;

    // Check for orphan nodes (no incoming or outgoing edges)
    for idx in graph.node_indices() {
        let in_degree = graph
            .edges_directed(idx, petgraph::Direction::Incoming)
            .count();
        let out_degree = graph
            .edges_directed(idx, petgraph::Direction::Outgoing)
            .count();

        if in_degree == 0 && out_degree == 0 {
            orphan_count += 1;
        }
    }

    // Check for self-loops
    for edge in graph.edge_references() {
        if edge.source() == edge.target() {
            self_loop_count += 1;
        }
    }

    if orphan_count > 0 {
        issues.push(format!(
            "Found {} orphan nodes (no relationships)",
            orphan_count
        ));
    }

    if self_loop_count > 0 {
        issues.push(format!("Found {} self-loops", self_loop_count));
    }

    ValidationResults {
        issues,
        orphan_count,
        self_loop_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::persistence::to_petgraph;
    use crate::graph::types::{
        ConceptNode, Edge, EdgeOrigin, GraphData, GraphMetadata, Node, Relationship, SourceNode,
    };

    fn create_test_graph_valid() -> ConceptGraph {
        let concept_a = Node::Concept(ConceptNode {
            id: "concept-a".to_string(),
            title: "Concept A".to_string(),
            category: "test".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let concept_b = Node::Concept(ConceptNode {
            id: "concept-b".to_string(),
            title: "Concept B".to_string(),
            category: "test".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let source = Node::Source(SourceNode {
            id: "test-source".to_string(),
            title: "Test Source".to_string(),
            author: "Test Author".to_string(),
            year: Some(2024),
            is_converted: false,
        });

        let edge = Edge {
            from: "concept-a".to_string(),
            to: "concept-b".to_string(),
            relationship: Relationship::Prerequisite,
            weight: 1.0,
            origin: EdgeOrigin::Extracted,
        };

        let graph_data = GraphData {
            version: "1.0".to_string(),
            nodes: vec![concept_a, concept_b, source],
            edges: vec![edge],
            metadata: Some(GraphMetadata {
                built_at: "2024-01-01T00:00:00Z".to_string(),
                source_cards_count: 1,
                build_duration_ms: 100,
            }),
        };

        to_petgraph(&graph_data)
    }

    fn create_test_graph_with_orphan() -> ConceptGraph {
        let concept_a = Node::Concept(ConceptNode {
            id: "concept-a".to_string(),
            title: "Concept A".to_string(),
            category: "test".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let orphan = Node::Concept(ConceptNode {
            id: "orphan".to_string(),
            title: "Orphan".to_string(),
            category: "test".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let graph_data = GraphData {
            version: "1.0".to_string(),
            nodes: vec![concept_a, orphan],
            edges: vec![],
            metadata: None,
        };

        to_petgraph(&graph_data)
    }

    fn create_test_graph_with_self_loop() -> ConceptGraph {
        let concept_a = Node::Concept(ConceptNode {
            id: "concept-a".to_string(),
            title: "Concept A".to_string(),
            category: "test".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let self_loop = Edge {
            from: "concept-a".to_string(),
            to: "concept-a".to_string(),
            relationship: Relationship::RelatesTo,
            weight: 0.7,
            origin: EdgeOrigin::Manual,
        };

        let graph_data = GraphData {
            version: "1.0".to_string(),
            nodes: vec![concept_a],
            edges: vec![self_loop],
            metadata: None,
        };

        to_petgraph(&graph_data)
    }

    #[test]
    fn test_validate_graph_valid() {
        let graph = create_test_graph_valid();
        let results = validate_graph(&graph);

        // Graph has 1 orphan (source node with no edges), so it's not fully valid
        assert!(!results.is_valid());
        assert_eq!(results.issues.len(), 1);
        assert_eq!(results.orphan_count, 1); // source node has no edges in this test
        assert_eq!(results.self_loop_count, 0);
        assert!(results
            .issues
            .iter()
            .any(|issue| issue.contains("orphan nodes")));
    }

    #[test]
    fn test_validate_graph_with_orphan() {
        let graph = create_test_graph_with_orphan();
        let results = validate_graph(&graph);

        assert!(!results.is_valid());
        assert_eq!(results.orphan_count, 2);
        assert_eq!(results.self_loop_count, 0);
        assert!(results
            .issues
            .iter()
            .any(|issue| issue.contains("orphan nodes")));
    }

    #[test]
    fn test_validate_graph_with_self_loop() {
        let graph = create_test_graph_with_self_loop();
        let results = validate_graph(&graph);

        assert!(!results.is_valid());
        assert_eq!(results.orphan_count, 0);
        assert_eq!(results.self_loop_count, 1);
        assert!(results
            .issues
            .iter()
            .any(|issue| issue.contains("self-loops")));
    }

    #[test]
    fn test_validation_results_is_valid() {
        let valid = ValidationResults {
            issues: vec![],
            orphan_count: 0,
            self_loop_count: 0,
        };
        assert!(valid.is_valid());

        let invalid = ValidationResults {
            issues: vec!["Found 2 orphan nodes".to_string()],
            orphan_count: 2,
            self_loop_count: 0,
        };
        assert!(!invalid.is_valid());
    }
}
