//! Graph traversal algorithms for concept relationship analysis.
//!
//! This module provides core graph traversal and analysis algorithms used by
//! MCP query tools to explore the concept graph:
//!
//! - **neighborhood()** - BFS exploration of N-hop neighborhood
//! - **shortest_path()** - BFS shortest path finding
//! - **prerequisites_sorted()** - DFS topological sort of prerequisites
//! - **dependents()** - DFS discovery of dependent concepts
//! - **degree_centrality()** - Measure connectivity by edge count
//! - **bridge_concepts()** - Find concepts connecting two categories

use std::collections::{HashMap, HashSet, VecDeque};

use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Direction;

use super::persistence::ConceptGraph;
use super::types::{Node, Relationship};

/// Find all nodes within N hops of a starting node.
///
/// Uses breadth-first search to explore the graph undirected (following edges
/// in both directions). Returns nodes with their distance from the start.
///
/// # Arguments
///
/// * `graph` - The concept graph to search
/// * `start` - Starting node index
/// * `max_depth` - Maximum number of hops to traverse
/// * `max_nodes` - Optional limit on number of nodes to return
///
/// # Returns
///
/// Vector of (NodeIndex, distance) tuples for all nodes within range.
/// Does not include the start node itself.
///
/// # Example
///
/// ```ignore
/// let neighborhood = neighborhood(&graph, start_idx, 2, Some(20));
/// for (node_idx, distance) in neighborhood {
///     println!("Node {} is {} hops away", node_idx.index(), distance);
/// }
/// ```
pub fn neighborhood(
    graph: &ConceptGraph,
    start: NodeIndex,
    max_depth: u32,
    max_nodes: Option<u32>,
) -> Vec<(NodeIndex, u32)> {
    let mut visited: HashMap<NodeIndex, u32> = HashMap::new();
    let mut queue: VecDeque<(NodeIndex, u32)> = VecDeque::new();

    queue.push_back((start, 0));
    visited.insert(start, 0);

    while let Some((node, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        if let Some(max) = max_nodes {
            if visited.len() >= max as usize {
                break;
            }
        }

        for neighbor in graph.neighbors_undirected(node) {
            if !visited.contains_key(&neighbor) {
                visited.insert(neighbor, depth + 1);
                queue.push_back((neighbor, depth + 1));
            }
        }
    }

    // Remove start node from results
    visited.remove(&start);
    visited.into_iter().collect()
}

/// Find shortest path between two nodes using BFS.
///
/// Searches for the shortest undirected path (following edges in both directions).
/// Uses BFS which guarantees finding the shortest path.
///
/// # Arguments
///
/// * `graph` - The concept graph to search
/// * `from` - Starting node index
/// * `to` - Target node index
/// * `max_depth` - Maximum path length to search
///
/// # Returns
///
/// `Some(Vec<NodeIndex>)` if a path exists within max_depth, `None` otherwise.
/// The returned path includes both the start and end nodes.
///
/// # Example
///
/// ```ignore
/// if let Some(path) = shortest_path(&graph, from_idx, to_idx, 5) {
///     println!("Path length: {}", path.len() - 1);
///     for node_idx in path {
///         println!("  {}", node_idx.index());
///     }
/// }
/// ```
pub fn shortest_path(
    graph: &ConceptGraph,
    from: NodeIndex,
    to: NodeIndex,
    max_depth: u32,
) -> Option<Vec<NodeIndex>> {
    if from == to {
        return Some(vec![from]);
    }

    let mut parent: HashMap<NodeIndex, NodeIndex> = HashMap::new();
    let mut queue: VecDeque<(NodeIndex, u32)> = VecDeque::new();

    queue.push_back((from, 0));
    parent.insert(from, from);

    while let Some((node, depth)) = queue.pop_front() {
        if node == to {
            // Reconstruct path by following parent pointers
            let mut path = vec![to];
            let mut current = to;
            while current != from {
                current = parent[&current];
                path.push(current);
            }
            path.reverse();
            return Some(path);
        }

        if depth >= max_depth {
            continue;
        }

        for neighbor in graph.neighbors_undirected(node) {
            if !parent.contains_key(&neighbor) {
                parent.insert(neighbor, node);
                queue.push_back((neighbor, depth + 1));
            }
        }
    }

    None
}

/// Get prerequisites in learning order (topologically sorted).
///
/// Uses depth-first search to collect all prerequisite edges (incoming edges with
/// Relationship::Prerequisite). Computes the depth of each prerequisite and returns
/// them sorted by depth (concepts with no prerequisites first).
///
/// # Arguments
///
/// * `graph` - The concept graph to search
/// * `target` - The node to find prerequisites for
/// * `max_depth` - Maximum depth to trace backwards
///
/// # Returns
///
/// Vector of NodeIndex values sorted in learning order (prerequisites first).
/// Does not include the target node itself.
///
/// # Example
///
/// ```ignore
/// let prereqs = prerequisites_sorted(&graph, target_idx, 3);
/// println!("Learn these concepts first:");
/// for prereq_idx in prereqs {
///     println!("  {}", prereq_idx.index());
/// }
/// ```
pub fn prerequisites_sorted(
    graph: &ConceptGraph,
    target: NodeIndex,
    max_depth: u32,
) -> Vec<NodeIndex> {
    let mut prereqs: HashSet<NodeIndex> = HashSet::new();
    let mut stack = vec![(target, 0u32)];

    // Collect all prerequisites using DFS
    while let Some((node, depth)) = stack.pop() {
        if depth > max_depth {
            continue;
        }

        for edge in graph.edges_directed(node, Direction::Incoming) {
            if matches!(edge.weight().relationship, Relationship::Prerequisite) {
                let prereq = edge.source();
                if !prereqs.contains(&prereq) && prereq != target {
                    prereqs.insert(prereq);
                    stack.push((prereq, depth + 1));
                }
            }
        }
    }

    // Compute depth for each prerequisite (longest path to it)
    let prereq_list: Vec<NodeIndex> = prereqs.iter().copied().collect();
    let prereq_set: HashSet<NodeIndex> = prereqs;
    let mut depths: HashMap<NodeIndex, u32> = HashMap::new();

    fn compute_depth(
        graph: &ConceptGraph,
        node: NodeIndex,
        depths: &mut HashMap<NodeIndex, u32>,
        prereq_set: &HashSet<NodeIndex>,
    ) -> u32 {
        if let Some(&d) = depths.get(&node) {
            return d;
        }

        let mut max_prereq_depth = 0;
        for edge in graph.edges_directed(node, Direction::Incoming) {
            if matches!(edge.weight().relationship, Relationship::Prerequisite) {
                let prereq = edge.source();
                if prereq_set.contains(&prereq) {
                    let d = compute_depth(graph, prereq, depths, prereq_set);
                    max_prereq_depth = max_prereq_depth.max(d + 1);
                }
            }
        }

        depths.insert(node, max_prereq_depth);
        max_prereq_depth
    }

    for &node in &prereq_list {
        compute_depth(graph, node, &mut depths, &prereq_set);
    }

    // Sort by depth (concepts with no prerequisites first)
    let mut sorted = prereq_list;
    sorted.sort_by_key(|n| depths.get(n).copied().unwrap_or(0));
    sorted
}

/// Get dependents (concepts that require this one as a prerequisite).
///
/// Uses depth-first search to follow outgoing Prerequisite edges to find all
/// concepts that depend on the given node.
///
/// # Arguments
///
/// * `graph` - The concept graph to search
/// * `node` - The node to find dependents for
/// * `max_depth` - Maximum depth to trace forward
///
/// # Returns
///
/// Vector of (NodeIndex, depth) tuples for all dependent concepts.
/// Does not include the starting node itself.
///
/// # Example
///
/// ```ignore
/// let deps = dependents(&graph, node_idx, 3);
/// println!("Concepts that require this:");
/// for (dep_idx, depth) in deps {
///     println!("  {} (depth: {})", dep_idx.index(), depth);
/// }
/// ```
pub fn dependents(
    graph: &ConceptGraph,
    node: NodeIndex,
    max_depth: u32,
) -> Vec<(NodeIndex, u32)> {
    let mut result: HashMap<NodeIndex, u32> = HashMap::new();
    let mut stack = vec![(node, 0u32)];

    while let Some((current, depth)) = stack.pop() {
        if depth > max_depth {
            continue;
        }

        for edge in graph.edges_directed(current, Direction::Outgoing) {
            if matches!(edge.weight().relationship, Relationship::Prerequisite) {
                let dependent = edge.target();
                if !result.contains_key(&dependent) && dependent != node {
                    result.insert(dependent, depth + 1);
                    stack.push((dependent, depth + 1));
                }
            }
        }
    }

    result.into_iter().collect()
}

/// Calculate degree centrality for all nodes.
///
/// Degree centrality is the simplest centrality metric: the total number of edges
/// connected to a node (in-degree + out-degree). Optionally filter to only Concept
/// nodes in a specific category.
///
/// # Arguments
///
/// * `graph` - The concept graph to analyze
/// * `category_filter` - Optional category filter (only count Concept nodes in this category)
///
/// # Returns
///
/// Vector of (NodeIndex, degree) tuples sorted by degree (highest first).
///
/// # Example
///
/// ```ignore
/// let central = degree_centrality(&graph, Some("harmony"));
/// println!("Most connected harmony concepts:");
/// for (idx, degree) in central.iter().take(10) {
///     println!("  {} (degree: {})", idx.index(), degree);
/// }
/// ```
pub fn degree_centrality(
    graph: &ConceptGraph,
    category_filter: Option<&str>,
) -> Vec<(NodeIndex, u32)> {
    let mut centrality: Vec<(NodeIndex, u32)> = Vec::new();

    for idx in graph.node_indices() {
        // Apply category filter if specified
        if let Some(cat) = category_filter {
            if let Node::Concept(c) = &graph[idx] {
                if c.category != cat {
                    continue;
                }
            } else {
                continue;
            }
        }

        // Count both incoming and outgoing edges
        let in_degree = graph.edges_directed(idx, Direction::Incoming).count();
        let out_degree = graph.edges_directed(idx, Direction::Outgoing).count();
        let degree = (in_degree + out_degree) as u32;
        centrality.push((idx, degree));
    }

    // Sort by degree (highest first)
    centrality.sort_by(|a, b| b.1.cmp(&a.1));
    centrality
}

/// Find bridge concepts connecting two categories.
///
/// A bridge concept is one that has connections to concepts in both categories.
/// The bridge score is computed as: sqrt(connections_a * connections_b), which
/// favors concepts with balanced connections to both categories.
///
/// # Arguments
///
/// * `graph` - The concept graph to analyze
/// * `category_a` - First category name
/// * `category_b` - Second category name
///
/// # Returns
///
/// Vector of (NodeIndex, connections_a, connections_b, bridge_score) tuples
/// sorted by bridge score (highest first).
///
/// # Example
///
/// ```ignore
/// let bridges = bridge_concepts(&graph, "harmony", "counterpoint");
/// println!("Bridge concepts:");
/// for (idx, conn_a, conn_b, score) in bridges {
///     println!("  {} connects {} and {} (score: {})",
///              idx.index(), conn_a, conn_b, score);
/// }
/// ```
pub fn bridge_concepts(
    graph: &ConceptGraph,
    category_a: &str,
    category_b: &str,
) -> Vec<(NodeIndex, u32, u32, f32)> {
    let mut bridges: Vec<(NodeIndex, u32, u32, f32)> = Vec::new();

    for idx in graph.node_indices() {
        // Only consider Concept nodes
        let Node::Concept(_) = &graph[idx] else {
            continue;
        };

        let mut connections_a = 0u32;
        let mut connections_b = 0u32;

        // Count connections to each category
        for neighbor in graph.neighbors_undirected(idx) {
            if let Node::Concept(c) = &graph[neighbor] {
                if c.category == category_a {
                    connections_a += 1;
                } else if c.category == category_b {
                    connections_b += 1;
                }
            }
        }

        // If connected to both categories, add as bridge
        if connections_a > 0 && connections_b > 0 {
            let bridge_score = ((connections_a * connections_b) as f32).sqrt();
            bridges.push((idx, connections_a, connections_b, bridge_score));
        }
    }

    // Sort by bridge score (highest first)
    bridges.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));
    bridges
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::types::{
        ConceptNode, Edge, EdgeOrigin, GraphData, GraphMetadata,
    };
    use crate::graph::persistence::to_petgraph;

    /// Create a test graph with known structure for algorithm testing.
    ///
    /// Graph structure:
    /// ```text
    /// interval (fundamentals) → triad (harmony) → seventh-chord (harmony)
    ///                             ↓
    ///                      voice-leading (counterpoint)
    /// ```
    ///
    /// Edges:
    /// - interval → triad (Prerequisite)
    /// - triad → seventh-chord (Prerequisite)
    /// - triad → voice-leading (RelatesTo)
    fn create_test_graph() -> ConceptGraph {
        let interval = Node::Concept(ConceptNode {
            id: "interval".to_string(),
            title: "Interval".to_string(),
            category: "fundamentals".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let triad = Node::Concept(ConceptNode {
            id: "triad".to_string(),
            title: "Triad".to_string(),
            category: "harmony".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let seventh = Node::Concept(ConceptNode {
            id: "seventh-chord".to_string(),
            title: "Seventh Chord".to_string(),
            category: "harmony".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let voice_leading = Node::Concept(ConceptNode {
            id: "voice-leading".to_string(),
            title: "Voice Leading".to_string(),
            category: "counterpoint".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let edge1 = Edge {
            from: "interval".to_string(),
            to: "triad".to_string(),
            relationship: Relationship::Prerequisite,
            weight: 1.0,
            origin: EdgeOrigin::Extracted,
        };

        let edge2 = Edge {
            from: "triad".to_string(),
            to: "seventh-chord".to_string(),
            relationship: Relationship::Prerequisite,
            weight: 1.0,
            origin: EdgeOrigin::Extracted,
        };

        let edge3 = Edge {
            from: "triad".to_string(),
            to: "voice-leading".to_string(),
            relationship: Relationship::RelatesTo,
            weight: 0.7,
            origin: EdgeOrigin::Extracted,
        };

        let graph_data = GraphData {
            version: "1.0".to_string(),
            nodes: vec![interval, triad, seventh, voice_leading],
            edges: vec![edge1, edge2, edge3],
            metadata: Some(GraphMetadata {
                built_at: "2024-01-01T00:00:00Z".to_string(),
                source_cards_count: 4,
                build_duration_ms: 100,
            }),
        };

        to_petgraph(&graph_data)
    }

    #[test]
    fn test_shortest_path_exists() {
        let graph = create_test_graph();

        // Get node indices
        let mut indices = Vec::new();
        for idx in graph.node_indices() {
            if let Node::Concept(c) = &graph[idx] {
                indices.push((c.id.clone(), idx));
            }
        }
        let nodes: HashMap<String, NodeIndex> = indices.into_iter().collect();

        let from = nodes["interval"];
        let to = nodes["seventh-chord"];

        let path = shortest_path(&graph, from, to, 5);

        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 3); // interval → triad → seventh-chord
        assert_eq!(path[0], from);
        assert_eq!(path[2], to);
    }

    #[test]
    fn test_shortest_path_not_found() {
        let graph = create_test_graph();

        // Create isolated node
        let from = NodeIndex::new(0);
        let to = NodeIndex::new(100); // Non-existent node

        let path = shortest_path(&graph, from, to, 5);
        assert!(path.is_none());
    }

    #[test]
    fn test_prerequisites_sorted() {
        let graph = create_test_graph();

        // Get node indices
        let mut indices = Vec::new();
        for idx in graph.node_indices() {
            if let Node::Concept(c) = &graph[idx] {
                indices.push((c.id.clone(), idx));
            }
        }
        let nodes: HashMap<String, NodeIndex> = indices.into_iter().collect();

        let target = nodes["seventh-chord"];
        let prereqs = prerequisites_sorted(&graph, target, 3);

        // Should find: interval, triad (in that order)
        assert_eq!(prereqs.len(), 2);

        // interval has no prerequisites, should come first
        if let Node::Concept(c) = &graph[prereqs[0]] {
            assert_eq!(c.id, "interval");
        } else {
            panic!("Expected Concept node");
        }

        // triad depends on interval, should come second
        if let Node::Concept(c) = &graph[prereqs[1]] {
            assert_eq!(c.id, "triad");
        } else {
            panic!("Expected Concept node");
        }
    }

    #[test]
    fn test_dependents() {
        let graph = create_test_graph();

        // Get node indices
        let mut indices = Vec::new();
        for idx in graph.node_indices() {
            if let Node::Concept(c) = &graph[idx] {
                indices.push((c.id.clone(), idx));
            }
        }
        let nodes: HashMap<String, NodeIndex> = indices.into_iter().collect();

        let node = nodes["triad"];
        let deps = dependents(&graph, node, 3);

        // Should find seventh-chord (triad is prerequisite for it)
        assert_eq!(deps.len(), 1);
        let (dep_idx, depth) = deps[0];
        assert_eq!(depth, 1);

        if let Node::Concept(c) = &graph[dep_idx] {
            assert_eq!(c.id, "seventh-chord");
        } else {
            panic!("Expected Concept node");
        }
    }

    #[test]
    fn test_degree_centrality() {
        let graph = create_test_graph();

        let centrality = degree_centrality(&graph, None);

        // triad has the most connections (3 edges: interval→triad, triad→seventh, triad→voice)
        let (most_central_idx, degree) = centrality[0];
        assert_eq!(degree, 3);

        if let Node::Concept(c) = &graph[most_central_idx] {
            assert_eq!(c.id, "triad");
        } else {
            panic!("Expected Concept node");
        }
    }

    #[test]
    fn test_bridge_concepts() {
        let graph = create_test_graph();

        let bridges = bridge_concepts(&graph, "harmony", "counterpoint");

        // triad is the bridge (connects harmony and counterpoint via voice-leading)
        assert_eq!(bridges.len(), 1);
        let (bridge_idx, conn_harm, conn_count, _score) = bridges[0];

        if let Node::Concept(c) = &graph[bridge_idx] {
            assert_eq!(c.id, "triad");
        } else {
            panic!("Expected Concept node");
        }

        // triad connects to 1 harmony concept (seventh-chord)
        // and 1 counterpoint concept (voice-leading)
        assert_eq!(conn_harm, 1);
        assert_eq!(conn_count, 1);
    }
}
