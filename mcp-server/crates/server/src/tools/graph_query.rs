//! Graph query tools for exploring concept relationships.
//!
//! This module provides MCP tools for traversing and querying the concept graph:
//! - get_related_concepts: Find related concepts with filtering
//! - find_concept_path: Find shortest path between concepts
//! - get_prerequisites: Get prerequisites in learning order
//! - get_concept_neighborhood: Get local subgraph around concept

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::state::AppState;

#[cfg(feature = "graph")]
use crate::graph::query::*;
#[cfg(feature = "graph")]
use crate::graph::types::{Node, Relationship};
#[cfg(feature = "graph")]
use crate::state::GraphState;
#[cfg(feature = "graph")]
use petgraph::graph::NodeIndex;
#[cfg(feature = "graph")]
use petgraph::visit::EdgeRef;
#[cfg(feature = "graph")]
use petgraph::Direction;
#[cfg(feature = "graph")]
use std::collections::{HashMap, HashSet, VecDeque};

// ============================================================================
// Parameter Types
// ============================================================================

/// Parameters for get_related_concepts.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetRelatedConceptsParams {
    /// Concept ID to query
    pub concept_id: String,
    /// Optional relationship types filter (comma-separated: "prerequisite,relates_to")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_types: Option<String>,
    /// Optional direction filter: "incoming", "outgoing", "both" (default: "both")
    #[serde(default = "default_direction")]
    pub direction: String,
    /// Depth to traverse (default: 1, max: 3)
    #[serde(default = "default_depth_1")]
    pub depth: u32,
}

fn default_direction() -> String {
    "both".to_string()
}

fn default_depth_1() -> u32 {
    1
}

/// Parameters for find_concept_path.
#[derive(Debug, Serialize, Deserialize)]
pub struct FindConceptPathParams {
    /// Starting concept ID
    pub from_id: String,
    /// Ending concept ID
    pub to_id: String,
    /// Maximum depth to search (default: 5, max: 8)
    #[serde(default = "default_depth_5")]
    pub max_depth: u32,
}

fn default_depth_5() -> u32 {
    5
}

/// Parameters for get_prerequisites.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetPrerequisitesParams {
    /// Concept ID to query
    pub concept_id: String,
    /// Depth to traverse (default: 3, max: 5)
    #[serde(default = "default_depth_3")]
    pub depth: u32,
}

fn default_depth_3() -> u32 {
    3
}

/// Parameters for get_concept_neighborhood.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetConceptNeighborhoodParams {
    /// Concept ID to query
    pub concept_id: String,
    /// Radius (number of hops, default: 2, max: 3)
    #[serde(default = "default_radius")]
    pub radius: u32,
    /// Maximum nodes to return (default: 30, max: 50)
    #[serde(default = "default_max_nodes")]
    pub max_nodes: u32,
}

fn default_radius() -> u32 {
    2
}

fn default_max_nodes() -> u32 {
    30
}

/// Parameters for get_dependents.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDependentsParams {
    /// Concept ID to query
    pub concept_id: String,
    /// Depth to traverse (default: 2, max: 4)
    #[serde(default = "default_depth_2")]
    pub depth: u32,
}

fn default_depth_2() -> u32 {
    2
}

/// Parameters for get_central_concepts.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetCentralConceptsParams {
    /// Optional category filter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Limit number of results (default: 10, max: 25)
    #[serde(default = "default_limit_10")]
    pub limit: u32,
}

fn default_limit_10() -> u32 {
    10
}

/// Parameters for get_concept_sources.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetConceptSourcesParams {
    /// Concept ID to query
    pub concept_id: String,
}

/// Parameters for get_concept_variants.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetConceptVariantsParams {
    /// Canonical concept ID
    pub canonical_id: String,
}

/// Parameters for find_bridge_concepts.
#[derive(Debug, Serialize, Deserialize)]
pub struct FindBridgeConceptsParams {
    /// First category
    pub category_a: String,
    /// Second category
    pub category_b: String,
    /// Limit number of results (default: 5, max: 15)
    #[serde(default = "default_limit_5")]
    pub limit: u32,
}

fn default_limit_5() -> u32 {
    5
}

/// Parameters for get_source_coverage.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetSourceCoverageParams {
    /// Source ID to query
    pub source_id: String,
}

// ============================================================================
// Tool Implementations - Must-Have
// ============================================================================

/// Get related concepts with optional filtering.
///
/// # Arguments
///
/// * `state` - Application state
/// * `params` - Query parameters
///
/// # Returns
///
/// Returns RelatedConceptsResponse with matching concepts.
///
/// # Errors
///
/// Returns error if graph is not loaded, node not found, or depth exceeds limit.
#[cfg(feature = "graph")]
pub async fn get_related_concepts(
    state: &AppState,
    params: GetRelatedConceptsParams,
) -> Result<RelatedConceptsResponse> {
    // Validate depth
    if params.depth > 3 {
        return Err(crate::error::Error::config(
            "Depth exceeds maximum of 3".to_string(),
        ));
    }

    // Get loaded graph
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

    // Lookup node
    let node_idx = loaded.node_index.get(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.concept_id))
    })?;

    // Parse relationship types filter
    let relationship_filter: Option<HashSet<Relationship>> =
        params.relationship_types.map(|types_str| {
            types_str
                .split(',')
                .filter_map(|s| match s.trim().to_lowercase().as_str() {
                    "prerequisite" => Some(Relationship::Prerequisite),
                    "relates_to" | "relatesto" => Some(Relationship::RelatesTo),
                    "extends" => Some(Relationship::Extends),
                    "introduces" => Some(Relationship::Introduces),
                    "covers" => Some(Relationship::Covers),
                    _ => None,
                })
                .collect()
        });

    // BFS traversal
    let mut queue: VecDeque<(NodeIndex, u32)> = VecDeque::new();
    let mut visited: HashMap<NodeIndex, u32> = HashMap::new();
    let mut related: Vec<RelatedConcept> = Vec::new();

    queue.push_back((*node_idx, 0));
    visited.insert(*node_idx, 0);

    while let Some((current_idx, current_depth)) = queue.pop_front() {
        if current_depth >= params.depth {
            continue;
        }

        // Determine which edges to traverse
        let edges: Vec<_> = match params.direction.as_str() {
            "incoming" => loaded
                .graph
                .edges_directed(current_idx, Direction::Incoming)
                .collect(),
            "outgoing" => loaded
                .graph
                .edges_directed(current_idx, Direction::Outgoing)
                .collect(),
            _ => {
                // "both"
                let mut edges = loaded
                    .graph
                    .edges_directed(current_idx, Direction::Incoming)
                    .collect::<Vec<_>>();
                edges.extend(
                    loaded
                        .graph
                        .edges_directed(current_idx, Direction::Outgoing),
                );
                edges
            }
        };

        for edge in edges {
            let edge_weight = edge.weight();

            // Apply relationship filter
            if let Some(ref filter) = relationship_filter {
                if !filter.contains(&edge_weight.relationship) {
                    continue;
                }
            }

            // Determine neighbor and direction
            let (neighbor_idx, direction) = if edge.source() == current_idx {
                (edge.target(), "outgoing")
            } else {
                (edge.source(), "incoming")
            };

            // Skip if already visited with shorter path
            if let Some(&prev_depth) = visited.get(&neighbor_idx) {
                if prev_depth <= current_depth + 1 {
                    continue;
                }
            }

            visited.insert(neighbor_idx, current_depth + 1);
            queue.push_back((neighbor_idx, current_depth + 1));

            // Skip the starting node itself
            if neighbor_idx == *node_idx {
                continue;
            }

            // Extract node details
            let neighbor_node = &loaded.graph[neighbor_idx];
            if let Node::Concept(concept) = neighbor_node {
                // Find the node ID from the reverse index
                let neighbor_id = loaded
                    .node_index
                    .iter()
                    .find(|(_, &idx)| idx == neighbor_idx)
                    .map(|(id, _)| id.clone())
                    .unwrap_or_default();

                related.push(RelatedConcept {
                    id: neighbor_id,
                    title: concept.title.clone(),
                    category: concept.category.clone(),
                    relationship: format!("{:?}", edge_weight.relationship),
                    direction: direction.to_string(),
                    weight: edge_weight.weight,
                    distance: current_depth + 1,
                });
            }
        }
    }

    Ok(RelatedConceptsResponse {
        concept_id: params.concept_id,
        depth: params.depth,
        total: related.len() as u32,
        related,
    })
}

/// Find shortest path between two concepts.
///
/// # Arguments
///
/// * `state` - Application state
/// * `params` - Query parameters
///
/// # Returns
///
/// Returns ConceptPathResponse with path if found.
///
/// # Errors
///
/// Returns error if graph is not loaded, nodes not found, or max_depth exceeds limit.
#[cfg(feature = "graph")]
pub async fn find_concept_path(
    state: &AppState,
    params: FindConceptPathParams,
) -> Result<ConceptPathResponse> {
    // Validate max_depth
    if params.max_depth > 8 {
        return Err(crate::error::Error::config(
            "Max depth exceeds maximum of 8".to_string(),
        ));
    }

    // Get loaded graph
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

    // Lookup nodes
    let from_idx = loaded.node_index.get(&params.from_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.from_id))
    })?;
    let to_idx = loaded.node_index.get(&params.to_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.to_id))
    })?;

    // Use shortest_path algorithm
    let path_result = crate::graph::algorithms::shortest_path(
        &loaded.graph,
        *from_idx,
        *to_idx,
        params.max_depth,
    );

    match path_result {
        Some(path_indices) => {
            // Build response with full path details
            let mut nodes = Vec::new();
            let mut edges = Vec::new();

            // Build node list
            for (i, &node_idx) in path_indices.iter().enumerate() {
                let node = &loaded.graph[node_idx];
                let node_id = loaded
                    .node_index
                    .iter()
                    .find(|(_, &idx)| idx == node_idx)
                    .map(|(id, _)| id.clone())
                    .unwrap_or_default();

                let (title, category) = match node {
                    Node::Concept(c) => (c.title.clone(), Some(c.category.clone())),
                    Node::Source(s) => (s.title.clone(), None),
                };

                nodes.push(crate::graph::query::PathNode {
                    id: node_id,
                    title,
                    category,
                    step: i as u32,
                });
            }

            // Build edge list
            for i in 0..(path_indices.len() - 1) {
                let from_idx = path_indices[i];
                let to_idx = path_indices[i + 1];

                // Find the edge between these nodes (check both directions)
                if let Some(edge) = loaded
                    .graph
                    .edges_directed(from_idx, Direction::Outgoing)
                    .find(|e| e.target() == to_idx)
                    .or_else(|| {
                        loaded
                            .graph
                            .edges_directed(from_idx, Direction::Incoming)
                            .find(|e| e.source() == to_idx)
                    })
                {
                    let from_id = loaded
                        .node_index
                        .iter()
                        .find(|(_, &idx)| idx == from_idx)
                        .map(|(id, _)| id.clone())
                        .unwrap_or_default();
                    let to_id = loaded
                        .node_index
                        .iter()
                        .find(|(_, &idx)| idx == to_idx)
                        .map(|(id, _)| id.clone())
                        .unwrap_or_default();

                    edges.push(crate::graph::query::PathEdge {
                        from: from_id,
                        to: to_id,
                        relationship: format!("{:?}", edge.weight().relationship),
                        step: i as u32,
                    });
                }
            }

            Ok(ConceptPathResponse {
                from: params.from_id,
                to: params.to_id,
                found: true,
                path_length: (path_indices.len() - 1) as u32,
                nodes,
                edges,
            })
        }
        None => Ok(ConceptPathResponse {
            from: params.from_id,
            to: params.to_id,
            found: false,
            path_length: 0,
            nodes: Vec::new(),
            edges: Vec::new(),
        }),
    }
}

/// Get prerequisites in learning order.
///
/// # Arguments
///
/// * `state` - Application state
/// * `params` - Query parameters
///
/// # Returns
///
/// Returns PrerequisitesResponse with prerequisites sorted topologically.
///
/// # Errors
///
/// Returns error if graph is not loaded, node not found, or depth exceeds limit.
#[cfg(feature = "graph")]
pub async fn get_prerequisites(
    state: &AppState,
    params: GetPrerequisitesParams,
) -> Result<PrerequisitesResponse> {
    // Validate depth
    if params.depth > 5 {
        return Err(crate::error::Error::config(
            "Depth exceeds maximum of 5".to_string(),
        ));
    }

    // Get loaded graph
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

    // Lookup node
    let node_idx = loaded.node_index.get(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.concept_id))
    })?;

    // Get concept title
    let concept_title = match &loaded.graph[*node_idx] {
        Node::Concept(c) => c.title.clone(),
        Node::Source(s) => s.title.clone(),
    };

    // Use prerequisites_sorted algorithm
    let prereq_indices =
        crate::graph::algorithms::prerequisites_sorted(&loaded.graph, *node_idx, params.depth);

    // Build response
    let mut prerequisites = Vec::new();
    let mut learning_order = Vec::new();

    for prereq_idx in prereq_indices {
        let node = &loaded.graph[prereq_idx];
        if let Node::Concept(concept) = node {
            let node_id = loaded
                .node_index
                .iter()
                .find(|(_, &idx)| idx == prereq_idx)
                .map(|(id, _)| id.clone())
                .unwrap_or_default();

            // Calculate depth (distance from target)
            let depth = calculate_prereq_depth(&loaded.graph, *node_idx, prereq_idx);

            learning_order.push(node_id.clone());
            prerequisites.push(crate::graph::query::PrerequisiteConcept {
                id: node_id,
                title: concept.title.clone(),
                category: concept.category.clone(),
                depth,
            });
        }
    }

    Ok(PrerequisitesResponse {
        concept_id: params.concept_id,
        concept_title,
        total: prerequisites.len() as u32,
        prerequisites,
        learning_order,
    })
}

/// Get local neighborhood subgraph.
///
/// # Arguments
///
/// * `state` - Application state
/// * `params` - Query parameters
///
/// # Returns
///
/// Returns NeighborhoodResponse with subgraph.
///
/// # Errors
///
/// Returns error if graph is not loaded, node not found, or parameters exceed limits.
#[cfg(feature = "graph")]
pub async fn get_concept_neighborhood(
    state: &AppState,
    params: GetConceptNeighborhoodParams,
) -> Result<NeighborhoodResponse> {
    // Validate parameters
    if params.radius > 3 {
        return Err(crate::error::Error::config(
            "Radius exceeds maximum of 3".to_string(),
        ));
    }
    if params.max_nodes > 50 {
        return Err(crate::error::Error::config(
            "Max nodes exceeds maximum of 50".to_string(),
        ));
    }

    // Get loaded graph
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

    // Lookup node
    let node_idx = loaded.node_index.get(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.concept_id))
    })?;

    // Use neighborhood algorithm
    let neighborhood_nodes = crate::graph::algorithms::neighborhood(
        &loaded.graph,
        *node_idx,
        params.radius,
        Some(params.max_nodes),
    );

    // Collect nodes
    let mut nodes = Vec::new();
    let mut node_set: HashSet<NodeIndex> = HashSet::new();

    for (neighbor_idx, distance) in neighborhood_nodes {
        node_set.insert(neighbor_idx);
        let node = &loaded.graph[neighbor_idx];
        let node_id = loaded
            .node_index
            .iter()
            .find(|(_, &idx)| idx == neighbor_idx)
            .map(|(id, _)| id.clone())
            .unwrap_or_default();

        let is_center = neighbor_idx == *node_idx;

        match node {
            Node::Concept(c) => {
                nodes.push(crate::graph::query::NeighborhoodNode {
                    id: node_id,
                    title: c.title.clone(),
                    node_type: "concept".to_string(),
                    category: Some(c.category.clone()),
                    distance,
                    is_center,
                });
            }
            Node::Source(s) => {
                nodes.push(crate::graph::query::NeighborhoodNode {
                    id: node_id,
                    title: s.title.clone(),
                    node_type: "source".to_string(),
                    category: None,
                    distance,
                    is_center,
                });
            }
        }
    }

    // Collect edges between nodes in the neighborhood
    let mut edges = Vec::new();
    for &node_idx in &node_set {
        for edge in loaded.graph.edges_directed(node_idx, Direction::Outgoing) {
            if node_set.contains(&edge.target()) {
                let from_id = loaded
                    .node_index
                    .iter()
                    .find(|(_, &idx)| idx == node_idx)
                    .map(|(id, _)| id.clone())
                    .unwrap_or_default();
                let to_id = loaded
                    .node_index
                    .iter()
                    .find(|(_, &idx)| idx == edge.target())
                    .map(|(id, _)| id.clone())
                    .unwrap_or_default();

                edges.push(crate::graph::query::NeighborhoodEdge {
                    from: from_id,
                    to: to_id,
                    relationship: format!("{:?}", edge.weight().relationship),
                });
            }
        }
    }

    Ok(NeighborhoodResponse {
        center: params.concept_id,
        radius: params.radius,
        node_count: nodes.len() as u32,
        edge_count: edges.len() as u32,
        nodes,
        edges,
    })
}

// ============================================================================
// Tool Implementations - Should-Have
// ============================================================================

/// Get concepts that depend on this concept.
///
/// # Arguments
///
/// * `state` - Application state
/// * `params` - Query parameters
///
/// # Returns
///
/// Returns DependentsResponse with dependent concepts.
///
/// # Errors
///
/// Returns error if graph is not loaded, node not found, or depth exceeds limit.
#[cfg(feature = "graph")]
pub async fn get_dependents(
    state: &AppState,
    params: GetDependentsParams,
) -> Result<DependentsResponse> {
    // Validate depth
    if params.depth > 4 {
        return Err(crate::error::Error::config(
            "Depth exceeds maximum of 4".to_string(),
        ));
    }

    // Get loaded graph
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

    // Lookup node
    let node_idx = loaded.node_index.get(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.concept_id))
    })?;

    // Get concept title
    let concept_title = match &loaded.graph[*node_idx] {
        Node::Concept(c) => c.title.clone(),
        Node::Source(s) => s.title.clone(),
    };

    // Use dependents algorithm
    let dependent_indices =
        crate::graph::algorithms::dependents(&loaded.graph, *node_idx, params.depth);

    // Build response
    let mut dependents = Vec::new();

    for (dependent_idx, distance) in dependent_indices {
        let node = &loaded.graph[dependent_idx];
        if let Node::Concept(concept) = node {
            let node_id = loaded
                .node_index
                .iter()
                .find(|(_, &idx)| idx == dependent_idx)
                .map(|(id, _)| id.clone())
                .unwrap_or_default();

            dependents.push(crate::graph::query::DependentConcept {
                id: node_id,
                title: concept.title.clone(),
                category: concept.category.clone(),
                depth: distance,
            });
        }
    }

    Ok(DependentsResponse {
        concept_id: params.concept_id,
        concept_title,
        total: dependents.len() as u32,
        dependents,
    })
}

/// Get most connected concepts by degree centrality.
///
/// # Arguments
///
/// * `state` - Application state
/// * `params` - Query parameters
///
/// # Returns
///
/// Returns CentralConceptsResponse with most connected concepts.
///
/// # Errors
///
/// Returns error if graph is not loaded or limit exceeds maximum.
#[cfg(feature = "graph")]
pub async fn get_central_concepts(
    state: &AppState,
    params: GetCentralConceptsParams,
) -> Result<CentralConceptsResponse> {
    // Validate limit
    if params.limit > 25 {
        return Err(crate::error::Error::config(
            "Limit exceeds maximum of 25".to_string(),
        ));
    }

    // Get loaded graph
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

    // Use degree_centrality algorithm
    let central_indices =
        crate::graph::algorithms::degree_centrality(&loaded.graph, params.category.as_deref());

    // Build response (limit results)
    let mut concepts = Vec::new();
    for (node_idx, degree) in central_indices.iter().take(params.limit as usize) {
        let node = &loaded.graph[*node_idx];
        if let Node::Concept(concept) = node {
            let node_id = loaded
                .node_index
                .iter()
                .find(|(_, &idx)| idx == *node_idx)
                .map(|(id, _)| id.clone())
                .unwrap_or_default();

            concepts.push(crate::graph::query::CentralConcept {
                id: node_id,
                title: concept.title.clone(),
                category: concept.category.clone(),
                connections: *degree,
            });
        }
    }

    Ok(CentralConceptsResponse {
        category: params.category,
        total: concepts.len() as u32,
        concepts,
    })
}

/// Get sources that cover a concept.
///
/// # Arguments
///
/// * `state` - Application state
/// * `params` - Query parameters
///
/// # Returns
///
/// Returns ConceptSourcesResponse with source information.
///
/// # Errors
///
/// Returns error if graph is not loaded or node not found.
#[cfg(feature = "graph")]
pub async fn get_concept_sources(
    state: &AppState,
    params: GetConceptSourcesParams,
) -> Result<ConceptSourcesResponse> {
    // Get loaded graph
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

    // Lookup node
    let node_idx = loaded.node_index.get(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.concept_id))
    })?;

    // Get concept title
    let concept_title = match &loaded.graph[*node_idx] {
        Node::Concept(c) => c.title.clone(),
        Node::Source(_) => {
            return Err(crate::error::Error::config(
                "Cannot get sources for a source node".to_string(),
            ))
        }
    };

    // Find all incoming edges from Source nodes
    let mut sources = Vec::new();
    for edge in loaded.graph.edges_directed(*node_idx, Direction::Incoming) {
        let source_idx = edge.source();
        let source_node = &loaded.graph[source_idx];

        if let Node::Source(source) = source_node {
            // Check if it's Introduces or Covers relationship
            let relationship = &edge.weight().relationship;
            if matches!(
                relationship,
                Relationship::Introduces | Relationship::Covers
            ) {
                let source_id = loaded
                    .node_index
                    .iter()
                    .find(|(_, &idx)| idx == source_idx)
                    .map(|(id, _)| id.clone())
                    .unwrap_or_default();

                sources.push(crate::graph::query::SourceCoverage {
                    source_id,
                    source_title: source.title.clone(),
                    source_author: source.author.clone(),
                    relationship: format!("{:?}", relationship),
                });
            }
        }
    }

    Ok(ConceptSourcesResponse {
        concept_id: params.concept_id,
        concept_title,
        total: sources.len() as u32,
        sources,
    })
}

/// Get variants of a canonical concept across different sources.
///
/// # Arguments
///
/// * `state` - Application state
/// * `params` - Query parameters
///
/// # Returns
///
/// Returns ConceptVariantsResponse with all source-specific variants.
///
/// # Errors
///
/// Returns error if graph is not loaded or canonical concept not found.
#[cfg(feature = "graph")]
pub async fn get_concept_variants(
    state: &AppState,
    params: GetConceptVariantsParams,
) -> Result<ConceptVariantsResponse> {
    // Get loaded graph
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

    // Verify canonical concept exists
    let canonical_idx = loaded.node_index.get(&params.canonical_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!(
            "Canonical concept not found: {}",
            params.canonical_id
        ))
    })?;

    let canonical_title = match &loaded.graph[*canonical_idx] {
        Node::Concept(c) => {
            if !c.is_canonical {
                return Err(crate::error::Error::config(format!(
                    "Concept {} is not marked as canonical",
                    params.canonical_id
                )));
            }
            c.title.clone()
        }
        Node::Source(_) => {
            return Err(crate::error::Error::config(
                "Cannot get variants for a source node".to_string(),
            ))
        }
    };

    // Find all concepts with matching canonical_id
    let mut variants = Vec::new();
    for node_idx in loaded.graph.node_indices() {
        let node = &loaded.graph[node_idx];
        if let Node::Concept(concept) = node {
            if let Some(ref canon_id) = concept.canonical_id {
                if canon_id == &params.canonical_id {
                    let node_id = loaded
                        .node_index
                        .iter()
                        .find(|(_, &idx)| idx == node_idx)
                        .map(|(id, _)| id.clone())
                        .unwrap_or_default();

                    // Get source title
                    let source_title =
                        if let Some(source_idx) = loaded.node_index.get(&concept.source_id) {
                            match &loaded.graph[*source_idx] {
                                Node::Source(s) => s.title.clone(),
                                _ => concept.source_id.clone(), // Fallback to ID
                            }
                        } else {
                            concept.source_id.clone() // Fallback to ID
                        };

                    variants.push(crate::graph::query::ConceptVariant {
                        id: node_id,
                        title: concept.title.clone(),
                        source_id: concept.source_id.clone(),
                        source_title,
                    });
                }
            }
        }
    }

    Ok(ConceptVariantsResponse {
        canonical_id: params.canonical_id,
        canonical_title,
        total: variants.len() as u32,
        variants,
    })
}

// ============================================================================
// Tool Implementations - Nice-to-Have
// ============================================================================

/// Find concepts that bridge two categories.
///
/// # Arguments
///
/// * `state` - Application state
/// * `params` - Query parameters
///
/// # Returns
///
/// Returns BridgeConceptsResponse with bridge concepts sorted by score.
///
/// # Errors
///
/// Returns error if graph is not loaded or limit exceeds maximum.
#[cfg(feature = "graph")]
pub async fn find_bridge_concepts(
    state: &AppState,
    params: FindBridgeConceptsParams,
) -> Result<BridgeConceptsResponse> {
    // Validate limit
    if params.limit > 15 {
        return Err(crate::error::Error::config(
            "Limit exceeds maximum of 15".to_string(),
        ));
    }

    // Get loaded graph
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

    // Use bridge_concepts algorithm
    let bridge_indices = crate::graph::algorithms::bridge_concepts(
        &loaded.graph,
        &params.category_a,
        &params.category_b,
    );

    // Build response (limit results)
    let mut bridges = Vec::new();
    for (node_idx, connections_a, connections_b, score) in
        bridge_indices.iter().take(params.limit as usize)
    {
        let node = &loaded.graph[*node_idx];
        if let Node::Concept(concept) = node {
            let node_id = loaded
                .node_index
                .iter()
                .find(|(_, &idx)| idx == *node_idx)
                .map(|(id, _)| id.clone())
                .unwrap_or_default();

            bridges.push(crate::graph::query::BridgeConcept {
                id: node_id,
                title: concept.title.clone(),
                category: concept.category.clone(),
                connections_to_a: *connections_a,
                connections_to_b: *connections_b,
                bridge_score: *score,
            });
        }
    }

    Ok(BridgeConceptsResponse {
        category_a: params.category_a,
        category_b: params.category_b,
        total: bridges.len() as u32,
        bridges,
    })
}

/// Get coverage information for a source.
///
/// # Arguments
///
/// * `state` - Application state
/// * `params` - Query parameters
///
/// # Returns
///
/// Returns SourceCoverageResponse with concepts covered by this source.
///
/// # Errors
///
/// Returns error if graph is not loaded or source not found.
#[cfg(feature = "graph")]
pub async fn get_source_coverage(
    state: &AppState,
    params: GetSourceCoverageParams,
) -> Result<SourceCoverageResponse> {
    // Get loaded graph
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

    // Lookup node
    let node_idx = loaded.node_index.get(&params.source_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.source_id))
    })?;

    // Verify it's a source node
    let (source_title, source_author) = match &loaded.graph[*node_idx] {
        Node::Source(s) => (s.title.clone(), s.author.clone()),
        Node::Concept(_) => {
            return Err(crate::error::Error::config(
                "Cannot get coverage for a concept node".to_string(),
            ))
        }
    };

    // Find all outgoing edges to concepts
    let mut introduces = Vec::new();
    let mut covers = Vec::new();

    for edge in loaded.graph.edges_directed(*node_idx, Direction::Outgoing) {
        let target_idx = edge.target();
        let target_node = &loaded.graph[target_idx];

        if let Node::Concept(concept) = target_node {
            let concept_id = loaded
                .node_index
                .iter()
                .find(|(_, &idx)| idx == target_idx)
                .map(|(id, _)| id.clone())
                .unwrap_or_default();

            let concept_brief = crate::graph::query::ConceptBrief {
                id: concept_id,
                title: concept.title.clone(),
                category: concept.category.clone(),
            };

            match edge.weight().relationship {
                Relationship::Introduces => introduces.push(concept_brief),
                Relationship::Covers => covers.push(concept_brief),
                _ => {} // Ignore other relationship types
            }
        }
    }

    let total_concepts = (introduces.len() + covers.len()) as u32;

    Ok(SourceCoverageResponse {
        source_id: params.source_id,
        source_title,
        source_author,
        total_concepts,
        introduces_count: introduces.len() as u32,
        covers_count: covers.len() as u32,
        introduces,
        covers,
    })
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Calculate prerequisite depth using BFS.
#[cfg(feature = "graph")]
fn calculate_prereq_depth(
    graph: &crate::graph::ConceptGraph,
    target: NodeIndex,
    prereq: NodeIndex,
) -> u32 {
    let mut queue: VecDeque<(NodeIndex, u32)> = VecDeque::new();
    let mut visited: HashSet<NodeIndex> = HashSet::new();

    queue.push_back((prereq, 0));
    visited.insert(prereq);

    while let Some((current, depth)) = queue.pop_front() {
        if current == target {
            return depth;
        }

        // Follow outgoing Prerequisite edges
        for edge in graph.edges_directed(current, Direction::Outgoing) {
            if matches!(edge.weight().relationship, Relationship::Prerequisite) {
                let neighbor = edge.target();
                if visited.insert(neighbor) {
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }
    }

    0 // Shouldn't reach here if prereq is actually a prerequisite
}

#[cfg(not(feature = "graph"))]
pub async fn get_related_concepts(
    _state: &AppState,
    _params: GetRelatedConceptsParams,
) -> Result<String> {
    Err(crate::error::Error::config(
        "Graph feature not enabled. Rebuild with --features graph".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn find_concept_path(
    _state: &AppState,
    _params: FindConceptPathParams,
) -> Result<String> {
    Err(crate::error::Error::config(
        "Graph feature not enabled. Rebuild with --features graph".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn get_prerequisites(
    _state: &AppState,
    _params: GetPrerequisitesParams,
) -> Result<String> {
    Err(crate::error::Error::config(
        "Graph feature not enabled. Rebuild with --features graph".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn get_concept_neighborhood(
    _state: &AppState,
    _params: GetConceptNeighborhoodParams,
) -> Result<String> {
    Err(crate::error::Error::config(
        "Graph feature not enabled. Rebuild with --features graph".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn get_dependents(_state: &AppState, _params: GetDependentsParams) -> Result<String> {
    Err(crate::error::Error::config(
        "Graph feature not enabled. Rebuild with --features graph".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn get_central_concepts(
    _state: &AppState,
    _params: GetCentralConceptsParams,
) -> Result<String> {
    Err(crate::error::Error::config(
        "Graph feature not enabled. Rebuild with --features graph".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn get_concept_sources(
    _state: &AppState,
    _params: GetConceptSourcesParams,
) -> Result<String> {
    Err(crate::error::Error::config(
        "Graph feature not enabled. Rebuild with --features graph".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn get_concept_variants(
    _state: &AppState,
    _params: GetConceptVariantsParams,
) -> Result<String> {
    Err(crate::error::Error::config(
        "Graph feature not enabled. Rebuild with --features graph".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn find_bridge_concepts(
    _state: &AppState,
    _params: FindBridgeConceptsParams,
) -> Result<String> {
    Err(crate::error::Error::config(
        "Graph feature not enabled. Rebuild with --features graph".to_string(),
    ))
}

#[cfg(not(feature = "graph"))]
pub async fn get_source_coverage(
    _state: &AppState,
    _params: GetSourceCoverageParams,
) -> Result<String> {
    Err(crate::error::Error::config(
        "Graph feature not enabled. Rebuild with --features graph".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_related_concepts_params_default() {
        let json = r#"{"concept_id": "test"}"#;
        let params: GetRelatedConceptsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.direction, "both");
        assert_eq!(params.depth, 1);
    }

    #[test]
    fn test_find_concept_path_params_default() {
        let json = r#"{"from_id": "a", "to_id": "b"}"#;
        let params: FindConceptPathParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.max_depth, 5);
    }

    #[test]
    fn test_get_prerequisites_params_default() {
        let json = r#"{"concept_id": "test"}"#;
        let params: GetPrerequisitesParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.depth, 3);
    }

    #[test]
    fn test_get_concept_neighborhood_params_default() {
        let json = r#"{"concept_id": "test"}"#;
        let params: GetConceptNeighborhoodParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.radius, 2);
        assert_eq!(params.max_nodes, 30);
    }

    #[test]
    fn test_get_dependents_params_default() {
        let json = r#"{"concept_id": "test"}"#;
        let params: GetDependentsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.depth, 2);
    }

    #[test]
    fn test_get_central_concepts_params_default() {
        let json = r#"{}"#;
        let params: GetCentralConceptsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.limit, 10);
        assert!(params.category.is_none());
    }

    #[test]
    fn test_get_concept_sources_params() {
        let json = r#"{"concept_id": "test"}"#;
        let params: GetConceptSourcesParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.concept_id, "test");
    }

    #[test]
    fn test_get_concept_variants_params() {
        let json = r#"{"canonical_id": "test"}"#;
        let params: GetConceptVariantsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.canonical_id, "test");
    }

    #[test]
    fn test_find_bridge_concepts_params_default() {
        let json = r#"{"category_a": "harmony", "category_b": "counterpoint"}"#;
        let params: FindBridgeConceptsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.limit, 5);
        assert_eq!(params.category_a, "harmony");
        assert_eq!(params.category_b, "counterpoint");
    }

    #[test]
    fn test_get_source_coverage_params() {
        let json = r#"{"source_id": "test-source"}"#;
        let params: GetSourceCoverageParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.source_id, "test-source");
    }
}
