---
number: 8
title: "Graph Database Phase 2 - Query Tools & Algorithms"
author: "String,"
component: All
tags: [change-me]
created: 2026-01-27
updated: 2026-01-27
state: Under Review
supersedes: null
superseded-by: null
version: 1.0
---

# Graph Database Phase 2 - Query Tools & Algorithms

## Overview

This phase builds on the graph infrastructure from Phase 1 to add query tools that Claude can use in conversations. These tools enable traversing relationships, finding paths, exploring neighborhoods, and discovering patterns in the concept graph.

**Depends on**: Phase 1 complete (graph loading, data model, basic inspection tools)

**Binary name**: `music-theory-mcp`

---

## Goals

1. Implement graph traversal algorithms (BFS, Dijkstra, etc.)
2. Add conversational query tools for exploring relationships
3. Support the hybrid canonical/source-specific model
4. Enable learning path generation (topological sort)
5. Provide centrality/importance metrics

---

## Algorithm Implementations

### Core Algorithms Needed

| Algorithm | Use Case | petgraph Support |
|-----------|----------|------------------|
| BFS | Neighborhood exploration, N-hop queries | `Bfs` iterator |
| Dijkstra | Shortest path between concepts | `dijkstra()` |
| DFS | Prerequisite chains | `Dfs` iterator |
| Topological Sort | Learning order | `toposort()` |
| Connected Components | Graph health | `connected_components()` |
| Degree Centrality | Most connected concepts | Manual (count edges) |

### Algorithm Wrapper Module

```rust
// src/graph/algorithms.rs

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{Bfs, Dfs, EdgeRef};
use petgraph::algo::{dijkstra, toposort};
use petgraph::Direction;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph::{Node, Edge, Relationship, ConceptGraph};

/// Find all nodes within N hops of a starting node
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

    visited.into_iter().collect()
}

/// Find shortest path between two nodes
pub fn shortest_path(
    graph: &ConceptGraph,
    from: NodeIndex,
    to: NodeIndex,
    max_depth: u32,
) -> Option<Vec<NodeIndex>> {
    let mut visited: HashMap<NodeIndex, NodeIndex> = HashMap::new();
    let mut queue: VecDeque<(NodeIndex, u32)> = VecDeque::new();

    queue.push_back((from, 0));
    visited.insert(from, from);

    while let Some((node, depth)) = queue.pop_front() {
        if node == to {
            let mut path = vec![to];
            let mut current = to;
            while current != from {
                current = visited[&current];
                path.push(current);
            }
            path.reverse();
            return Some(path);
        }

        if depth >= max_depth {
            continue;
        }

        for neighbor in graph.neighbors_undirected(node) {
            if !visited.contains_key(&neighbor) {
                visited.insert(neighbor, node);
                queue.push_back((neighbor, depth + 1));
            }
        }
    }

    None
}

/// Get prerequisites in learning order (topological sort)
pub fn prerequisites_sorted(
    graph: &ConceptGraph,
    target: NodeIndex,
    max_depth: u32,
) -> Vec<NodeIndex> {
    let mut prereqs: HashSet<NodeIndex> = HashSet::new();
    let mut stack = vec![(target, 0u32)];

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

    let prereq_list: Vec<NodeIndex> = prereqs.into_iter().collect();
    let mut depths: HashMap<NodeIndex, u32> = HashMap::new();
    let prereq_set: HashSet<NodeIndex> = prereq_list.iter().cloned().collect();

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

    let mut sorted = prereq_list;
    sorted.sort_by_key(|n| depths.get(n).unwrap_or(&0));
    sorted
}

/// Get dependents (concepts that require this one)
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

/// Calculate degree centrality
pub fn degree_centrality(
    graph: &ConceptGraph,
    category_filter: Option<&str>,
) -> Vec<(NodeIndex, u32)> {
    let mut centrality: Vec<(NodeIndex, u32)> = Vec::new();

    for idx in graph.node_indices() {
        if let Some(cat) = category_filter {
            if let Node::Concept(c) = &graph[idx] {
                if c.category != cat {
                    continue;
                }
            } else {
                continue;
            }
        }

        let degree = graph.edges(idx).count() as u32;
        centrality.push((idx, degree));
    }

    centrality.sort_by(|a, b| b.1.cmp(&a.1));
    centrality
}

/// Find bridge concepts between two categories
pub fn bridge_concepts(
    graph: &ConceptGraph,
    category_a: &str,
    category_b: &str,
) -> Vec<(NodeIndex, u32, u32, f32)> {
    let mut bridges: Vec<(NodeIndex, u32, u32, f32)> = Vec::new();

    for idx in graph.node_indices() {
        let Node::Concept(_) = &graph[idx] else { continue };

        let mut connections_a = 0u32;
        let mut connections_b = 0u32;

        for neighbor in graph.neighbors_undirected(idx) {
            if let Node::Concept(c) = &graph[neighbor] {
                if c.category == category_a {
                    connections_a += 1;
                } else if c.category == category_b {
                    connections_b += 1;
                }
            }
        }

        if connections_a > 0 && connections_b > 0 {
            let bridge_score = ((connections_a * connections_b) as f32).sqrt();
            bridges.push((idx, connections_a, connections_b, bridge_score));
        }
    }

    bridges.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));
    bridges
}
```

---

## MCP Tools

### Tool: `get_related_concepts`

```rust
/// Find concepts related to a given concept
#[tool(
    name = "get_related_concepts",
    description = "Find concepts related to a given concept, with optional filtering by relationship type and direction"
)]
pub async fn get_related_concepts(
    state: Arc<AppState>,
    #[arg(description = "Concept ID to find relationships for")]
    concept_id: String,
    #[arg(description = "Filter by relationship types (comma-separated): relates_to, prerequisite, extends, same_as")]
    relationship_types: Option<String>,
    #[arg(description = "Filter by direction: incoming, outgoing, or both (default: both)")]
    direction: Option<String>,
    #[arg(description = "How many hops to traverse (default: 1, max: 3)")]
    depth: Option<u32>,
) -> Result<RelatedConceptsResponse, McpError>;

#[derive(Serialize)]
pub struct RelatedConceptsResponse {
    concept_id: String,
    depth: u32,
    total: u32,
    related: Vec<RelatedConcept>,
}

#[derive(Serialize)]
pub struct RelatedConcept {
    id: String,
    title: String,
    category: String,
    relationship: Relationship,
    direction: String,
    weight: f32,
    distance: u32,
}
```

### Tool: `find_concept_path`

```rust
/// Find paths between two concepts
#[tool(
    name = "find_concept_path",
    description = "Find the shortest path between two concepts, showing how they connect"
)]
pub async fn find_concept_path(
    state: Arc<AppState>,
    #[arg(description = "Starting concept ID")]
    from_id: String,
    #[arg(description = "Target concept ID")]
    to_id: String,
    #[arg(description = "Maximum path length (default: 5, max: 8)")]
    max_depth: Option<u32>,
) -> Result<ConceptPathResponse, McpError>;

#[derive(Serialize)]
pub struct ConceptPathResponse {
    from: String,
    to: String,
    found: bool,
    paths: Vec<ConceptPath>,
}

#[derive(Serialize)]
pub struct ConceptPath {
    length: u32,
    nodes: Vec<PathNode>,
    edges: Vec<PathEdge>,
}
```

### Tool: `get_prerequisites`

```rust
/// Get prerequisite concepts in learning order
#[tool(
    name = "get_prerequisites",
    description = "Get all prerequisites for a concept, sorted in recommended learning order"
)]
pub async fn get_prerequisites(
    state: Arc<AppState>,
    #[arg(description = "Concept ID to find prerequisites for")]
    concept_id: String,
    #[arg(description = "How far back to trace (default: 3, max: 5)")]
    depth: Option<u32>,
) -> Result<PrerequisitesResponse, McpError>;

#[derive(Serialize)]
pub struct PrerequisitesResponse {
    concept_id: String,
    concept_title: String,
    total: u32,
    prerequisites: Vec<PrerequisiteConcept>,
    learning_order: Vec<String>,  // IDs in order to learn
}
```

### Tool: `get_concept_neighborhood`

```rust
/// Get the neighborhood subgraph around a concept
#[tool(
    name = "get_concept_neighborhood",
    description = "Get all concepts within N hops of a concept"
)]
pub async fn get_concept_neighborhood(
    state: Arc<AppState>,
    #[arg(description = "Center concept ID")]
    concept_id: String,
    #[arg(description = "How many hops out (default: 2, max: 3)")]
    radius: Option<u32>,
    #[arg(description = "Maximum nodes to return (default: 30, max: 50)")]
    max_nodes: Option<u32>,
) -> Result<NeighborhoodResponse, McpError>;

#[derive(Serialize)]
pub struct NeighborhoodResponse {
    center: String,
    radius: u32,
    node_count: u32,
    edge_count: u32,
    nodes: Vec<NeighborhoodNode>,
    edges: Vec<NeighborhoodEdge>,
}
```

### Tool: `get_dependents`

```rust
/// Get concepts that depend on this one
#[tool(
    name = "get_dependents",
    description = "Find concepts that require this concept as a prerequisite"
)]
pub async fn get_dependents(
    state: Arc<AppState>,
    #[arg(description = "Concept ID")]
    concept_id: String,
    #[arg(description = "How many levels forward (default: 2, max: 4)")]
    depth: Option<u32>,
) -> Result<DependentsResponse, McpError>;

#[derive(Serialize)]
pub struct DependentsResponse {
    concept_id: String,
    concept_title: String,
    total: u32,
    dependents: Vec<DependentConcept>,
}
```

### Tool: `get_central_concepts`

```rust
/// Get the most central/connected concepts
#[tool(
    name = "get_central_concepts",
    description = "Find the most connected concepts, optionally filtered by category"
)]
pub async fn get_central_concepts(
    state: Arc<AppState>,
    #[arg(description = "Filter by category")]
    category: Option<String>,
    #[arg(description = "Number of results (default: 10, max: 25)")]
    limit: Option<u32>,
) -> Result<CentralConceptsResponse, McpError>;

#[derive(Serialize)]
pub struct CentralConceptsResponse {
    category: Option<String>,
    total: u32,
    concepts: Vec<CentralConcept>,
}

#[derive(Serialize)]
pub struct CentralConcept {
    id: String,
    title: String,
    category: String,
    connections: u32,
}
```

### Tool: `get_concept_sources`

```rust
/// Get sources that cover a concept
#[tool(
    name = "get_concept_sources",
    description = "Find which sources introduce or cover a concept"
)]
pub async fn get_concept_sources(
    state: Arc<AppState>,
    #[arg(description = "Concept ID")]
    concept_id: String,
) -> Result<ConceptSourcesResponse, McpError>;

#[derive(Serialize)]
pub struct ConceptSourcesResponse {
    concept_id: String,
    concept_title: String,
    total: u32,
    sources: Vec<SourceCoverage>,
}
```

### Tool: `get_concept_variants`

```rust
/// Get source-specific variants of a canonical concept
#[tool(
    name = "get_concept_variants",
    description = "Get all source-specific variants of a canonical concept"
)]
pub async fn get_concept_variants(
    state: Arc<AppState>,
    #[arg(description = "Canonical concept ID")]
    canonical_id: String,
) -> Result<ConceptVariantsResponse, McpError>;

#[derive(Serialize)]
pub struct ConceptVariantsResponse {
    canonical_id: String,
    canonical_title: String,
    total: u32,
    variants: Vec<ConceptVariant>,
}
```

### Tool: `find_bridge_concepts`

```rust
/// Find concepts that bridge two categories
#[tool(
    name = "find_bridge_concepts",
    description = "Find concepts that connect two categories"
)]
pub async fn find_bridge_concepts(
    state: Arc<AppState>,
    #[arg(description = "First category")]
    category_a: String,
    #[arg(description = "Second category")]
    category_b: String,
    #[arg(description = "Number of results (default: 5, max: 15)")]
    limit: Option<u32>,
) -> Result<BridgeConceptsResponse, McpError>;

#[derive(Serialize)]
pub struct BridgeConceptsResponse {
    category_a: String,
    category_b: String,
    total: u32,
    bridges: Vec<BridgeConcept>,
}

#[derive(Serialize)]
pub struct BridgeConcept {
    id: String,
    title: String,
    category: String,
    connections_to_a: u32,
    connections_to_b: u32,
    bridge_score: f32,
}
```

### Tool: `get_source_coverage`

```rust
/// Get all concepts covered by a source
#[tool(
    name = "get_source_coverage",
    description = "Get all concepts that a source introduces or covers"
)]
pub async fn get_source_coverage(
    state: Arc<AppState>,
    #[arg(description = "Source ID")]
    source_id: String,
) -> Result<SourceCoverageResponse, McpError>;

#[derive(Serialize)]
pub struct SourceCoverageResponse {
    source_id: String,
    source_title: String,
    source_author: String,
    total_concepts: u32,
    introduces_count: u32,
    covers_count: u32,
    introduces: Vec<ConceptBrief>,
    covers: Vec<ConceptBrief>,
}

#[derive(Serialize)]
pub struct ConceptBrief {
    id: String,
    title: String,
    category: String,
}
```

---

## Tool Summary

| Priority | Tool | Description |
|----------|------|-------------|
| **Must** | `get_related_concepts` | Find related concepts with filtering |
| **Must** | `find_concept_path` | Find shortest path between concepts |
| **Must** | `get_prerequisites` | Get prerequisites in learning order |
| **Must** | `get_concept_neighborhood` | Get local subgraph |
| **Should** | `get_dependents` | Find concepts that build on this |
| **Should** | `get_central_concepts` | Find most connected concepts |
| **Should** | `get_concept_sources` | Which sources cover this concept? |
| **Should** | `get_concept_variants` | Source-specific variants |
| **Nice** | `find_bridge_concepts` | Concepts connecting two categories |
| **Nice** | `get_source_coverage` | What does a source cover? |

---

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_graph() -> ConceptGraph {
        // Build a small test graph with known structure
        let mut graph = DiGraph::new();

        let interval = graph.add_node(concept_node("interval", "fundamentals"));
        let triad = graph.add_node(concept_node("triad", "harmony"));
        let seventh = graph.add_node(concept_node("seventh-chord", "harmony"));
        let voice_leading = graph.add_node(concept_node("voice-leading", "counterpoint"));

        graph.add_edge(interval, triad, prereq_edge());
        graph.add_edge(triad, seventh, prereq_edge());
        graph.add_edge(triad, voice_leading, relates_edge());

        graph
    }

    #[test]
    fn test_shortest_path() {
        let graph = build_test_graph();
        let path = algorithms::shortest_path(&graph, idx(0), idx(2), 5);

        assert!(path.is_some());
        assert_eq!(path.unwrap().len(), 3); // interval -> triad -> seventh
    }

    #[test]
    fn test_prerequisites_sorted() {
        let graph = build_test_graph();
        let prereqs = algorithms::prerequisites_sorted(&graph, idx(2), 3);

        assert_eq!(prereqs.len(), 2);
        // interval should come before triad
        assert_eq!(prereqs[0], idx(0));
        assert_eq!(prereqs[1], idx(1));
    }

    #[test]
    fn test_degree_centrality() {
        let graph = build_test_graph();
        let centrality = algorithms::degree_centrality(&graph, None);

        // triad has most connections
        assert_eq!(centrality[0].0, idx(1));
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_get_related_concepts_tool() {
    let state = setup_test_state().await;

    let response = get_related_concepts(
        state, "suspension".into(), None, None, Some(1)
    ).await.unwrap();

    assert!(!response.related.is_empty());
}

#[tokio::test]
async fn test_find_path_tool() {
    let state = setup_test_state().await;

    let response = find_concept_path(
        state, "interval".into(), "fugue".into(), Some(6)
    ).await.unwrap();

    assert!(response.found);
}
```

---

## Success Criteria

### Must Have (Phase 2 Complete)

- [ ] `get_related_concepts` works with depth and direction filtering
- [ ] `find_concept_path` finds shortest paths between concepts
- [ ] `get_prerequisites` returns topologically sorted learning order
- [ ] `get_concept_neighborhood` returns local subgraph with edges
- [ ] All tools handle "graph not loaded" gracefully
- [ ] All tools validate input parameters

### Should Have

- [ ] `get_dependents` traces forward through prerequisite chains
- [ ] `get_central_concepts` ranks by degree with category filter
- [ ] `get_concept_sources` finds which sources cover a concept
- [ ] `get_concept_variants` supports hybrid model queries

### Nice to Have

- [ ] `find_bridge_concepts` identifies cross-category connectors
- [ ] `get_source_coverage` shows what a source covers
- [ ] All queries complete in < 100ms

---

## Future Enhancements (Out of Scope)

1. Weighted pathfinding using edge weights
2. Multiple path finding (top-N paths)
3. Betweenness centrality metric
4. GraphViz DOT export for visualization
5. Incremental graph updates without full reload
6. Query result caching
