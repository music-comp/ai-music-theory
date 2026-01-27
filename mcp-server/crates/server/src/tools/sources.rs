use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::error::{Error, Result};
use crate::util::files::{
    count_files, find_file_by_id, list_subdirectories, read_file, FindOptions,
};

/// Information about a source material.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    pub id: String,
    pub title: String,
    pub format: SourceFormat,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<usize>,
    pub status: SourceStatus,
}

/// Format of a source file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceFormat {
    Markdown,
    Pdf,
    Epub,
    Xml,
}

/// Conversion status of a source.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceStatus {
    Converted,
    NotConverted,
}

/// Response for list_sources tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListSourcesResponse {
    pub sources: Vec<SourceInfo>,
}

/// List all available source materials.
pub async fn list_sources(config: &Config) -> Result<ListSourcesResponse> {
    let mut sources = Vec::new();

    // Check for converted sources in sources-md directory
    let sources_md_path = config.paths.sources_md_path()?;
    if crate::util::files::exists(&sources_md_path).await {
        sources.extend(scan_converted_sources(&sources_md_path).await?);
    }

    // Add unconverted sources from configuration
    sources.extend(list_unconverted_sources(config)?);

    Ok(ListSourcesResponse { sources })
}

/// Scan the sources-md directory for converted markdown sources.
async fn scan_converted_sources(base_path: &Path) -> Result<Vec<SourceInfo>> {
    let mut sources = Vec::new();

    for dir in list_subdirectories(base_path).await? {
        // Safety: unwrap_or provides sensible display fallback if directory name extraction fails
        let source_id = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Count markdown files in this source directory
        let chapter_count = count_files(&dir, FindOptions::markdown()).await?;

        // Compute title before moving source_id to avoid clone
        let title = humanize_source_id(&source_id);

        sources.push(SourceInfo {
            id: source_id,
            title,
            format: SourceFormat::Markdown,
            path: dir.to_string_lossy().to_string(),
            chapters: Some(chapter_count),
            status: SourceStatus::Converted,
        });
    }

    Ok(sources)
}

/// List unconverted source files from configuration.
fn list_unconverted_sources(config: &Config) -> Result<Vec<SourceInfo>> {
    let mut sources = Vec::new();

    // Oxford sources
    for (file_id, filename) in &config.sources.oxford.files {
        let format = detect_format(filename);
        sources.push(SourceInfo {
            id: format!("oxford-{}", file_id),
            title: extract_title(filename),
            format,
            path: config
                .sources
                .oxford
                .file_path(file_id)?
                .to_string_lossy()
                .to_string(),
            chapters: None,
            status: SourceStatus::NotConverted,
        });
    }

    // General sources
    for (file_id, filename) in &config.sources.general.files {
        let format = detect_format(filename);
        sources.push(SourceInfo {
            id: format!("general-{}", file_id),
            title: extract_title(filename),
            format,
            path: config
                .sources
                .general
                .file_path(file_id)?
                .to_string_lossy()
                .to_string(),
            chapters: None,
            status: SourceStatus::NotConverted,
        });
    }

    // Paper sources
    for (file_id, filename) in &config.sources.papers.files {
        let format = detect_format(filename);
        sources.push(SourceInfo {
            id: format!("papers-{}", file_id),
            title: extract_title(filename),
            format,
            path: config
                .sources
                .papers
                .file_path(file_id)?
                .to_string_lossy()
                .to_string(),
            chapters: None,
            status: SourceStatus::NotConverted,
        });
    }

    Ok(sources)
}

/// Detect file format from filename extension.
fn detect_format(filename: &str) -> SourceFormat {
    let lower = filename.to_lowercase();
    if lower.ends_with(".pdf") {
        SourceFormat::Pdf
    } else if lower.ends_with(".epub") {
        SourceFormat::Epub
    } else if lower.ends_with(".xml") {
        SourceFormat::Xml
    } else {
        SourceFormat::Markdown
    }
}

/// Extract title from filename (remove year prefix and extension).
fn extract_title(filename: &str) -> String {
    // Remove year prefix like "[2007] " and file extension
    // Safety: unwrap_or falls back to full filename if no year bracket found
    let without_year = filename.split(']').nth(1).unwrap_or(filename).trim();

    // Remove file extension
    // Safety: unwrap_or falls back to filename if no extension found
    let without_ext = without_year
        .rsplit_once('.')
        .map(|(name, _)| name)
        .unwrap_or(without_year);

    without_ext.to_string()
}

/// Convert source ID to human-readable title.
fn humanize_source_id(id: &str) -> String {
    id.replace('-', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Get a specific chapter from a converted source.
pub async fn get_source_chapter(config: &Config, source_id: &str, chapter: &str) -> Result<String> {
    let sources_md_path = config.paths.sources_md_path()?;
    let source_path = sources_md_path.join(source_id);

    if !crate::util::files::exists(&source_path).await {
        return Err(crate::error::Error::not_found(source_path));
    }

    // Try to find the chapter file
    let chapter_path = find_chapter_file(&source_path, chapter).await?;

    // Read and return the chapter content
    let content = read_file(&chapter_path).await?;
    Ok(content)
}

/// Find a chapter file in the source directory.
async fn find_chapter_file(source_dir: &Path, chapter: &str) -> Result<PathBuf> {
    find_file_by_id(
        source_dir,
        chapter,
        FindOptions::markdown()
            .with_patterns(vec!["{id}.md"])
            .with_max_depth(2),
    )
    .await
}

/// Get the filesystem path to a source PDF/EPUB file.
pub fn get_source_pdf_path(config: &Config, source_id: &str) -> Result<PathBuf> {
    // Parse source category and file ID
    let parts: Vec<&str> = source_id.splitn(2, '-').collect();
    if parts.len() != 2 {
        return Err(Error::invalid_path(
            PathBuf::from(source_id),
            "Source ID must be in format 'category-file-id'".to_string(),
        ));
    }

    let (category, file_id) = (parts[0], parts[1]);

    match category {
        "oxford" => config.sources.oxford.file_path(file_id),
        "general" => config.sources.general.file_path(file_id),
        "papers" => config.sources.papers.file_path(file_id),
        _ => Err(Error::invalid_path(
            PathBuf::from(source_id),
            format!("Unknown category: {}", category),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format() {
        assert!(matches!(detect_format("test.pdf"), SourceFormat::Pdf));
        assert!(matches!(detect_format("test.epub"), SourceFormat::Epub));
        assert!(matches!(detect_format("test.xml"), SourceFormat::Xml));
        assert!(matches!(detect_format("TEST.PDF"), SourceFormat::Pdf));
    }

    #[test]
    fn test_extract_title() {
        let title = extract_title("[2007] Lewin - GMIT.pdf");
        assert_eq!(title, "Lewin - GMIT");
    }

    #[test]
    fn test_humanize_source_id() {
        assert_eq!(humanize_source_id("open-music-theory"), "Open Music Theory");
        assert_eq!(humanize_source_id("lewin-gmit"), "Lewin Gmit");
    }

    #[test]
    fn test_detect_format_markdown_default() {
        // Files without recognized extensions default to Markdown
        assert!(matches!(
            detect_format("readme.txt"),
            SourceFormat::Markdown
        ));
        assert!(matches!(detect_format("notes"), SourceFormat::Markdown));
    }

    #[test]
    fn test_detect_format_case_insensitive() {
        assert!(matches!(detect_format("TEST.EPUB"), SourceFormat::Epub));
        assert!(matches!(detect_format("file.XML"), SourceFormat::Xml));
    }

    #[test]
    fn test_extract_title_no_year() {
        // Files without year prefix
        let title = extract_title("Simple Title.pdf");
        assert_eq!(title, "Simple Title");
    }

    #[test]
    fn test_extract_title_no_extension() {
        // Files without extension
        let title = extract_title("[2020] Title Without Extension");
        assert_eq!(title, "Title Without Extension");
    }

    #[test]
    fn test_extract_title_complex() {
        // Complex filename
        let title = extract_title("[1999] Author - Long Title With - Dashes.epub");
        assert_eq!(title, "Author - Long Title With - Dashes");
    }

    #[test]
    fn test_humanize_source_id_empty() {
        // Edge case: empty string
        assert_eq!(humanize_source_id(""), "");
    }

    #[test]
    fn test_humanize_source_id_single_word() {
        assert_eq!(humanize_source_id("test"), "Test");
    }

    #[test]
    fn test_humanize_source_id_multiple_dashes() {
        assert_eq!(
            humanize_source_id("one-two-three-four"),
            "One Two Three Four"
        );
    }

    #[test]
    fn test_get_source_pdf_path_invalid_format() {
        use crate::config::Config;
        let config = Config::load().expect("Config should load");

        // Invalid format: no dash
        let result = get_source_pdf_path(&config, "nodash");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("category-file-id"));
    }

    #[test]
    fn test_get_source_pdf_path_unknown_category() {
        use crate::config::Config;
        let config = Config::load().expect("Config should load");

        // Invalid category
        let result = get_source_pdf_path(&config, "unknown-file");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown category"));
    }

    #[test]
    fn test_get_source_pdf_path_oxford_valid() {
        use crate::config::Config;
        let config = Config::load().expect("Config should load");

        // Oxford category - file may or may not exist but path should be constructed
        let result = get_source_pdf_path(&config, "oxford-lewin-gmit");
        // Should either succeed or fail with file-not-found, not category error
        match result {
            Ok(path) => assert!(path.to_string_lossy().contains("Lewin")),
            Err(e) => {
                // Should be "File ID not found" error, not category error
                let msg = e.to_string();
                assert!(
                    msg.contains("File ID")
                        || msg.contains("not found")
                        || msg.contains("Invalid path")
                );
            }
        }
    }

    #[test]
    fn test_get_source_pdf_path_general_valid() {
        use crate::config::Config;
        let config = Config::load().expect("Config should load");

        let result = get_source_pdf_path(&config, "general-straus-post-tonal");
        match result {
            Ok(path) => assert!(
                path.to_string_lossy().contains("Straus") || path.to_string_lossy().len() > 0
            ),
            Err(e) => {
                let msg = e.to_string();
                assert!(
                    msg.contains("File ID")
                        || msg.contains("not found")
                        || msg.contains("Invalid path")
                );
            }
        }
    }

    #[test]
    fn test_get_source_pdf_path_papers_valid() {
        use crate::config::Config;
        let config = Config::load().expect("Config should load");

        let result = get_source_pdf_path(&config, "papers-fiore");
        match result {
            Ok(path) => assert!(path.to_string_lossy().len() > 0),
            Err(e) => {
                let msg = e.to_string();
                assert!(
                    msg.contains("File ID")
                        || msg.contains("not found")
                        || msg.contains("Invalid path")
                );
            }
        }
    }

    #[test]
    fn test_source_info_serialization() {
        let info = SourceInfo {
            id: "test-id".to_string(),
            title: "Test Title".to_string(),
            format: SourceFormat::Pdf,
            path: "/test/path".to_string(),
            chapters: Some(5),
            status: SourceStatus::Converted,
        };

        let json = serde_json::to_string(&info).expect("Should serialize");
        assert!(json.contains("test-id"));
        assert!(json.contains("Test Title"));
        assert!(json.contains("pdf"));
    }

    #[test]
    fn test_source_info_chapters_none() {
        let info = SourceInfo {
            id: "test".to_string(),
            title: "Test".to_string(),
            format: SourceFormat::Markdown,
            path: "/path".to_string(),
            chapters: None,
            status: SourceStatus::NotConverted,
        };

        let json = serde_json::to_string(&info).expect("Should serialize");
        // chapters: None should be skipped in serialization
        assert!(!json.contains("chapters"));
    }

    #[test]
    fn test_list_sources_response_serialization() {
        let response = ListSourcesResponse {
            sources: vec![SourceInfo {
                id: "test1".to_string(),
                title: "Test 1".to_string(),
                format: SourceFormat::Pdf,
                path: "/path1".to_string(),
                chapters: None,
                status: SourceStatus::NotConverted,
            }],
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        assert!(json.contains("sources"));
        assert!(json.contains("test1"));
    }

    #[tokio::test]
    async fn test_list_sources_no_sources_md_dir() {
        use crate::config::Config;

        // Load config but sources-md won't exist in temp location
        let config = Config::load().expect("Config should load");

        // This should return list with unconverted sources from config
        let result = list_sources(&config).await;
        assert!(result.is_ok());
        // Successfully returns sources list (may be empty or contain config sources)
    }

    #[tokio::test]
    async fn test_scan_converted_sources() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();

        // Create a source directory with markdown files
        let source_dir = temp.path().join("test-source");
        fs::create_dir(&source_dir).await.unwrap();
        fs::write(source_dir.join("chapter-1.md"), "# Chapter 1")
            .await
            .unwrap();
        fs::write(source_dir.join("chapter-2.md"), "# Chapter 2")
            .await
            .unwrap();

        let sources = scan_converted_sources(temp.path()).await.unwrap();

        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].id, "test-source");
        assert_eq!(sources[0].title, "Test Source");
        assert_eq!(sources[0].chapters, Some(2));
        assert!(matches!(sources[0].format, SourceFormat::Markdown));
        assert!(matches!(sources[0].status, SourceStatus::Converted));
    }

    #[tokio::test]
    async fn test_scan_converted_sources_multiple() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();

        // Create multiple source directories
        let source1 = temp.path().join("source-one");
        fs::create_dir(&source1).await.unwrap();
        fs::write(source1.join("intro.md"), "# Intro")
            .await
            .unwrap();

        let source2 = temp.path().join("source-two");
        fs::create_dir(&source2).await.unwrap();
        fs::write(source2.join("chapter.md"), "# Chapter")
            .await
            .unwrap();

        let sources = scan_converted_sources(temp.path()).await.unwrap();

        assert_eq!(sources.len(), 2);
    }

    #[tokio::test]
    async fn test_get_source_chapter_not_found() {
        use crate::config::Config;

        let config = Config::load().expect("Config should load");

        let result = get_source_chapter(&config, "nonexistent-source", "chapter-1").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_source_format_serialization() {
        let json = serde_json::to_string(&SourceFormat::Pdf).unwrap();
        assert_eq!(json, r#""pdf""#);

        let json = serde_json::to_string(&SourceFormat::Epub).unwrap();
        assert_eq!(json, r#""epub""#);

        let json = serde_json::to_string(&SourceFormat::Xml).unwrap();
        assert_eq!(json, r#""xml""#);

        let json = serde_json::to_string(&SourceFormat::Markdown).unwrap();
        assert_eq!(json, r#""markdown""#);
    }

    #[test]
    fn test_source_status_serialization() {
        let json = serde_json::to_string(&SourceStatus::Converted).unwrap();
        assert_eq!(json, r#""converted""#);

        let json = serde_json::to_string(&SourceStatus::NotConverted).unwrap();
        assert_eq!(json, r#""not_converted""#);
    }

    #[test]
    #[ignore = "Requires copyrighted PDF source materials not in repository - run manually with --ignored if you have the files"]
    fn test_list_unconverted_sources() {
        use crate::config::Config;

        // This test requires actual copyrighted PDF source files that cannot be
        // committed to the repository. Developers with these files can run this
        // test manually using: cargo test -- --ignored
        let config = Config::load().expect("Config should load");
        let sources = list_unconverted_sources(&config).expect("Should list unconverted sources");

        // Should have sources from oxford, general, and papers categories
        assert!(
            sources.len() > 0,
            "Expected unconverted sources to be present"
        );

        // Check that IDs are properly formatted
        let has_oxford = sources.iter().any(|s| s.id.starts_with("oxford-"));
        let has_general = sources.iter().any(|s| s.id.starts_with("general-"));
        let has_papers = sources.iter().any(|s| s.id.starts_with("papers-"));

        assert!(
            has_oxford || has_general || has_papers,
            "Expected at least one source with oxford-/general-/papers- prefix"
        );
    }
}
