//! Search functionality for concept cards.
//!
//! This module provides search document representation and search utilities
//! for both simple search and Tantivy-based full-text search.

mod builder;
mod document;
mod indexer;
mod schema;

// Public exports for Phase 5+ (index lifecycle)
#[allow(unused_imports)]
pub use builder::{build_index, IndexStats};
pub use document::SearchDocument;

// Internal modules used by search backends
#[allow(unused_imports)]
pub(crate) use indexer::Indexer;
#[allow(unused_imports)]
pub(crate) use schema::SearchSchema;
