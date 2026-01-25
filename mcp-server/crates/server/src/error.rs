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
