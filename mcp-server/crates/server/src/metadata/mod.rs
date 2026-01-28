//! Metadata extraction and management for concept cards.
//!
//! This module provides centralized metadata extraction from concept card files,
//! consolidating all metadata sources (frontmatter, markdown structure, filesystem)
//! into a single consistent interface.

mod extraction;

pub use extraction::{extract_concept_metadata, ConceptMetadata};

// v0.3.0 universal metadata support (used in FTS and source tools)
#[cfg(feature = "fts")]
pub use extraction::UniversalMetadata;
pub use extraction::{extract_metadata, ContentType};
