# Music Theory MCP Server

A Model Context Protocol (MCP) server that provides access to comprehensive music theory educational materials, including converted source texts, concept cards, and topic guides.

## Status

✅ **Compiles & Tests Pass** - Core implementation complete, tool registration in progress

### Current Implementation

- ✅ **rmcp 0.14 integrated** - Using official Rust MCP SDK (2M+ downloads)
- ✅ **Configuration management** - TOML with path expansion
- ✅ **Error handling** - Canonical pattern with backtraces (EH-17)
- ✅ **8 Tools implemented** (functions ready, registration in progress):
  - `list_sources` - List all source materials
  - `get_source_chapter` - Retrieve specific chapters
  - `get_source_pdf_path` - Get filesystem paths to PDFs/EPUBs
  - `list_concepts` - List concept cards with filtering
  - `get_concept` - Retrieve specific concepts
  - `search_concepts` - Full-text search with ranking
  - `list_guides` - List topic guides
  - `get_guide` - Retrieve specific guides
- ✅ **4 Resources implemented**:
  - `skill://conventions` - Notation conventions
  - `skill://scope` - Topics & objectives
  - `skill://sources` - Bibliography
  - `skill://index` - Complete index
- ✅ **Test suite** - 13/13 tests passing
- ✅ **MCP ServerHandler** - Basic server structure working
- ⚠️ **Tool registration** - Integration with rmcp routing in progress

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

## Building

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=info cargo run
```

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

## Next Steps

1. **MCP SDK Integration**
   - Monitor Rust MCP SDK development
   - Integrate when available
   - Wire up tools and resources

2. **Enhanced Search**
   - Implement full-text search with tantivy
   - Add ranking and relevance scoring
   - Support advanced query syntax

3. **Additional Tools**
   - `get_conventions` - Access notation conventions
   - `get_skill_doc` - Retrieve skill documentation
   - Index building and management

4. **Performance**
   - Cache configuration
   - Lazy-load resources
   - Optimize file scanning

## License

Educational use - see individual source materials for licensing.

## Attribution

All source materials are properly attributed. See `SOURCES.md` for complete bibliography.
