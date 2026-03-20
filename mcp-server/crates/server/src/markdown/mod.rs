//! Markdown parsing utilities using pulldown-cmark and yaml_serde.
//!
//! This module provides centralized markdown processing functionality:
//! - YAML frontmatter extraction and deserialization
//! - Heading and paragraph extraction
//! - Text content extraction for search and indexing
//!
//! All markdown files in the server use these utilities for consistency.

mod frontmatter;
mod parser;

pub use frontmatter::{extract_frontmatter, extract_frontmatter_with_path, strip_frontmatter};
pub use parser::{extract_first_heading, extract_first_paragraph};
