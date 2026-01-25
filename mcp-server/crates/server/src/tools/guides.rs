use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::error::Result;
use crate::markdown::{extract_first_heading, extract_first_paragraph, extract_frontmatter};
use crate::util::files::{find_all_files, find_file_by_id, read_file, FindOptions};

/// Information about a guide document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuideInfo {
    pub id: String,
    pub title: String,
    pub topic: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Response for list_guides tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListGuidesResponse {
    pub guides: Vec<GuideInfo>,
    pub total: usize,
}

/// List all available topic guides.
pub async fn list_guides(config: &Config) -> Result<ListGuidesResponse> {
    let guides_path = config.paths.guides_path()?;

    if !crate::util::files::exists(&guides_path).await {
        return Ok(ListGuidesResponse {
            guides: Vec::new(),
            total: 0,
        });
    }

    let guides = scan_guides(&guides_path).await?;
    let total = guides.len();

    Ok(ListGuidesResponse { guides, total })
}

/// Scan the guides directory for available guides.
async fn scan_guides(base_path: &Path) -> Result<Vec<GuideInfo>> {
    let mut guides = Vec::new();

    // Find all markdown files in the guides directory
    let files = find_all_files(base_path, FindOptions::markdown()).await?;

    for file_info in files {
        let path = &file_info.path;

        // Extract topic from directory structure
        let topic = extract_topic(base_path, path);

        // Extract guide ID from filename
        let guide_id = file_info.stem.clone();

        // Extract title and description from file content
        let (title, description) = extract_title_and_description(path).await?;

        guides.push(GuideInfo {
            id: guide_id,
            title,
            topic,
            path: path.to_string_lossy().to_string(),
            description,
        });
    }

    // Sort by topic and then by title
    guides.sort_by(|a, b| a.topic.cmp(&b.topic).then_with(|| a.title.cmp(&b.title)));

    Ok(guides)
}

/// Extract topic from the path relative to base.
fn extract_topic(base: &Path, file_path: &Path) -> String {
    // Safety: unwrap_or provides sensible display fallback if topic extraction fails
    file_path
        .parent()
        .and_then(|parent| parent.strip_prefix(base).ok())
        .and_then(|relative| relative.components().next())
        .and_then(|component| component.as_os_str().to_str())
        .unwrap_or("general")
        .to_string()
}

/// Extract title and description from markdown file.
async fn extract_title_and_description(path: &Path) -> Result<(String, Option<String>)> {
    let content = read_file(path).await?;

    // Extract frontmatter
    let (frontmatter, body) = extract_frontmatter(&content)?;

    // Get title from frontmatter or first heading
    let title = frontmatter
        .as_ref()
        .and_then(|fm| fm.title.clone())
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| {
            // Fallback to filename
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled Guide")
                .replace(['-', '_'], " ")
        });

    // Get description from frontmatter or first paragraph
    let description = frontmatter
        .and_then(|fm| fm.description)
        .or_else(|| extract_first_paragraph(body, 300));

    Ok((title, description))
}

/// Get a specific guide by ID.
pub async fn get_guide(config: &Config, guide_id: &str) -> Result<String> {
    let guides_path = config.paths.guides_path()?;

    // Search for the guide file
    let guide_path = find_guide_file(&guides_path, guide_id).await?;

    // Read and return the content
    let content = read_file(&guide_path).await?;
    Ok(content)
}

/// Find a guide file by ID.
async fn find_guide_file(base_path: &Path, guide_id: &str) -> Result<PathBuf> {
    find_file_by_id(
        base_path,
        guide_id,
        FindOptions::markdown().with_patterns(vec!["{id}.md", "{id}/README.md", "{id}/index.md"]),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_topic() {
        let base = PathBuf::from("/guides");
        let file_path = PathBuf::from("/guides/harmony/chord-progressions.md");
        let topic = extract_topic(&base, &file_path);
        assert_eq!(topic, "harmony");
    }

    #[test]
    fn test_extract_topic_root() {
        let base = PathBuf::from("/guides");
        let file_path = PathBuf::from("/guides/introduction.md");
        let topic = extract_topic(&base, &file_path);
        assert_eq!(topic, "general");
    }
}
