//! Music Theory MCP Server library.
//!
//! This module exposes the core functionality for integration testing
//! and potential library usage.

pub mod cli;
pub mod config;
pub mod error;
#[cfg(feature = "graph")]
pub mod graph;
pub mod markdown;
pub mod metadata;
pub mod resources;
pub mod search;
pub mod server;
pub mod sources;
pub mod state;
pub mod tools;
pub mod util;

// Re-export commonly used types
pub use config::Config;
pub use error::{Error, Result};
pub use search::SearchDocument;
pub use state::AppState;
pub use tools::search::{
    search_concepts, SearchConceptsParams, SearchConceptsResponse, SearchResult,
};

// FTS-specific re-exports (only when feature enabled)
#[cfg(feature = "fts")]
pub use search::{build_index, IndexStats};
#[cfg(feature = "fts")]
pub use state::initialize_fts;
