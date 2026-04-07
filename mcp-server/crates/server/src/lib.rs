//! Music Theory MCP Server library.
//!
//! This module exposes the core functionality for integration testing
//! and potential library usage.

pub mod cache;
pub mod cli;
pub mod config;
pub mod error;
#[cfg(feature = "graph")]
pub mod graph;
pub mod resources;
pub mod search;
pub mod server;
pub mod state;
pub mod tools;

// Re-export commonly used types
pub use config::{Config, LanceDbConfig};
pub use error::{Error, Result};
pub use fabryk::fts::SearchDocument;
pub use state::AppState;

// FTS-specific re-exports (only when feature enabled)
#[cfg(feature = "fts")]
pub use search::{build_index, IndexStats};
#[cfg(feature = "fts")]
pub use state::initialize_fts;
