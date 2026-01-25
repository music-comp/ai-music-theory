use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::Config;
use crate::error::{Error, Result};

/// Information about a concept card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptInfo {
    pub id: String,
    pub title: String,
    pub category: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
}

/// Response for list_concepts tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListConceptsResponse {
    pub concepts: Vec<ConceptInfo>,
    pub total: usize,
}

/// Parameters for list_concepts tool.
#[derive(Debug, Deserialize)]
pub struct ListConceptsParams {
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub limit: Option<usize>,
}

/// List all available concept cards, optionally filtered by category.
pub fn list_concepts(
    config: &Config,
    params: Option<ListConceptsParams>,
) -> Result<ListConceptsResponse> {
    let concept_cards_path = config.paths.concept_cards_path()?;

    if !concept_cards_path.exists() {
        return Ok(ListConceptsResponse {
            concepts: Vec::new(),
            total: 0,
        });
    }

    let mut concepts = scan_concept_cards(&concept_cards_path)?;

    // Filter by category if specified
    if let Some(params) = params {
        if let Some(category) = params.category {
            concepts.retain(|c| c.category == category);
        }

        // Apply limit if specified
        if let Some(limit) = params.limit {
            concepts.truncate(limit);
        }
    }

    let total = concepts.len();

    Ok(ListConceptsResponse { concepts, total })
}

/// Scan the concept cards directory.
fn scan_concept_cards(base_path: &Path) -> Result<Vec<ConceptInfo>> {
    let mut concepts = Vec::new();

    // Walk through all markdown files in the concept cards directory
    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
    {
        let path = entry.path();

        // Determine category from directory structure
        let category = extract_category(base_path, path);

        // Extract concept ID from filename
        // Safety: unwrap_or provides sensible display fallback if filename extraction fails
        let concept_id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Try to extract title and preview from file content
        let (title, preview) = extract_title_and_preview(path)?;

        concepts.push(ConceptInfo {
            id: concept_id,
            title,
            category,
            path: path.to_string_lossy().to_string(),
            preview,
        });
    }

    // Sort by category and then by title
    concepts.sort_by(|a, b| {
        a.category
            .cmp(&b.category)
            .then_with(|| a.title.cmp(&b.title))
    });

    Ok(concepts)
}

/// Extract category from the path relative to base.
fn extract_category(base: &Path, file_path: &Path) -> String {
    // Safety: unwrap_or provides sensible display fallback if category extraction fails
    file_path
        .parent()
        .and_then(|parent| parent.strip_prefix(base).ok())
        .and_then(|relative| relative.components().next())
        .and_then(|component| component.as_os_str().to_str())
        .unwrap_or("uncategorized")
        .to_string()
}

/// Extract title and preview from markdown file.
fn extract_title_and_preview(path: &Path) -> Result<(String, Option<String>)> {
    let content = fs::read_to_string(path)?;

    let mut title = String::new();
    let mut preview = String::new();
    let mut in_frontmatter = false;
    let mut frontmatter_count = 0;

    for line in content.lines().take(20) {
        // Handle YAML frontmatter
        if line.trim() == "---" {
            frontmatter_count += 1;
            if frontmatter_count == 1 {
                in_frontmatter = true;
                continue;
            } else if frontmatter_count == 2 {
                in_frontmatter = false;
                continue;
            }
        }

        if in_frontmatter {
            if let Some(stripped) = line.strip_prefix("title:") {
                title = stripped.trim().trim_matches('"').to_string();
            }
            continue;
        }

        // Extract first heading as title if not in frontmatter
        if title.is_empty() && line.starts_with('#') {
            title = line.trim_start_matches('#').trim().to_string();
            continue;
        }

        // Collect first paragraph as preview
        if !title.is_empty() && !line.trim().is_empty() && preview.len() < 200 {
            preview.push_str(line);
            preview.push(' ');
        }

        if preview.len() >= 200 {
            break;
        }
    }

    // Use filename as fallback title
    if title.is_empty() {
        // Safety: unwrap_or provides sensible display fallback if filename extraction fails
        title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .replace('-', " ");
    }

    let preview_opt = if preview.is_empty() {
        None
    } else {
        Some(preview.trim().to_string())
    };

    Ok((title, preview_opt))
}

/// Get a specific concept card by ID.
pub fn get_concept(config: &Config, concept_id: &str) -> Result<String> {
    let concept_cards_path = config.paths.concept_cards_path()?;

    // Search for the concept file
    let concept_path = find_concept_file(&concept_cards_path, concept_id)?;

    // Read and return the content
    let content = fs::read_to_string(&concept_path)?;
    Ok(content)
}

/// Find a concept file by ID.
fn find_concept_file(base_path: &Path, concept_id: &str) -> Result<PathBuf> {
    // Try common extensions and naming patterns
    let patterns = vec![
        format!("{}.md", concept_id),
        format!("{}/README.md", concept_id),
        concept_id.to_string(),
    ];

    for pattern in patterns {
        let path = base_path.join(&pattern);
        if path.exists() {
            return Ok(path);
        }
    }

    // Search recursively
    for entry in WalkDir::new(base_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            if stem == concept_id {
                return Ok(path.to_path_buf());
            }
        }
    }

    Err(Error::not_found(base_path.join(concept_id)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_category() {
        let base = PathBuf::from("/concepts");
        let file_path = PathBuf::from("/concepts/harmony/triads.md");
        let category = extract_category(&base, &file_path);
        assert_eq!(category, "harmony");
    }

    #[test]
    fn test_extract_category_nested() {
        let base = PathBuf::from("/concepts");
        let file_path = PathBuf::from("/concepts/harmony/advanced/neo-riemannian.md");
        let category = extract_category(&base, &file_path);
        assert_eq!(category, "harmony");
    }
}
