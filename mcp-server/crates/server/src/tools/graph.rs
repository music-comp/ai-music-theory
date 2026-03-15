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
use crate::graph::{
    from_fabryk_relationship, is_concept_node, is_source_node, node_category, node_id, node_title,
    source_author, source_is_converted, source_year, validate_graph,
};
#[cfg(feature = "graph")]
use fabryk::core::ServiceState;
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
pub struct GetNodeEdgesParams {
    /// Node ID to query
    pub node_id: String,
    /// Optional direction filter ("incoming", "outgoing", "both")
    #[serde(default = "default_direction")]
    pub direction: String,
}

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
        let svc_state = state.graph_service.state();
        match svc_state {
            ServiceState::Ready => {
                let guard = state.graph_data.read().unwrap();
                let loaded = guard.as_ref().unwrap();
                Ok(GraphStatusResponse {
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
                })
            }
            ServiceState::Starting => Ok(GraphStatusResponse {
                enabled: true,
                status: "loading".to_string(),
                error: None,
                stats: None,
                loaded_at: None,
            }),
            ServiceState::Failed(msg) => Ok(GraphStatusResponse {
                enabled: true,
                status: "failed".to_string(),
                error: Some(msg),
                stats: None,
                loaded_at: None,
            }),
            _ => Ok(GraphStatusResponse {
                enabled: true,
                status: "not_loaded".to_string(),
                error: None,
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

    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Count by relationship type
    let mut rel_counts: HashMap<String, usize> = HashMap::new();
    for edge in loaded.data.graph.edge_references() {
        let rel_name = from_fabryk_relationship(&edge.weight().relationship);
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
    for node in loaded.data.iter_nodes() {
        if is_concept_node(node) {
            let cat = node_category(node).to_string();
            *category_counts.entry(cat).or_insert(0) += 1;
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
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    let result = validate_graph(&loaded.data);

    let mut issues = Vec::new();
    let mut orphan_count = 0;
    let mut self_loop_count = 0;

    // Extract orphan and self-loop counts from validation issues
    for issue in result.errors.iter().chain(result.warnings.iter()) {
        if issue.code.contains("orphan") {
            orphan_count = issue.nodes.len();
        } else if issue.code.contains("self_loop") || issue.code.contains("self-loop") {
            self_loop_count = issue.edges.len().max(1);
        }
        issues.push(format!("[{}] {}", issue.code, issue.message));
    }

    Ok(GraphValidateResponse {
        valid: result.valid && issues.is_empty(),
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
pub async fn get_node(state: &AppState, node_id_param: &str) -> Result<NodeInfo> {
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    let node = loaded.data.get_node(node_id_param).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", node_id_param))
    })?;

    let idx = loaded.data.get_index(node_id_param).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node index not found: {}", node_id_param))
    })?;

    let in_degree = loaded
        .data
        .graph
        .edges_directed(idx, petgraph::Direction::Incoming)
        .count();
    let out_degree = loaded
        .data
        .graph
        .edges_directed(idx, petgraph::Direction::Outgoing)
        .count();

    let (ntype, details) = if is_concept_node(node) {
        (
            "concept".to_string(),
            NodeDetails::Concept {
                title: node_title(node).to_string(),
                category: node_category(node).to_string(),
                source_id: node.source_id.clone().unwrap_or_default(),
                canonical_id: node.canonical_id.clone(),
                is_canonical: node.is_canonical,
            },
        )
    } else if is_source_node(node) {
        (
            "source".to_string(),
            NodeDetails::Source {
                title: node_title(node).to_string(),
                author: source_author(node).to_string(),
                year: source_year(node),
                is_converted: source_is_converted(node),
            },
        )
    } else {
        (
            "unknown".to_string(),
            NodeDetails::Concept {
                title: node_title(node).to_string(),
                category: node_category(node).to_string(),
                source_id: node.source_id.clone().unwrap_or_default(),
                canonical_id: node.canonical_id.clone(),
                is_canonical: node.is_canonical,
            },
        )
    };

    Ok(NodeInfo {
        id: node_id_param.to_string(),
        node_type: ntype,
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
    node_id_param: &str,
    direction: &str,
) -> Result<NodeEdgesResponse> {
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    let idx = loaded.data.get_index(node_id_param).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", node_id_param))
    })?;

    let mut incoming = Vec::new();
    let mut outgoing = Vec::new();

    if direction == "incoming" || direction == "both" {
        for edge in loaded
            .data
            .graph
            .edges_directed(idx, petgraph::Direction::Incoming)
        {
            let from_node = &loaded.data.graph[edge.source()];

            incoming.push(EdgeInfo {
                from: node_id(from_node).to_string(),
                to: node_id_param.to_string(),
                relationship: from_fabryk_relationship(&edge.weight().relationship),
                weight: edge.weight().weight,
                origin: format!("{:?}", edge.weight().origin),
            });
        }
    }

    if direction == "outgoing" || direction == "both" {
        for edge in loaded
            .data
            .graph
            .edges_directed(idx, petgraph::Direction::Outgoing)
        {
            let to_node = &loaded.data.graph[edge.target()];

            outgoing.push(EdgeInfo {
                from: node_id_param.to_string(),
                to: node_id(to_node).to_string(),
                relationship: from_fabryk_relationship(&edge.weight().relationship),
                weight: edge.weight().weight,
                origin: format!("{:?}", edge.weight().origin),
            });
        }
    }

    Ok(NodeEdgesResponse {
        node_id: node_id_param.to_string(),
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

    #[test]
    fn test_get_node_edges_params_explicit_direction() {
        let json = r#"{"node_id":"test","direction":"incoming"}"#;
        let params: GetNodeEdgesParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.node_id, "test");
        assert_eq!(params.direction, "incoming");
    }

    #[test]
    fn test_graph_status_response_skip_none_fields() {
        let response = GraphStatusResponse {
            enabled: false,
            status: "not_available".to_string(),
            error: None,
            stats: None,
            loaded_at: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(!json.contains("error"));
        assert!(!json.contains("stats"));
        assert!(!json.contains("loaded_at"));
    }

    #[test]
    fn test_graph_status_response_with_error() {
        let response = GraphStatusResponse {
            enabled: true,
            status: "failed".to_string(),
            error: Some("Graph load failed".to_string()),
            stats: None,
            loaded_at: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"error\":\"Graph load failed\""));
        assert!(json.contains("\"status\":\"failed\""));
    }

    #[test]
    fn test_graph_status_response_deserialization() {
        let json = r#"{"enabled":true,"status":"loaded","stats":{"node_count":5,"edge_count":3,"concept_count":4,"source_count":1},"loaded_at":"2024-01-01T00:00:00Z"}"#;
        let response: GraphStatusResponse = serde_json::from_str(json).unwrap();
        assert!(response.enabled);
        assert_eq!(response.status, "loaded");
        assert!(response.error.is_none());
        let stats = response.stats.unwrap();
        assert_eq!(stats.node_count, 5);
        assert_eq!(stats.edge_count, 3);
        assert_eq!(stats.concept_count, 4);
        assert_eq!(stats.source_count, 1);
        assert_eq!(response.loaded_at.unwrap(), "2024-01-01T00:00:00Z");
    }

    #[test]
    fn test_node_info_source_serialization() {
        let info = NodeInfo {
            id: "test-source".to_string(),
            node_type: "source".to_string(),
            details: NodeDetails::Source {
                title: "Test Source".to_string(),
                author: "Author Name".to_string(),
                year: Some(2024),
                is_converted: true,
            },
            in_degree: 0,
            out_degree: 5,
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"id\":\"test-source\""));
        assert!(json.contains("\"node_type\":\"source\""));
        assert!(json.contains("\"author\":\"Author Name\""));
        assert!(json.contains("\"year\":2024"));
        assert!(json.contains("\"is_converted\":true"));
        assert!(json.contains("\"out_degree\":5"));
    }

    #[test]
    fn test_node_info_source_no_year() {
        let info = NodeInfo {
            id: "src-2".to_string(),
            node_type: "source".to_string(),
            details: NodeDetails::Source {
                title: "Unknown Year Source".to_string(),
                author: "Somebody".to_string(),
                year: None,
                is_converted: false,
            },
            in_degree: 1,
            out_degree: 0,
        };
        let json = serde_json::to_string(&info).unwrap();
        // year: None is skipped by skip_serializing_if
        assert!(!json.contains("\"year\""));
        assert!(json.contains("\"is_converted\":false"));
    }

    #[test]
    fn test_node_details_concept_with_canonical_id() {
        let info = NodeInfo {
            id: "concept-alias".to_string(),
            node_type: "concept".to_string(),
            details: NodeDetails::Concept {
                title: "Alias Concept".to_string(),
                category: "harmony".to_string(),
                source_id: "src-1".to_string(),
                canonical_id: Some("concept-canonical".to_string()),
                is_canonical: false,
            },
            in_degree: 0,
            out_degree: 1,
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"canonical_id\":\"concept-canonical\""));
        assert!(json.contains("\"is_canonical\":false"));
    }

    #[test]
    fn test_node_details_concept_without_canonical_id() {
        let details = NodeDetails::Concept {
            title: "Root Concept".to_string(),
            category: "fundamentals".to_string(),
            source_id: "src-1".to_string(),
            canonical_id: None,
            is_canonical: true,
        };
        let json = serde_json::to_string(&details).unwrap();
        assert!(!json.contains("canonical_id"));
        assert!(json.contains("\"is_canonical\":true"));
    }

    #[test]
    fn test_graph_stats_response_serialization() {
        let response = GraphStatsResponse {
            nodes: NodeCounts {
                total: 50,
                concepts: 40,
                sources: 10,
            },
            edge_count: 75,
            relationships: vec![
                RelationshipCount {
                    relationship: "Prerequisite".to_string(),
                    count: 30,
                },
                RelationshipCount {
                    relationship: "Introduces".to_string(),
                    count: 45,
                },
            ],
            categories: vec![CategoryCount {
                category: "harmony".to_string(),
                count: 20,
            }],
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"total\":50"));
        assert!(json.contains("\"concepts\":40"));
        assert!(json.contains("\"sources\":10"));
        assert!(json.contains("\"edge_count\":75"));
        assert!(json.contains("\"Prerequisite\""));
        assert!(json.contains("\"harmony\""));
    }

    #[test]
    fn test_graph_validate_response_serialization() {
        let response = GraphValidateResponse {
            valid: false,
            issues: vec![
                "[orphan_nodes] Found 2 orphan nodes".to_string(),
                "[self_loop] Found self-loop".to_string(),
            ],
            orphan_count: 2,
            self_loop_count: 1,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"valid\":false"));
        assert!(json.contains("orphan_nodes"));
        assert!(json.contains("\"orphan_count\":2"));
        assert!(json.contains("\"self_loop_count\":1"));
    }

    #[test]
    fn test_graph_validate_response_valid() {
        let response = GraphValidateResponse {
            valid: true,
            issues: vec![],
            orphan_count: 0,
            self_loop_count: 0,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"valid\":true"));
        assert!(json.contains("\"issues\":[]"));
    }

    #[test]
    fn test_node_edges_response_serialization() {
        let response = NodeEdgesResponse {
            node_id: "concept-a".to_string(),
            direction: "both".to_string(),
            incoming: vec![EdgeInfo {
                from: "test-source".to_string(),
                to: "concept-a".to_string(),
                relationship: "Introduces".to_string(),
                weight: 1.0,
                origin: "Extracted".to_string(),
            }],
            outgoing: vec![EdgeInfo {
                from: "concept-a".to_string(),
                to: "concept-b".to_string(),
                relationship: "Prerequisite".to_string(),
                weight: 0.8,
                origin: "Extracted".to_string(),
            }],
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"node_id\":\"concept-a\""));
        assert!(json.contains("\"direction\":\"both\""));
        assert!(json.contains("\"incoming\""));
        assert!(json.contains("\"outgoing\""));
        assert!(json.contains("\"weight\":0.8"));
    }

    #[test]
    fn test_node_edges_response_empty() {
        let response = NodeEdgesResponse {
            node_id: "isolated-node".to_string(),
            direction: "both".to_string(),
            incoming: vec![],
            outgoing: vec![],
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"incoming\":[]"));
        assert!(json.contains("\"outgoing\":[]"));
    }

    #[test]
    fn test_default_direction_fn() {
        assert_eq!(default_direction(), "both");
    }

    #[test]
    fn test_edge_info_deserialization() {
        let json = r#"{"from":"a","to":"b","relationship":"Prerequisite","weight":0.5,"origin":"Extracted"}"#;
        let edge: EdgeInfo = serde_json::from_str(json).unwrap();
        assert_eq!(edge.from, "a");
        assert_eq!(edge.to, "b");
        assert_eq!(edge.relationship, "Prerequisite");
        assert!((edge.weight - 0.5).abs() < f32::EPSILON);
        assert_eq!(edge.origin, "Extracted");
    }

    #[test]
    fn test_basic_graph_stats_deserialization() {
        let json = r#"{"node_count":10,"edge_count":20,"concept_count":8,"source_count":2}"#;
        let stats: BasicGraphStats = serde_json::from_str(json).unwrap();
        assert_eq!(stats.node_count, 10);
        assert_eq!(stats.edge_count, 20);
        assert_eq!(stats.concept_count, 8);
        assert_eq!(stats.source_count, 2);
    }

    // Functional tests for graph tools
    #[cfg(feature = "graph")]
    mod functional {
        use super::*;
        use crate::config::Config;
        use crate::graph::{GraphStats, LoadedGraph};
        use fabryk::core::ServiceState;
        use fabryk::graph::{Edge, Node, NodeType, Relationship};
        use std::sync::Arc;

        /// Helper to create AppState with a test graph
        async fn create_test_state() -> Arc<AppState> {
            let mut data = fabryk::graph::GraphData::new();

            // Source node
            data.add_node(
                Node::new("test-source", "Test Source")
                    .with_node_type(NodeType::Custom("source".to_string()))
                    .with_metadata("author", serde_json::json!("Test Author"))
                    .with_metadata("year", serde_json::json!(2024))
                    .with_metadata("is_converted", serde_json::json!(true)),
            );

            // Concept nodes
            data.add_node(
                Node::new("concept-a", "Concept A")
                    .with_category("harmony")
                    .with_source("test-source"),
            );
            data.add_node(
                Node::new("concept-b", "Concept B")
                    .with_category("fundamentals")
                    .with_source("test-source"),
            );

            // Edges
            data.add_edge(Edge::new(
                "test-source",
                "concept-a",
                Relationship::Introduces,
            ))
            .expect("edge");
            data.add_edge(Edge::new(
                "concept-a",
                "concept-b",
                Relationship::Prerequisite,
            ))
            .expect("edge");

            let stats = GraphStats {
                node_count: 3,
                edge_count: 2,
                concept_count: 2,
                source_count: 1,
            };

            let loaded = LoadedGraph {
                data,
                loaded_at: chrono::Utc::now(),
                stats,
            };

            // Create AppState with default config
            let config = Config::load().unwrap();
            let state = Arc::new(AppState::new(config).await.unwrap());

            // Store graph data and mark service ready
            {
                let mut graph_guard = state.graph_data.write().unwrap();
                *graph_guard = Some(loaded);
            }
            state.graph_service.set_state(ServiceState::Ready);

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
            assert!(response.categories.iter().any(|c| c.category == "harmony"));
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

            if let NodeDetails::Source { title, author, .. } = info.details {
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

        #[tokio::test]
        async fn test_graph_status_starting() {
            let config = Config::load().unwrap();
            let state = Arc::new(AppState::new(config).await.unwrap());
            state.graph_service.set_state(ServiceState::Starting);

            let response = graph_status(&state).await.unwrap();
            assert!(response.enabled);
            assert_eq!(response.status, "loading");
            assert!(response.error.is_none());
            assert!(response.stats.is_none());
            assert!(response.loaded_at.is_none());
        }

        #[tokio::test]
        async fn test_graph_status_failed() {
            let config = Config::load().unwrap();
            let state = Arc::new(AppState::new(config).await.unwrap());
            state
                .graph_service
                .set_state(ServiceState::Failed("disk full".to_string()));

            let response = graph_status(&state).await.unwrap();
            assert!(response.enabled);
            assert_eq!(response.status, "failed");
            assert_eq!(response.error, Some("disk full".to_string()));
            assert!(response.stats.is_none());
            assert!(response.loaded_at.is_none());
        }

        #[tokio::test]
        async fn test_get_node_not_loaded() {
            let config = Config::load().unwrap();
            let state = Arc::new(AppState::new(config).await.unwrap());

            let result = get_node(&state, "any-node").await;
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("not loaded"));
        }

        #[tokio::test]
        async fn test_get_node_edges_not_loaded() {
            let config = Config::load().unwrap();
            let state = Arc::new(AppState::new(config).await.unwrap());

            let result = get_node_edges(&state, "any-node", "both").await;
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("not loaded"));
        }

        #[tokio::test]
        async fn test_get_node_source_details() {
            let state = create_test_state().await;
            let info = get_node(&state, "test-source").await.unwrap();

            if let NodeDetails::Source {
                title,
                author,
                year,
                is_converted,
            } = info.details
            {
                assert_eq!(title, "Test Source");
                assert_eq!(author, "Test Author");
                assert_eq!(year, Some(2024));
                assert!(is_converted);
            } else {
                panic!("Expected Source node details");
            }
        }

        #[tokio::test]
        async fn test_get_node_concept_b_details() {
            let state = create_test_state().await;
            let info = get_node(&state, "concept-b").await.unwrap();

            assert_eq!(info.id, "concept-b");
            assert_eq!(info.node_type, "concept");
            // concept-b has one incoming edge (from concept-a) and no outgoing
            assert_eq!(info.in_degree, 1);
            assert_eq!(info.out_degree, 0);

            if let NodeDetails::Concept {
                title, category, ..
            } = info.details
            {
                assert_eq!(title, "Concept B");
                assert_eq!(category, "fundamentals");
            } else {
                panic!("Expected Concept node details");
            }
        }

        #[tokio::test]
        async fn test_get_node_edges_source_outgoing() {
            let state = create_test_state().await;
            let response = get_node_edges(&state, "test-source", "outgoing")
                .await
                .unwrap();

            assert_eq!(response.node_id, "test-source");
            assert_eq!(response.direction, "outgoing");
            assert!(response.incoming.is_empty());
            assert_eq!(response.outgoing.len(), 1);
            assert_eq!(response.outgoing[0].to, "concept-a");
            assert_eq!(response.outgoing[0].from, "test-source");
        }

        #[tokio::test]
        async fn test_get_node_edges_leaf_node() {
            let state = create_test_state().await;
            // concept-b has incoming only, no outgoing
            let response = get_node_edges(&state, "concept-b", "both").await.unwrap();

            assert_eq!(response.incoming.len(), 1);
            assert!(response.outgoing.is_empty());
        }
    }
}
