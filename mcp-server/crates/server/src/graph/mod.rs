//! Graph database module for concept relationships.
//!
//! This module provides a graph representation of music theory concepts and their relationships.
//! It extracts relationship information from concept cards and builds a queryable graph structure.
//!
//! # Architecture
//!
//! - **types**: Core data structures (Node, Edge, Relationship)
//! - **parser**: Extracts relationships from markdown
//! - **builder**: Builds graph from concept cards
//! - **persistence**: Saves/loads graph with rkyv caching
//! - **loader**: Async loading for AppState
//! - **validation**: Graph integrity checks
//! - **stats**: Statistics computation
//! - **cli**: CLI command handlers
//!
//! # Feature Gate
//!
//! This module is feature-gated with `#[cfg(feature = "graph")]` and only compiles
//! when the `graph` feature is enabled.

pub mod types;
pub mod parser;
pub mod builder;
pub mod persistence;
pub mod loader;
pub mod cli;

// Re-export commonly used types
pub use types::{
    ConceptNode, Edge, EdgeOrigin, GraphData, GraphMetadata, Node, Relationship, SourceNode,
};
pub use parser::{parse_related_concepts, RelatedConcepts};
pub use builder::GraphBuilder;
pub use persistence::{load_graph, save_graph, to_petgraph, ConceptGraph};
pub use loader::{load_concept_graph, GraphStats, LoadedGraph};
