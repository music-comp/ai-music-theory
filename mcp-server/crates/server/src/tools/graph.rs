//! Graph database tools for querying the concept graph.
//!
//! This module provides MCP tools for inspecting the concept graph:
//! - graph_status: Check if graph is loaded
//! - graph_stats: Detailed statistics
//! - graph_validate: Integrity checks
//! - get_node: Get node by ID
//! - get_node_edges: Get edges for a node

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::state::AppState;

#[cfg(feature = "graph")]
use crate::graph::types::Node;
#[cfg(feature = "graph")]
use crate::state::GraphState;
#[cfg(feature = "graph")]
use petgraph::visit::EdgeRef;

/// Graph status response.
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphStatusResponse {
    /// Whether graph feature is enabled
    pub enabled: bool,
    /// Current status ("not_loaded", "loading", "loaded", "failed")
    pub status: String,
    /// Error message if status is "failed"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Basic statistics if loaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<BasicGraphStats>,
    /// When the graph was loaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loaded_at: Option<String>,
}

/// Basic graph statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct BasicGraphStats {
    pub node_count: u32,
    pub edge_count: u32,
    pub concept_count: u32,
    pub source_count: u32,
}

/// Detailed graph statistics response.
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphStatsResponse {
    /// Node counts by type
    pub nodes: NodeCounts,
    /// Edge count
    pub edge_count: u32,
    /// Relationship type counts
    pub relationships: Vec<RelationshipCount>,
    /// Category counts (for concepts)
    pub categories: Vec<CategoryCount>,
}

/// Node counts by type.
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeCounts {
    pub total: u32,
    pub concepts: u32,
    pub sources: u32,
}

/// Relationship type count.
#[derive(Debug, Serialize, Deserialize)]
pub struct RelationshipCount {
    pub relationship: String,
    pub count: usize,
}

/// Category count.
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryCount {
    pub category: String,
    pub count: usize,
}

/// Graph validation response.
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphValidateResponse {
    /// Whether the graph is valid
    pub valid: bool,
    /// Validation issues found
    pub issues: Vec<String>,
    /// Orphan node count (no edges)
    pub orphan_count: usize,
    /// Self-loop count
    pub self_loop_count: usize,
}

/// Node information response.
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node ID
    pub id: String,
    /// Node type ("concept" or "source")
    pub node_type: String,
    /// Node details
    #[serde(flatten)]
    pub details: NodeDetails,
    /// Incoming edge count
    pub in_degree: usize,
    /// Outgoing edge count
    pub out_degree: usize,
}

/// Node details (concept or source).
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeDetails {
    Concept {
        title: String,
        category: String,
        source_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        canonical_id: Option<String>,
        is_canonical: bool,
    },
    Source {
        title: String,
        author: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        year: Option<u16>,
        is_converted: bool,
    },
}

/// Edge information.
#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeInfo {
    /// Source node ID
    pub from: String,
    /// Target node ID
    pub to: String,
    /// Relationship type
    pub relationship: String,
    /// Edge weight
    pub weight: f32,
    /// Edge origin ("extracted", "manual", "inferred")
    pub origin: String,
}

/// Get node edges parameters.
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)] // Used in tests and server.rs
pub struct GetNodeEdgesParams {
    /// Node ID to query
    pub node_id: String,
    /// Optional direction filter ("incoming", "outgoing", "both")
    #[serde(default = "default_direction")]
    pub direction: String,
}

#[allow(dead_code)] // Used by GetNodeEdgesParams in tests
fn default_direction() -> String {
    "both".to_string()
}

/// Node edges response.
#[derive(Debug, Serialize, Deserialize)]
pub struct NodeEdgesResponse {
    /// Node ID
    pub node_id: String,
    /// Direction filter used
    pub direction: String,
    /// Incoming edges
    pub incoming: Vec<EdgeInfo>,
    /// Outgoing edges
    pub outgoing: Vec<EdgeInfo>,
}

/// Get graph status and basic statistics.
///
/// # Arguments
///
/// * `state` - Application state
///
/// # Returns
///
/// Returns GraphStatusResponse with current status.
#[allow(unused_variables)] // state only used when graph feature is enabled
pub async fn graph_status(state: &AppState) -> Result<GraphStatusResponse> {
    #[cfg(feature = "graph")]
    {
        let graph_state = state.graph.read().unwrap();
        match &*graph_state {
            GraphState::NotLoaded => Ok(GraphStatusResponse {
                enabled: true,
                status: "not_loaded".to_string(),
                error: None,
                stats: None,
                loaded_at: None,
            }),
            GraphState::Loading => Ok(GraphStatusResponse {
                enabled: true,
                status: "loading".to_string(),
                error: None,
                stats: None,
                loaded_at: None,
            }),
            GraphState::Loaded(loaded) => Ok(GraphStatusResponse {
                enabled: true,
                status: "loaded".to_string(),
                error: None,
                stats: Some(BasicGraphStats {
                    node_count: loaded.stats.node_count,
                    edge_count: loaded.stats.edge_count,
                    concept_count: loaded.stats.concept_count,
                    source_count: loaded.stats.source_count,
                }),
                loaded_at: Some(loaded.loaded_at.to_rfc3339()),
            }),
            GraphState::Failed(error) => Ok(GraphStatusResponse {
                enabled: true,
                status: "failed".to_string(),
                error: Some(error.clone()),
                stats: None,
                loaded_at: None,
            }),
        }
    }

    #[cfg(not(feature = "graph"))]
    Ok(GraphStatusResponse {
        enabled: false,
        status: "not_available".to_string(),
        error: Some("Graph feature not enabled in this build".to_string()),
        stats: None,
        loaded_at: None,
    })
}

/// Get detailed graph statistics.
///
/// # Arguments
///
/// * `state` - Application state
///
/// # Returns
///
/// Returns GraphStatsResponse with detailed statistics.
///
/// # Errors
///
/// Returns error if graph is not loaded.
#[cfg(feature = "graph")]
pub async fn graph_stats(state: &AppState) -> Result<GraphStatsResponse> {
    use std::collections::HashMap;

    let graph_state = state.graph.read().unwrap();
    let loaded = match &*graph_state {
        GraphState::Loaded(loaded) => loaded,
        GraphState::NotLoaded => {
            return Err(crate::error::Error::not_found_msg("Graph not loaded yet"))
        }
        GraphState::Loading => {
            return Err(crate::error::Error::not_found_msg(
                "Graph is currently loading",
            ))
        }
        GraphState::Failed(error) => {
            return Err(crate::error::Error::config(format!(
                "Graph failed to load: {}",
                error
            )))
        }
    };

    let graph = &loaded.graph;

    // Count by relationship type
    let mut rel_counts: HashMap<String, usize> = HashMap::new();
    for edge in graph.edge_references() {
        let rel_name = format!("{:?}", edge.weight().relationship);
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

    // Count by category
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

    Ok(GraphStatsResponse {
        nodes: NodeCounts {
            total: loaded.stats.node_count,
            concepts: loaded.stats.concept_count,
            sources: loaded.stats.source_count,
        },
        edge_count: loaded.stats.edge_count,
        relationships,
        categories,
    })
}

/// Validate graph integrity.
///
/// # Arguments
///
/// * `state` - Application state
///
/// # Returns
///
/// Returns GraphValidateResponse with validation results.
///
/// # Errors
///
/// Returns error if graph is not loaded.
#[cfg(feature = "graph")]
pub async fn graph_validate(state: &AppState) -> Result<GraphValidateResponse> {
    let graph_state = state.graph.read().unwrap();
    let loaded = match &*graph_state {
        GraphState::Loaded(loaded) => loaded,
        GraphState::NotLoaded => {
            return Err(crate::error::Error::not_found_msg("Graph not loaded yet"))
        }
        GraphState::Loading => {
            return Err(crate::error::Error::not_found_msg(
                "Graph is currently loading",
            ))
        }
        GraphState::Failed(error) => {
            return Err(crate::error::Error::config(format!(
                "Graph failed to load: {}",
                error
            )))
        }
    };

    let graph = &loaded.graph;

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

    Ok(GraphValidateResponse {
        valid: issues.is_empty(),
        issues,
        orphan_count,
        self_loop_count,
    })
}

/// Get node information by ID.
///
/// # Arguments
///
/// * `state` - Application state
/// * `node_id` - Node ID to query
///
/// # Returns
///
/// Returns NodeInfo with node details and degree counts.
///
/// # Errors
///
/// Returns error if graph is not loaded or node not found.
#[cfg(feature = "graph")]
pub async fn get_node(state: &AppState, node_id: &str) -> Result<NodeInfo> {
    let graph_state = state.graph.read().unwrap();
    let loaded = match &*graph_state {
        GraphState::Loaded(loaded) => loaded,
        GraphState::NotLoaded => {
            return Err(crate::error::Error::not_found_msg("Graph not loaded yet"))
        }
        GraphState::Loading => {
            return Err(crate::error::Error::not_found_msg(
                "Graph is currently loading",
            ))
        }
        GraphState::Failed(error) => {
            return Err(crate::error::Error::config(format!(
                "Graph failed to load: {}",
                error
            )))
        }
    };

    let node_idx = loaded.node_index.get(node_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", node_id))
    })?;

    let node = &loaded.graph[*node_idx];
    let in_degree = loaded
        .graph
        .edges_directed(*node_idx, petgraph::Direction::Incoming)
        .count();
    let out_degree = loaded
        .graph
        .edges_directed(*node_idx, petgraph::Direction::Outgoing)
        .count();

    let (node_type, details) = match node {
        Node::Concept(c) => (
            "concept".to_string(),
            NodeDetails::Concept {
                title: c.title.clone(),
                category: c.category.clone(),
                source_id: c.source_id.clone(),
                canonical_id: c.canonical_id.clone(),
                is_canonical: c.is_canonical,
            },
        ),
        Node::Source(s) => (
            "source".to_string(),
            NodeDetails::Source {
                title: s.title.clone(),
                author: s.author.clone(),
                year: s.year,
                is_converted: s.is_converted,
            },
        ),
    };

    Ok(NodeInfo {
        id: node_id.to_string(),
        node_type,
        details,
        in_degree,
        out_degree,
    })
}

/// Get edges for a node.
///
/// # Arguments
///
/// * `state` - Application state
/// * `node_id` - Node ID to query
/// * `direction` - Direction filter ("incoming", "outgoing", "both")
///
/// # Returns
///
/// Returns NodeEdgesResponse with edge lists.
///
/// # Errors
///
/// Returns error if graph is not loaded or node not found.
#[cfg(feature = "graph")]
pub async fn get_node_edges(
    state: &AppState,
    node_id: &str,
    direction: &str,
) -> Result<NodeEdgesResponse> {
    let graph_state = state.graph.read().unwrap();
    let loaded = match &*graph_state {
        GraphState::Loaded(loaded) => loaded,
        GraphState::NotLoaded => {
            return Err(crate::error::Error::not_found_msg("Graph not loaded yet"))
        }
        GraphState::Loading => {
            return Err(crate::error::Error::not_found_msg(
                "Graph is currently loading",
            ))
        }
        GraphState::Failed(error) => {
            return Err(crate::error::Error::config(format!(
                "Graph failed to load: {}",
                error
            )))
        }
    };

    let node_idx = loaded.node_index.get(node_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", node_id))
    })?;

    let mut incoming = Vec::new();
    let mut outgoing = Vec::new();

    if direction == "incoming" || direction == "both" {
        for edge in loaded
            .graph
            .edges_directed(*node_idx, petgraph::Direction::Incoming)
        {
            let from_node = &loaded.graph[edge.source()];
            let from_id = match from_node {
                Node::Concept(c) => c.id.clone(),
                Node::Source(s) => s.id.clone(),
            };

            incoming.push(EdgeInfo {
                from: from_id,
                to: node_id.to_string(),
                relationship: format!("{:?}", edge.weight().relationship),
                weight: edge.weight().weight,
                origin: format!("{:?}", edge.weight().origin),
            });
        }
    }

    if direction == "outgoing" || direction == "both" {
        for edge in loaded
            .graph
            .edges_directed(*node_idx, petgraph::Direction::Outgoing)
        {
            let to_node = &loaded.graph[edge.target()];
            let to_id = match to_node {
                Node::Concept(c) => c.id.clone(),
                Node::Source(s) => s.id.clone(),
            };

            outgoing.push(EdgeInfo {
                from: node_id.to_string(),
                to: to_id,
                relationship: format!("{:?}", edge.weight().relationship),
                weight: edge.weight().weight,
                origin: format!("{:?}", edge.weight().origin),
            });
        }
    }

    Ok(NodeEdgesResponse {
        node_id: node_id.to_string(),
        direction: direction.to_string(),
        incoming,
        outgoing,
    })
}

// Non-feature versions that return "not available" errors
#[cfg(not(feature = "graph"))]
pub async fn graph_stats(_state: &AppState) -> Result<GraphStatsResponse> {
    Err(crate::error::Error::config(
        "Graph feature not enabled in this build".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn graph_validate(_state: &AppState) -> Result<GraphValidateResponse> {
    Err(crate::error::Error::config(
        "Graph feature not enabled in this build".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn get_node(_state: &AppState, _node_id: &str) -> Result<NodeInfo> {
    Err(crate::error::Error::config(
        "Graph feature not enabled in this build".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn get_node_edges(
    _state: &AppState,
    _node_id: &str,
    _direction: &str,
) -> Result<NodeEdgesResponse> {
    Err(crate::error::Error::config(
        "Graph feature not enabled in this build".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_status_response_serialization() {
        let response = GraphStatusResponse {
            enabled: true,
            status: "loaded".to_string(),
            error: None,
            stats: Some(BasicGraphStats {
                node_count: 100,
                edge_count: 150,
                concept_count: 80,
                source_count: 20,
            }),
            loaded_at: Some("2024-01-01T00:00:00Z".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"enabled\":true"));
        assert!(json.contains("\"status\":\"loaded\""));
        assert!(json.contains("\"node_count\":100"));
    }

    #[test]
    fn test_node_info_serialization() {
        let info = NodeInfo {
            id: "test-concept".to_string(),
            node_type: "concept".to_string(),
            details: NodeDetails::Concept {
                title: "Test Concept".to_string(),
                category: "test".to_string(),
                source_id: "test-source".to_string(),
                canonical_id: None,
                is_canonical: true,
            },
            in_degree: 2,
            out_degree: 3,
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"id\":\"test-concept\""));
        assert!(json.contains("\"node_type\":\"concept\""));
        assert!(json.contains("\"in_degree\":2"));
    }

    #[test]
    fn test_edge_info_serialization() {
        let edge = EdgeInfo {
            from: "concept-a".to_string(),
            to: "concept-b".to_string(),
            relationship: "Prerequisite".to_string(),
            weight: 1.0,
            origin: "Extracted".to_string(),
        };

        let json = serde_json::to_string(&edge).unwrap();
        assert!(json.contains("\"from\":\"concept-a\""));
        assert!(json.contains("\"to\":\"concept-b\""));
        assert!(json.contains("\"relationship\":\"Prerequisite\""));
    }

    #[test]
    fn test_get_node_edges_params_default_direction() {
        let json = r#"{"node_id":"test"}"#;
        let params: GetNodeEdgesParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.direction, "both");
    }

    // Functional tests for graph tools
    #[cfg(feature = "graph")]
    mod functional {
        use super::*;
        use crate::config::Config;
        use crate::graph::types::{ConceptNode, Edge, EdgeOrigin, GraphData, Relationship, SourceNode};
        use crate::graph::LoadedGraph;
        use crate::state::GraphState;
        use std::sync::Arc;

        /// Helper to create AppState with a test graph
        async fn create_test_state() -> Arc<AppState> {
            // Create test graph data
            let nodes = vec![
                Node::Source(SourceNode {
                    id: "test-source".to_string(),
                    title: "Test Source".to_string(),
                    author: "Test Author".to_string(),
                    year: Some(2024),
                    is_converted: true,
                }),
                Node::Concept(ConceptNode {
                    id: "concept-a".to_string(),
                    title: "Concept A".to_string(),
                    category: "harmony".to_string(),
                    source_id: "test-source".to_string(),
                    canonical_id: None,
                    is_canonical: true,
                }),
                Node::Concept(ConceptNode {
                    id: "concept-b".to_string(),
                    title: "Concept B".to_string(),
                    category: "fundamentals".to_string(),
                    source_id: "test-source".to_string(),
                    canonical_id: None,
                    is_canonical: true,
                }),
            ];

            let edges = vec![
                Edge {
                    from: "test-source".to_string(),
                    to: "concept-a".to_string(),
                    relationship: Relationship::Introduces,
                    weight: 1.0,
                    origin: EdgeOrigin::Extracted,
                },
                Edge {
                    from: "concept-a".to_string(),
                    to: "concept-b".to_string(),
                    relationship: Relationship::Prerequisite,
                    weight: 1.0,
                    origin: EdgeOrigin::Extracted,
                },
            ];

            let graph_data = GraphData {
                version: "1.0".to_string(),
                nodes,
                edges,
                metadata: None,
            };

            // Convert to petgraph and build loaded graph
            let graph = crate::graph::persistence::to_petgraph(&graph_data);

            // Build node index
            let mut node_index = std::collections::HashMap::new();
            for idx in graph.node_indices() {
                let id = match &graph[idx] {
                    Node::Concept(c) => c.id.clone(),
                    Node::Source(s) => s.id.clone(),
                };
                node_index.insert(id, idx);
            }

            // Compute stats
            let stats = crate::graph::GraphStats {
                node_count: 3,
                edge_count: 2,
                concept_count: 2,
                source_count: 1,
            };

            let loaded = LoadedGraph {
                graph,
                node_index,
                loaded_at: chrono::Utc::now(),
                stats,
            };

            // Create AppState with default config
            let config = Config::load().unwrap();
            let state = Arc::new(AppState::new(config).await.unwrap());

            // Replace graph with loaded test graph
            {
                let mut graph_guard = state.graph.write().unwrap();
                *graph_guard = GraphState::Loaded(loaded);
            }

            state
        }

        #[tokio::test]
        async fn test_graph_status_loaded() {
            let state = create_test_state().await;
            let response = graph_status(&state).await.unwrap();

            assert!(response.enabled);
            assert_eq!(response.status, "loaded");
            assert!(response.error.is_none());
            assert!(response.stats.is_some());
            assert!(response.loaded_at.is_some());

            let stats = response.stats.unwrap();
            assert_eq!(stats.node_count, 3);
            assert_eq!(stats.edge_count, 2);
            assert_eq!(stats.concept_count, 2);
            assert_eq!(stats.source_count, 1);
        }

        #[tokio::test]
        async fn test_graph_status_not_loaded() {
            let config = Config::load().unwrap();
            let state = Arc::new(AppState::new(config).await.unwrap());

            let response = graph_status(&state).await.unwrap();
            assert!(response.enabled);
            assert_eq!(response.status, "not_loaded");
            assert!(response.stats.is_none());
        }

        #[tokio::test]
        async fn test_graph_stats() {
            let state = create_test_state().await;
            let response = graph_stats(&state).await.unwrap();

            assert_eq!(response.nodes.total, 3);
            assert_eq!(response.nodes.concepts, 2);
            assert_eq!(response.nodes.sources, 1);
            assert_eq!(response.edge_count, 2);

            assert_eq!(response.relationships.len(), 2);
            assert_eq!(response.categories.len(), 2);

            // Verify harmony category exists
            assert!(response
                .categories
                .iter()
                .any(|c| c.category == "harmony"));
        }

        #[tokio::test]
        async fn test_graph_validate() {
            let state = create_test_state().await;
            let response = graph_validate(&state).await.unwrap();

            assert!(response.valid);
            assert_eq!(response.orphan_count, 0);
            assert_eq!(response.self_loop_count, 0);
            assert!(response.issues.is_empty());
        }

        #[tokio::test]
        async fn test_get_node_concept() {
            let state = create_test_state().await;
            let info = get_node(&state, "concept-a").await.unwrap();

            assert_eq!(info.id, "concept-a");
            assert_eq!(info.node_type, "concept");
            assert_eq!(info.in_degree, 1);
            assert_eq!(info.out_degree, 1);

            if let NodeDetails::Concept {
                title, category, ..
            } = info.details
            {
                assert_eq!(title, "Concept A");
                assert_eq!(category, "harmony");
            } else {
                panic!("Expected Concept node details");
            }
        }

        #[tokio::test]
        async fn test_get_node_source() {
            let state = create_test_state().await;
            let info = get_node(&state, "test-source").await.unwrap();

            assert_eq!(info.id, "test-source");
            assert_eq!(info.node_type, "source");
            assert_eq!(info.in_degree, 0);
            assert_eq!(info.out_degree, 1);

            if let NodeDetails::Source {
                title, author, ..
            } = info.details
            {
                assert_eq!(title, "Test Source");
                assert_eq!(author, "Test Author");
            } else {
                panic!("Expected Source node details");
            }
        }

        #[tokio::test]
        async fn test_get_node_not_found() {
            let state = create_test_state().await;
            let result = get_node(&state, "nonexistent").await;
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("not found"));
        }

        #[tokio::test]
        async fn test_get_node_edges_both() {
            let state = create_test_state().await;
            let response = get_node_edges(&state, "concept-a", "both").await.unwrap();

            assert_eq!(response.node_id, "concept-a");
            assert_eq!(response.direction, "both");
            assert_eq!(response.incoming.len(), 1);
            assert_eq!(response.outgoing.len(), 1);

            assert_eq!(response.incoming[0].from, "test-source");
            assert_eq!(response.outgoing[0].to, "concept-b");
        }

        #[tokio::test]
        async fn test_get_node_edges_incoming() {
            let state = create_test_state().await;
            let response = get_node_edges(&state, "concept-a", "incoming")
                .await
                .unwrap();

            assert_eq!(response.direction, "incoming");
            assert_eq!(response.incoming.len(), 1);
            assert_eq!(response.outgoing.len(), 0);
        }

        #[tokio::test]
        async fn test_get_node_edges_outgoing() {
            let state = create_test_state().await;
            let response = get_node_edges(&state, "concept-a", "outgoing")
                .await
                .unwrap();

            assert_eq!(response.direction, "outgoing");
            assert_eq!(response.incoming.len(), 0);
            assert_eq!(response.outgoing.len(), 1);
        }

        #[tokio::test]
        async fn test_get_node_edges_not_found() {
            let state = create_test_state().await;
            let result = get_node_edges(&state, "nonexistent", "both").await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_graph_stats_not_loaded() {
            let config = Config::load().unwrap();
            let state = Arc::new(AppState::new(config).await.unwrap());

            let result = graph_stats(&state).await;
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("not loaded"));
        }

        #[tokio::test]
        async fn test_graph_validate_not_loaded() {
            let config = Config::load().unwrap();
            let state = Arc::new(AppState::new(config).await.unwrap());

            let result = graph_validate(&state).await;
            assert!(result.is_err());
        }
    }
}
