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
    use tempfile::TempDir;
    use tokio::fs;

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

    #[test]
    fn test_extract_topic_nested() {
        let base = PathBuf::from("/guides");
        let file_path = PathBuf::from("/guides/harmony/advanced/modal-interchange.md");
        let topic = extract_topic(&base, &file_path);
        // Should extract first level only
        assert_eq!(topic, "harmony");
    }

    #[test]
    fn test_extract_topic_no_parent() {
        let base = PathBuf::from("/guides");
        let file_path = PathBuf::from("guide.md");
        let topic = extract_topic(&base, &file_path);
        assert_eq!(topic, "general");
    }

    #[test]
    fn test_guide_info_serialization() {
        let info = GuideInfo {
            id: "test-guide".to_string(),
            title: "Test Guide".to_string(),
            topic: "harmony".to_string(),
            path: "/path/to/guide.md".to_string(),
            description: Some("This is a description".to_string()),
        };

        let json = serde_json::to_string(&info).expect("Should serialize");
        assert!(json.contains("test-guide"));
        assert!(json.contains("Test Guide"));
        assert!(json.contains("harmony"));
        assert!(json.contains("This is a description"));
    }

    #[test]
    fn test_guide_info_serialization_no_description() {
        let info = GuideInfo {
            id: "test".to_string(),
            title: "Test".to_string(),
            topic: "general".to_string(),
            path: "/path".to_string(),
            description: None,
        };

        let json = serde_json::to_string(&info).expect("Should serialize");
        // description: None should be skipped
        assert!(!json.contains("description"));
    }

    #[test]
    fn test_list_guides_response_serialization() {
        let response = ListGuidesResponse {
            guides: vec![GuideInfo {
                id: "test".to_string(),
                title: "Test".to_string(),
                topic: "harmony".to_string(),
                path: "/path".to_string(),
                description: None,
            }],
            total: 1,
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        assert!(json.contains("guides"));
        assert!(json.contains("total"));
        assert!(json.contains("\"total\":1"));
    }

    #[tokio::test]
    async fn test_scan_guides() {
        let temp = TempDir::new().unwrap();
        let guides_dir = temp.path().join("guides");
        let harmony_dir = guides_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        // Create test guide files
        fs::write(
            harmony_dir.join("chord-progressions.md"),
            "# Chord Progressions\n\nGuide to harmonic progressions.",
        )
        .await
        .unwrap();
        fs::write(
            harmony_dir.join("voice-leading.md"),
            "# Voice Leading\n\nGuide to connecting chords.",
        )
        .await
        .unwrap();

        let guides = scan_guides(&guides_dir).await.unwrap();
        assert_eq!(guides.len(), 2);
        assert_eq!(guides[0].topic, "harmony");
        assert_eq!(guides[0].id, "chord-progressions");
        assert_eq!(guides[0].title, "Chord Progressions");
    }

    #[tokio::test]
    async fn test_extract_title_and_description_with_frontmatter() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("guide.md");
        fs::write(
            &file_path,
            "---\ntitle: Custom Guide Title\ndescription: Custom description\n---\n\n# Heading\n\nFirst paragraph.",
        )
        .await
        .unwrap();

        let (title, description) = extract_title_and_description(&file_path).await.unwrap();
        assert_eq!(title, "Custom Guide Title");
        assert_eq!(description, Some("Custom description".to_string()));
    }

    #[tokio::test]
    async fn test_extract_title_and_description_with_heading() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("guide.md");
        fs::write(&file_path, "# Main Guide Heading\n\nDescription text.")
            .await
            .unwrap();

        let (title, description) = extract_title_and_description(&file_path).await.unwrap();
        assert_eq!(title, "Main Guide Heading");
        assert_eq!(description, Some("Description text.".to_string()));
    }

    #[tokio::test]
    async fn test_extract_title_and_description_fallback_to_filename() {
        let temp = TempDir::new().unwrap();
        let file_path = temp.path().join("my-guide.md");
        fs::write(&file_path, "Content without heading")
            .await
            .unwrap();

        let (title, _description) = extract_title_and_description(&file_path).await.unwrap();
        assert_eq!(title, "my guide"); // Filename with dashes/underscores replaced
    }

    #[tokio::test]
    async fn test_find_guide_file() {
        let temp = TempDir::new().unwrap();
        let guides_dir = temp.path().join("guides");
        let topic_dir = guides_dir.join("harmony");
        fs::create_dir_all(&topic_dir).await.unwrap();

        // Create guide file
        fs::write(topic_dir.join("triads.md"), "# Triads Guide")
            .await
            .unwrap();

        let found = find_guide_file(&guides_dir, "triads").await.unwrap();
        assert!(found.ends_with("triads.md"));
    }

    #[tokio::test]
    async fn test_find_guide_file_readme() {
        let temp = TempDir::new().unwrap();
        let guides_dir = temp.path().join("guides");
        // Pattern {id}/README.md expects guides/advanced-harmony/README.md
        let guide_dir = guides_dir.join("advanced-harmony");
        fs::create_dir_all(&guide_dir).await.unwrap();

        // Create guide as README.md
        fs::write(guide_dir.join("README.md"), "# Advanced Harmony")
            .await
            .unwrap();

        let found = find_guide_file(&guides_dir, "advanced-harmony").await.unwrap();
        assert!(found.ends_with("README.md"));
    }

    #[tokio::test]
    async fn test_find_guide_file_index() {
        let temp = TempDir::new().unwrap();
        let guides_dir = temp.path().join("guides");
        let guide_dir = guides_dir.join("intro");
        fs::create_dir_all(&guide_dir).await.unwrap();

        // Create guide as index.md
        fs::write(guide_dir.join("index.md"), "# Introduction")
            .await
            .unwrap();

        let found = find_guide_file(&guides_dir, "intro").await.unwrap();
        assert!(found.ends_with("index.md"));
    }

    #[tokio::test]
    async fn test_list_guides_empty_directory() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let guides_dir = temp.path().join("nonexistent");

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.guides = guides_dir.to_string_lossy().to_string();

        let response = list_guides(&test_config).await.unwrap();
        assert_eq!(response.guides.len(), 0);
        assert_eq!(response.total, 0);
    }

    #[tokio::test]
    async fn test_get_guide() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let guides_dir = temp.path();
        let topic_dir = guides_dir.join("harmony");
        fs::create_dir_all(&topic_dir).await.unwrap();

        let content = "# Triads Guide\n\nA guide about triads.";
        fs::write(topic_dir.join("triads.md"), content)
            .await
            .unwrap();

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.guides = guides_dir.to_string_lossy().to_string();

        let result = get_guide(&test_config, "triads").await.unwrap();
        assert_eq!(result, content);
    }
}
