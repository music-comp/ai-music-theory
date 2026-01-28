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
//! - **algorithms**: Graph traversal algorithms (BFS, DFS, etc.)
//! - **query**: Response types for MCP query tools
//! - **validation**: Graph integrity checks
//! - **stats**: Statistics computation
//! - **cli**: CLI command handlers
//!
//! # Feature Gate
//!
//! This module is feature-gated with `#[cfg(feature = "graph")]` and only compiles
//! when the `graph` feature is enabled.

pub mod algorithms;
pub mod builder;
pub mod cli;
pub mod loader;
pub mod parser;
pub mod persistence;
pub mod query;
pub mod stats;
pub mod types;
pub mod validation;

// Re-export commonly used types
#[allow(unused_imports)] // Public API re-exports
pub use algorithms::{
    bridge_concepts, degree_centrality, dependents, neighborhood, prerequisites_sorted,
    shortest_path,
};
#[allow(unused_imports)]
pub use builder::GraphBuilder;
#[allow(unused_imports)]
pub use loader::{load_concept_graph, GraphStats, LoadedGraph};
#[allow(unused_imports)]
pub use parser::{parse_related_concepts, RelatedConcepts};
#[allow(unused_imports)]
pub use persistence::{load_graph, save_graph, to_petgraph, ConceptGraph};
#[allow(unused_imports)]
pub use query::{
    BridgeConceptsResponse, CentralConceptsResponse, ConceptPathResponse, ConceptSourcesResponse,
    ConceptVariantsResponse, DependentsResponse, NeighborhoodResponse, PrerequisitesResponse,
    RelatedConceptsResponse, SourceCoverageResponse,
};
#[allow(unused_imports)]
pub use stats::{
    compute_all_stats, compute_degree_stats, count_by_category, count_by_relationship,
    count_nodes_by_type, CategoryCount, DegreeStats, GraphStatistics, NodeCounts,
    RelationshipCount,
};
#[allow(unused_imports)]
pub use types::{
    ConceptNode, Edge, EdgeOrigin, GraphData, GraphMetadata, Node, Relationship, SourceNode,
};
#[allow(unused_imports)]
pub use validation::{validate_graph, ValidationResults};
