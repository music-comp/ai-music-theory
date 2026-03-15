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
/// This extends fabryk's `McpErrorExt` trait (which provides a no-context
/// `to_mcp_error()`) with a context-aware variant used throughout the
/// server's tool handlers.
pub trait McpErrorContextExt {
    /// Convert to MCP ErrorData with a descriptive context string.
    ///
    /// Maps error types to MCP protocol error codes:
    /// - NotFound/FileNotFound → RESOURCE_NOT_FOUND
    /// - Config → INVALID_PARAMS
    /// - All other errors → INTERNAL_ERROR
    ///
    /// # Arguments
    /// * `context` - A descriptive context string (e.g., "Error listing sources")
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
    use std::error::Error as StdError;
    use std::path::PathBuf;

    #[test]
    fn test_error_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = Error::io(io_err);
        assert!(err.is_io());
        assert!(!err.is_not_found());
        assert!(!err.is_config());
        assert!(err.to_string().contains("I/O error"));
    }

    #[test]
    fn test_error_io_with_path() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
        let path = PathBuf::from("/test/path.txt");
        let err = Error::io_with_path(io_err, &path);
        assert!(err.is_io());
        let msg = err.to_string();
        assert!(msg.contains("/test/path.txt"));
        assert!(msg.contains("permission denied"));
    }

    #[test]
    fn test_error_config() {
        let err = Error::config("invalid configuration".to_string());
        assert!(err.is_config());
        assert!(!err.is_io());
        assert!(!err.is_not_found());
        assert!(err.to_string().contains("Configuration error"));
        assert!(err.to_string().contains("invalid configuration"));
    }

    #[test]
    fn test_error_file_not_found() {
        let path = PathBuf::from("/missing/file.txt");
        let err = Error::file_not_found(path);
        assert!(err.is_not_found());
        assert!(!err.is_io());
        assert!(!err.is_config());
        let msg = err.to_string();
        assert!(msg.contains("File not found"));
        assert!(msg.contains("/missing/file.txt"));
    }

    #[test]
    fn test_error_not_found_msg() {
        let err = Error::not_found_msg("concept not found");
        assert!(err.is_not_found());
        assert!(!err.is_io());
        assert!(!err.is_config());
        let msg = err.to_string();
        assert!(msg.contains("not found"));
        assert!(msg.contains("concept not found"));
    }

    #[test]
    fn test_error_invalid_path() {
        let path = PathBuf::from("/bad/path");
        let err = Error::invalid_path(path, "invalid characters".to_string());
        assert!(!err.is_io());
        assert!(!err.is_not_found());
        assert!(!err.is_config());
        let msg = err.to_string();
        assert!(msg.contains("Invalid path"));
        assert!(msg.contains("/bad/path"));
        assert!(msg.contains("invalid characters"));
    }

    #[test]
    fn test_error_parse_error() {
        let err = Error::parse("syntax error at line 5");
        assert!(err.is_parse());
        assert!(!err.is_io());
        assert!(!err.is_not_found());
        assert!(!err.is_config());
        assert!(err.to_string().contains("Parse error"));
        assert!(err.to_string().contains("syntax error at line 5"));
    }

    #[test]
    fn test_error_search_error() {
        let err = Error::operation("index corrupted");
        assert!(!err.is_io());
        assert!(!err.is_not_found());
        assert!(!err.is_config());
        assert!(err.to_string().contains("index corrupted"));
    }

    #[test]
    fn test_error_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "connection lost");
        let err: Error = io_err.into();
        assert!(err.is_io());
        assert!(err.to_string().contains("connection lost"));
    }

    #[test]
    fn test_error_source_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let err = Error::io(io_err);
        assert!(err.source().is_some());
    }

    #[test]
    fn test_error_source_non_io() {
        let err = Error::config("test".to_string());
        assert!(err.source().is_none());
    }

    #[test]
    fn test_to_mcp_error_not_found() {
        let err = Error::file_not_found(PathBuf::from("/test.txt"));
        let mcp_err = err.to_mcp_error("Error reading file");
        assert_eq!(mcp_err.code, ErrorCode::RESOURCE_NOT_FOUND);
        assert!(mcp_err.message.contains("Not found"));
    }

    #[test]
    fn test_to_mcp_error_not_found_msg() {
        let err = Error::not_found_msg("concept not found");
        let mcp_err = err.to_mcp_error("Error finding concept");
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
    fn test_to_mcp_error_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let err = Error::io(io_err);
        let mcp_err = err.to_mcp_error("Error accessing file");
        assert_eq!(mcp_err.code, ErrorCode::INTERNAL_ERROR);
        assert!(mcp_err.message.contains("Error accessing file"));
    }

    #[test]
    fn test_error_debug_format() {
        let err = Error::config("test error".to_string());
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Config"));
        assert!(debug_str.contains("test error"));
    }

    #[test]
    fn test_error_display_all_variants() {
        let errors: Vec<Error> = vec![
            Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "io")),
            Error::io_with_path(
                std::io::Error::new(std::io::ErrorKind::NotFound, "io"),
                "/path",
            ),
            Error::config("config".to_string()),
            Error::not_found("Type", "id"),
            Error::file_not_found("/path"),
            Error::invalid_path("/path", "reason"),
            Error::parse("parse"),
            Error::operation("operation"),
        ];

        for err in errors {
            let display = err.to_string();
            assert!(
                !display.is_empty(),
                "Display should produce non-empty string"
            );
        }
    }
}
