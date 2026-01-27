# Music Theory MCP Server

A Model Context Protocol (MCP) server that provides access to comprehensive music theory educational materials, including converted source texts, concept cards, and topic guides.

## Status

✅ **Fully Functional** - All core features implemented and working

### Current Implementation

- ✅ **rmcp 0.14 integrated** - Using official Rust MCP SDK (2M+ downloads)
- ✅ **Configuration management** - confyg for TOML + ENV with path expansion
- ✅ **Logging** - twyg for beautiful colored output
- ✅ **Error handling** - Canonical pattern with backtraces (EH-17)
- ✅ **10 Tools registered and working**:
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
- ✅ **4 Resources implemented** (ready for registration):
  - `skill://conventions` - Notation conventions
  - `skill://scope` - Topics & objectives
  - `skill://sources` - Bibliography
  - `skill://index` - Complete index
- ✅ **Test suite** - 324+ tests passing (with FTS feature)
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

The server supports two search backends for the `search_concepts` tool:

- **Simple** (default) - Linear scan, always available, suitable for <500 concept cards
- **Tantivy** - Full-text search engine, optional feature, recommended for 500+ concept cards

Configure the search backend in `config/default.toml`:

```toml
[search]
# Backend selection: "simple" or "tantivy"
# Note: "tantivy" requires building with --features fts
backend = "simple"

# Tantivy index directory (relative to skill root)
index_path = ".tantivy-index"

# Rebuild index on startup
# With async background indexing, this is less critical:
# - false (recommended): Builds only if missing or stale
# - true: Forces rebuild on every startup
rebuild_on_startup = false

# Snippet context size in characters
snippet_size = 200

# Enable fuzzy search (typo tolerance, requires --features fts)
fuzzy_search = false

# Maximum edit distance for fuzzy matching (1-2)
fuzzy_distance = 2
```

**Using Tantivy Search (Optional Feature):**

Tantivy is an optional feature that must be enabled at build time:

```bash
# Build with FTS support
cargo build --release --features fts

# Or run with FTS support
cargo run --features fts
```

**Setting up Tantivy:**

1. Build with `--features fts`
2. Set `backend = "tantivy"` in config
3. Build the index using the CLI:
   ```bash
   ./target/release/music-theory-mcp index
   ```
4. Start the server (index loads instantly):
   ```bash
   ./target/release/music-theory-mcp serve
   # or just: ./target/release/music-theory-mcp
   ```

**Non-Blocking Indexing:**

The server uses async background indexing:
- Server starts immediately (<1 second)
- Index builds in the background if needed
- Simple search available during indexing
- Automatic switch to FTS when ready
- Index freshness checked via content hash (no unnecessary rebuilds)

**CLI Commands:**

```bash
# Serve (default command)
music-theory-mcp serve

# Build/rebuild index (with FTS feature)
music-theory-mcp index
music-theory-mcp index --force  # Force rebuild even if fresh

# Show index status (with FTS feature)
music-theory-mcp status
```

**Benefits of Tantivy:**
- 10-100x faster search (sub-millisecond queries)
- Typo tolerance with fuzzy search
- Phrase queries and boolean operators (future)
- Scales to 10,000+ documents
- Background indexing (non-blocking startup)
- Content hash tracking (avoid unnecessary rebuilds)

**When to Switch:**
- Simple backend works well up to ~500 concept cards
- Switch to Tantivy when search latency becomes noticeable
- The index requires ~1-2 seconds to build for 200 documents (background)

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

## Building

```bash
# Build without FTS (simple search only, smaller binary)
cargo build
cargo build --release

# Build with FTS support (optional Tantivy backend)
cargo build --features fts
cargo build --release --features fts

# Run tests
cargo test                    # Without FTS
cargo test --features fts     # With FTS

# Run with logging
RUST_LOG=info cargo run
RUST_LOG=info cargo run --features fts
```

**Binary Size:**
- Without FTS: ~2.6M (simple search only)
- With FTS: ~6.3M (both backends available)

**Feature Flags:**
- `fts` - Enables Tantivy full-text search backend (optional)

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
   # Without FTS (smaller, simple search only)
   cargo build --release

   # With FTS (full-text search support)
   cargo build --release --features fts
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
3. Try using one of the 10 available tools:
   - `health` - Check server status and active search backend
   - `list_concepts` - List all concept cards
   - `list_categories` - Browse concepts by category
   - `search_concepts` - Search for specific topics
   - `list_guides` - Browse topic guides
   - `get_source_chapter` - Access source material chapters

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
