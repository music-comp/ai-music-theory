pub mod concepts;
pub mod guides;
pub mod health;
pub mod questions;
pub mod search;
pub mod sources;

pub mod graph;
pub mod graph_query;
pub mod semantic;

// Tool function exports - Used by server.rs via #[tool] macro expansion.
// The rmcp macro system uses these via qualified paths (tools::sources::list_sources, etc.),
// so they appear unused to clippy but are actually invoked through the macro-generated code.
pub use concepts::{get_concept, list_categories, list_concepts};
pub use graph::{get_node, get_node_edges, graph_stats, graph_status, graph_validate};
pub use graph_query::{
    find_bridge_concepts, find_concept_path, get_central_concepts, get_concept_neighborhood,
    get_concept_sources, get_concept_variants, get_dependents, get_learning_path,
    get_prerequisites, get_related_concepts, get_source_coverage,
};
pub use guides::{get_guide, list_guides};
pub use health::get_health;
pub use questions::search_by_question;
pub use search::search_concepts;
pub use semantic::semantic_search;
pub use sources::{get_source_chapter, get_source_pdf_path, list_sources};

// Re-export common types for tool implementations
pub use crate::error::{Error, Result};
