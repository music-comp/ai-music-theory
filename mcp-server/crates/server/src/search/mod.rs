//! Search functionality for concept cards.
//!
//! This module provides search document representation and search utilities
//! for both simple search and Tantivy-based full-text search.

pub mod backend;
mod document;
mod simple_search;

// FTS-specific modules (only when feature enabled)
#[cfg(feature = "fts")]
mod builder;
#[cfg(feature = "fts")]
pub(crate) mod freshness;
#[cfg(feature = "fts")]
mod indexer;
#[cfg(feature = "fts")]
mod query;
#[cfg(feature = "fts")]
mod schema;
#[cfg(feature = "fts")]
mod stopwords;
#[cfg(feature = "fts")]
mod tantivy_search;

// Public exports
#[allow(unused_imports)]
pub use backend::SearchBackend;
pub use document::SearchDocument;
pub use simple_search::SimpleSearch;

// FTS-specific exports (only when feature enabled)
#[cfg(feature = "fts")]
pub use builder::{build_index, IndexStats};
#[cfg(feature = "fts")]
#[allow(unused_imports)]
pub use stopwords::StopwordFilter;
#[cfg(feature = "fts")]
pub use tantivy_search::TantivySearch;

// Internal FTS modules
#[cfg(feature = "fts")]
pub(crate) use freshness::{compute_content_hash, is_index_fresh, save_metadata, IndexMetadata};
#[cfg(feature = "fts")]
pub(crate) use indexer::Indexer;
#[cfg(feature = "fts")]
pub(crate) use query::QueryBuilder;
#[cfg(feature = "fts")]
pub(crate) use schema::SearchSchema;
