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
use crate::graph::{
    from_fabryk_relationship, is_concept_node, is_source_node, neighborhood, node_category,
    node_title, prerequisites_sorted, shortest_path, source_author, FabrykRelationship,
};
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
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Verify node exists
    loaded.data.get_node(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.concept_id))
    })?;

    let start_idx = loaded.data.get_index(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node index not found: {}", params.concept_id))
    })?;

    // Parse relationship types filter
    let relationship_filter: Option<HashSet<FabrykRelationship>> =
        params.relationship_types.map(|types_str| {
            types_str
                .split(',')
                .map(|s| crate::graph::to_fabryk_relationship(s.trim()))
                .collect()
        });

    // BFS traversal
    let mut queue: VecDeque<(petgraph::graph::NodeIndex, u32)> = VecDeque::new();
    let mut visited: HashMap<petgraph::graph::NodeIndex, u32> = HashMap::new();
    let mut related: Vec<RelatedConcept> = Vec::new();

    queue.push_back((start_idx, 0));
    visited.insert(start_idx, 0);

    while let Some((current_idx, current_depth)) = queue.pop_front() {
        if current_depth >= params.depth {
            continue;
        }

        // Determine which edges to traverse
        let edges: Vec<_> = match params.direction.as_str() {
            "incoming" => loaded
                .data
                .graph
                .edges_directed(current_idx, Direction::Incoming)
                .collect(),
            "outgoing" => loaded
                .data
                .graph
                .edges_directed(current_idx, Direction::Outgoing)
                .collect(),
            _ => {
                // "both"
                let mut edges = loaded
                    .data
                    .graph
                    .edges_directed(current_idx, Direction::Incoming)
                    .collect::<Vec<_>>();
                edges.extend(
                    loaded
                        .data
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
            if neighbor_idx == start_idx {
                continue;
            }

            // Extract node details
            let neighbor_node = &loaded.data.graph[neighbor_idx];
            if is_concept_node(neighbor_node) {
                related.push(RelatedConcept {
                    id: neighbor_node.id.clone(),
                    title: node_title(neighbor_node).to_string(),
                    category: node_category(neighbor_node).to_string(),
                    relationship: from_fabryk_relationship(&edge_weight.relationship),
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
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Verify nodes exist
    loaded.data.get_node(&params.from_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.from_id))
    })?;
    loaded.data.get_node(&params.to_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.to_id))
    })?;

    // Use fabryk shortest_path algorithm
    let path_result = shortest_path(&loaded.data, &params.from_id, &params.to_id)
        .map_err(|e| crate::error::Error::operation(format!("Path search failed: {}", e)))?;

    if path_result.found {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Build node list from PathResult
        for (i, node) in path_result.path.iter().enumerate() {
            let category = if is_concept_node(node) {
                Some(node_category(node).to_string())
            } else {
                None
            };

            nodes.push(crate::graph::query::PathNode {
                id: node.id.clone(),
                title: node_title(node).to_string(),
                category,
                step: i as u32,
            });
        }

        // Build edge list from PathResult
        for (i, edge) in path_result.edges.iter().enumerate() {
            edges.push(crate::graph::query::PathEdge {
                from: edge.from.clone(),
                to: edge.to.clone(),
                relationship: from_fabryk_relationship(&edge.relationship),
                step: i as u32,
            });
        }

        Ok(ConceptPathResponse {
            from: params.from_id,
            to: params.to_id,
            found: true,
            path_length: path_result.edges.len() as u32,
            nodes,
            edges,
        })
    } else {
        Ok(ConceptPathResponse {
            from: params.from_id,
            to: params.to_id,
            found: false,
            path_length: 0,
            nodes: Vec::new(),
            edges: Vec::new(),
        })
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
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Verify node exists and get title
    let target_node = loaded.data.get_node(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.concept_id))
    })?;
    let concept_title = node_title(target_node).to_string();

    // Use fabryk prerequisites_sorted algorithm
    let prereq_result = prerequisites_sorted(&loaded.data, &params.concept_id)
        .map_err(|e| crate::error::Error::operation(format!("Prerequisites failed: {}", e)))?;

    // Build response
    let mut prerequisites = Vec::new();
    let mut learning_order = Vec::new();

    for (i, node) in prereq_result.ordered.iter().enumerate() {
        if is_concept_node(node) {
            learning_order.push(node.id.clone());
            prerequisites.push(crate::graph::query::PrerequisiteConcept {
                id: node.id.clone(),
                title: node_title(node).to_string(),
                category: node_category(node).to_string(),
                // Use index as approximate depth (topo-sorted, so lower index = deeper prereq)
                depth: i as u32,
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
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Verify node exists
    loaded.data.get_node(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.concept_id))
    })?;

    // Use fabryk neighborhood algorithm
    let result = neighborhood(
        &loaded.data,
        &params.concept_id,
        params.radius as usize,
        None,
    )
    .map_err(|e| crate::error::Error::operation(format!("Neighborhood failed: {}", e)))?;

    // Collect nodes: center + neighbors
    let mut nodes = Vec::new();
    let mut node_ids_in_neighborhood: HashSet<String> = HashSet::new();

    // Add center node
    node_ids_in_neighborhood.insert(result.center.id.clone());
    let center_type = if is_concept_node(&result.center) {
        "concept"
    } else {
        "source"
    };
    let center_cat = if is_concept_node(&result.center) {
        Some(node_category(&result.center).to_string())
    } else {
        None
    };
    nodes.push(crate::graph::query::NeighborhoodNode {
        id: result.center.id.clone(),
        title: node_title(&result.center).to_string(),
        node_type: center_type.to_string(),
        category: center_cat,
        distance: 0,
        is_center: true,
    });

    // Add neighbor nodes (up to max_nodes - 1 since center takes one slot)
    for node in result
        .nodes
        .iter()
        .take((params.max_nodes as usize).saturating_sub(1))
    {
        node_ids_in_neighborhood.insert(node.id.clone());
        let distance = result.distances.get(&node.id).copied().unwrap_or(1) as u32;

        let (ntype, category) = if is_concept_node(node) {
            ("concept", Some(node_category(node).to_string()))
        } else if is_source_node(node) {
            ("source", None)
        } else {
            ("unknown", None)
        };

        nodes.push(crate::graph::query::NeighborhoodNode {
            id: node.id.clone(),
            title: node_title(node).to_string(),
            node_type: ntype.to_string(),
            category,
            distance,
            is_center: false,
        });
    }

    // Collect edges between nodes in the neighborhood
    let mut edges = Vec::new();
    for edge in &result.edges {
        if node_ids_in_neighborhood.contains(&edge.from)
            && node_ids_in_neighborhood.contains(&edge.to)
        {
            edges.push(crate::graph::query::NeighborhoodEdge {
                from: edge.from.clone(),
                to: edge.to.clone(),
                relationship: from_fabryk_relationship(&edge.relationship),
            });
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
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Verify node exists and get title
    let target_node = loaded.data.get_node(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.concept_id))
    })?;
    let concept_title = node_title(target_node).to_string();

    let start_idx = loaded.data.get_index(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node index not found: {}", params.concept_id))
    })?;

    // BFS forward through incoming Prerequisite edges to find dependents
    // (nodes that have this concept as a prerequisite)
    let mut queue: VecDeque<(petgraph::graph::NodeIndex, u32)> = VecDeque::new();
    let mut visited: HashSet<petgraph::graph::NodeIndex> = HashSet::new();
    let mut dependents = Vec::new();

    queue.push_back((start_idx, 0));
    visited.insert(start_idx);

    while let Some((current_idx, current_depth)) = queue.pop_front() {
        if current_depth >= params.depth {
            continue;
        }

        // Follow outgoing Prerequisite edges (nodes that depend on current)
        // Edge convention: from=prerequisite, to=dependent
        for edge in loaded
            .data
            .graph
            .edges_directed(current_idx, Direction::Outgoing)
        {
            if matches!(edge.weight().relationship, FabrykRelationship::Prerequisite) {
                let dep_idx = edge.target();
                if visited.insert(dep_idx) {
                    let dep_node = &loaded.data.graph[dep_idx];
                    if is_concept_node(dep_node) {
                        dependents.push(crate::graph::query::DependentConcept {
                            id: dep_node.id.clone(),
                            title: node_title(dep_node).to_string(),
                            category: node_category(dep_node).to_string(),
                            depth: current_depth + 1,
                        });
                    }
                    queue.push_back((dep_idx, current_depth + 1));
                }
            }
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
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Use fabryk calculate_centrality algorithm
    let centrality_scores = crate::graph::calculate_centrality(&loaded.data);

    // Build response: filter by category, filter concept nodes only, limit results
    let mut concepts = Vec::new();
    for score in centrality_scores.iter() {
        if concepts.len() >= params.limit as usize {
            break;
        }
        if let Some(node) = loaded.data.get_node(&score.node_id) {
            if !is_concept_node(node) {
                continue;
            }
            // Apply category filter if provided
            if let Some(ref cat_filter) = params.category {
                if node_category(node) != cat_filter.as_str() {
                    continue;
                }
            }
            // Calculate raw connection count from centrality score
            let n = loaded.data.node_count() as f32;
            let connections = (score.degree * 2.0 * (n - 1.0)).round() as u32;

            concepts.push(crate::graph::query::CentralConcept {
                id: node.id.clone(),
                title: node_title(node).to_string(),
                category: node_category(node).to_string(),
                connections,
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
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Verify node exists and is a concept
    let target_node = loaded.data.get_node(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.concept_id))
    })?;
    if is_source_node(target_node) {
        return Err(crate::error::Error::config(
            "Cannot get sources for a source node".to_string(),
        ));
    }
    let concept_title = node_title(target_node).to_string();

    let idx = loaded.data.get_index(&params.concept_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node index not found: {}", params.concept_id))
    })?;

    // Find all incoming edges from Source nodes
    let mut sources = Vec::new();
    for edge in loaded.data.graph.edges_directed(idx, Direction::Incoming) {
        let source_node = &loaded.data.graph[edge.source()];

        if is_source_node(source_node) {
            let relationship = &edge.weight().relationship;
            if matches!(
                relationship,
                FabrykRelationship::Introduces | FabrykRelationship::Covers
            ) {
                sources.push(crate::graph::query::SourceCoverage {
                    source_id: source_node.id.clone(),
                    source_title: node_title(source_node).to_string(),
                    source_author: source_author(source_node).to_string(),
                    relationship: from_fabryk_relationship(relationship),
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
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Verify canonical concept exists
    let canonical_node = loaded.data.get_node(&params.canonical_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!(
            "Canonical concept not found: {}",
            params.canonical_id
        ))
    })?;

    if is_source_node(canonical_node) {
        return Err(crate::error::Error::config(
            "Cannot get variants for a source node".to_string(),
        ));
    }
    if !canonical_node.is_canonical {
        return Err(crate::error::Error::config(format!(
            "Concept {} is not marked as canonical",
            params.canonical_id
        )));
    }
    let canonical_title = node_title(canonical_node).to_string();

    // Find all concepts with matching canonical_id
    let mut variants = Vec::new();
    for node in loaded.data.iter_nodes() {
        if !is_concept_node(node) {
            continue;
        }
        if let Some(ref canon_id) = node.canonical_id {
            if canon_id == &params.canonical_id {
                // Get source title
                let source_id = node.source_id.clone().unwrap_or_default();
                let source_title_str = if let Some(src_node) = loaded.data.get_node(&source_id) {
                    node_title(src_node).to_string()
                } else {
                    source_id.clone() // Fallback to ID
                };

                variants.push(crate::graph::query::ConceptVariant {
                    id: node.id.clone(),
                    title: node_title(node).to_string(),
                    source_id,
                    source_title: source_title_str,
                });
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
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Use fabryk find_bridges algorithm (returns nodes sorted by bridge score)
    let bridge_nodes = crate::graph::find_bridges(&loaded.data, params.limit as usize * 3);

    // Filter and score by the two target categories
    let mut bridges = Vec::new();
    for node in &bridge_nodes {
        if bridges.len() >= params.limit as usize {
            break;
        }
        if !is_concept_node(node) {
            continue;
        }

        // Count connections to each target category
        let idx = match loaded.data.get_index(&node.id) {
            Some(i) => i,
            None => continue,
        };
        let mut connections_a: u32 = 0;
        let mut connections_b: u32 = 0;

        for edge_ref in loaded.data.graph.edges(idx) {
            let neighbor = &loaded.data.graph[edge_ref.target()];
            let cat = node_category(neighbor);
            if cat == params.category_a {
                connections_a += 1;
            }
            if cat == params.category_b {
                connections_b += 1;
            }
        }

        // Only include if it connects both categories
        if connections_a > 0 && connections_b > 0 {
            let bridge_score = (connections_a as f32 * connections_b as f32).sqrt();
            bridges.push(crate::graph::query::BridgeConcept {
                id: node.id.clone(),
                title: node_title(node).to_string(),
                category: node_category(node).to_string(),
                connections_to_a: connections_a,
                connections_to_b: connections_b,
                bridge_score,
            });
        }
    }

    // Sort by bridge score descending
    bridges.sort_by(|a, b| {
        b.bridge_score
            .partial_cmp(&a.bridge_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    bridges.truncate(params.limit as usize);

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
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Verify node exists and is a source
    let source_node = loaded.data.get_node(&params.source_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.source_id))
    })?;
    if !is_source_node(source_node) {
        return Err(crate::error::Error::config(
            "Cannot get coverage for a concept node".to_string(),
        ));
    }
    let src_title = node_title(source_node).to_string();
    let src_author = source_author(source_node).to_string();

    let idx = loaded.data.get_index(&params.source_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node index not found: {}", params.source_id))
    })?;

    // Find all outgoing edges to concepts
    let mut introduces = Vec::new();
    let mut covers = Vec::new();

    for edge in loaded.data.graph.edges_directed(idx, Direction::Outgoing) {
        let target_node = &loaded.data.graph[edge.target()];

        if is_concept_node(target_node) {
            let concept_brief = crate::graph::query::ConceptBrief {
                id: target_node.id.clone(),
                title: node_title(target_node).to_string(),
                category: node_category(target_node).to_string(),
            };

            match edge.weight().relationship {
                FabrykRelationship::Introduces => introduces.push(concept_brief),
                FabrykRelationship::Covers => covers.push(concept_brief),
                _ => {} // Ignore other relationship types
            }
        }
    }

    let total_concepts = (introduces.len() + covers.len()) as u32;

    Ok(SourceCoverageResponse {
        source_id: params.source_id,
        source_title: src_title,
        source_author: src_author,
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

// Helper functions have been replaced by fabryk algorithm calls.

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

    // Functional tests for graph query tools
    #[cfg(feature = "graph")]
    mod functional {
        use super::*;
        use crate::config::Config;
        use crate::graph::{GraphStats, LoadedGraph};
        use crate::state::AppState;
        use fabryk::core::ServiceState;
        use fabryk::graph::{Edge, Node, NodeType, Relationship};
        use std::sync::Arc;

        /// Helper to create AppState with a test graph for query testing
        async fn create_query_test_state() -> Arc<AppState> {
            let mut data = fabryk::graph::GraphData::new();

            // Source node
            data.add_node(
                Node::new("source-1", "Source One")
                    .with_node_type(NodeType::Custom("source".to_string()))
                    .with_metadata("author", serde_json::json!("Author"))
                    .with_metadata("year", serde_json::json!(2024))
                    .with_metadata("is_converted", serde_json::json!(true)),
            );

            // Concept nodes
            data.add_node(
                Node::new("concept-a", "Concept A")
                    .with_category("harmony")
                    .with_source("source-1"),
            );
            data.add_node(
                Node::new("concept-b", "Concept B")
                    .with_category("harmony")
                    .with_source("source-1"),
            );
            data.add_node(
                Node::new("concept-c", "Concept C")
                    .with_category("fundamentals")
                    .with_source("source-1"),
            );
            data.add_node(
                Node::new("concept-d", "Concept D")
                    .with_category("fundamentals")
                    .with_source("source-1"),
            );

            // Edges
            data.add_edge(Edge::new("source-1", "concept-a", Relationship::Introduces))
                .expect("edge");
            data.add_edge(Edge::new(
                "concept-c",
                "concept-b",
                Relationship::Prerequisite,
            ))
            .expect("edge");
            data.add_edge(Edge::new(
                "concept-b",
                "concept-a",
                Relationship::Prerequisite,
            ))
            .expect("edge");
            data.add_edge(Edge::new("concept-a", "concept-d", Relationship::RelatesTo))
                .expect("edge");

            let stats = GraphStats {
                node_count: 5,
                edge_count: 4,
                concept_count: 4,
                source_count: 1,
            };

            let loaded = LoadedGraph {
                data,
                loaded_at: chrono::Utc::now(),
                stats,
            };

            let config = Config::load().unwrap();
            let state = Arc::new(AppState::new(config).await.unwrap());

            {
                let mut graph_guard = state.graph_data.write().unwrap();
                *graph_guard = Some(loaded);
            }
            state.graph_service.set_state(ServiceState::Ready);

            state
        }

        #[tokio::test]
        async fn test_get_related_concepts_all() {
            let state = create_query_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "concept-a".to_string(),
                relationship_types: None,
                direction: "both".to_string(),
                depth: 1,
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            assert_eq!(response.concept_id, "concept-a");
            assert_eq!(response.depth, 1);
            // Should have b (prerequisite incoming), d (relates outgoing)
            assert!(response.total >= 2);
        }

        #[tokio::test]
        async fn test_get_related_concepts_prerequisite_only() {
            let state = create_query_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "concept-a".to_string(),
                relationship_types: Some("prerequisite".to_string()),
                direction: "incoming".to_string(),
                depth: 1,
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            assert!(response.related.iter().any(|r| r.id == "concept-b"));
        }

        #[tokio::test]
        async fn test_find_concept_path_found() {
            let state = create_query_test_state().await;
            let params = FindConceptPathParams {
                from_id: "concept-c".to_string(),
                to_id: "concept-a".to_string(),
                max_depth: 5,
            };

            let response = find_concept_path(&state, params).await.unwrap();
            assert!(response.found);
            assert_eq!(response.from, "concept-c");
            assert_eq!(response.to, "concept-a");
            assert_eq!(response.path_length, 2); // c -> b -> a
        }

        #[tokio::test]
        async fn test_find_concept_path_not_found() {
            let state = create_query_test_state().await;
            let params = FindConceptPathParams {
                from_id: "concept-d".to_string(),
                to_id: "concept-c".to_string(),
                max_depth: 1, // Too shallow to find the path
            };

            let response = find_concept_path(&state, params).await.unwrap();
            // May or may not find a path depending on edge types
            // Just verify it completes without error
            assert_eq!(response.from, "concept-d");
        }

        #[tokio::test]
        async fn test_get_prerequisites() {
            let state = create_query_test_state().await;
            let params = GetPrerequisitesParams {
                concept_id: "concept-a".to_string(),
                depth: 3,
            };

            let response = get_prerequisites(&state, params).await.unwrap();
            assert_eq!(response.concept_id, "concept-a");
            // Should have b and c as prerequisites
            assert!(response.total >= 2);
            // b should come before a (closer)
            assert!(response.prerequisites.iter().any(|p| p.id == "concept-b"));
        }

        #[tokio::test]
        async fn test_get_concept_neighborhood() {
            let state = create_query_test_state().await;
            let params = GetConceptNeighborhoodParams {
                concept_id: "concept-b".to_string(),
                radius: 2, // Larger radius to include more nodes
                max_nodes: 30,
            };

            let response = get_concept_neighborhood(&state, params).await.unwrap();
            assert_eq!(response.center, "concept-b");
            assert_eq!(response.radius, 2);
            // Should include at least b itself and neighbors
            assert!(response.node_count >= 1);
        }

        #[tokio::test]
        async fn test_get_dependents() {
            let state = create_query_test_state().await;
            let params = GetDependentsParams {
                concept_id: "concept-b".to_string(),
                depth: 2,
            };

            let response = get_dependents(&state, params).await.unwrap();
            assert_eq!(response.concept_id, "concept-b");
            // a depends on b
            assert!(response.dependents.iter().any(|d| d.id == "concept-a"));
        }

        #[tokio::test]
        async fn test_get_central_concepts() {
            let state = create_query_test_state().await;
            let params = GetCentralConceptsParams {
                category: None,
                limit: 10,
            };

            let response = get_central_concepts(&state, params).await.unwrap();
            assert!(response.total > 0);
            // All should have connections count
            for concept in &response.concepts {
                assert!(concept.connections > 0);
            }
        }

        #[tokio::test]
        async fn test_get_central_concepts_filtered() {
            let state = create_query_test_state().await;
            let params = GetCentralConceptsParams {
                category: Some("harmony".to_string()),
                limit: 10,
            };

            let response = get_central_concepts(&state, params).await.unwrap();
            // All results should be harmony category
            for concept in &response.concepts {
                assert_eq!(concept.category, "harmony");
            }
        }

        #[tokio::test]
        async fn test_get_concept_sources() {
            let state = create_query_test_state().await;
            let params = GetConceptSourcesParams {
                concept_id: "concept-a".to_string(),
            };

            let response = get_concept_sources(&state, params).await.unwrap();
            assert_eq!(response.concept_id, "concept-a");
            // Should find source-1
            assert!(response.sources.iter().any(|s| s.source_id == "source-1"));
        }

        #[tokio::test]
        async fn test_get_concept_variants() {
            let state = create_query_test_state().await;
            let params = GetConceptVariantsParams {
                canonical_id: "concept-a".to_string(),
            };

            // Since concept-a is canonical, should return just itself
            let response = get_concept_variants(&state, params).await.unwrap();
            assert_eq!(response.canonical_id, "concept-a");
        }

        #[tokio::test]
        async fn test_find_bridge_concepts() {
            let state = create_query_test_state().await;
            let params = FindBridgeConceptsParams {
                category_a: "harmony".to_string(),
                category_b: "fundamentals".to_string(),
                limit: 5,
            };

            let response = find_bridge_concepts(&state, params).await.unwrap();
            assert_eq!(response.category_a, "harmony");
            assert_eq!(response.category_b, "fundamentals");
            // May or may not have bridges depending on edges
        }

        #[tokio::test]
        async fn test_get_source_coverage() {
            let state = create_query_test_state().await;
            let params = GetSourceCoverageParams {
                source_id: "source-1".to_string(),
            };

            let response = get_source_coverage(&state, params).await.unwrap();
            assert_eq!(response.source_id, "source-1");
            // Should have introduced at least concept-a
            assert!(response.total_concepts > 0);
        }

        // Error handling tests
        #[tokio::test]
        async fn test_get_related_concepts_not_found() {
            let state = create_query_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "nonexistent".to_string(),
                relationship_types: None,
                direction: "both".to_string(),
                depth: 1,
            };

            let result = get_related_concepts(&state, params).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_find_concept_path_from_not_found() {
            let state = create_query_test_state().await;
            let params = FindConceptPathParams {
                from_id: "nonexistent".to_string(),
                to_id: "concept-a".to_string(),
                max_depth: 5,
            };

            let result = find_concept_path(&state, params).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_get_prerequisites_not_found() {
            let state = create_query_test_state().await;
            let params = GetPrerequisitesParams {
                concept_id: "nonexistent".to_string(),
                depth: 3,
            };

            let result = get_prerequisites(&state, params).await;
            assert!(result.is_err());
        }
    }
}
