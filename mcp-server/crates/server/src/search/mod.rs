//! Search functionality for concept cards.
//!
//! This module provides search document representation and search utilities
//! for both simple search and Tantivy-based full-text search.

pub mod backend;
mod builder;
mod document;
mod indexer;
mod query;
mod schema;
mod simple_search;
mod tantivy_search;

// Public exports
pub use backend::create_search_backend;
#[allow(unused_imports)]
pub use builder::{build_index, IndexStats};
pub use document::SearchDocument;

// Internal modules used by search backends
#[allow(unused_imports)]
pub(crate) use indexer::Indexer;
#[allow(unused_imports)]
pub(crate) use query::QueryBuilder;
#[allow(unused_imports)]
pub(crate) use schema::SearchSchema;
#[allow(unused_imports)]
pub(crate) use simple_search::SimpleSearch;
#[allow(unused_imports)]
pub(crate) use tantivy_search::TantivySearch;
