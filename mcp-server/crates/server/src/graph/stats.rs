//! Graph statistics functions.
//!
//! This module provides functions for computing statistics about the concept graph:
//! - Node counts by type
//! - Category distribution
//! - Relationship type distribution
//! - Degree statistics

use std::collections::HashMap;

use super::persistence::ConceptGraph;
use super::types::Node;

/// Node counts by type.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeCounts {
    pub total: usize,
    pub concepts: usize,
    pub sources: usize,
}

/// Category statistics.
#[derive(Clone, Debug, PartialEq)]
pub struct CategoryCount {
    pub category: String,
    pub count: usize,
}

/// Relationship type statistics.
#[derive(Clone, Debug, PartialEq)]
pub struct RelationshipCount {
    pub relationship: String,
    pub count: usize,
}

/// Degree statistics (in/out edges per node).
#[derive(Clone, Debug, PartialEq)]
pub struct DegreeStats {
    pub average_in_degree: f64,
    pub average_out_degree: f64,
    pub max_in_degree: usize,
    pub max_out_degree: usize,
}

/// Complete graph statistics.
#[derive(Clone, Debug)]
pub struct GraphStatistics {
    pub nodes: NodeCounts,
    pub edge_count: usize,
    pub categories: Vec<CategoryCount>,
    pub relationships: Vec<RelationshipCount>,
    pub degrees: DegreeStats,
}

/// Count nodes by type.
///
/// # Arguments
///
/// * `graph` - The concept graph
///
/// # Returns
///
/// Returns NodeCounts with total, concept, and source counts.
pub fn count_nodes_by_type(graph: &ConceptGraph) -> NodeCounts {
    let mut concepts = 0;
    let mut sources = 0;

    for node in graph.node_weights() {
        match node {
            Node::Concept(_) => concepts += 1,
            Node::Source(_) => sources += 1,
        }
    }

    NodeCounts {
        total: concepts + sources,
        concepts,
        sources,
    }
}

/// Count concepts by category.
///
/// # Arguments
///
/// * `graph` - The concept graph
///
/// # Returns
///
/// Returns a vector of CategoryCount, sorted by count (descending).
pub fn count_by_category(graph: &ConceptGraph) -> Vec<CategoryCount> {
    let mut category_counts: HashMap<String, usize> = HashMap::new();

    for node in graph.node_weights() {
        if let Node::Concept(c) = node {
            *category_counts.entry(c.category.clone()).or_insert(0) += 1;
        }
    }

    let mut categories: Vec<CategoryCount> = category_counts
        .into_iter()
        .map(|(category, count)| CategoryCount { category, count })
        .collect();

    categories.sort_by_key(|c| std::cmp::Reverse(c.count));

    categories
}

/// Count edges by relationship type.
///
/// # Arguments
///
/// * `graph` - The concept graph
///
/// # Returns
///
/// Returns a vector of RelationshipCount, sorted by count (descending).
pub fn count_by_relationship(graph: &ConceptGraph) -> Vec<RelationshipCount> {
    let mut rel_counts: HashMap<String, usize> = HashMap::new();

    for edge in graph.edge_weights() {
        let rel_name = format!("{:?}", edge.relationship);
        *rel_counts.entry(rel_name).or_insert(0) += 1;
    }

    let mut relationships: Vec<RelationshipCount> = rel_counts
        .into_iter()
        .map(|(relationship, count)| RelationshipCount {
            relationship,
            count,
        })
        .collect();

    relationships.sort_by_key(|r| std::cmp::Reverse(r.count));

    relationships
}

/// Compute degree statistics.
///
/// # Arguments
///
/// * `graph` - The concept graph
///
/// # Returns
///
/// Returns DegreeStats with average and max in/out degrees.
pub fn compute_degree_stats(graph: &ConceptGraph) -> DegreeStats {
    let node_count = graph.node_count();
    if node_count == 0 {
        return DegreeStats {
            average_in_degree: 0.0,
            average_out_degree: 0.0,
            max_in_degree: 0,
            max_out_degree: 0,
        };
    }

    let mut total_in = 0;
    let mut total_out = 0;
    let mut max_in = 0;
    let mut max_out = 0;

    for idx in graph.node_indices() {
        let in_degree = graph
            .edges_directed(idx, petgraph::Direction::Incoming)
            .count();
        let out_degree = graph
            .edges_directed(idx, petgraph::Direction::Outgoing)
            .count();

        total_in += in_degree;
        total_out += out_degree;
        max_in = max_in.max(in_degree);
        max_out = max_out.max(out_degree);
    }

    DegreeStats {
        average_in_degree: total_in as f64 / node_count as f64,
        average_out_degree: total_out as f64 / node_count as f64,
        max_in_degree: max_in,
        max_out_degree: max_out,
    }
}

/// Compute all graph statistics.
///
/// # Arguments
///
/// * `graph` - The concept graph
///
/// # Returns
///
/// Returns GraphStatistics with complete statistics.
pub fn compute_all_stats(graph: &ConceptGraph) -> GraphStatistics {
    GraphStatistics {
        nodes: count_nodes_by_type(graph),
        edge_count: graph.edge_count(),
        categories: count_by_category(graph),
        relationships: count_by_relationship(graph),
        degrees: compute_degree_stats(graph),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::types::{ConceptNode, Edge, EdgeOrigin, GraphData, Relationship, SourceNode, GraphMetadata};
    use crate::graph::persistence::to_petgraph;

    fn create_test_graph() -> ConceptGraph {
        let concept_a = Node::Concept(ConceptNode {
            id: "concept-a".to_string(),
            title: "Concept A".to_string(),
            category: "harmony".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let concept_b = Node::Concept(ConceptNode {
            id: "concept-b".to_string(),
            title: "Concept B".to_string(),
            category: "harmony".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let concept_c = Node::Concept(ConceptNode {
            id: "concept-c".to_string(),
            title: "Concept C".to_string(),
            category: "rhythm".to_string(),
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

        let edge1 = Edge {
            from: "concept-a".to_string(),
            to: "concept-b".to_string(),
            relationship: Relationship::Prerequisite,
            weight: 1.0,
            origin: EdgeOrigin::Extracted,
        };

        let edge2 = Edge {
            from: "concept-b".to_string(),
            to: "concept-c".to_string(),
            relationship: Relationship::RelatesTo,
            weight: 0.7,
            origin: EdgeOrigin::Extracted,
        };

        let edge3 = Edge {
            from: "test-source".to_string(),
            to: "concept-a".to_string(),
            relationship: Relationship::Introduces,
            weight: 1.0,
            origin: EdgeOrigin::Extracted,
        };

        let graph_data = GraphData {
            version: "1.0".to_string(),
            nodes: vec![concept_a, concept_b, concept_c, source],
            edges: vec![edge1, edge2, edge3],
            metadata: Some(GraphMetadata {
                built_at: "2024-01-01T00:00:00Z".to_string(),
                source_cards_count: 1,
                build_duration_ms: 100,
            }),
        };

        to_petgraph(&graph_data)
    }

    #[test]
    fn test_count_nodes_by_type() {
        let graph = create_test_graph();
        let counts = count_nodes_by_type(&graph);

        assert_eq!(counts.total, 4);
        assert_eq!(counts.concepts, 3);
        assert_eq!(counts.sources, 1);
    }

    #[test]
    fn test_count_by_category() {
        let graph = create_test_graph();
        let categories = count_by_category(&graph);

        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].category, "harmony");
        assert_eq!(categories[0].count, 2);
        assert_eq!(categories[1].category, "rhythm");
        assert_eq!(categories[1].count, 1);
    }

    #[test]
    fn test_count_by_relationship() {
        let graph = create_test_graph();
        let relationships = count_by_relationship(&graph);

        assert_eq!(relationships.len(), 3);
        // All have count 1, so order may vary
        assert!(relationships
            .iter()
            .any(|r| r.relationship == "Prerequisite" && r.count == 1));
        assert!(relationships
            .iter()
            .any(|r| r.relationship == "RelatesTo" && r.count == 1));
        assert!(relationships
            .iter()
            .any(|r| r.relationship == "Introduces" && r.count == 1));
    }

    #[test]
    fn test_compute_degree_stats() {
        let graph = create_test_graph();
        let stats = compute_degree_stats(&graph);

        // 4 nodes, 3 edges
        // Total in-degree = 3 (each edge has one target)
        // Total out-degree = 3 (each edge has one source)
        assert_eq!(stats.average_in_degree, 0.75);
        assert_eq!(stats.average_out_degree, 0.75);
        assert!(stats.max_in_degree >= 1);
        assert!(stats.max_out_degree >= 1);
    }

    #[test]
    fn test_compute_degree_stats_empty_graph() {
        let graph_data = GraphData {
            version: "1.0".to_string(),
            nodes: vec![],
            edges: vec![],
            metadata: None,
        };
        let graph = to_petgraph(&graph_data);
        let stats = compute_degree_stats(&graph);

        assert_eq!(stats.average_in_degree, 0.0);
        assert_eq!(stats.average_out_degree, 0.0);
        assert_eq!(stats.max_in_degree, 0);
        assert_eq!(stats.max_out_degree, 0);
    }

    #[test]
    fn test_compute_all_stats() {
        let graph = create_test_graph();
        let stats = compute_all_stats(&graph);

        assert_eq!(stats.nodes.total, 4);
        assert_eq!(stats.nodes.concepts, 3);
        assert_eq!(stats.nodes.sources, 1);
        assert_eq!(stats.edge_count, 3);
        assert_eq!(stats.categories.len(), 2);
        assert_eq!(stats.relationships.len(), 3);
        assert_eq!(stats.degrees.average_in_degree, 0.75);
    }
}
