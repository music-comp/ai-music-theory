use std::backtrace::Backtrace;
use std::fmt;
use std::path::PathBuf;

use rmcp::model::{ErrorCode, ErrorData};

/// Main error type for the music theory MCP server.
pub struct Error {
    kind: ErrorKind,
    backtrace: Backtrace,
}

/// Internal error variants.
#[derive(Debug)]
pub(crate) enum ErrorKind {
    Io(std::io::Error),
    Config(String),
    NotFound {
        path: PathBuf,
    },
    NotFoundMsg {
        message: String,
    },
    InvalidPath {
        path: PathBuf,
        reason: String,
    },
    // Allow unused - will be used when markdown parsing features are implemented
    #[allow(dead_code)]
    ParseError {
        message: String,
    },
    // Allow unused - will be used when search features are fully implemented
    #[allow(dead_code)]
    SearchError {
        message: String,
    },
}

impl Error {
    pub(crate) fn io(err: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::Io(err),
            backtrace: Backtrace::capture(),
        }
    }

    pub(crate) fn io_with_path(err: std::io::Error, path: &std::path::Path) -> Self {
        // Create an IO error with path context
        let contextualized =
            std::io::Error::new(err.kind(), format!("{}: {}", path.display(), err));
        Self {
            kind: ErrorKind::Io(contextualized),
            backtrace: Backtrace::capture(),
        }
    }

    pub(crate) fn config(message: String) -> Self {
        Self {
            kind: ErrorKind::Config(message),
            backtrace: Backtrace::capture(),
        }
    }

    pub(crate) fn not_found(path: PathBuf) -> Self {
        Self {
            kind: ErrorKind::NotFound { path },
            backtrace: Backtrace::capture(),
        }
    }

    pub(crate) fn not_found_msg(message: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::NotFoundMsg {
                message: message.into(),
            },
            backtrace: Backtrace::capture(),
        }
    }

    pub(crate) fn invalid_path(path: PathBuf, reason: String) -> Self {
        Self {
            kind: ErrorKind::InvalidPath { path, reason },
            backtrace: Backtrace::capture(),
        }
    }

    // Allow unused - will be used when markdown parsing features are implemented
    #[allow(dead_code)]
    pub(crate) fn parse_error(message: String) -> Self {
        Self {
            kind: ErrorKind::ParseError { message },
            backtrace: Backtrace::capture(),
        }
    }

    // Allow unused - will be used when search features are fully implemented
    #[allow(dead_code)]
    pub(crate) fn search_error(message: String) -> Self {
        Self {
            kind: ErrorKind::SearchError { message },
            backtrace: Backtrace::capture(),
        }
    }

    /// Check if this is an I/O error.
    /// Available for error handling but not currently used in tool handlers
    #[allow(dead_code)]
    pub fn is_io(&self) -> bool {
        matches!(self.kind, ErrorKind::Io(_))
    }

    /// Check if this is a not-found error.
    pub fn is_not_found(&self) -> bool {
        matches!(
            self.kind,
            ErrorKind::NotFound { .. } | ErrorKind::NotFoundMsg { .. }
        )
    }

    /// Check if this is a configuration error.
    pub fn is_config(&self) -> bool {
        matches!(self.kind, ErrorKind::Config(_))
    }

    /// Convert internal error to MCP ErrorData with appropriate error code.
    ///
    /// Maps our internal error types to MCP protocol error codes:
    /// - NotFound errors → RESOURCE_NOT_FOUND
    /// - Config errors → INVALID_PARAMS
    /// - All other errors → INTERNAL_ERROR
    ///
    /// # Arguments
    /// * `context` - A descriptive context string (e.g., "Error listing sources")
    pub fn to_mcp_error(&self, context: &str) -> ErrorData {
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Io(e) => write!(f, "I/O error: {}", e),
            ErrorKind::Config(msg) => write!(f, "Configuration error: {}", msg),
            ErrorKind::NotFound { path } => {
                write!(f, "File not found: {}", path.display())
            }
            ErrorKind::NotFoundMsg { message } => {
                write!(f, "Not found: {}", message)
            }
            ErrorKind::InvalidPath { path, reason } => {
                write!(f, "Invalid path {}: {}", path.display(), reason)
            }
            ErrorKind::ParseError { message } => write!(f, "Parse error: {}", message),
            ErrorKind::SearchError { message } => write!(f, "Search error: {}", message),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self, self.backtrace)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::io(err)
    }
}

/// Result type alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

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
        assert!(msg.contains("I/O error"));
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
    fn test_error_not_found() {
        let path = PathBuf::from("/missing/file.txt");
        let err = Error::not_found(path.clone());
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
        assert!(err.to_string().contains("Not found"));
        assert!(err.to_string().contains("concept not found"));
    }

    #[test]
    fn test_error_invalid_path() {
        let path = PathBuf::from("/bad/path");
        let err = Error::invalid_path(path.clone(), "invalid characters".to_string());
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
        let err = Error::parse_error("syntax error at line 5".to_string());
        assert!(!err.is_io());
        assert!(!err.is_not_found());
        assert!(!err.is_config());
        assert!(err.to_string().contains("Parse error"));
        assert!(err.to_string().contains("syntax error at line 5"));
    }

    #[test]
    fn test_error_search_error() {
        let err = Error::search_error("index corrupted".to_string());
        assert!(!err.is_io());
        assert!(!err.is_not_found());
        assert!(!err.is_config());
        assert!(err.to_string().contains("Search error"));
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
        let err = Error::not_found(PathBuf::from("/test.txt"));
        let mcp_err = err.to_mcp_error("Error reading file");
        assert_eq!(mcp_err.code, rmcp::model::ErrorCode::RESOURCE_NOT_FOUND);
        assert!(mcp_err.message.contains("Not found"));
    }

    #[test]
    fn test_to_mcp_error_not_found_msg() {
        let err = Error::not_found_msg("concept not found");
        let mcp_err = err.to_mcp_error("Error finding concept");
        assert_eq!(mcp_err.code, rmcp::model::ErrorCode::RESOURCE_NOT_FOUND);
        assert!(mcp_err.message.contains("Not found"));
    }

    #[test]
    fn test_to_mcp_error_config() {
        let err = Error::config("missing field".to_string());
        let mcp_err = err.to_mcp_error("Error loading config");
        assert_eq!(mcp_err.code, rmcp::model::ErrorCode::INVALID_PARAMS);
        assert!(mcp_err.message.contains("Configuration error"));
    }

    #[test]
    fn test_to_mcp_error_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied");
        let err = Error::io(io_err);
        let mcp_err = err.to_mcp_error("Error accessing file");
        assert_eq!(mcp_err.code, rmcp::model::ErrorCode::INTERNAL_ERROR);
        assert!(mcp_err.message.contains("Error accessing file"));
    }

    #[test]
    fn test_error_debug_format() {
        let err = Error::config("test error".to_string());
        let debug_str = format!("{:?}", err);
        // Debug format should include the error message
        assert!(debug_str.contains("Configuration error"));
        assert!(debug_str.contains("test error"));
    }

    #[test]
    fn test_error_display_all_variants() {
        // Test Display for each ErrorKind variant
        let errors = vec![
            Error::io(std::io::Error::new(std::io::ErrorKind::NotFound, "io")),
            Error::config("config".to_string()),
            Error::not_found(PathBuf::from("/path")),
            Error::not_found_msg("msg"),
            Error::invalid_path(PathBuf::from("/path"), "reason".to_string()),
            Error::parse_error("parse".to_string()),
            Error::search_error("search".to_string()),
        ];

        for err in errors {
            let display = err.to_string();
            assert!(!display.is_empty(), "Display should produce non-empty string");
        }
    }
}
