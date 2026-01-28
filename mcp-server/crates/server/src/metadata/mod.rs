//! Metadata extraction and management for concept cards.
//!
//! This module provides centralized metadata extraction from concept card files,
//! consolidating all metadata sources (frontmatter, markdown structure, filesystem)
//! into a single consistent interface.

mod extraction;

pub use extraction::{
    extract_concept_metadata, extract_metadata, ConceptMetadata, ContentType, UniversalMetadata,
};
