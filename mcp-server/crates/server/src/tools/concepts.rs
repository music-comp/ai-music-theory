use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::error::Result;
use crate::markdown::{extract_first_heading, extract_first_paragraph, extract_frontmatter};
use crate::util::files::{find_all_files, find_file_by_id, read_file, FindOptions};

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
pub async fn list_concepts(
    config: &Config,
    params: Option<ListConceptsParams>,
) -> Result<ListConceptsResponse> {
    let concept_cards_path = config.paths.concept_cards_path()?;

    if !crate::util::files::exists(&concept_cards_path).await {
        return Ok(ListConceptsResponse {
            concepts: Vec::new(),
            total: 0,
        });
    }

    let mut concepts = scan_concept_cards(&concept_cards_path).await?;

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
async fn scan_concept_cards(base_path: &Path) -> Result<Vec<ConceptInfo>> {
    let mut concepts = Vec::new();

    // Find all markdown files in the concept cards directory
    let files = find_all_files(base_path, FindOptions::markdown()).await?;

    for file_info in files {
        let path = &file_info.path;

        // Determine category from directory structure
        let category = extract_category(base_path, path);

        // Extract concept ID from filename
        let concept_id = file_info.stem.clone();

        // Try to extract title and preview from file content
        let (title, preview) = extract_title_and_preview(path).await?;

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
async fn extract_title_and_preview(path: &Path) -> Result<(String, Option<String>)> {
    let content = read_file(path).await?;

    // Extract frontmatter
    let (frontmatter, body) = extract_frontmatter(&content)?;

    // Get title from frontmatter or first heading
    let title = frontmatter
        .and_then(|fm| fm.title)
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| {
            // Fallback to filename
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .replace('-', " ")
        });

    // Extract first paragraph as preview
    let preview = extract_first_paragraph(body, 200);

    Ok((title, preview))
}

/// Get a specific concept card by ID.
pub async fn get_concept(config: &Config, concept_id: &str) -> Result<String> {
    let concept_cards_path = config.paths.concept_cards_path()?;

    // Search for the concept file
    let concept_path = find_concept_file(&concept_cards_path, concept_id).await?;

    // Read and return the content
    let content = read_file(&concept_path).await?;
    Ok(content)
}

/// Find a concept file by ID.
async fn find_concept_file(base_path: &Path, concept_id: &str) -> Result<PathBuf> {
    find_file_by_id(
        base_path,
        concept_id,
        FindOptions::markdown().with_patterns(vec!["{id}.md", "{id}/README.md"]),
    )
    .await
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
