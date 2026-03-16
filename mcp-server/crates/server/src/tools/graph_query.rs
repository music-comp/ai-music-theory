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
    /// Optional tier filter: "foundational", "intermediate", "advanced"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Optional minimum confidence threshold: "low", "medium", "high"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<String>,
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
    /// Optional tier filter: "foundational", "intermediate", "advanced"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Optional minimum confidence threshold: "low", "medium", "high"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<String>,
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
    /// Optional tier filter: "foundational", "intermediate", "advanced"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Optional minimum confidence threshold: "low", "medium", "high"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<String>,
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
    /// Optional tier filter: "foundational", "intermediate", "advanced"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Optional minimum confidence threshold: "low", "medium", "high"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<String>,
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

/// Parameters for get_learning_path.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetLearningPathParams {
    /// Target concept to learn
    pub target_id: String,
    /// Optional tier filter: "foundational", "intermediate", "advanced"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Optional minimum confidence threshold: "low", "medium", "high"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<String>,
}

// ============================================================================
// Response Types
// ============================================================================

/// Response for get_learning_path tool.
#[derive(Debug, Serialize)]
pub struct LearningPathResponse {
    /// Target concept ID
    pub target_id: String,
    /// Target concept title
    pub target_title: String,
    /// Total number of steps in the learning path
    pub total_steps: usize,
    /// Ordered learning steps
    pub steps: Vec<LearningStep>,
}

/// A single step in a learning path.
#[derive(Debug, Serialize)]
pub struct LearningStep {
    /// 1-based order in the learning path
    pub order: usize,
    /// Concept ID
    pub concept_id: String,
    /// Concept title
    pub title: String,
    /// Concept category
    pub category: String,
    /// Prerequisite depth tier: "foundational", "intermediate", "advanced"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Extraction quality: "high", "medium", "low"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraction_confidence: Option<String>,
}

// ============================================================================
// Metadata Filtering Helpers
// ============================================================================

/// Map a confidence level string to a numeric rank for ordering.
///
/// Returns `None` for unrecognised values so callers can decide how to handle
/// them (typically: exclude the result when a `min_confidence` filter is active).
#[cfg(feature = "graph")]
fn confidence_rank(level: &str) -> Option<u32> {
    match level {
        "low" => Some(1),
        "medium" => Some(2),
        "high" => Some(3),
        _ => None,
    }
}

/// Check whether a graph node passes the optional tier and min_confidence filters.
///
/// * If `tier` is `Some`, the node must have `metadata["tier"]` equal to that value.
/// * If `min_confidence` is `Some`, the node must have `metadata["extraction_confidence"]`
///   whose rank is >= the threshold rank.  Nodes **without** confidence metadata are
///   excluded when a `min_confidence` filter is set.
#[cfg(feature = "graph")]
fn node_passes_filters(
    node: &fabryk::graph::Node,
    tier: &Option<String>,
    min_confidence: &Option<String>,
) -> bool {
    // Tier filter
    if let Some(ref t) = tier {
        let node_tier = node.metadata.get("tier").and_then(|v| v.as_str());
        match node_tier {
            Some(nt) if nt == t.as_str() => {}
            _ => return false,
        }
    }

    // Confidence filter
    if let Some(ref mc) = min_confidence {
        let threshold = match confidence_rank(mc) {
            Some(r) => r,
            None => return false, // unrecognised threshold value — exclude everything
        };
        let node_confidence = node
            .metadata
            .get("extraction_confidence")
            .and_then(|v| v.as_str());
        match node_confidence {
            Some(nc) => match confidence_rank(nc) {
                Some(rank) if rank >= threshold => {}
                _ => return false,
            },
            None => return false, // no confidence metadata — exclude
        }
    }

    true
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
            if is_concept_node(neighbor_node)
                && node_passes_filters(neighbor_node, &params.tier, &params.min_confidence)
            {
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
        if is_concept_node(node)
            && node_passes_filters(node, &params.tier, &params.min_confidence)
        {
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
        .filter(|n| node_passes_filters(n, &params.tier, &params.min_confidence))
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

    // Build response: filter by category, tier, confidence, concept nodes only, limit results
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
            // Apply tier and confidence filters
            if !node_passes_filters(node, &params.tier, &params.min_confidence) {
                continue;
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
// get_learning_path
// ============================================================================

/// Get a topologically sorted learning path to a target concept.
///
/// Returns the prerequisite chain with tier annotations, followed by the
/// target concept itself as the final step. This builds on `prerequisites_sorted`
/// but provides a learning-focused response format.
///
/// # Arguments
///
/// * `state` - Application state
/// * `params` - Query parameters
///
/// # Returns
///
/// Returns `LearningPathResponse` with ordered learning steps.
///
/// # Errors
///
/// Returns error if graph is not loaded or the target node is not found.
#[cfg(feature = "graph")]
pub async fn get_learning_path(
    state: &AppState,
    params: GetLearningPathParams,
) -> Result<LearningPathResponse> {
    // Get loaded graph
    let guard = state.require_graph()?;
    let loaded = guard.as_ref().unwrap();

    // Verify target node exists and get its details
    let target_node = loaded.data.get_node(&params.target_id).ok_or_else(|| {
        crate::error::Error::not_found_msg(format!("Node not found: {}", params.target_id))
    })?;
    let target_title = node_title(target_node).to_string();

    // Use fabryk prerequisites_sorted algorithm
    let prereq_result = prerequisites_sorted(&loaded.data, &params.target_id)
        .map_err(|e| crate::error::Error::operation(format!("Prerequisites failed: {}", e)))?;

    // Build learning steps from ordered prerequisites
    let mut steps = Vec::new();
    let mut order = 1;

    for node in &prereq_result.ordered {
        if is_concept_node(node) && node_passes_filters(node, &params.tier, &params.min_confidence)
        {
            let tier = node
                .metadata
                .get("tier")
                .and_then(|v| v.as_str())
                .map(String::from);
            let extraction_confidence = node
                .metadata
                .get("extraction_confidence")
                .and_then(|v| v.as_str())
                .map(String::from);

            steps.push(LearningStep {
                order,
                concept_id: node.id.clone(),
                title: node_title(node).to_string(),
                category: node_category(node).to_string(),
                tier,
                extraction_confidence,
            });
            order += 1;
        }
    }

    // Add the target concept itself as the final step (if it passes filters)
    if node_passes_filters(target_node, &params.tier, &params.min_confidence) {
        let tier = target_node
            .metadata
            .get("tier")
            .and_then(|v| v.as_str())
            .map(String::from);
        let extraction_confidence = target_node
            .metadata
            .get("extraction_confidence")
            .and_then(|v| v.as_str())
            .map(String::from);

        steps.push(LearningStep {
            order,
            concept_id: params.target_id.clone(),
            title: target_title.clone(),
            category: node_category(target_node).to_string(),
            tier,
            extraction_confidence,
        });
    }

    let total_steps = steps.len();

    Ok(LearningPathResponse {
        target_id: params.target_id,
        target_title,
        total_steps,
        steps,
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

#[cfg(not(feature = "graph"))]
pub async fn get_learning_path(
    _state: &AppState,
    _params: GetLearningPathParams,
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
                tier: None,
                min_confidence: None,
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
                tier: None,
                min_confidence: None,
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
                tier: None,
                min_confidence: None,
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
                tier: None,
                min_confidence: None,
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
                tier: None,
                min_confidence: None,
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
                tier: None,
                min_confidence: None,
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
                tier: None,
                min_confidence: None,
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
                tier: None,
                min_confidence: None,
            };

            let result = get_prerequisites(&state, params).await;
            assert!(result.is_err());
        }

        // ---------------------------------------------------------------
        // Depth/limit validation error paths
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_related_concepts_depth_exceeds_max() {
            let state = create_query_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "concept-a".to_string(),
                relationship_types: None,
                direction: "both".to_string(),
                depth: 4, // max is 3
                tier: None,
                min_confidence: None,
            };

            let result = get_related_concepts(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("Depth exceeds maximum of 3"));
        }

        #[tokio::test]
        async fn test_find_concept_path_max_depth_exceeds_max() {
            let state = create_query_test_state().await;
            let params = FindConceptPathParams {
                from_id: "concept-a".to_string(),
                to_id: "concept-b".to_string(),
                max_depth: 9, // max is 8
            };

            let result = find_concept_path(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("Max depth exceeds maximum of 8"));
        }

        #[tokio::test]
        async fn test_get_prerequisites_depth_exceeds_max() {
            let state = create_query_test_state().await;
            let params = GetPrerequisitesParams {
                concept_id: "concept-a".to_string(),
                depth: 6, // max is 5
                tier: None,
                min_confidence: None,
            };

            let result = get_prerequisites(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("Depth exceeds maximum of 5"));
        }

        #[tokio::test]
        async fn test_get_concept_neighborhood_radius_exceeds_max() {
            let state = create_query_test_state().await;
            let params = GetConceptNeighborhoodParams {
                concept_id: "concept-a".to_string(),
                radius: 4, // max is 3
                max_nodes: 30,
                tier: None,
                min_confidence: None,
            };

            let result = get_concept_neighborhood(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("Radius exceeds maximum of 3"));
        }

        #[tokio::test]
        async fn test_get_concept_neighborhood_max_nodes_exceeds_max() {
            let state = create_query_test_state().await;
            let params = GetConceptNeighborhoodParams {
                concept_id: "concept-a".to_string(),
                radius: 2,
                max_nodes: 51, // max is 50
                tier: None,
                min_confidence: None,
            };

            let result = get_concept_neighborhood(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("Max nodes exceeds maximum of 50"));
        }

        #[tokio::test]
        async fn test_get_dependents_depth_exceeds_max() {
            let state = create_query_test_state().await;
            let params = GetDependentsParams {
                concept_id: "concept-a".to_string(),
                depth: 5, // max is 4
            };

            let result = get_dependents(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("Depth exceeds maximum of 4"));
        }

        #[tokio::test]
        async fn test_get_central_concepts_limit_exceeds_max() {
            let state = create_query_test_state().await;
            let params = GetCentralConceptsParams {
                category: None,
                limit: 26, // max is 25
                tier: None,
                min_confidence: None,
            };

            let result = get_central_concepts(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("Limit exceeds maximum of 25"));
        }

        #[tokio::test]
        async fn test_find_bridge_concepts_limit_exceeds_max() {
            let state = create_query_test_state().await;
            let params = FindBridgeConceptsParams {
                category_a: "harmony".to_string(),
                category_b: "fundamentals".to_string(),
                limit: 16, // max is 15
            };

            let result = find_bridge_concepts(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("Limit exceeds maximum of 15"));
        }

        // ---------------------------------------------------------------
        // Node-not-found error paths for remaining tools
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_find_concept_path_to_not_found() {
            let state = create_query_test_state().await;
            let params = FindConceptPathParams {
                from_id: "concept-a".to_string(),
                to_id: "nonexistent".to_string(),
                max_depth: 5,
            };

            let result = find_concept_path(&state, params).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_get_concept_neighborhood_not_found() {
            let state = create_query_test_state().await;
            let params = GetConceptNeighborhoodParams {
                concept_id: "nonexistent".to_string(),
                radius: 2,
                max_nodes: 30,
                tier: None,
                min_confidence: None,
            };

            let result = get_concept_neighborhood(&state, params).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_get_dependents_not_found() {
            let state = create_query_test_state().await;
            let params = GetDependentsParams {
                concept_id: "nonexistent".to_string(),
                depth: 2,
            };

            let result = get_dependents(&state, params).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_get_concept_sources_not_found() {
            let state = create_query_test_state().await;
            let params = GetConceptSourcesParams {
                concept_id: "nonexistent".to_string(),
            };

            let result = get_concept_sources(&state, params).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_get_concept_variants_not_found() {
            let state = create_query_test_state().await;
            let params = GetConceptVariantsParams {
                canonical_id: "nonexistent".to_string(),
            };

            let result = get_concept_variants(&state, params).await;
            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_get_source_coverage_not_found() {
            let state = create_query_test_state().await;
            let params = GetSourceCoverageParams {
                source_id: "nonexistent".to_string(),
            };

            let result = get_source_coverage(&state, params).await;
            assert!(result.is_err());
        }

        // ---------------------------------------------------------------
        // Wrong node type error paths
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_concept_sources_on_source_node() {
            let state = create_query_test_state().await;
            let params = GetConceptSourcesParams {
                concept_id: "source-1".to_string(),
            };

            let result = get_concept_sources(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("Cannot get sources for a source node"));
        }

        #[tokio::test]
        async fn test_get_concept_variants_on_source_node() {
            let state = create_query_test_state().await;
            let params = GetConceptVariantsParams {
                canonical_id: "source-1".to_string(),
            };

            let result = get_concept_variants(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("Cannot get variants for a source node"));
        }

        #[tokio::test]
        async fn test_get_concept_variants_not_canonical() {
            // Create a state with a non-canonical concept
            let mut data = fabryk::graph::GraphData::new();
            data.add_node(
                Node::new("variant-a", "Variant A")
                    .with_category("harmony")
                    .as_variant_of("canonical-x"),
            );

            let stats = GraphStats {
                node_count: 1,
                edge_count: 0,
                concept_count: 1,
                source_count: 0,
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

            let params = GetConceptVariantsParams {
                canonical_id: "variant-a".to_string(),
            };

            let result = get_concept_variants(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("not marked as canonical"));
        }

        #[tokio::test]
        async fn test_get_source_coverage_on_concept_node() {
            let state = create_query_test_state().await;
            let params = GetSourceCoverageParams {
                source_id: "concept-a".to_string(),
            };

            let result = get_source_coverage(&state, params).await;
            assert!(result.is_err());
            let err_str = format!("{}", result.unwrap_err());
            assert!(err_str.contains("Cannot get coverage for a concept node"));
        }

        // ---------------------------------------------------------------
        // Direction-specific traversal
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_related_concepts_outgoing_only() {
            let state = create_query_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "concept-a".to_string(),
                relationship_types: None,
                direction: "outgoing".to_string(),
                depth: 1,
                tier: None,
                min_confidence: None,
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            // All results should be outgoing direction
            for r in &response.related {
                assert_eq!(r.direction, "outgoing");
            }
        }

        #[tokio::test]
        async fn test_get_related_concepts_incoming_only() {
            let state = create_query_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "concept-a".to_string(),
                relationship_types: None,
                direction: "incoming".to_string(),
                depth: 1,
                tier: None,
                min_confidence: None,
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            // All results should be incoming direction
            for r in &response.related {
                assert_eq!(r.direction, "incoming");
            }
        }

        // ---------------------------------------------------------------
        // Deeper BFS traversal
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_related_concepts_depth_2() {
            let state = create_query_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "concept-a".to_string(),
                relationship_types: None,
                direction: "both".to_string(),
                depth: 2,
                tier: None,
                min_confidence: None,
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            assert_eq!(response.depth, 2);
            // Should find more concepts at depth 2 than depth 1
            assert!(response.total >= 2);
        }

        #[tokio::test]
        async fn test_get_related_concepts_depth_3() {
            let state = create_query_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "concept-c".to_string(),
                relationship_types: None,
                direction: "both".to_string(),
                depth: 3,
                tier: None,
                min_confidence: None,
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            assert_eq!(response.depth, 3);
        }

        // ---------------------------------------------------------------
        // Relationship filter edge cases
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_related_concepts_multiple_relationship_filter() {
            let state = create_query_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "concept-a".to_string(),
                relationship_types: Some("prerequisite,relates_to".to_string()),
                direction: "both".to_string(),
                depth: 1,
                tier: None,
                min_confidence: None,
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            // All results should be either Prerequisite or RelatesTo
            for r in &response.related {
                assert!(
                    r.relationship == "Prerequisite" || r.relationship == "RelatesTo",
                    "Unexpected relationship: {}",
                    r.relationship
                );
            }
        }

        #[tokio::test]
        async fn test_get_related_concepts_no_matching_filter() {
            let state = create_query_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "concept-a".to_string(),
                relationship_types: Some("covers".to_string()),
                direction: "both".to_string(),
                depth: 1,
                tier: None,
                min_confidence: None,
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            // No matching edges with "covers" between concepts
            // May be 0 or may have source edges -- just verify no error
            assert_eq!(response.concept_id, "concept-a");
        }

        // ---------------------------------------------------------------
        // find_concept_path edge cases
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_find_concept_path_same_node() {
            let state = create_query_test_state().await;
            let params = FindConceptPathParams {
                from_id: "concept-a".to_string(),
                to_id: "concept-a".to_string(),
                max_depth: 5,
            };

            let response = find_concept_path(&state, params).await.unwrap();
            assert_eq!(response.from, "concept-a");
            assert_eq!(response.to, "concept-a");
        }

        #[tokio::test]
        async fn test_find_concept_path_nodes_exist_path_details() {
            let state = create_query_test_state().await;
            let params = FindConceptPathParams {
                from_id: "concept-c".to_string(),
                to_id: "concept-a".to_string(),
                max_depth: 5,
            };

            let response = find_concept_path(&state, params).await.unwrap();
            assert!(response.found);
            // Verify nodes and edges are populated
            assert!(!response.nodes.is_empty());
            assert!(!response.edges.is_empty());
            // First node should be from, last should be to
            assert_eq!(response.nodes.first().unwrap().id, "concept-c");
            assert_eq!(response.nodes.last().unwrap().id, "concept-a");
            // Verify step numbers
            for (i, node) in response.nodes.iter().enumerate() {
                assert_eq!(node.step, i as u32);
            }
            for (i, edge) in response.edges.iter().enumerate() {
                assert_eq!(edge.step, i as u32);
            }
        }

        // ---------------------------------------------------------------
        // get_prerequisites edge cases
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_prerequisites_node_with_no_prerequisites() {
            let state = create_query_test_state().await;
            let params = GetPrerequisitesParams {
                concept_id: "concept-c".to_string(),
                depth: 3,
                tier: None,
                min_confidence: None,
            };

            let response = get_prerequisites(&state, params).await.unwrap();
            assert_eq!(response.concept_id, "concept-c");
            assert_eq!(response.concept_title, "Concept C");
            // concept-c has no prerequisites in the test graph
            assert_eq!(response.total, 0);
            assert!(response.learning_order.is_empty());
        }

        #[tokio::test]
        async fn test_get_prerequisites_response_fields() {
            let state = create_query_test_state().await;
            let params = GetPrerequisitesParams {
                concept_id: "concept-a".to_string(),
                depth: 3,
                tier: None,
                min_confidence: None,
            };

            let response = get_prerequisites(&state, params).await.unwrap();
            assert_eq!(response.concept_id, "concept-a");
            assert_eq!(response.concept_title, "Concept A");
            // Verify prerequisites have proper fields
            for p in &response.prerequisites {
                assert!(!p.id.is_empty());
                assert!(!p.title.is_empty());
                assert!(!p.category.is_empty());
            }
        }

        // ---------------------------------------------------------------
        // get_concept_neighborhood edge cases
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_concept_neighborhood_small_max_nodes() {
            let state = create_query_test_state().await;
            let params = GetConceptNeighborhoodParams {
                concept_id: "concept-a".to_string(),
                radius: 2,
                max_nodes: 2, // Very small limit
                tier: None,
                min_confidence: None,
            };

            let response = get_concept_neighborhood(&state, params).await.unwrap();
            // Should not exceed max_nodes
            assert!(response.node_count <= 2);
            // Center node should always be present
            assert!(response.nodes.iter().any(|n| n.is_center));
        }

        #[tokio::test]
        async fn test_get_concept_neighborhood_radius_1() {
            let state = create_query_test_state().await;
            let params = GetConceptNeighborhoodParams {
                concept_id: "concept-b".to_string(),
                radius: 1,
                max_nodes: 30,
                tier: None,
                min_confidence: None,
            };

            let response = get_concept_neighborhood(&state, params).await.unwrap();
            assert_eq!(response.center, "concept-b");
            assert_eq!(response.radius, 1);
            // Center should be present
            let center = response.nodes.iter().find(|n| n.is_center).unwrap();
            assert_eq!(center.id, "concept-b");
            assert_eq!(center.distance, 0);
        }

        #[tokio::test]
        async fn test_get_concept_neighborhood_verifies_node_types() {
            let state = create_query_test_state().await;
            let params = GetConceptNeighborhoodParams {
                concept_id: "concept-a".to_string(),
                radius: 2,
                max_nodes: 30,
                tier: None,
                min_confidence: None,
            };

            let response = get_concept_neighborhood(&state, params).await.unwrap();
            // Every node should have a valid type
            for node in &response.nodes {
                assert!(
                    node.node_type == "concept"
                        || node.node_type == "source"
                        || node.node_type == "unknown",
                    "Unexpected node type: {}",
                    node.node_type
                );
                // Concepts should have categories, sources should not
                if node.node_type == "concept" {
                    assert!(node.category.is_some());
                }
            }
        }

        #[tokio::test]
        async fn test_get_concept_neighborhood_edges_reference_existing_nodes() {
            let state = create_query_test_state().await;
            let params = GetConceptNeighborhoodParams {
                concept_id: "concept-b".to_string(),
                radius: 2,
                max_nodes: 30,
                tier: None,
                min_confidence: None,
            };

            let response = get_concept_neighborhood(&state, params).await.unwrap();
            let node_ids: std::collections::HashSet<&str> =
                response.nodes.iter().map(|n| n.id.as_str()).collect();

            // Every edge should reference nodes in the neighborhood
            for edge in &response.edges {
                assert!(
                    node_ids.contains(edge.from.as_str()),
                    "Edge from {} not in neighborhood",
                    edge.from
                );
                assert!(
                    node_ids.contains(edge.to.as_str()),
                    "Edge to {} not in neighborhood",
                    edge.to
                );
            }
        }

        // ---------------------------------------------------------------
        // get_dependents edge cases
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_dependents_no_dependents() {
            let state = create_query_test_state().await;
            let params = GetDependentsParams {
                concept_id: "concept-d".to_string(),
                depth: 2,
            };

            let response = get_dependents(&state, params).await.unwrap();
            assert_eq!(response.concept_id, "concept-d");
            assert_eq!(response.concept_title, "Concept D");
            // concept-d has no outgoing prerequisite edges
            assert_eq!(response.total, 0);
        }

        #[tokio::test]
        async fn test_get_dependents_chain() {
            let state = create_query_test_state().await;
            // concept-c -> concept-b -> concept-a via prerequisites
            let params = GetDependentsParams {
                concept_id: "concept-c".to_string(),
                depth: 2,
            };

            let response = get_dependents(&state, params).await.unwrap();
            assert_eq!(response.concept_id, "concept-c");
            // Should find concept-b (depth 1) and concept-a (depth 2)
            assert!(response.total >= 1);
            assert!(response.dependents.iter().any(|d| d.id == "concept-b"));
        }

        #[tokio::test]
        async fn test_get_dependents_depth_1() {
            let state = create_query_test_state().await;
            let params = GetDependentsParams {
                concept_id: "concept-c".to_string(),
                depth: 1,
            };

            let response = get_dependents(&state, params).await.unwrap();
            // At depth 1, should only find direct dependents
            for d in &response.dependents {
                assert_eq!(d.depth, 1);
            }
        }

        // ---------------------------------------------------------------
        // get_central_concepts edge cases
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_central_concepts_nonexistent_category() {
            let state = create_query_test_state().await;
            let params = GetCentralConceptsParams {
                category: Some("nonexistent-category".to_string()),
                limit: 10,
                tier: None,
                min_confidence: None,
            };

            let response = get_central_concepts(&state, params).await.unwrap();
            assert_eq!(response.total, 0);
            assert!(response.concepts.is_empty());
        }

        #[tokio::test]
        async fn test_get_central_concepts_small_limit() {
            let state = create_query_test_state().await;
            let params = GetCentralConceptsParams {
                category: None,
                limit: 1,
                tier: None,
                min_confidence: None,
            };

            let response = get_central_concepts(&state, params).await.unwrap();
            assert!(response.total <= 1);
        }

        // ---------------------------------------------------------------
        // get_concept_sources edge cases
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_concept_sources_no_sources() {
            let state = create_query_test_state().await;
            // concept-d has no incoming source edges
            let params = GetConceptSourcesParams {
                concept_id: "concept-d".to_string(),
            };

            let response = get_concept_sources(&state, params).await.unwrap();
            assert_eq!(response.concept_id, "concept-d");
            assert_eq!(response.concept_title, "Concept D");
            assert_eq!(response.total, 0);
            assert!(response.sources.is_empty());
        }

        #[tokio::test]
        async fn test_get_concept_sources_verifies_fields() {
            let state = create_query_test_state().await;
            let params = GetConceptSourcesParams {
                concept_id: "concept-a".to_string(),
            };

            let response = get_concept_sources(&state, params).await.unwrap();
            for s in &response.sources {
                assert!(!s.source_id.is_empty());
                assert!(!s.source_title.is_empty());
                assert!(!s.source_author.is_empty());
                assert!(
                    s.relationship == "Introduces" || s.relationship == "Covers",
                    "Unexpected relationship: {}",
                    s.relationship
                );
            }
        }

        // ---------------------------------------------------------------
        // get_concept_variants with actual variants
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_concept_variants_with_variants() {
            let mut data = fabryk::graph::GraphData::new();

            // Canonical concept
            data.add_node(Node::new("canonical-x", "Canonical X").with_category("harmony"));

            // Variant concepts
            data.add_node(
                Node::new("variant-x-1", "Variant X (Source 1)")
                    .with_category("harmony")
                    .with_source("src-1")
                    .as_variant_of("canonical-x"),
            );
            data.add_node(
                Node::new("variant-x-2", "Variant X (Source 2)")
                    .with_category("harmony")
                    .with_source("src-2")
                    .as_variant_of("canonical-x"),
            );

            // Source nodes
            data.add_node(
                Node::new("src-1", "Source 1")
                    .with_node_type(NodeType::Custom("source".to_string()))
                    .with_metadata("author", serde_json::json!("Author 1")),
            );
            data.add_node(
                Node::new("src-2", "Source 2")
                    .with_node_type(NodeType::Custom("source".to_string()))
                    .with_metadata("author", serde_json::json!("Author 2")),
            );

            let stats = GraphStats {
                node_count: 5,
                edge_count: 0,
                concept_count: 3,
                source_count: 2,
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

            let params = GetConceptVariantsParams {
                canonical_id: "canonical-x".to_string(),
            };

            let response = get_concept_variants(&state, params).await.unwrap();
            assert_eq!(response.canonical_id, "canonical-x");
            assert_eq!(response.canonical_title, "Canonical X");
            assert_eq!(response.total, 2);

            let variant_ids: Vec<&str> = response.variants.iter().map(|v| v.id.as_str()).collect();
            assert!(variant_ids.contains(&"variant-x-1"));
            assert!(variant_ids.contains(&"variant-x-2"));
        }

        #[tokio::test]
        async fn test_get_concept_variants_with_missing_source_node() {
            let mut data = fabryk::graph::GraphData::new();

            data.add_node(Node::new("canonical-y", "Canonical Y").with_category("harmony"));
            data.add_node(
                Node::new("variant-y-1", "Variant Y")
                    .with_category("harmony")
                    .with_source("missing-source")
                    .as_variant_of("canonical-y"),
            );

            let stats = GraphStats {
                node_count: 2,
                edge_count: 0,
                concept_count: 2,
                source_count: 0,
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

            let params = GetConceptVariantsParams {
                canonical_id: "canonical-y".to_string(),
            };

            let response = get_concept_variants(&state, params).await.unwrap();
            assert_eq!(response.total, 1);
            // Should fall back to source_id as title
            assert_eq!(response.variants[0].source_id, "missing-source");
            assert_eq!(response.variants[0].source_title, "missing-source");
        }

        // ---------------------------------------------------------------
        // get_source_coverage edge cases
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_get_source_coverage_with_covers_edges() {
            let mut data = fabryk::graph::GraphData::new();

            data.add_node(
                Node::new("src-x", "Source X")
                    .with_node_type(NodeType::Custom("source".to_string()))
                    .with_metadata("author", serde_json::json!("Author X")),
            );
            data.add_node(Node::new("c1", "Concept 1").with_category("harmony"));
            data.add_node(Node::new("c2", "Concept 2").with_category("rhythm"));
            data.add_node(Node::new("c3", "Concept 3").with_category("melody"));

            data.add_edge(Edge::new("src-x", "c1", Relationship::Introduces))
                .expect("edge");
            data.add_edge(Edge::new("src-x", "c2", Relationship::Covers))
                .expect("edge");
            data.add_edge(Edge::new("src-x", "c3", Relationship::Covers))
                .expect("edge");

            let stats = GraphStats {
                node_count: 4,
                edge_count: 3,
                concept_count: 3,
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

            let params = GetSourceCoverageParams {
                source_id: "src-x".to_string(),
            };

            let response = get_source_coverage(&state, params).await.unwrap();
            assert_eq!(response.source_id, "src-x");
            assert_eq!(response.source_title, "Source X");
            assert_eq!(response.source_author, "Author X");
            assert_eq!(response.total_concepts, 3);
            assert_eq!(response.introduces_count, 1);
            assert_eq!(response.covers_count, 2);
            assert_eq!(response.introduces.len(), 1);
            assert_eq!(response.covers.len(), 2);
        }

        #[tokio::test]
        async fn test_get_source_coverage_no_coverage() {
            let mut data = fabryk::graph::GraphData::new();

            data.add_node(
                Node::new("src-empty", "Empty Source")
                    .with_node_type(NodeType::Custom("source".to_string()))
                    .with_metadata("author", serde_json::json!("Nobody")),
            );

            let stats = GraphStats {
                node_count: 1,
                edge_count: 0,
                concept_count: 0,
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

            let params = GetSourceCoverageParams {
                source_id: "src-empty".to_string(),
            };

            let response = get_source_coverage(&state, params).await.unwrap();
            assert_eq!(response.total_concepts, 0);
            assert_eq!(response.introduces_count, 0);
            assert_eq!(response.covers_count, 0);
        }

        #[tokio::test]
        async fn test_get_source_coverage_ignores_non_coverage_edges() {
            let mut data = fabryk::graph::GraphData::new();

            data.add_node(
                Node::new("src-z", "Source Z")
                    .with_node_type(NodeType::Custom("source".to_string()))
                    .with_metadata("author", serde_json::json!("Author Z")),
            );
            data.add_node(Node::new("c-z", "Concept Z").with_category("test"));

            // RelatesTo edge from source -- should be ignored
            data.add_edge(Edge::new("src-z", "c-z", Relationship::RelatesTo))
                .expect("edge");

            let stats = GraphStats {
                node_count: 2,
                edge_count: 1,
                concept_count: 1,
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

            let params = GetSourceCoverageParams {
                source_id: "src-z".to_string(),
            };

            let response = get_source_coverage(&state, params).await.unwrap();
            assert_eq!(response.total_concepts, 0);
        }

        // ---------------------------------------------------------------
        // find_concept_path: no path (disconnected)
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_find_concept_path_disconnected_nodes() {
            let mut data = fabryk::graph::GraphData::new();
            data.add_node(Node::new("isolated-a", "Isolated A").with_category("test"));
            data.add_node(Node::new("isolated-b", "Isolated B").with_category("test"));

            let stats = GraphStats {
                node_count: 2,
                edge_count: 0,
                concept_count: 2,
                source_count: 0,
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

            let params = FindConceptPathParams {
                from_id: "isolated-a".to_string(),
                to_id: "isolated-b".to_string(),
                max_depth: 5,
            };

            let response = find_concept_path(&state, params).await.unwrap();
            assert!(!response.found);
            assert_eq!(response.path_length, 0);
            assert!(response.nodes.is_empty());
            assert!(response.edges.is_empty());
        }

        // ---------------------------------------------------------------
        // find_concept_path: path through source nodes
        // ---------------------------------------------------------------

        #[tokio::test]
        async fn test_find_concept_path_with_non_concept_in_path() {
            let state = create_query_test_state().await;
            // Try path that may go through source node
            let params = FindConceptPathParams {
                from_id: "source-1".to_string(),
                to_id: "concept-b".to_string(),
                max_depth: 5,
            };

            let response = find_concept_path(&state, params).await.unwrap();
            // Just verify it completes without error
            assert_eq!(response.from, "source-1");
            assert_eq!(response.to, "concept-b");
            // Non-concept nodes should have category=None
            if response.found {
                for node in &response.nodes {
                    if node.id == "source-1" {
                        assert!(node.category.is_none());
                    }
                }
            }
        }

        // ---------------------------------------------------------------
        // Tier and confidence metadata filtering
        // ---------------------------------------------------------------

        /// Helper to create AppState with metadata-annotated nodes.
        async fn create_metadata_test_state() -> Arc<AppState> {
            let mut data = fabryk::graph::GraphData::new();

            // Source node
            data.add_node(
                Node::new("source-1", "Source One")
                    .with_node_type(NodeType::Custom("source".to_string()))
                    .with_metadata("author", serde_json::json!("Author")),
            );

            // Concept nodes with tier and confidence metadata
            data.add_node(
                Node::new("c-found-high", "Foundational High")
                    .with_category("harmony")
                    .with_source("source-1")
                    .with_metadata("tier", serde_json::json!("foundational"))
                    .with_metadata("extraction_confidence", serde_json::json!("high")),
            );
            data.add_node(
                Node::new("c-found-med", "Foundational Medium")
                    .with_category("harmony")
                    .with_source("source-1")
                    .with_metadata("tier", serde_json::json!("foundational"))
                    .with_metadata("extraction_confidence", serde_json::json!("medium")),
            );
            data.add_node(
                Node::new("c-inter-high", "Intermediate High")
                    .with_category("harmony")
                    .with_source("source-1")
                    .with_metadata("tier", serde_json::json!("intermediate"))
                    .with_metadata("extraction_confidence", serde_json::json!("high")),
            );
            data.add_node(
                Node::new("c-adv-low", "Advanced Low")
                    .with_category("harmony")
                    .with_source("source-1")
                    .with_metadata("tier", serde_json::json!("advanced"))
                    .with_metadata("extraction_confidence", serde_json::json!("low")),
            );
            // Node with no tier/confidence metadata
            data.add_node(
                Node::new("c-no-meta", "No Metadata")
                    .with_category("harmony")
                    .with_source("source-1"),
            );

            // Hub node that connects to all others
            data.add_node(
                Node::new("c-hub", "Hub Concept")
                    .with_category("fundamentals")
                    .with_source("source-1")
                    .with_metadata("tier", serde_json::json!("foundational"))
                    .with_metadata("extraction_confidence", serde_json::json!("high")),
            );

            // Edges: hub relates to all concept nodes
            data.add_edge(Edge::new("c-hub", "c-found-high", Relationship::RelatesTo))
                .expect("edge");
            data.add_edge(Edge::new("c-hub", "c-found-med", Relationship::RelatesTo))
                .expect("edge");
            data.add_edge(Edge::new("c-hub", "c-inter-high", Relationship::RelatesTo))
                .expect("edge");
            data.add_edge(Edge::new("c-hub", "c-adv-low", Relationship::RelatesTo))
                .expect("edge");
            data.add_edge(Edge::new("c-hub", "c-no-meta", Relationship::RelatesTo))
                .expect("edge");
            // Prerequisite chain: c-found-high -> c-inter-high -> c-adv-low
            data.add_edge(Edge::new(
                "c-found-high",
                "c-inter-high",
                Relationship::Prerequisite,
            ))
            .expect("edge");
            data.add_edge(Edge::new(
                "c-inter-high",
                "c-adv-low",
                Relationship::Prerequisite,
            ))
            .expect("edge");
            // Source introduces hub
            data.add_edge(Edge::new("source-1", "c-hub", Relationship::Introduces))
                .expect("edge");

            let stats = GraphStats {
                node_count: 7,
                edge_count: 8,
                concept_count: 6,
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
        async fn test_get_related_concepts_filter_by_tier() {
            let state = create_metadata_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "c-hub".to_string(),
                relationship_types: None,
                direction: "both".to_string(),
                depth: 1,
                tier: Some("foundational".to_string()),
                min_confidence: None,
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            // Only foundational nodes should appear
            for r in &response.related {
                assert!(
                    r.id == "c-found-high" || r.id == "c-found-med",
                    "Unexpected node in tier-filtered results: {}",
                    r.id
                );
            }
            assert!(response.total >= 2);
        }

        #[tokio::test]
        async fn test_get_related_concepts_filter_by_min_confidence_high() {
            let state = create_metadata_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "c-hub".to_string(),
                relationship_types: None,
                direction: "both".to_string(),
                depth: 1,
                tier: None,
                min_confidence: Some("high".to_string()),
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            // Only high-confidence nodes should appear; c-no-meta is excluded
            for r in &response.related {
                assert!(
                    r.id == "c-found-high" || r.id == "c-inter-high",
                    "Unexpected node in confidence-filtered results: {}",
                    r.id
                );
            }
        }

        #[tokio::test]
        async fn test_get_related_concepts_filter_by_min_confidence_medium() {
            let state = create_metadata_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "c-hub".to_string(),
                relationship_types: None,
                direction: "both".to_string(),
                depth: 1,
                tier: None,
                min_confidence: Some("medium".to_string()),
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            // medium and high confidence nodes should appear; low and no-meta excluded
            let ids: Vec<&str> = response.related.iter().map(|r| r.id.as_str()).collect();
            assert!(ids.contains(&"c-found-high"));
            assert!(ids.contains(&"c-found-med"));
            assert!(ids.contains(&"c-inter-high"));
            assert!(!ids.contains(&"c-adv-low"));
            assert!(!ids.contains(&"c-no-meta"));
        }

        #[tokio::test]
        async fn test_get_related_concepts_filter_tier_and_confidence() {
            let state = create_metadata_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "c-hub".to_string(),
                relationship_types: None,
                direction: "both".to_string(),
                depth: 1,
                tier: Some("foundational".to_string()),
                min_confidence: Some("high".to_string()),
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            // Only foundational + high confidence
            assert_eq!(response.total, 1);
            assert_eq!(response.related[0].id, "c-found-high");
        }

        #[tokio::test]
        async fn test_get_related_concepts_filter_excludes_no_metadata() {
            let state = create_metadata_test_state().await;
            let params = GetRelatedConceptsParams {
                concept_id: "c-hub".to_string(),
                relationship_types: None,
                direction: "both".to_string(),
                depth: 1,
                tier: None,
                min_confidence: Some("low".to_string()),
            };

            let response = get_related_concepts(&state, params).await.unwrap();
            let ids: Vec<&str> = response.related.iter().map(|r| r.id.as_str()).collect();
            // c-no-meta has no confidence metadata and should be excluded
            assert!(!ids.contains(&"c-no-meta"));
            // c-adv-low has "low" which is >= threshold "low"
            assert!(ids.contains(&"c-adv-low"));
        }

        #[tokio::test]
        async fn test_get_prerequisites_filter_by_tier() {
            let state = create_metadata_test_state().await;
            // c-adv-low has prerequisites: c-inter-high, c-found-high
            let params = GetPrerequisitesParams {
                concept_id: "c-adv-low".to_string(),
                depth: 3,
                tier: Some("intermediate".to_string()),
                min_confidence: None,
            };

            let response = get_prerequisites(&state, params).await.unwrap();
            // Only intermediate-tier prerequisites
            for p in &response.prerequisites {
                assert_eq!(p.id, "c-inter-high");
            }
        }

        #[tokio::test]
        async fn test_get_central_concepts_filter_by_tier() {
            let state = create_metadata_test_state().await;
            let params = GetCentralConceptsParams {
                category: None,
                limit: 25,
                tier: Some("foundational".to_string()),
                min_confidence: None,
            };

            let response = get_central_concepts(&state, params).await.unwrap();
            // All returned concepts must be foundational
            let ids: Vec<&str> = response.concepts.iter().map(|c| c.id.as_str()).collect();
            for id in &ids {
                assert!(
                    *id == "c-found-high" || *id == "c-found-med" || *id == "c-hub",
                    "Non-foundational concept in filtered results: {}",
                    id
                );
            }
        }

        #[tokio::test]
        async fn test_get_central_concepts_filter_by_min_confidence() {
            let state = create_metadata_test_state().await;
            let params = GetCentralConceptsParams {
                category: None,
                limit: 25,
                tier: None,
                min_confidence: Some("high".to_string()),
            };

            let response = get_central_concepts(&state, params).await.unwrap();
            let ids: Vec<&str> = response.concepts.iter().map(|c| c.id.as_str()).collect();
            // Only high-confidence nodes
            assert!(!ids.contains(&"c-adv-low"));
            assert!(!ids.contains(&"c-no-meta"));
            assert!(!ids.contains(&"c-found-med"));
        }
    }

    // -------------------------------------------------------------------
    // Serialization round-trip tests for parameter types
    // -------------------------------------------------------------------

    #[test]
    fn test_get_related_concepts_params_with_all_fields() {
        let json = r#"{"concept_id": "test", "relationship_types": "prerequisite,extends", "direction": "outgoing", "depth": 2}"#;
        let params: GetRelatedConceptsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.concept_id, "test");
        assert_eq!(
            params.relationship_types,
            Some("prerequisite,extends".to_string())
        );
        assert_eq!(params.direction, "outgoing");
        assert_eq!(params.depth, 2);
    }

    #[test]
    fn test_find_concept_path_params_with_all_fields() {
        let json = r#"{"from_id": "a", "to_id": "b", "max_depth": 3}"#;
        let params: FindConceptPathParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.from_id, "a");
        assert_eq!(params.to_id, "b");
        assert_eq!(params.max_depth, 3);
    }

    #[test]
    fn test_get_prerequisites_params_with_all_fields() {
        let json = r#"{"concept_id": "test", "depth": 5}"#;
        let params: GetPrerequisitesParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.concept_id, "test");
        assert_eq!(params.depth, 5);
    }

    #[test]
    fn test_get_concept_neighborhood_params_with_all_fields() {
        let json = r#"{"concept_id": "test", "radius": 3, "max_nodes": 50}"#;
        let params: GetConceptNeighborhoodParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.concept_id, "test");
        assert_eq!(params.radius, 3);
        assert_eq!(params.max_nodes, 50);
    }

    #[test]
    fn test_get_dependents_params_with_all_fields() {
        let json = r#"{"concept_id": "test", "depth": 4}"#;
        let params: GetDependentsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.concept_id, "test");
        assert_eq!(params.depth, 4);
    }

    #[test]
    fn test_get_central_concepts_params_with_category() {
        let json = r#"{"category": "harmony", "limit": 20}"#;
        let params: GetCentralConceptsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.category, Some("harmony".to_string()));
        assert_eq!(params.limit, 20);
    }

    #[test]
    fn test_find_bridge_concepts_params_with_all_fields() {
        let json = r#"{"category_a": "harmony", "category_b": "rhythm", "limit": 10}"#;
        let params: FindBridgeConceptsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.category_a, "harmony");
        assert_eq!(params.category_b, "rhythm");
        assert_eq!(params.limit, 10);
    }

    #[test]
    fn test_get_related_concepts_params_serialization_round_trip() {
        let params = GetRelatedConceptsParams {
            concept_id: "test".to_string(),
            relationship_types: Some("prerequisite".to_string()),
            direction: "outgoing".to_string(),
            depth: 2,
            tier: None,
            min_confidence: None,
        };
        let json = serde_json::to_string(&params).unwrap();
        let deserialized: GetRelatedConceptsParams = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.concept_id, "test");
        assert_eq!(
            deserialized.relationship_types,
            Some("prerequisite".to_string())
        );
    }

    #[test]
    fn test_get_related_concepts_params_serialization_skip_none() {
        let params = GetRelatedConceptsParams {
            concept_id: "test".to_string(),
            relationship_types: None,
            direction: "both".to_string(),
            depth: 1,
            tier: None,
            min_confidence: None,
        };
        let json = serde_json::to_string(&params).unwrap();
        // relationship_types should be skipped when None
        assert!(!json.contains("relationship_types"));
    }

    // -------------------------------------------------------------------
    // confidence_rank unit tests
    // -------------------------------------------------------------------

    #[test]
    #[cfg(feature = "graph")]
    fn test_confidence_rank_ordering() {
        assert_eq!(confidence_rank("low"), Some(1));
        assert_eq!(confidence_rank("medium"), Some(2));
        assert_eq!(confidence_rank("high"), Some(3));
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_confidence_rank_unknown_returns_none() {
        assert_eq!(confidence_rank("unknown"), None);
        assert_eq!(confidence_rank(""), None);
        assert_eq!(confidence_rank("very_high"), None);
    }

    // -------------------------------------------------------------------
    // Deserialization with tier and min_confidence fields
    // -------------------------------------------------------------------

    #[test]
    fn test_get_related_concepts_params_with_tier_and_confidence() {
        let json = r#"{"concept_id": "test", "tier": "foundational", "min_confidence": "high"}"#;
        let params: GetRelatedConceptsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.tier, Some("foundational".to_string()));
        assert_eq!(params.min_confidence, Some("high".to_string()));
        // defaults still apply
        assert_eq!(params.direction, "both");
        assert_eq!(params.depth, 1);
    }

    #[test]
    fn test_get_prerequisites_params_with_tier_and_confidence() {
        let json =
            r#"{"concept_id": "test", "depth": 2, "tier": "intermediate", "min_confidence": "medium"}"#;
        let params: GetPrerequisitesParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.tier, Some("intermediate".to_string()));
        assert_eq!(params.min_confidence, Some("medium".to_string()));
        assert_eq!(params.depth, 2);
    }

    #[test]
    fn test_get_concept_neighborhood_params_with_tier_and_confidence() {
        let json =
            r#"{"concept_id": "test", "tier": "advanced", "min_confidence": "low"}"#;
        let params: GetConceptNeighborhoodParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.tier, Some("advanced".to_string()));
        assert_eq!(params.min_confidence, Some("low".to_string()));
        assert_eq!(params.radius, 2); // default
        assert_eq!(params.max_nodes, 30); // default
    }

    #[test]
    fn test_get_central_concepts_params_with_tier_and_confidence() {
        let json = r#"{"tier": "foundational", "min_confidence": "high"}"#;
        let params: GetCentralConceptsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.tier, Some("foundational".to_string()));
        assert_eq!(params.min_confidence, Some("high".to_string()));
        assert_eq!(params.limit, 10); // default
        assert!(params.category.is_none());
    }

    #[test]
    fn test_params_without_tier_and_confidence_default_to_none() {
        let json = r#"{"concept_id": "test"}"#;
        let params: GetRelatedConceptsParams = serde_json::from_str(json).unwrap();
        assert!(params.tier.is_none());
        assert!(params.min_confidence.is_none());

        let params: GetPrerequisitesParams = serde_json::from_str(json).unwrap();
        assert!(params.tier.is_none());
        assert!(params.min_confidence.is_none());

        let params: GetConceptNeighborhoodParams = serde_json::from_str(json).unwrap();
        assert!(params.tier.is_none());
        assert!(params.min_confidence.is_none());

        let json = r#"{}"#;
        let params: GetCentralConceptsParams = serde_json::from_str(json).unwrap();
        assert!(params.tier.is_none());
        assert!(params.min_confidence.is_none());
    }

    // -------------------------------------------------------------------
    // GetLearningPathParams and LearningPathResponse tests
    // -------------------------------------------------------------------

    #[test]
    fn test_get_learning_path_params_deserialization_minimal() {
        let json = r#"{"target_id": "seventh-chord"}"#;
        let params: GetLearningPathParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.target_id, "seventh-chord");
        assert!(params.tier.is_none());
        assert!(params.min_confidence.is_none());
    }

    #[test]
    fn test_get_learning_path_params_deserialization_with_filters() {
        let json =
            r#"{"target_id": "seventh-chord", "tier": "foundational", "min_confidence": "high"}"#;
        let params: GetLearningPathParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.target_id, "seventh-chord");
        assert_eq!(params.tier, Some("foundational".to_string()));
        assert_eq!(params.min_confidence, Some("high".to_string()));
    }

    #[test]
    fn test_get_learning_path_params_serialization_skip_none() {
        let params = GetLearningPathParams {
            target_id: "interval".to_string(),
            tier: None,
            min_confidence: None,
        };
        let json = serde_json::to_string(&params).unwrap();
        assert!(json.contains("\"target_id\":\"interval\""));
        assert!(!json.contains("tier"));
        assert!(!json.contains("min_confidence"));
    }

    #[test]
    fn test_learning_path_response_serialization() {
        let response = LearningPathResponse {
            target_id: "seventh-chord".to_string(),
            target_title: "Seventh Chord".to_string(),
            total_steps: 2,
            steps: vec![
                LearningStep {
                    order: 1,
                    concept_id: "triad".to_string(),
                    title: "Triad".to_string(),
                    category: "harmony".to_string(),
                    tier: Some("foundational".to_string()),
                    extraction_confidence: Some("high".to_string()),
                },
                LearningStep {
                    order: 2,
                    concept_id: "seventh-chord".to_string(),
                    title: "Seventh Chord".to_string(),
                    category: "harmony".to_string(),
                    tier: Some("intermediate".to_string()),
                    extraction_confidence: None,
                },
            ],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"target_id\":\"seventh-chord\""));
        assert!(json.contains("\"total_steps\":2"));
        assert!(json.contains("\"order\":1"));
        assert!(json.contains("\"concept_id\":\"triad\""));
        // tier=None fields should be skipped, but present ones should appear
        assert!(json.contains("\"tier\":\"foundational\""));
        // Second step has extraction_confidence=None, should be skipped
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        let step2 = &parsed["steps"][1];
        assert!(step2.get("extraction_confidence").is_none());
    }

    #[test]
    fn test_learning_step_serialization_no_optional_fields() {
        let step = LearningStep {
            order: 1,
            concept_id: "interval".to_string(),
            title: "Interval".to_string(),
            category: "fundamentals".to_string(),
            tier: None,
            extraction_confidence: None,
        };

        let json = serde_json::to_string(&step).unwrap();
        assert!(json.contains("\"order\":1"));
        assert!(json.contains("\"concept_id\":\"interval\""));
        assert!(!json.contains("tier"));
        assert!(!json.contains("extraction_confidence"));
    }
}
