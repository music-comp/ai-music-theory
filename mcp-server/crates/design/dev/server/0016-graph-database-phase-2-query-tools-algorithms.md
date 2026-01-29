# Graph Database Phase 2 - Query Tools & Algorithms

## Overview

Implement graph traversal algorithms and 10 new MCP query tools that enable Claude to explore concept relationships, find learning paths, and discover patterns during conversations.

**Builds on:** Phase 1 (graph infrastructure, loading, basic inspection)
**Design Document:** `crates/design/docs/02-under-review/0008-graph-database-phase-2-query-tools-algorithms.md`

## Architecture

### Module Structure

```
crates/server/src/graph/
├── algorithms.rs         # NEW - Core traversal algorithms
└── query.rs              # NEW - MCP tool response types

crates/server/src/tools/
└── graph_query.rs        # NEW - MCP tool implementations
```

### Key Algorithms to Implement

| Algorithm | Function | Use Case |
|-----------|----------|----------|
| BFS neighborhood | `neighborhood()` | N-hop exploration |
| Shortest path | `shortest_path()` | Find connections |
| Prerequisites | `prerequisites_sorted()` | Learning order (topological) |
| Dependents | `dependents()` | What builds on this? |
| Centrality | `degree_centrality()` | Most connected concepts |
| Bridge finder | `bridge_concepts()` | Cross-category connectors |

## Implementation Phases

### Phase 1: Algorithm Module (3-4 hours)

**File to create:** `crates/server/src/graph/algorithms.rs`

**Functions to implement:**

1. **`neighborhood(graph, start, max_depth, max_nodes)`**
   - BFS traversal collecting nodes within N hops
   - Returns `Vec<(NodeIndex, u32)>` with distance
   - Respects max_nodes limit to prevent huge results
   - Uses HashMap to track visited nodes

2. **`shortest_path(graph, from, to, max_depth)`**
   - BFS to find shortest undirected path
   - Returns `Option<Vec<NodeIndex>>` or None if no path
   - Tracks parent pointers to reconstruct path
   - Stops after max_depth to prevent infinite search

3. **`prerequisites_sorted(graph, target, max_depth)`**
   - DFS to collect all prerequisite edges (incoming)
   - Compute depth for each prerequisite recursively
   - Return topologically sorted (depth order)
   - Filters only Relationship::Prerequisite edges

4. **`dependents(graph, node, max_depth)`**
   - DFS following outgoing Prerequisite edges
   - Returns `Vec<(NodeIndex, u32)>` with distance
   - Finds concepts that require this as prerequisite

5. **`degree_centrality(graph, category_filter)`**
   - Count total edges for each node
   - Optional category filter for Concept nodes
   - Returns `Vec<(NodeIndex, u32)>` sorted by degree
   - Simple metric: sum of in-degree + out-degree

6. **`bridge_concepts(graph, category_a, category_b)`**
   - Find concepts with connections to both categories
   - Count connections to each category
   - Compute bridge score: sqrt(connections_a * connections_b)
   - Returns `Vec<(NodeIndex, u32, u32, f32)>` sorted by score

**Tests (6 tests):**

- Build small test graph with known structure
- Test shortest_path finds correct path length
- Test prerequisites_sorted returns correct order
- Test dependents finds forward dependencies
- Test degree_centrality identifies most connected
- Test bridge_concepts with two categories

**Integration with Phase 1:**

- Uses `ConceptGraph` type from persistence.rs
- Uses `Node`, `Edge`, `Relationship` from types.rs
- Operates on petgraph DiGraph API
- Pure functions, no state mutation

**Milestone:** All algorithms working with comprehensive tests

### Phase 2: Query Response Types (1-2 hours)

**File to create:** `crates/server/src/graph/query.rs`

**Response types to define (all must implement Serialize + Deserialize):**

1. **RelatedConceptsResponse** - Related concepts with filtering
2. **ConceptPathResponse** - Shortest path between concepts
3. **PrerequisitesResponse** - Prerequisites in learning order
4. **NeighborhoodResponse** - Local subgraph around concept
5. **DependentsResponse** - Concepts that depend on this
6. **CentralConceptsResponse** - Most connected concepts
7. **ConceptSourcesResponse** - Sources covering a concept
8. **ConceptVariantsResponse** - Source-specific variants
9. **BridgeConceptsResponse** - Cross-category connectors
10. **SourceCoverageResponse** - What a source covers

Each response type includes:

- Top-level metadata (counts, IDs, titles)
- List of result structs with node details (id, title, category)
- Additional fields specific to query type (distance, depth, score, etc.)

**Tests (10 serialization tests):**

- One test per response type verifying JSON structure
- Verify all fields present in serialized output
- Test with sample data

**Milestone:** All response types defined with serialization tests

### Phase 3: Must-Have Query Tools (5-6 hours)

**File to create:** `crates/server/src/tools/graph_query.rs`

**4 Must-Have Tools:**

1. **`get_related_concepts`**
   - Parameters: concept_id, relationship_types (optional CSV), direction (optional), depth (default 1, max 3)
   - Algorithm: BFS collecting edges matching filters
   - Response: RelatedConceptsResponse
   - Validation: depth ≤ 3, concept exists

2. **`find_concept_path`**
   - Parameters: from_id, to_id, max_depth (default 5, max 8)
   - Algorithm: `shortest_path()` from algorithms.rs
   - Response: ConceptPathResponse with node/edge lists
   - Validation: max_depth ≤ 8, both nodes exist

3. **`get_prerequisites`**
   - Parameters: concept_id, depth (default 3, max 5)
   - Algorithm: `prerequisites_sorted()` from algorithms.rs
   - Response: PrerequisitesResponse with learning order
   - Validation: depth ≤ 5, concept exists

4. **`get_concept_neighborhood`**
   - Parameters: concept_id, radius (default 2, max 3), max_nodes (default 30, max 50)
   - Algorithm: `neighborhood()` from algorithms.rs, then collect edges
   - Response: NeighborhoodResponse with subgraph
   - Validation: radius ≤ 3, max_nodes ≤ 50

**Common patterns:**

- All tools check GraphState::Loaded
- Use loaded.node_index for fast ID lookups
- Convert NodeIndex back to ID for responses
- Extract node details (title, category) for responses
- Handle "not found" errors gracefully

**Tests (8 tests):**

- 4 unit tests: one per tool with mock graph
- 4 integration tests: one per tool with actual data

**Milestone:** Core query tools working and tested

### Phase 4: Should-Have Query Tools (4-5 hours)

**Add to `crates/server/src/tools/graph_query.rs`:**

1. **`get_dependents`**
   - Parameters: concept_id, depth (default 2, max 4)
   - Algorithm: `dependents()` from algorithms.rs
   - Response: DependentsResponse
   - Validation: depth ≤ 4

2. **`get_central_concepts`**
   - Parameters: category (optional), limit (default 10, max 25)
   - Algorithm: `degree_centrality()` from algorithms.rs
   - Response: CentralConceptsResponse
   - Validation: limit ≤ 25

3. **`get_concept_sources`**
   - Parameters: concept_id
   - Algorithm: Find all edges from Source nodes to this concept
   - Filter by Relationship::Introduces or Relationship::Covers
   - Response: ConceptSourcesResponse
   - Validation: concept exists

4. **`get_concept_variants`**
   - Parameters: canonical_id
   - Algorithm: Find all Concept nodes where canonical_id matches
   - Response: ConceptVariantsResponse
   - Validation: canonical concept exists

**Tests (8 tests):**

- 4 unit tests: one per tool
- 4 integration tests: one per tool

**Milestone:** All should-have tools working

### Phase 5: Nice-to-Have Query Tools (3-4 hours)

**Add to `crates/server/src/tools/graph_query.rs`:**

1. **`find_bridge_concepts`**
   - Parameters: category_a, category_b, limit (default 5, max 15)
   - Algorithm: `bridge_concepts()` from algorithms.rs
   - Response: BridgeConceptsResponse
   - Validation: limit ≤ 15, categories exist

2. **`get_source_coverage`**
    - Parameters: source_id
    - Algorithm: Find all outgoing edges from source node
    - Group by Relationship::Introduces vs Covers
    - Response: SourceCoverageResponse
    - Validation: source exists

**Tests (4 tests):**

- 2 unit tests: one per tool
- 2 integration tests: one per tool

**Milestone:** All 10 tools implemented

### Phase 6: MCP Server Integration (2 hours)

**Files to modify:**

- `crates/server/src/tools/mod.rs` - Add graph_query module
- `crates/server/src/server.rs` - Register 10 new tools
- `crates/server/src/graph/mod.rs` - Export algorithms and query modules

**Parameter types for server.rs:**

```rust
// Must-have tools
pub struct GetRelatedConceptsParams { ... }
pub struct FindConceptPathParams { ... }
pub struct GetPrerequisitesParams { ... }
pub struct GetConceptNeighborhoodParams { ... }

// Should-have tools
pub struct GetDependentsParams { ... }
pub struct GetCentralConceptsParams { ... }
pub struct GetConceptSourcesParams { ... }
pub struct GetConceptVariantsParams { ... }

// Nice-to-have tools
pub struct FindBridgeConceptsParams { ... }
pub struct GetSourceCoverageParams { ... }
```

**Tool registration:**

- Follow existing pattern from graph tools (Phase 1)
- Each tool gets `#[tool(description = "...")]` attribute
- Handle Parameters<T> unwrapping
- Map errors with `.to_mcp_error(context)`
- Return `CallToolResult::success(vec![Content::text(json)])`

**Feature gating:**

- All query tools always compiled (like Phase 1 tools)
- Return "feature not enabled" errors when graph disabled
- Ensures tool_router macro works without feature

**Tests:**

- Verify all 10 tools register correctly
- Verify tool descriptions are clear
- Verify parameter schemas work with inspector

**Milestone:** All tools accessible via MCP inspector

### Phase 7: Documentation & Polish (1-2 hours)

**Tasks:**

1. Add comprehensive doc comments to all public functions
2. Add module-level docs to algorithms.rs and query.rs
3. Verify all tests pass (≥95% coverage target)
4. Run clippy and fix any warnings
5. Run cargo fmt
6. Update tool descriptions for clarity

**Documentation checklist:**

- [ ] algorithms.rs has module docs explaining each algorithm
- [ ] All public functions have doc comments with examples
- [ ] query.rs has module docs explaining response types
- [ ] All response types have field documentation
- [ ] Tool descriptions are clear and concise

**Quality checklist:**

- [ ] cargo test --features graph --lib passes
- [ ] cargo test --lib passes without graph feature
- [ ] cargo clippy --features graph --lib clean
- [ ] cargo fmt applied
- [ ] No compiler warnings
- [ ] Test coverage ≥95%

**Milestone:** Code quality standards met

## Critical Implementation Details

### Algorithm Design Decisions

**BFS vs DFS:**

- Use BFS for shortest paths (breadth-first finds shortest)
- Use DFS for prerequisite chains (depth-first follows dependencies)
- Both respect max_depth to prevent infinite recursion

**Undirected vs Directed:**

- `neighborhood()` and `shortest_path()` use undirected (both directions)
- `prerequisites_sorted()` and `dependents()` use directed (specific edge direction)
- petgraph provides both: `neighbors_undirected()` and `edges_directed()`

**Performance Limits:**

- All tools have max_depth/max_nodes limits
- Prevents expensive queries on large graphs
- Design doc specifies limits (e.g., max_depth=8, max_nodes=50)

### Response Building Pattern

For each tool:

1. Validate parameters (depth limits, node exists)
2. Get LoadedGraph from AppState
3. Lookup NodeIndex using loaded.node_index
4. Call algorithm function
5. Convert NodeIndex results back to IDs
6. Extract node details (title, category) from graph
7. Build response struct
8. Serialize to JSON

### Relationship Filtering

**For `get_related_concepts`:**

- Parse `relationship_types` CSV: "prerequisite,relates_to,extends"
- Map to `HashSet<Relationship>` for fast lookup
- Filter edges: `filter(|e| relationship_set.contains(&e.weight().relationship))`

**Direction filtering:**

- "incoming": Only edges where this node is target
- "outgoing": Only edges where this node is source
- "both" (default): All edges connected to node

### Error Handling

**Common errors:**

- Graph not loaded → "Graph not loaded yet"
- Node not found → "Node not found: {id}"
- Invalid depth → "Depth exceeds maximum of {max}"
- Path not found → `found: false` in response (not an error)

**Error propagation:**

- Use `?` operator for Result propagation
- Convert to MCP errors with `.to_mcp_error(context)`
- Provide helpful context in error messages

## Critical Files

### Files to Create

1. `/Users/oubiwann/lab/music-comp/ai-music-theory/mcp-server/crates/server/src/graph/algorithms.rs`
2. `/Users/oubiwann/lab/music-comp/ai-music-theory/mcp-server/crates/server/src/graph/query.rs`
3. `/Users/oubiwann/lab/music-comp/ai-music-theory/mcp-server/crates/server/src/tools/graph_query.rs`

### Files to Modify

1. `/Users/oubiwann/lab/music-comp/ai-music-theory/mcp-server/crates/server/src/graph/mod.rs` - Add algorithms and query modules
2. `/Users/oubiwann/lab/music-comp/ai-music-theory/mcp-server/crates/server/src/tools/mod.rs` - Add graph_query module
3. `/Users/oubiwann/lab/music-comp/ai-music-theory/mcp-server/crates/server/src/server.rs` - Register 10 new tools with parameters

## Testing Strategy

### Unit Tests (30+ tests)

**algorithms.rs tests (6):**

- Build small test graph with known structure (4-5 nodes)
- Test each algorithm function independently
- Verify correct results on known input
- Test edge cases (no path, empty neighborhood, etc.)

**query.rs tests (10):**

- One serialization test per response type
- Verify JSON structure matches expectations
- Test with realistic sample data

**graph_query.rs tests (14+):**

- One unit test per tool (10 tools)
- Mock graph with controlled structure
- Verify response format and data
- Additional tests for filtering/validation (4+ tests)

### Integration Tests (10)

**Tool integration tests:**

- One test per tool using real graph data
- Test with actual concept IDs from test fixtures
- Verify end-to-end: parameters → algorithm → response
- Test error cases (not found, invalid depth, etc.)

## Verification Checklist

After implementation:

**Build & Test:**

- [ ] `cargo test --features graph --lib` - All tests pass
- [ ] `cargo test --lib` - Tests pass without graph feature
- [ ] `cargo clippy --features graph --lib` - No warnings
- [ ] `cargo fmt --check` - Code formatted
- [ ] Test coverage ≥95%

**Functional Verification:**

- [ ] Build test graph: `cargo run --features graph --bin music-theory-mcp graph build`
- [ ] Start server: `cargo run --features graph --bin music-theory-mcp serve`
- [ ] Test via inspector: `npx @modelcontextprotocol/inspector target/debug/music-theory-mcp`

**MCP Tool Testing:**

1. **get_related_concepts** - Find relationships for a concept
2. **find_concept_path** - Path between two concepts
3. **get_prerequisites** - Prerequisites for a concept
4. **get_concept_neighborhood** - N-hop neighborhood
5. **get_dependents** - What requires this concept?
6. **get_central_concepts** - Most connected concepts
7. **get_concept_sources** - Sources covering concept
8. **get_concept_variants** - Source-specific variants
9. **find_bridge_concepts** - Cross-category bridges
10. **get_source_coverage** - What does source cover?

**Performance:**

- [ ] All queries complete in < 100ms on typical graph (200 nodes)
- [ ] No queries hang or timeout
- [ ] Memory usage reasonable (< 100MB)

## Success Criteria

### Must Have (Phase 2 Complete)

- [ ] 6 core algorithms implemented and tested
- [ ] 4 must-have tools working: related, path, prerequisites, neighborhood
- [ ] All tools validate parameters (depth limits, node existence)
- [ ] All tools handle "graph not loaded" gracefully
- [ ] All tools return well-structured JSON responses
- [ ] Integration with MCP inspector successful

### Should Have

- [ ] 4 should-have tools working: dependents, central, sources, variants
- [ ] All 10 tools accessible via MCP
- [ ] Comprehensive test coverage (≥95%)
- [ ] Clean clippy and fmt

### Nice to Have

- [ ] 2 nice-to-have tools working: bridge_concepts, source_coverage
- [ ] Performance < 100ms for typical queries
- [ ] Clear, helpful error messages

## Estimated Timeline

- **Phase 1** (Algorithms): 3-4 hours
- **Phase 2** (Response Types): 1-2 hours
- **Phase 3** (Must-Have Tools): 5-6 hours
- **Phase 4** (Should-Have Tools): 4-5 hours
- **Phase 5** (Nice-to-Have Tools): 3-4 hours
- **Phase 6** (Server Integration): 2 hours
- **Phase 7** (Polish): 1-2 hours

**Total: 19-27 hours of focused implementation**

## Dependencies

All dependencies already satisfied from Phase 1:

- petgraph 0.6 - Graph algorithms
- serde/serde_json - Serialization
- tokio - Async runtime

No new dependencies needed.
