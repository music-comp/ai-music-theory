//! Graph loader for async initialization.
//!
//! This module provides functions for loading the graph asynchronously
//! and computing statistics for AppState.

use std::collections::HashMap;
use std::path::Path;

use petgraph::graph::NodeIndex;

use crate::error::Result;

use super::persistence::{load_graph, ConceptGraph};
use super::types::Node;

/// Statistics about the loaded graph.
#[derive(Clone, Debug)]
pub struct GraphStats {
    pub node_count: u32,
    pub edge_count: u32,
    pub concept_count: u32,
    pub source_count: u32,
}

/// Loaded graph with metadata and indices.
#[derive(Clone)]
pub struct LoadedGraph {
    pub graph: ConceptGraph,
    pub node_index: HashMap<String, NodeIndex>,
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    pub stats: GraphStats,
}

/// Load the concept graph for use in AppState.
///
/// This function loads the graph and builds the node index for fast lookups.
///
/// # Arguments
///
/// * `data_dir` - Base data directory
///
/// # Returns
///
/// Returns LoadedGraph with graph, indices, and statistics.
///
/// # Errors
///
/// Returns error if graph cannot be loaded.
pub async fn load_concept_graph(data_dir: &Path) -> Result<LoadedGraph> {
    let graph = load_graph(data_dir).await?;
    let stats = compute_graph_stats(&graph);
    let node_index = build_node_index(&graph);

    Ok(LoadedGraph {
        graph,
        node_index,
        loaded_at: chrono::Utc::now(),
        stats,
    })
}

/// Build a node index for fast ID lookups.
///
/// Maps concept/source IDs to their NodeIndex in the graph.
fn build_node_index(graph: &ConceptGraph) -> HashMap<String, NodeIndex> {
    let mut index = HashMap::new();

    for idx in graph.node_indices() {
        let id = match &graph[idx] {
            Node::Concept(c) => c.id.clone(),
            Node::Source(s) => s.id.clone(),
        };
        index.insert(id, idx);
    }

    index
}

/// Compute statistics about the graph.
fn compute_graph_stats(graph: &ConceptGraph) -> GraphStats {
    let mut concept_count = 0;
    let mut source_count = 0;

    for node in graph.node_weights() {
        match node {
            Node::Concept(_) => concept_count += 1,
            Node::Source(_) => source_count += 1,
        }
    }

    GraphStats {
        node_count: graph.node_count() as u32,
        edge_count: graph.edge_count() as u32,
        concept_count,
        source_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::graph::types::{ConceptNode, SourceNode, Edge, Relationship, EdgeOrigin, GraphData, GraphMetadata};
    use crate::graph::persistence::save_graph;

    async fn create_test_data_dir() -> TempDir {
        let temp_dir = TempDir::new().unwrap();

        let concept = Node::Concept(ConceptNode {
            id: "test-concept".to_string(),
            title: "Test Concept".to_string(),
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
            from: "test-source".to_string(),
            to: "test-concept".to_string(),
            relationship: Relationship::Introduces,
            weight: 1.0,
            origin: EdgeOrigin::Extracted,
        };

        let graph_data = GraphData {
            version: "1.0".to_string(),
            nodes: vec![concept, source],
            edges: vec![edge],
            metadata: Some(GraphMetadata {
                built_at: "2024-01-01T00:00:00Z".to_string(),
                source_cards_count: 1,
                build_duration_ms: 100,
            }),
        };

        save_graph(&graph_data, temp_dir.path()).await.unwrap();

        temp_dir
    }

    #[tokio::test]
    async fn test_load_concept_graph() {
        let temp_dir = create_test_data_dir().await;

        let loaded = load_concept_graph(temp_dir.path()).await.unwrap();

        assert_eq!(loaded.stats.node_count, 2);
        assert_eq!(loaded.stats.edge_count, 1);
        assert_eq!(loaded.stats.concept_count, 1);
        assert_eq!(loaded.stats.source_count, 1);
        assert_eq!(loaded.node_index.len(), 2);
        assert!(loaded.node_index.contains_key("test-concept"));
        assert!(loaded.node_index.contains_key("test-source"));
    }

    #[tokio::test]
    async fn test_compute_graph_stats() {
        let temp_dir = create_test_data_dir().await;
        let graph = load_graph(temp_dir.path()).await.unwrap();

        let stats = compute_graph_stats(&graph);

        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.edge_count, 1);
        assert_eq!(stats.concept_count, 1);
        assert_eq!(stats.source_count, 1);
    }

    #[tokio::test]
    async fn test_build_node_index() {
        let temp_dir = create_test_data_dir().await;
        let graph = load_graph(temp_dir.path()).await.unwrap();

        let index = build_node_index(&graph);

        assert_eq!(index.len(), 2);
        assert!(index.contains_key("test-concept"));
        assert!(index.contains_key("test-source"));

        // Verify we can use the index to look up nodes
        let concept_idx = index.get("test-concept").unwrap();
        let node = &graph[*concept_idx];
        assert!(matches!(node, Node::Concept(_)));
    }
}
