# Graph Database Phase 1 - Implementation Plan

## Overview

Implement a feature-gated graph database system (`--features graph`) that extracts concept relationships from markdown files, builds a graph structure, persists it with efficient caching, and provides CLI commands and MCP tools for querying.

**Design Document:** `crates/design/docs/02-under-review/0007-graph-database-phase-1-generation-infrastructure.md`

## Architecture

### Module Structure
```
crates/server/src/graph/              # NEW MODULE
├── mod.rs                            # Module exports, feature gate
├── types.rs                          # Data model (Node, Edge, Relationship, GraphData)
├── builder.rs                        # Graph build pipeline
├── parser.rs                         # "Related Concepts" markdown parser
├── persistence.rs                    # rkyv + mmap + blake3 caching
├── loader.rs                         # Async graph loading for AppState
├── validation.rs                     # Graph integrity validation
├── stats.rs                          # Statistics computation
└── cli.rs                            # CLI command handlers
```

### Data Directory Structure
```
data/                                 # NEW (at project root)
├── graphs/                           # Git-tracked
│   ├── concept_graph.json            # Source of truth
│   └── manual_edges.json             # Optional manual overrides
└── .cache/                           # Git-ignored
    ├── concept_graph.rkyv            # Binary cache
    └── graph_hash                    # Cache validation hash
```

## Implementation Phases

### Phase 1: Data Model & Parsing (2-3 hours)

**Files to create:**
- `crates/server/src/graph/mod.rs`
- `crates/server/src/graph/types.rs`
- `crates/server/src/graph/parser.rs`

**Key types:**
```rust
// Node enum with Concept and Source variants
pub enum Node {
    Concept(ConceptNode),
    Source(SourceNode),
}

// ConceptNode: id, title, category, source_id, canonical_id, is_canonical
// SourceNode: id, title, author, year, is_converted
// Edge: from, to, relationship, weight, origin
// Relationship: RelatesTo, Prerequisite, Extends, SameAs, Introduces, Covers, Cites
// EdgeOrigin: Extracted, Manual, Inferred
// GraphData: version, nodes, edges, metadata

// All types derive: Archive, Serialize, Deserialize (rkyv) + SerdeSerialize, SerdeDeserialize (serde)
```

**Parser implementation:**
Parse "Related Concepts" section from markdown body:
```markdown
## Related Concepts
- **Prerequisite**: concept-id-1, concept-id-2
- **Leads to**: concept-id-3
- **See also**: concept-id-4, concept-id-5
```

Returns `RelatedConcepts { prerequisite: Vec<String>, leads_to: Vec<String>, see_also: Vec<String> }`

**Tests:**
- Types: serialization round-trips (rkyv + serde)
- Parser: valid sections, missing sections, malformed input, ID normalization

**Milestone:** Can parse concept cards and extract relationships

### Phase 2: Build Pipeline (3-4 hours)

**Files to create:**
- `crates/server/src/graph/builder.rs`

**GraphBuilder implementation:**
```rust
pub struct GraphBuilder {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    node_ids: HashSet<String>,
    warnings: Vec<String>,
}

impl GraphBuilder {
    pub fn build(data_dir: &Path) -> Result<(GraphData, Vec<String>)> {
        // 1. Load sources from config → SourceNode
        // 2. Scan concept cards → ConceptNode
        // 3. Parse Related Concepts → extract edges:
        //    - Prerequisite: from prereq_id to card_id
        //    - Leads to: from card_id to leads_to_id
        //    - See also: from card_id to see_also_id (weight: 0.7)
        // 4. Create Introduces edges: source → concept
        // 5. Merge manual_edges.json (if exists)
        // 6. Deduplicate edges (keep highest weight)
    }
}
```

**Integration points:**
- Reuse `config::SourcesConfig` for sources
- Reuse `util::files::find_all_files` for scanning
- Reuse `metadata::extract_concept_metadata` for metadata
- Use new `parser::parse_related_concepts` for relationships

**Tests:**
- Build with valid cards
- Handle missing references (generate warnings)
- Deduplicate edges correctly
- Merge manual edges (override existing)

**Milestone:** Can build GraphData from concept cards directory

### Phase 3: Persistence Layer (2-3 hours)

**Files to create:**
- `crates/server/src/graph/persistence.rs`

**Key functions:**
```rust
pub async fn save_graph(graph_data: &GraphData, data_dir: &Path) -> Result<()>
pub async fn load_graph(data_dir: &Path) -> Result<ConceptGraph>
```

**Cache strategy:**
1. JSON is source of truth
2. Compute blake3 hash of JSON
3. Store hash in `data/.cache/graph_hash`
4. On load: compare hashes → cache hit → mmap rkyv → instant load
5. Cache miss → parse JSON → save rkyv cache → return graph

**Atomic writes:** Write to `.rkyv.tmp`, then rename (atomic)

**Tests:**
- Save/load round-trip
- Cache invalidation (hash changes)
- Cache hit (hash matches)
- Handle corrupt cache gracefully

**Milestone:** Can persist and load graph with efficient caching

### Phase 4: CLI Commands (2-3 hours)

**Files to modify:**
- `crates/server/src/cli.rs` - Add `Graph(GraphCommands)` subcommand

**Files to create:**
- `crates/server/src/graph/cli.rs`

**Commands:**
1. `graph build [--dry-run] [--verbose]` - Build graph from cards
2. `graph validate` - Check integrity (orphans, self-loops, broken refs)
3. `graph stats` - Show counts by category, relationship type
4. `graph compile` - Rebuild rkyv cache from JSON

**Implementation:**
```rust
pub async fn handle_graph_command(cmd: GraphSubcommand, config: &Config) -> Result<()>
```

**Tests:**
- CLI execution for all commands
- Dry-run mode doesn't write files
- Verbose mode shows details

**Milestone:** Can run `music-theory-mcp graph build` from terminal

### Phase 5: AppState Integration (2-3 hours)

**Files to modify:**
- `crates/server/src/state.rs`

**Files to create:**
- `crates/server/src/graph/loader.rs`

**AppState changes:**
```rust
pub struct AppState {
    // ... existing fields ...

    #[cfg(feature = "graph")]
    pub graph: RwLock<GraphState>,  // NEW
}

#[cfg(feature = "graph")]
pub enum GraphState {
    NotLoaded,
    Loading,
    Loaded(LoadedGraph),
    Failed(String),
}

#[cfg(feature = "graph")]
pub struct LoadedGraph {
    pub graph: ConceptGraph,                           // petgraph::DiGraph<Node, Edge>
    pub node_index: HashMap<String, NodeIndex>,        // Fast ID lookup
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    pub stats: GraphStats,
}
```

**Async initialization:**
Spawn background task in `AppState::initialize()` to load graph (mirrors FTS pattern)

**Tests:**
- AppState with no graph → NotLoaded
- AppState with valid cache → Loaded
- AppState with stale cache → rebuilds

**Milestone:** Graph loads automatically on server startup

### Phase 6: MCP Tools (3-4 hours)

**Files to create:**
- `crates/server/src/tools/graph.rs`

**Files to modify:**
- `crates/server/src/server.rs` - Register graph tools

**Tools to implement:**

1. **`graph_status`** - Check if graph is loaded, return status + basic stats
2. **`graph_stats`** - Detailed statistics (categories, relationships, orphans)
3. **`graph_validate`** - Integrity check (orphans, self-loops)
4. **`get_node`** - Get node by ID with in/out degree
5. **`get_node_edges`** - Get all edges for a node (with direction filter)

**Tool signatures:**
```rust
async fn graph_status(&self) -> Result<CallToolResult, ErrorData>
async fn graph_stats(&self) -> Result<CallToolResult, ErrorData>
async fn graph_validate(&self) -> Result<CallToolResult, ErrorData>
async fn get_node(&self, params: Parameters<GetNodeParams>) -> Result<CallToolResult, ErrorData>
async fn get_node_edges(&self, params: Parameters<GetNodeEdgesParams>) -> Result<CallToolResult, ErrorData>
```

**Error handling:** Return friendly errors when graph not loaded

**Tests:**
- Tools when graph not loaded → error
- Tools when graph loaded → return data
- Edge direction filtering works
- JSON serialization of responses

**Milestone:** MCP tools accessible via inspector

### Phase 7: Validation & Stats (2 hours)

**Files to create:**
- `crates/server/src/graph/validation.rs`
- `crates/server/src/graph/stats.rs`

**Validation checks:**
- Orphan nodes (no incoming or outgoing edges)
- Self-loops (edge from node to itself)
- Broken references (edges pointing to non-existent nodes)

**Statistics:**
- Count nodes by type (concept vs source)
- Count by category
- Count by relationship type
- Average degree (in/out)

**Tests:**
- Detect each validation issue
- Compute correct counts

**Milestone:** `graph validate` and `graph stats` work correctly

### Phase 8: Feature Gate & Dependencies (1 hour)

**Files to modify:**
- `crates/server/Cargo.toml`
- `Cargo.toml` (workspace)

**Cargo.toml changes:**
```toml
[features]
default = []
fts = ["dep:tantivy", "dep:regex", "dep:stop-words"]
graph = ["dep:petgraph", "dep:rkyv", "dep:memmap2", "dep:blake3", "dep:chrono"]
full = ["fts", "graph"]

[dependencies]
petgraph = { version = "0.6", optional = true }
rkyv = { version = "0.7", features = ["validation"], optional = true }
memmap2 = { version = "0.9", optional = true }
blake3 = { version = "1.5", optional = true }
chrono = { version = "0.4", features = ["serde"] }
```

**Feature gating:**
- All graph module code: `#[cfg(feature = "graph")]`
- CLI commands: `#[cfg(feature = "graph")]`
- MCP tools: `#[cfg(feature = "graph")]`
- AppState fields: `#[cfg(feature = "graph")]`

**Milestone:** Can build with/without graph feature

### Phase 9: Polish & Documentation (1-2 hours)

**Tasks:**
- Add doc comments to all public items
- Create example `data/graphs/manual_edges.json`
- Update README with graph feature documentation
- Update .gitignore: `data/.cache/`
- Run full test suite
- Verify ≥95% coverage

**Milestone:** Feature complete and production-ready

## Critical Implementation Details

### Relationship Mapping

| Markdown | Edge Direction | Relationship | Weight |
|----------|---------------|--------------|--------|
| `Prerequisite: X` | `from: X, to: card` | `Prerequisite` | 1.0 |
| `Leads to: Y` | `from: card, to: Y` | `Prerequisite` | 1.0 |
| `See also: Z` | `from: card, to: Z` | `RelatesTo` | 0.7 |

**Rationale:** "Prerequisite: X" means "X is prerequisite FOR this card"

### Concept ID Normalization

All concept IDs must be normalized before lookup:
- Lowercase
- Trim whitespace
- Replace spaces with hyphens

Handle mismatches gracefully with warnings.

### Error Handling

Follow existing patterns:
- Return `Result<T>` from fallible functions
- Use `crate::error::Error` enum
- Add variants as needed: `GraphBuildError`, `ValidationError`
- Propagate with `?`
- Convert to MCP errors in tools: `error.to_mcp_error(context)`

### Logging

Use `tracing` crate (already in codebase):
```rust
tracing::info!("Building graph from {} concept cards", count);
tracing::warn!("Concept '{}' references unknown prerequisite '{}'", card_id, prereq);
tracing::error!("Failed to load graph: {}", error);
```

## Critical Files to Modify

1. **`crates/server/src/state.rs`** - Add GraphState, async loader
2. **`crates/server/src/cli.rs`** - Add Graph subcommand
3. **`crates/server/src/server.rs`** - Register graph tools
4. **`crates/server/Cargo.toml`** - Add dependencies, feature gate
5. **`Cargo.toml`** (workspace) - Add workspace dependencies
6. **`.gitignore`** - Add `data/.cache/`

## Critical Files to Create

1. **`crates/server/src/graph/builder.rs`** - Build pipeline (MOST CRITICAL)
2. **`crates/server/src/graph/types.rs`** - Data structures
3. **`crates/server/src/graph/parser.rs`** - Markdown relationship parser
4. **`crates/server/src/graph/persistence.rs`** - rkyv + mmap + caching
5. **`crates/server/src/graph/loader.rs`** - Async loading
6. **`crates/server/src/tools/graph.rs`** - MCP tools

## Testing Strategy

### Test Coverage Target: ≥95%

**Unit tests per module:**
- types.rs: Serialization round-trips, enum matching
- parser.rs: Valid/invalid/missing sections, ID normalization
- builder.rs: Valid builds, warnings, deduplication, manual edges
- persistence.rs: Save/load, cache hits/misses, corruption handling
- loader.rs: Node index, stats computation
- validation.rs: Orphans, self-loops, broken refs
- stats.rs: Count accuracy

**Integration tests:**
- Full build pipeline with temp directory
- CLI commands end-to-end
- AppState initialization scenarios
- MCP tools with/without loaded graph

**Test fixtures:**
Create minimal test data in `test-data/`:
```
test-data/
├── concept-cards/
│   ├── concept-a.md  (Prerequisites: concept-b)
│   ├── concept-b.md  (Leads to: concept-c)
│   └── concept-c.md  (See also: concept-a)
```

Use `tempfile` crate for isolated test environments.

## Verification Checklist

After implementation:

- [ ] All tests pass: `cargo test --features graph`
- [ ] Test coverage ≥95%: `cargo tarpaulin --features graph`
- [ ] Linting passes: `cargo clippy --features graph`
- [ ] Format check: `cargo fmt --check`
- [ ] No compiler warnings
- [ ] CLI builds successfully: `cargo build --features graph --release`
- [ ] Can run `music-theory-mcp graph build` and create JSON
- [ ] Server starts with graph feature enabled
- [ ] MCP tools work via inspector
- [ ] Graph loads from cache on second startup (instant)
- [ ] Cache invalidates when JSON changes

## End-to-End Verification

1. **Build graph:**
   ```bash
   cargo build --features graph --release
   ./target/release/music-theory-mcp graph build --verbose
   ```
   Expect: `data/graphs/concept_graph.json` created with ~200 nodes

2. **Check cache:**
   ```bash
   ls -lh data/.cache/
   ```
   Expect: `concept_graph.rkyv` and `graph_hash` files

3. **Run stats:**
   ```bash
   ./target/release/music-theory-mcp graph stats
   ```
   Expect: Node/edge counts, category breakdown

4. **Run validation:**
   ```bash
   ./target/release/music-theory-mcp graph validate
   ```
   Expect: Validation report

5. **Start server:**
   ```bash
   ./target/release/music-theory-mcp serve
   ```
   Check logs for: "Concept graph ready"

6. **Test MCP tools:**
   ```bash
   npx @modelcontextprotocol/inspector ./target/release/music-theory-mcp
   ```
   Call: `graph_status`, `graph_stats`, `get_node`, `get_node_edges`

## Estimated Timeline

- Phase 1 (Data Model & Parsing): 2-3 hours
- Phase 2 (Build Pipeline): 3-4 hours
- Phase 3 (Persistence): 2-3 hours
- Phase 4 (CLI Commands): 2-3 hours
- Phase 5 (AppState Integration): 2-3 hours
- Phase 6 (MCP Tools): 3-4 hours
- Phase 7 (Validation & Stats): 2 hours
- Phase 8 (Feature Gate): 1 hour
- Phase 9 (Polish): 1-2 hours

**Total: 18-24 hours of focused implementation**

## Dependencies

All optional, feature-gated:
- **petgraph 0.6** - Graph data structure
- **rkyv 0.7** - Zero-copy deserialization
- **memmap2 0.9** - Memory-mapped file I/O
- **blake3 1.5** - Fast cryptographic hashing
- **chrono 0.4** - DateTime handling (already in workspace)

## Success Criteria

Phase 1 complete when:
- [ ] `music-theory-mcp graph build` creates valid JSON
- [ ] Graph loads async on server startup with rkyv caching
- [ ] Cache invalidation works (hash-based)
- [ ] 5 MCP tools functional: graph_status, graph_stats, graph_validate, get_node, get_node_edges
- [ ] 4 CLI commands work: build, validate, stats, compile
- [ ] Feature-gated with `--features graph`
- [ ] Test coverage ≥95%
- [ ] All warnings shown during build (broken references)
- [ ] Manual edges override support works
