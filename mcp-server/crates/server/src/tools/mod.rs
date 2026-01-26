pub mod concepts;
pub mod guides;
pub mod search;
pub mod sources;

// Tool function exports - Used by server.rs via #[tool] macro expansion.
// The rmcp macro system uses these via qualified paths (tools::sources::list_sources, etc.),
// so they appear unused to clippy but are actually invoked through the macro-generated code.
#[allow(unused_imports)]
pub use concepts::{get_concept, list_categories, list_concepts};
#[allow(unused_imports)]
pub use guides::{get_guide, list_guides};
#[allow(unused_imports)]
pub use search::search_concepts;
#[allow(unused_imports)]
pub use sources::{get_source_chapter, get_source_pdf_path, list_sources};

// Re-export common types for tool implementations
#[allow(unused_imports)]
pub use crate::error::{Error, Result};
