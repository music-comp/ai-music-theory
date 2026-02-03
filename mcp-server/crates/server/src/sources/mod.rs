//! Source management module for validating and managing source references.
//!
//! This module provides tools to:
//! - Scan concept cards for source references
//! - Validate references against configuration and filesystem
//! - Manage source title aliases
//! - Resolve source references with fuzzy matching

pub mod cli;
pub mod resolver;
pub mod scanner;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use resolver::SourceResolver;
pub use scanner::{scan_concept_cards, scan_concept_cards_with_stats, ScanStats};
pub use types::{
    MissingFile, MissingSource, ResolutionMethod, SourceReference, SourceSuggestion,
    ValidationReport, ValidationStats,
};
pub use validator::validate_sources;
