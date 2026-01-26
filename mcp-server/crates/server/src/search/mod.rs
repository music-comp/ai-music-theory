//! Search functionality for concept cards.
//!
//! This module provides search document representation and search utilities
//! for both simple search and Tantivy-based full-text search.

mod document;
mod schema;

pub use document::SearchDocument;

// SearchSchema is used internally by search backends, not exposed publicly yet
#[allow(unused_imports)]
pub(crate) use schema::SearchSchema;
