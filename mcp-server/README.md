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

## Status

✅ **Fully Functional** - All core features implemented and working

- 8 MCP tools registered and operational
- Beautiful structured logging with twyg
- Configuration management with confyg
- All tests passing

See [`crates/server/STATUS.md`](crates/server/STATUS.md) for detailed status.

## Dependencies

This workspace uses the following key dependencies:

- **rmcp 0.14** - Official Rust MCP SDK
- **confyg 0.3** - TOML + ENV configuration
- **twyg 0.6** - Beautiful colored logging
- **tokio** - Async runtime

See the workspace `Cargo.toml` for the complete dependency list.

## License

Apache-2.0

## Author

Duncan McGreggor <oubiwann@gmail.com>
