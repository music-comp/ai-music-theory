//! Error types for the music theory MCP server.
//!
//! Re-exports `fabryk::core::Error` and `fabryk::core::Result` as the primary
//! error types. Adds MCP-specific error mapping via `McpErrorContextExt`.

use fabryk_mcp::model::{ErrorCode, ErrorData};

// Re-export fabryk error types as crate-level error types.
pub use fabryk::core::Error;
pub use fabryk::core::Result;

/// Extension trait for converting errors to MCP ErrorData with context.
///
/// Extends fabryk's `McpErrorExt` (which provides no-context `to_mcp_error()`)
/// with a context-aware variant used throughout the server's tool handlers.
// TODO: Phase 3 — consider upstreaming to fabryk_mcp_core::McpErrorExt
pub trait McpErrorContextExt {
    /// Convert to MCP ErrorData with a descriptive context string.
    ///
    /// Maps error types to MCP protocol error codes:
    /// - NotFound/FileNotFound → RESOURCE_NOT_FOUND
    /// - Config → INVALID_PARAMS
    /// - All other errors → INTERNAL_ERROR
    fn to_mcp_error(&self, context: &str) -> ErrorData;
}

impl McpErrorContextExt for Error {
    fn to_mcp_error(&self, context: &str) -> ErrorData {
        let (code, msg) = if self.is_not_found() {
            (
                ErrorCode::RESOURCE_NOT_FOUND,
                format!("Not found: {}", self),
            )
        } else if self.is_config() {
            (
                ErrorCode::INVALID_PARAMS,
                format!("Configuration error: {}", self),
            )
        } else {
            (ErrorCode::INTERNAL_ERROR, format!("{}: {}", context, self))
        };
        ErrorData::new(code, msg, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_to_mcp_error_not_found() {
        let err = Error::file_not_found(PathBuf::from("/test.txt"));
        let mcp_err = err.to_mcp_error("Error reading file");
        assert_eq!(mcp_err.code, ErrorCode::RESOURCE_NOT_FOUND);
        assert!(mcp_err.message.contains("Not found"));
    }

    #[test]
    fn test_to_mcp_error_config() {
        let err = Error::config("missing field".to_string());
        let mcp_err = err.to_mcp_error("Error loading config");
        assert_eq!(mcp_err.code, ErrorCode::INVALID_PARAMS);
        assert!(mcp_err.message.contains("Configuration error"));
    }

    #[test]
    fn test_to_mcp_error_internal() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let err = Error::io(io_err);
        let mcp_err = err.to_mcp_error("Error accessing file");
        assert_eq!(mcp_err.code, ErrorCode::INTERNAL_ERROR);
        assert!(mcp_err.message.contains("Error accessing file"));
    }
}
