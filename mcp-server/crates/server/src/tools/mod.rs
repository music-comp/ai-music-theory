pub mod concepts;
pub mod guides;
pub mod search;
pub mod sources;

// Allow unused imports - these are part of the planned API and will be used
// when the full tool registration is implemented
#[allow(unused_imports)]
pub use concepts::{get_concept, list_concepts};
#[allow(unused_imports)]
pub use guides::{get_guide, list_guides};
#[allow(unused_imports)]
pub use search::search_concepts;
#[allow(unused_imports)]
pub use sources::{get_source_chapter, get_source_pdf_path, list_sources};

// Re-export common types
#[allow(unused_imports)]
pub use crate::error::{Error, Result};
