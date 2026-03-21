# Music Theory AI Skill - MCP Server

A Rust workspace providing a Model Context Protocol (MCP) server for accessing comprehensive music theory educational materials.

## Project Structure

This is a Rust workspace with the following crates:

- **`crates/server/`** - The main MCP server implementation
  - See [`crates/server/README.md`](crates/server/README.md) for details
  - See [`crates/server/STATUS.md`](crates/server/STATUS.md) for current status

- **`crates/design/`** - Design documentation and architectural decision records
  - Contains design docs, planning documents, and ADRs

## Quick Start

### Building

```bash
# Build the entire workspace
cargo build

# Build just the server
cargo build -p music-theory-mcp
```

### Testing

```bash
# Run all tests
cargo test

# Run server tests only
cargo test -p music-theory-mcp
```

### Running the Server

```bash
# Run from workspace root
cargo run -p music-theory-mcp

# Or from the server crate directory
cd crates/server
cargo run
```

### Integrating with Claude Desktop

The MCP server can be integrated with Claude Desktop using the stdio transport. You can run it in development mode (using `cargo run`) or production mode (using a compiled binary).

For complete step-by-step instructions including configuration, verification, and troubleshooting, see the **[Using with Claude Desktop](crates/server/README.md#using-with-claude-desktop)** section in the server documentation.

## Status

✅ **Fully Functional** - All core features implemented and working

- **34 MCP tools** registered and operational (10 base + 9 music theory computation + 15 graph)
- **Graph database** for exploring concept relationships (v0.3.0)
- **Full-text search** with Tantivy backend (v0.2.0)
- Beautiful structured logging with twyg
- Configuration management with confyg
- 360+ tests passing

See [`crates/server/STATUS.md`](crates/server/STATUS.md) for detailed status and [`crates/server/README.md`](crates/server/README.md) for complete documentation.

## Dependencies

This workspace uses the following key dependencies:

- **rmcp 0.14** - Official Rust MCP SDK
- **confyg 0.3** - TOML + ENV configuration
- **twyg 0.6** - Beautiful colored logging
- **tokio** - Async runtime
- **tantivy 0.22** - Full-text search engine (optional, with `fts` feature)
- **petgraph 0.6** - Graph data structures and algorithms (optional, with `graph` feature)
- **rkyv 0.8** - Zero-copy deserialization for graph caching (optional, with `graph` feature)

See the workspace `Cargo.toml` for the complete dependency list.

## License

Apache-2.0

## Author

Duncan McGreggor <oubiwann@gmail.com>
