# Music Theory MCP Server

A Model Context Protocol (MCP) server that provides access to comprehensive music theory educational materials, including converted source texts, concept cards, and topic guides.

## Status

✅ **Fully Functional** - All core features implemented and working

### Current Implementation

- ✅ **rmcp 0.14 integrated** - Using official Rust MCP SDK (2M+ downloads)
- ✅ **Configuration management** - confyg for TOML + ENV with path expansion
- ✅ **Logging** - twyg for beautiful colored output
- ✅ **Error handling** - Canonical pattern with backtraces (EH-17)
- ✅ **25 Tools registered and working**:
  - **Content Access (10 tools):**
    - `list_sources` - List all source materials with metadata
    - `get_source_chapter` - Retrieve specific chapters
    - `get_source_pdf_path` - Get filesystem paths to PDFs/EPUBs
    - `list_concepts` - List concept cards with optional filtering
    - `list_categories` - List all distinct concept categories with counts
    - `get_concept` - Retrieve specific concepts
    - `search_concepts` - Full-text search with ranking (includes backend field)
    - `list_guides` - List topic guides
    - `get_guide` - Retrieve specific guides
    - `health` - Get server health and search backend status
  - **Graph Database (15 tools, requires `--features graph`):**
    - `graph_status` - Get graph loading state and statistics
    - `graph_stats` - Detailed graph statistics (categories, relationships, degree)
    - `graph_validate` - Check graph integrity (orphans, self-loops)
    - `get_node` - Get node information by ID
    - `get_node_edges` - Get all edges for a node with direction filtering
    - `get_related_concepts` - Find related concepts with relationship filtering
    - `find_concept_path` - Find shortest path between concepts
    - `get_prerequisites` - Get prerequisites in topological learning order
    - `get_concept_neighborhood` - Get local subgraph around a concept
    - `get_dependents` - Find concepts that depend on this as prerequisite
    - `get_central_concepts` - Find most connected concepts by degree centrality
    - `get_concept_sources` - Get all sources that introduce/cover a concept
    - `get_concept_variants` - Get source-specific variants of canonical concept
    - `find_bridge_concepts` - Find concepts bridging two categories
    - `get_source_coverage` - Get all concepts introduced/covered by a source
- ✅ **4 Resources implemented** (ready for registration):
  - `skill://conventions` - Notation conventions
  - `skill://scope` - Topics & objectives
  - `skill://sources` - Bibliography
  - `skill://index` - Complete index
- ✅ **Test suite** - 360+ tests passing (with all features)
- ✅ **MCP Server** - Full ServerHandler with tool routing via macros
- ✅ **Stdio transport** - Server verified working

## Project Structure

```
mcp-server/
├── Cargo.toml              # Project dependencies
├── config/
│   └── default.toml        # Configuration with paths and sources
├── src/
│   ├── main.rs             # Server entry point
│   ├── config.rs           # Configuration loading and management
│   ├── error.rs            # Canonical error types (EH-17 pattern)
│   ├── tools/              # Tool implementations
│   │   ├── mod.rs
│   │   ├── sources.rs      # Source material tools
│   │   ├── concepts.rs     # Concept card tools
│   │   ├── guides.rs       # Topic guide tools
│   │   └── search.rs       # Search functionality
│   └── resources/          # Static resource providers
│       └── mod.rs
└── assets/ai/              # Rust development guidelines (symlink)
```

## Configuration

The server is configured via `config/default.toml`:

```toml
[server]
name = "music-theory-skill"
version = "0.1.0"

[paths]
base = "~/lab/music-comp/ai-music-theory"
sources_md = "${paths.base}/sources-md"
concept_cards = "${paths.base}/concept-cards"
# ... more paths

[sources.oxford]
path = "~/Dropbox/Apps/Oxford University Press"

[sources.oxford.files]
lewin-gmit = "[2007] Lewin - GMIT.pdf"
# ... more sources
```

### Path Variables

- Supports `~` expansion for home directory
- Supports variable interpolation (`${paths.base}`)
- All paths are expanded at runtime

### Search Configuration

**Version 0.2.0 Update:** Tantivy is now the default search backend for significantly better search quality!

The server supports two search backends for the `search_concepts` tool:

- **Tantivy** (default, recommended) - Full-text search engine with advanced query support
- **Simple** (fallback) - Linear scan, suitable for small collections without an index

#### Search Quality Features (Tantivy Backend)

✅ **Multi-word query support** - `"fugue subject answer"` works correctly
✅ **Smart AND/OR logic** - 2 words require both, 3+ words use intelligent OR matching
✅ **Stopword filtering** - `"what is a cadence"` → searches for `"cadence"` (500+ stopwords)
✅ **Phrase search** - `"perfect authentic cadence"` matches exact phrase
✅ **Stemming** - `write`, `writing`, `written` all match
✅ **Relevance ranking** - BM25 algorithm for better result ordering
✅ **Category filtering** - Scope searches to specific categories (harmony, counterpoint, etc.)
✅ **Typo tolerance** - Optional fuzzy search for typo correction
✅ **Improved snippets** - All results return non-empty, contextual snippets

#### Quick Start with Tantivy

**1. Build with FTS support:**

```bash
cargo build --release --features fts
```

**2. Build the search index:**

```bash
./target/release/music-theory-mcp index
```

**3. Start the server:**

```bash
./target/release/music-theory-mcp serve
```

That's it! The server will use the improved Tantivy backend by default.

#### Configuration

Configure search behavior in `config/default.toml`:

```toml
[search]
# Backend selection: "tantivy" (default) or "simple" (fallback)
# Note: "tantivy" requires building with --features fts
backend = "tantivy"

# Tantivy index directory (relative to skill root)
index_path = ".tantivy-index"

# Rebuild index on startup
rebuild_on_startup = false

# Snippet context size in characters
snippet_size = 200

# Enable fuzzy search (typo tolerance)
fuzzy_search = false
fuzzy_distance = 2

# Query mode for multi-word queries
# - "smart": 2 words = AND, 3+ words = OR with 60% minimum match (recommended)
# - "and": All terms must be present (strict)
# - "or": Any term matches (maximum recall)
# - { minimum_match = 0.75 }: At least N% of terms must match
query_mode = "smart"

# Minimum match percentage for OR queries with 3+ terms
minimum_match_percent = 0.6

# Enable stopword filtering for natural language queries
enable_stopwords = true

# Custom stopwords (beyond default English stopwords)
custom_stopwords = []

# Domain-specific terms to preserve (never filtered)
stopword_allowlist = ["I", "V", "ii", "IV", "vi", "vii", "i", "v", "iv", "do", "re", "mi", "fa", "sol", "la", "ti"]
```

#### Query Syntax

The Tantivy backend supports advanced query syntax:

**Basic queries:**
```
cadence                    → Single term search
authentic cadence          → Smart mode: both terms required (AND)
fugue subject answer       → Smart mode: 2 of 3 terms required (60% minimum)
```

**Natural language:**
```
what is a cadence          → Stopwords filtered: "cadence"
how to write counterpoint  → Filtered: "write counterpoint"
V I resolution             → Roman numerals preserved
```

**Phrase search (exact matching):**
```
"perfect authentic cadence"    → Exact phrase
"leading tone" resolution      → Phrase + additional term
"V I" "IV V"                   → Multiple phrases (OR)
```

**Query Mode Examples:**

| Query | Smart Mode | AND Mode | OR Mode |
|-------|------------|----------|---------|
| `authentic cadence` | Both required | Both required | Either matches |
| `fugue subject answer` | 2 of 3 required (60%) | All 3 required | Any 1 matches |

#### CLI Commands

```bash
# Start server (default command)
music-theory-mcp serve

# Build or rebuild index (requires --features fts)
music-theory-mcp index
music-theory-mcp index --force  # Force rebuild

# Show index status
music-theory-mcp status
```

**Example status output:**
```
Index Status:
  Location:     /path/to/.tantivy-index
  Documents:    200
  Status:       ✓ Current
```

#### Migration from Simple Backend

If you're upgrading from v0.1.0, see `MIGRATION.md` for detailed migration instructions.

**Quick migration:**
1. Rebuild with `--features fts`
2. Run `music-theory-mcp index`
3. Restart server

**Fallback option:**
Set `backend = "simple"` in config (no index required, suitable for smaller collections).

#### Performance Comparison

| Metric | Simple Backend | Tantivy Backend |
|--------|----------------|-----------------|
| **Query latency** | ~50-100ms | ~5-20ms |
| **Memory usage** | ~50MB | ~100-150MB |
| **Multi-word queries** | ❌ Poor | ✅ Excellent |
| **Natural language** | ❌ Poor | ✅ Excellent |
| **Phrase search** | ❌ No | ✅ Yes |
| **Relevance ranking** | ⚠️ Basic | ✅ BM25 |

**Search Response Format:**

The `search_concepts` tool now includes a `backend` field indicating which backend served the request:

```json
{
  "results": [...],
  "total": 10,
  "query": "harmony",
  "backend": "tantivy"  // or "simple"
}
```

### Graph Database Configuration

**Version 0.3.0 Update:** The server now includes a powerful graph database for exploring concept relationships!

The graph database builds a network of music theory concepts and their relationships from concept card frontmatter. It enables advanced queries like finding prerequisites, exploring neighborhoods, discovering learning paths, and identifying bridging concepts.

#### Graph Features

✅ **Relationship extraction** - Parses frontmatter to build concept graph
✅ **6 graph algorithms** - BFS, DFS, shortest path, topological sort, centrality, bridge finding
✅ **15 MCP query tools** - Comprehensive graph traversal and analysis
✅ **Zero-copy loading** - rkyv serialization with mmap for instant startup
✅ **Async background loading** - Server starts immediately, graph loads in background
✅ **Hash-based caching** - Automatic cache invalidation when concept cards change
✅ **Validation tools** - Detect orphan nodes, self-loops, broken references
✅ **Statistics** - Node counts, category distribution, degree statistics

#### Quick Start with Graph

**1. Build with graph support:**

```bash
cargo build --release --features graph
```

**2. Build the graph database:**

```bash
./target/release/music-theory-mcp graph build
```

This command:
- Scans all concept cards in `concept-cards/` directory
- Extracts relationships from frontmatter (`related_concepts` field)
- Builds node and edge data structures
- Saves to `data/graphs/concept_graph.json`
- Creates rkyv cache at `data/graphs/concept_graph.rkyv` for fast loading

**3. Verify the graph:**

```bash
# Check graph integrity
./target/release/music-theory-mcp graph validate

# View statistics
./target/release/music-theory-mcp graph stats
```

**4. Start the server:**

```bash
./target/release/music-theory-mcp serve
```

The graph loads asynchronously in the background. Use the `graph_status` tool to check loading progress.

#### CLI Commands

```bash
# Build graph from concept cards
music-theory-mcp graph build
music-theory-mcp graph build --verbose    # Show extracted relationships
music-theory-mcp graph build --dry-run    # Preview without writing

# Validate graph integrity
music-theory-mcp graph validate

# Show statistics
music-theory-mcp graph stats

# Rebuild rkyv cache (if corrupted)
music-theory-mcp graph compile
```

**Example build output:**
```
Building concept graph...
  Scanning concept cards...
  Found 187 concept cards
  Extracted 342 relationships
  Building graph...
    Nodes: 187 concepts, 8 sources
    Edges: 342 relationships
  Writing graph to data/graphs/concept_graph.json...
  Creating rkyv cache...
✓ Graph built successfully
```

**Example stats output:**
```
Graph Statistics:
  Nodes: 195 (187 concepts, 8 sources)
  Edges: 342
  Categories:
    - harmony: 45 concepts
    - counterpoint: 32 concepts
    - form: 28 concepts
    ...
  Relationships:
    - Prerequisite: 156 edges
    - RelatesTo: 98 edges
    - Introduces: 88 edges
  Degree Statistics:
    - Average in-degree: 1.75
    - Average out-degree: 1.75
    - Max in-degree: 12
    - Max out-degree: 8
```

#### Available Graph Tools

The graph feature provides 15 MCP tools for exploring concept relationships:

**Graph Inspection (5 tools):**
- `graph_status` - Get loading state, statistics, and readiness
- `graph_stats` - Detailed statistics (categories, relationships, degree)
- `graph_validate` - Integrity checks (orphans, self-loops, broken refs)
- `get_node` - Get node by ID with in/out degree counts
- `get_node_edges` - Get edges for a node with direction filtering

**Graph Traversal (10 tools):**
- `get_related_concepts` - Find related concepts with filtering
  - Filter by relationship type (prerequisite, relates_to, etc.)
  - Filter by direction (incoming, outgoing, both)
  - Traverse up to 3 hops (depth parameter)
- `find_concept_path` - Find shortest path between two concepts
  - Returns nodes and edges along the path
  - Useful for discovering learning connections
- `get_prerequisites` - Get prerequisites in topological order
  - Returns concepts in learning order (foundational → advanced)
  - Configurable depth (up to 5 levels)
- `get_concept_neighborhood` - Get N-hop neighborhood subgraph
  - Explore concepts within radius around a starting point
  - Configurable max nodes to prevent large results
- `get_dependents` - Find what depends on this concept
  - Discover advanced topics that build on a concept
  - Inverse of prerequisites
- `get_central_concepts` - Most connected concepts by degree
  - Find "hub" concepts with many relationships
  - Optional category filtering
- `get_concept_sources` - Which sources cover this concept
  - Find all texts that introduce or explain a concept
- `get_concept_variants` - Source-specific concept variants
  - Find how different sources present the same concept
- `find_bridge_concepts` - Concepts connecting two categories
  - Discover concepts that span multiple areas
  - Scored by connection strength
- `get_source_coverage` - What concepts does a source cover
  - See all concepts introduced by a particular text

#### Graph Data Format

Concept cards declare relationships in frontmatter:

```yaml
---
id: seventh-chord
title: Seventh Chord
category: harmony
related_concepts:
  - id: triad
    relationship: prerequisite
    reason: Seventh chords are built on triads
  - id: dominant-seventh
    relationship: example
    reason: Most common type of seventh chord
  - id: voice-leading
    relationship: applies_to
    reason: Voice leading rules apply to seventh chords
---
```

**Supported relationship types:**
- `prerequisite` - Required prior knowledge
- `relates_to` - General connection
- `example` - Specific instance
- `applies_to` - Principle application
- `extends` - Advanced version
- `contrasts_with` - Opposite or alternative
- `introduces` - Source introduces concept
- `covers` - Source explains concept

#### Graph Loading

The graph loads asynchronously on server startup:

1. **Server starts immediately** - Tools available within ~1 second
2. **Background loading** - Graph loads from rkyv cache (instant) or JSON (slower)
3. **Cache invalidation** - Automatically rebuilds if concept cards changed
4. **Tool availability** - Graph tools work once loading completes

Check loading status with the `graph_status` tool:

```json
{
  "enabled": true,
  "status": "loaded",  // or "not_loaded", "loading", "failed"
  "stats": {
    "node_count": 195,
    "edge_count": 342,
    "concept_count": 187,
    "source_count": 8
  },
  "loaded_at": "2026-01-28T12:34:56Z"
}
```

#### Configuration

Graph settings in `config/default.toml`:

```toml
[paths]
# Base directory for skill data
base = "~/lab/music-comp/ai-music-theory"

# Concept cards directory (source for graph)
concept_cards = "${paths.base}/concept-cards"

# Graph database directory
graphs = "${paths.base}/mcp-server/data/graphs"
```

The graph builder:
- Scans all markdown files in `concept_cards/`
- Extracts `related_concepts` from frontmatter
- Builds directed graph with typed edges
- Saves JSON and rkyv cache to `data/graphs/`

#### Performance

| Metric | Cold Start (JSON) | Warm Start (rkyv) |
|--------|------------------|-------------------|
| **Load time** | ~50-100ms | ~1-5ms |
| **Memory usage** | ~5MB | ~5MB |
| **Query latency** | ~1-10ms | ~1-10ms |
| **Build time** | ~200-500ms | N/A |

**Graph loading is non-blocking:**
- Server starts in <1 second regardless of graph size
- Simple content tools work immediately
- Graph tools return "not ready" until loading completes
- Automatic background rebuild if concept cards change

#### Usage Examples

**Find prerequisites for a concept:**
```json
// Tool: get_prerequisites
{
  "concept_id": "fugue",
  "depth": 3
}

// Response: Prerequisites in learning order
{
  "concept_id": "fugue",
  "total": 5,
  "prerequisites": [
    {
      "id": "interval",
      "title": "Interval",
      "category": "fundamentals",
      "depth": 3
    },
    {
      "id": "counterpoint",
      "title": "Counterpoint",
      "category": "counterpoint",
      "depth": 2
    },
    {
      "id": "imitation",
      "title": "Imitation",
      "category": "counterpoint",
      "depth": 1
    },
    ...
  ]
}
```

**Find concepts bridging two categories:**
```json
// Tool: find_bridge_concepts
{
  "category_a": "harmony",
  "category_b": "counterpoint",
  "limit": 5
}

// Response: Concepts connecting both areas
{
  "category_a": "harmony",
  "category_b": "counterpoint",
  "total": 5,
  "bridges": [
    {
      "id": "suspension",
      "title": "Suspension",
      "category": "counterpoint",
      "connections_a": 8,  // Links to 8 harmony concepts
      "connections_b": 12, // Links to 12 counterpoint concepts
      "bridge_score": 9.8  // sqrt(8 * 12)
    },
    ...
  ]
}
```

**Explore neighborhood around a concept:**
```json
// Tool: get_concept_neighborhood
{
  "concept_id": "cadence",
  "radius": 2,
  "max_nodes": 30
}

// Response: Subgraph within 2 hops
{
  "concept_id": "cadence",
  "radius": 2,
  "total_nodes": 25,
  "total_edges": 42,
  "nodes": [
    {
      "id": "cadence",
      "title": "Cadence",
      "node_type": "concept",
      "category": "harmony",
      "distance": 0,
      "is_center": true
    },
    {
      "id": "perfect-authentic-cadence",
      "title": "Perfect Authentic Cadence",
      "node_type": "concept",
      "category": "harmony",
      "distance": 1,
      "is_center": false
    },
    ...
  ],
  "edges": [...]
}
```

#### Migration from v0.2.0

If you're upgrading from v0.2.0 without graph support:

**Quick migration:**
1. Rebuild with `--features graph`
2. Run `music-theory-mcp graph build`
3. Restart server

The graph feature is optional. If not built with `--features graph`, the 15 graph tools will return "feature not enabled" errors.

#### Troubleshooting

**Graph not loading:**
- Check `data/graphs/` directory exists and is writable
- Verify concept cards have valid YAML frontmatter
- Run `music-theory-mcp graph validate` to check integrity
- Check logs for parsing errors

**Build errors:**
- Ensure concept cards follow frontmatter format
- Check for invalid relationship types in `related_concepts`
- Verify concept IDs in relationships match actual card IDs
- Use `--verbose` flag to see extraction details

**Performance issues:**
- Graph too large? Increase query depth limits in code
- Cache corrupt? Run `music-theory-mcp graph compile` to rebuild
- JSON slow? rkyv cache should load in <5ms; check file exists

## Building

```bash
# Build without optional features (minimal, smaller binary)
cargo build
cargo build --release

# Build with FTS support (Tantivy search backend)
cargo build --features fts
cargo build --release --features fts

# Build with graph support (concept relationship graph)
cargo build --features graph
cargo build --release --features graph

# Build with all features (recommended for full functionality)
cargo build --features fts,graph
cargo build --release --features fts,graph

# Run tests
cargo test                           # Without optional features
cargo test --features fts            # With FTS only
cargo test --features graph          # With graph only
cargo test --features fts,graph      # With all features

# Run with logging
RUST_LOG=info cargo run
RUST_LOG=info cargo run --features fts,graph
```

**Binary Size:**
- Without features: ~2.6M (10 content tools only)
- With FTS: ~6.3M (adds Tantivy search backend)
- With graph: ~4.2M (adds graph database + 15 tools)
- With all features: ~8.5M (full functionality)

**Feature Flags:**
- `fts` - Enables Tantivy full-text search backend (optional)
- `graph` - Enables graph database and 15 relationship query tools (optional)

Both features are optional and can be enabled independently or together.

## Using with Claude Desktop

Claude Desktop can launch and communicate with this MCP server using the stdio transport. You have two options for running the server:

### Option 1: Development Mode (Using Cargo)

This approach rebuilds the server on each launch, useful during development:

1. **Locate your Claude Desktop config file:**
   ```
   ~/Library/Application Support/Claude/claude_desktop_config.json
   ```

2. **Add the server configuration:**
   ```json
   {
     "mcpServers": {
       "music-theory": {
         "command": "cargo",
         "args": [
           "run",
           "--manifest-path",
           "/Users/YOUR_USERNAME/path/to/ai-music-theory/mcp-server/Cargo.toml"
         ]
       }
     }
   }
   ```

   **Note:** Replace the path with your actual project location.

3. **Restart Claude Desktop** completely (quit and relaunch)

### Option 2: Production Mode (Using Compiled Binary)

This approach is faster and recommended for regular use:

1. **Build the release binary:**
   ```bash
   # Minimal build (10 content tools, smaller binary)
   cargo build --release

   # With FTS (adds Tantivy search backend)
   cargo build --release --features fts

   # With graph (adds graph database + 15 relationship tools)
   cargo build --release --features graph

   # Full build (recommended - all 25 tools)
   cargo build --release --features fts,graph
   ```

2. **Locate your Claude Desktop config file:**
   ```
   ~/Library/Application Support/Claude/claude_desktop_config.json
   ```

3. **Add the server configuration:**
   ```json
   {
     "mcpServers": {
       "music-theory": {
         "command": "/Users/YOUR_USERNAME/path/to/ai-music-theory/mcp-server/target/release/music-theory-mcp"
       }
     }
   }
   ```

   **Note:** Replace the path with your actual project location.

4. **Restart Claude Desktop** completely (quit and relaunch)

### Verifying the Connection

Once Claude Desktop restarts:

1. Open a new conversation
2. Look for the server connection indicator (usually in the UI)
3. Try using the available tools (10 base + optional FTS/graph tools):
   - **Always available (10 tools):**
     - `health` - Check server status and active search backend
     - `list_concepts` - List all concept cards
     - `list_categories` - Browse concepts by category
     - `search_concepts` - Search for specific topics
     - `list_guides` - Browse topic guides
     - `get_source_chapter` - Access source material chapters
   - **If built with `--features graph` (15 additional tools):**
     - `graph_status` - Check graph loading state
     - `get_prerequisites` - Find prerequisite concepts
     - `find_concept_path` - Discover learning paths
     - `get_related_concepts` - Explore concept relationships
     - `find_bridge_concepts` - Find concepts connecting categories
     - ...and 10 more graph query tools

**Checking Backend Status:**

Use the `health` tool to verify the server is running and see which search backend is active:

```json
{
  "status": "ok",
  "backend": {
    "active": "simple",  // or "tantivy" if FTS is ready
    "fts_enabled": true,  // only if built with --features fts
    "fts_ready": false,   // only if built with --features fts
    "index_stats": null   // populated when FTS is ready
  }
}
```

You can also check Claude Desktop's developer console for connection logs.

### Troubleshooting

**Server not appearing in Claude Desktop:**
- Verify the config file path is correct (no typos)
- Check that the config JSON is valid (use a JSON validator)
- Ensure Claude Desktop was fully restarted (quit, don't just close window)
- For cargo mode: verify cargo is in your PATH
- For binary mode: verify the binary exists at the specified path

**Server crashes on startup:**
- Check that all configured paths in `config/default.toml` exist
- Ensure you have read permissions for the configured directories
- Look for error logs in Claude Desktop's developer console
- Try running the server manually to see error messages:
  ```bash
  cargo run  # or ./target/release/music-theory-mcp
  ```

**Tools not working:**
- Verify the data directories configured in `config/default.toml` contain the expected files
- Check that markdown files have valid frontmatter (YAML between `---` delimiters)
- Ensure concept cards, guides, and source materials are in the correct locations

For more details on server configuration, see the **Configuration** section below.

## Development Guidelines

This project follows the comprehensive Rust guidelines in `assets/ai/ai-rust/guides/`:

1. **Anti-Patterns** (`11-anti-patterns.md`) - Always load first
2. **Core Idioms** (`01-core-idioms.md`) - Standard patterns
3. **Error Handling** (`03-error-handling.md`) - Canonical error structs
4. **Project Structure** (`12-project-structure.md`) - Module organization

See `CLAUDE.md` for AI assistant guidance and `assets/ai/CLAUDE-CODE-COVERAGE.md` for testing standards.

### Error Handling

All errors follow the EH-17 canonical pattern:

```rust
pub struct Error {
    kind: ErrorKind,           // Private enum
    backtrace: Backtrace,      // Always captured
}

// Public helper methods instead of exposing enum
impl Error {
    pub fn is_io(&self) -> bool { ... }
    pub fn is_not_found(&self) -> bool { ... }
}
```

### Testing

Target: 95%+ code coverage

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_list_sources
```

Test naming convention: `test_<function>_<scenario>_<expectation>`

## Source Materials

The server provides access to music theory texts including:

### Transformational Theory
- Lewin - *Generalized Musical Intervals and Transformations* (2007)

### Geometry of Music
- Tymoczko - *A Geometry of Music* (2011)
- Tymoczko - *Tonality: An Owner's Manual* (2023)

### Neo-Riemannian Theory
- Cohn - *Audacious Euphony* (2012)
- Gollin - *The Oxford Handbook of Neo-Riemannian Music Theories* (2012)

### Post-Tonal Theory
- Straus - *Introduction to Post-Tonal Theory* (2016)

### Online Resources
- Gotham - *Open Music Theory* (2022)
- Hutchinson - *Music Theory for the 21st-Century Classroom* (2023)

## Architecture

### Feature-Gated Design

The server uses Cargo feature flags for optional functionality:

- **Default build** - Simple search backend, minimal dependencies, 2.6M binary
- **FTS feature** - Adds Tantivy backend, 6.3M binary, optional

### Application State

The server uses a shared `AppState` struct for managing search backends:

```rust
pub struct AppState {
    config: Config,
    simple_backend: Arc<SimpleSearch>,         // Always available
    fts_backend: Arc<RwLock<Option<Arc<TantivySearch>>>>,  // Hot-swappable
    fts_ready: Arc<AtomicBool>,                // Lock-free readiness flag
}
```

**Backend Selection:**
- FTS not ready → Simple backend
- FTS ready → Tantivy backend
- Automatic switch when background indexing completes

### Non-Blocking Indexing

The server implements async background indexing:

1. Server starts immediately (<1 second)
2. Background task checks index freshness (content hash)
3. If needed, builds/rebuilds index asynchronously
4. Simple search handles queries during indexing
5. Automatic switch to FTS when ready

**Index Freshness:**
- Computes hash of all concept card paths + modification times
- Stored in `metadata.json` alongside index
- Avoids unnecessary rebuilds when content unchanged

## CLI Commands

The server provides a CLI for index management (requires `--features fts`):

```bash
# Start server (default command)
music-theory-mcp
music-theory-mcp serve

# Build or rebuild index
music-theory-mcp index
music-theory-mcp index --force  # Force rebuild even if fresh

# Show index status and statistics
music-theory-mcp status
```

**Example status output:**
```
Index Status:
  Location:     /path/to/.tantivy-index
  Documents:    187
  Status:       ✓ Current
```

## Next Steps

1. **Integration Testing**
   - Test with Claude Desktop and other MCP clients
   - Verify all 10 tools work with real data
   - Document usage examples

2. **Resource Registration**
   - Wire up the 4 static resources
   - Enable resource URIs in ServerHandler
   - Test resource delivery

3. **Enhanced Search**
   - ✅ Tantivy full-text indexing implemented
   - ✅ Async background indexing
   - Add relevance tuning
   - Support advanced query syntax (phrase queries, boolean operators)

4. **Performance Optimization**
   - Add configuration caching
   - Implement lazy-loading for resources
   - Optimize file scanning operations

## License

Educational use - see individual source materials for licensing.

## Attribution

All source materials are properly attributed. See `SOURCES.md` for complete bibliography.
