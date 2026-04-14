//! Error types for the music theory MCP server.
//!
//! Re-exports `fabryk::core::Error` and `fabryk::core::Result` as the primary
//! error types. Re-exports `McpErrorContextExt` from fabryk-mcp-core for
//! context-aware MCP error mapping.

// Re-export fabryk error types as crate-level error types.
pub use fabryk::core::Error;
pub use fabryk::core::Result;

// Re-export MCP error context trait from fabryk.
pub use fabryk_mcp::McpErrorContextExt;
