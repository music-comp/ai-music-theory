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
