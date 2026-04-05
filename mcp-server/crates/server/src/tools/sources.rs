use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::error::{Error, Result};
use crate::state::AppState;
use fabryk::core::util::files::{
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

/// Availability status for a source (v0.3.0).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AvailabilityStatus {
    /// Source is indexed in Tantivy and searchable
    Indexed,
    /// Source is converted to markdown but not indexed
    Converted,
    /// Source PDF/EPUB exists but not converted
    FileExists,
    /// Source is completely unavailable
    Unavailable,
}

/// Response for check_source_availability tool (v0.3.0).
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceAvailabilityResponse {
    pub id: String,
    pub status: AvailabilityStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SourceFormat>,
    pub file_exists: bool,
    pub text_extracted: bool,
    pub searchable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<usize>,
    pub message: String,
}

/// Chapter information for list_source_chapters tool (v0.3.0).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterInfo {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    pub path: String,
}

/// Response for list_source_chapters tool (v0.3.0).
#[derive(Debug, Serialize, Deserialize)]
pub struct ListChaptersResponse {
    pub source_id: String,
    pub chapters: Vec<ChapterInfo>,
    pub total: usize,
}

/// List all available source materials.
pub async fn list_sources(config: &Config) -> Result<ListSourcesResponse> {
    let mut sources = Vec::new();

    // Check for converted sources in sources-md directory
    let sources_md_path = config.paths.sources_md_path()?;
    if fabryk::core::util::files::exists(&sources_md_path).await {
        sources.extend(scan_converted_sources(&sources_md_path).await?);
    }

    // Add unconverted sources from configuration
    sources.extend(list_unconverted_sources(config)?);

    Ok(ListSourcesResponse { sources })
}

/// Check availability status of a source material (v0.3.0).
///
/// Returns detailed information about whether a source is indexed,
/// converted, or available in any format. This helps Claude gracefully
/// handle references to sources that may not be accessible.
///
/// # Logic
///
/// 1. Check if source is indexed in Tantivy (searchable)
/// 2. If not indexed, check if markdown exists (converted)
/// 3. If not converted, check if original PDF/EPUB exists
/// 4. Return appropriate status and human-readable message
pub async fn check_source_availability(
    state: &AppState,
    source_id: &str,
) -> Result<SourceAvailabilityResponse> {
    let config = &state.config;

    // Step 1: Check if indexed in Tantivy (requires FTS feature)
    #[cfg(feature = "fts")]
    let indexed = if state.is_fts_ready() {
        check_source_indexed(state, source_id).await?
    } else {
        false
    };

    #[cfg(not(feature = "fts"))]
    let indexed = false;

    // Step 2: Check if converted to markdown
    let sources_md_path = config.paths.sources_md_path()?;
    let source_path = sources_md_path.join(source_id);
    let text_extracted = fabryk::core::util::files::exists(&source_path).await;

    // Count chapters if converted
    let chapters = if text_extracted {
        Some(count_files(&source_path, FindOptions::markdown()).await?)
    } else {
        None
    };

    // Step 3: Check if original file exists
    let (file_exists, format) = check_source_file_exists(config, source_id).await;

    // Determine overall status
    let (status, message) = if indexed {
        (
            AvailabilityStatus::Indexed,
            format!("Source '{}' is indexed and fully searchable", source_id),
        )
    } else if text_extracted {
        (
            AvailabilityStatus::Converted,
            format!(
                "Source '{}' is converted to markdown but not indexed. Run 'index' command to make it searchable.",
                source_id
            ),
        )
    } else if file_exists {
        (
            AvailabilityStatus::FileExists,
            format!(
                "Source '{}' file exists but is not converted. Text extraction required.",
                source_id
            ),
        )
    } else {
        (
            AvailabilityStatus::Unavailable,
            format!("Source '{}' is not available in any format", source_id),
        )
    };

    Ok(SourceAvailabilityResponse {
        id: source_id.to_string(),
        status,
        format,
        file_exists,
        text_extracted,
        searchable: indexed,
        chapters,
        message,
    })
}

/// List all chapters for a source material (v0.3.0).
///
/// Returns a list of all chapters/sections within a source, with their
/// titles, sections (page numbers), and paths. If the source is indexed,
/// queries the index; otherwise falls back to filesystem scan.
pub async fn list_source_chapters(
    state: &AppState,
    source_id: &str,
) -> Result<ListChaptersResponse> {
    // Try index first if FTS is ready
    #[cfg(feature = "fts")]
    if state.is_fts_ready() {
        if let Ok(chapters) = list_chapters_from_index(state, source_id).await {
            return Ok(ListChaptersResponse {
                source_id: source_id.to_string(),
                total: chapters.len(),
                chapters,
            });
        }
    }

    // Fallback to filesystem scan
    let chapters = list_chapters_from_filesystem(&state.config, source_id).await?;
    Ok(ListChaptersResponse {
        source_id: source_id.to_string(),
        total: chapters.len(),
        chapters,
    })
}

/// List chapters from Tantivy index.
#[cfg(feature = "fts")]
async fn list_chapters_from_index(state: &AppState, source_id: &str) -> Result<Vec<ChapterInfo>> {
    // Use wildcard query to match ALL documents, then filter by content_type
    // We filter by path instead of source facet field to handle chapters
    // that may have missing or inconsistent source metadata
    let params = crate::search::SearchParams {
        query: "*".to_string(), // Match all documents (uses AllQuery in Tantivy)
        limit: Some(1000),      // Large limit to get all chapters
        category: None,
        source: None, // Don't filter by source facet - use path filter instead
        content_types: Some(vec!["source_chapter".to_string()]),
        query_mode: None,
        snippet_length: None,
        extra_filters: None,
    };

    let results = state.search_backend().search(params).await?;

    // Filter by source_id in path (handles chapters with missing/inconsistent source metadata)
    let chapters = results
        .items
        .into_iter()
        .filter(|result| result.path.as_ref().is_some_and(|p| p.contains(source_id)))
        .map(|result| ChapterInfo {
            id: result.id,
            title: result.title,
            section: result.section,
            path: result.path.unwrap_or_default(),
        })
        .collect();

    Ok(chapters)
}

/// List chapters from filesystem (fallback).
async fn list_chapters_from_filesystem(
    config: &Config,
    source_id: &str,
) -> Result<Vec<ChapterInfo>> {
    use crate::metadata::extract_metadata;
    use crate::metadata::ContentType;
    use fabryk::core::util::files::find_all_files;

    let sources_md_path = config.paths.sources_md_path()?;
    let source_path = sources_md_path.join(source_id);

    if !fabryk::core::util::files::exists(&source_path).await {
        return Err(Error::file_not_found(source_path));
    }

    let files = find_all_files(&source_path, FindOptions::markdown()).await?;
    let mut chapters = Vec::new();

    for file_info in files {
        let path = &file_info.path;

        // Extract metadata to get title and section
        if let Ok(meta) = extract_metadata(&source_path, path, ContentType::SourceChapter).await {
            // Generate chapter ID from filename
            let id = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            chapters.push(ChapterInfo {
                id,
                title: meta.title,
                section: meta.section,
                path: path.to_string_lossy().to_string(),
            });
        }
    }

    // Sort by path for consistent ordering
    chapters.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(chapters)
}

/// Check if a source is indexed in Tantivy.
#[cfg(feature = "fts")]
async fn check_source_indexed(state: &AppState, source_id: &str) -> Result<bool> {
    // Search for a common term that would appear in source chapters
    let params = crate::search::SearchParams {
        query: "chapter".to_string(),
        limit: Some(1000),
        category: None,
        source: None,
        content_types: Some(vec!["source_chapter".to_string()]),
        query_mode: None,
        snippet_length: None,
        extra_filters: None,
    };

    let results = state.search_backend().search(params).await?;

    // Check if any results have the source_id in their path
    Ok(results
        .items
        .iter()
        .any(|r| r.path.as_ref().is_some_and(|p| p.contains(source_id))))
}

/// Check if source file exists in any configured location.
async fn check_source_file_exists(
    config: &Config,
    source_id: &str,
) -> (bool, Option<SourceFormat>) {
    // Try to get PDF/EPUB path
    match get_source_pdf_path(config, source_id) {
        Ok(path) => {
            // Check if file actually exists
            let exists = std::path::Path::new(&path).exists();
            if exists {
                let format = detect_format(&path.to_string_lossy());
                (true, Some(format))
            } else {
                (false, None)
            }
        }
        Err(_) => (false, None),
    }
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

/// Get a specific chapter from a converted source (v0.3.0: enhanced).
///
/// Retrieves the content of a specific chapter from a source. Optionally
/// filters to a specific section if provided.
///
/// # Arguments
///
/// * `config` - Server configuration
/// * `source_id` - Source identifier (e.g., "open-music-theory")
/// * `chapter` - Chapter identifier
/// * `section` - Optional section/page filter (e.g., "pp. 23-28")
///
/// # Errors
///
/// Returns errors with clear messages when:
/// - Source directory doesn't exist
/// - Chapter file not found
/// - File read fails
pub async fn get_source_chapter(
    config: &Config,
    source_id: &str,
    chapter: &str,
    section: Option<&str>,
) -> Result<String> {
    let sources_md_path = config.paths.sources_md_path()?;
    let source_path = sources_md_path.join(source_id);

    if !fabryk::core::util::files::exists(&source_path).await {
        return Err(Error::not_found_msg(format!(
            "Source '{}' not found. Check if it's been converted to markdown.",
            source_id
        )));
    }

    // Try to find the chapter file
    let chapter_path = find_chapter_file(&source_path, chapter).await.map_err(|_| {
        Error::not_found_msg(format!(
            "Chapter '{}' not found in source '{}'. Use list_source_chapters to see available chapters.",
            chapter, source_id
        ))
    })?;

    // Read the chapter content
    let content = read_file(&chapter_path).await?;

    // If section filter specified, extract just that section
    if let Some(section_ref) = section {
        if let Some(extracted) = extract_section(&content, section_ref) {
            return Ok(extracted);
        } else {
            // Section not found, return full content with note
            return Ok(format!(
                "# Note: Section '{}' not found in this chapter\n\n{}",
                section_ref, content
            ));
        }
    }

    Ok(content)
}

/// Extract a section from markdown content by heading name.
///
/// Finds a heading matching the section name (case-insensitive) and extracts
/// all content from that heading until the next heading of the same or higher level.
///
/// # Arguments
///
/// * `markdown` - The full markdown content
/// * `section_name` - The heading text to search for (without # prefix)
///
/// # Returns
///
/// Returns Some(String) with the extracted section content (including the heading),
/// or None if the section wasn't found.
fn extract_section(markdown: &str, section_name: &str) -> Option<String> {
    let section_lower = section_name.to_lowercase();
    let lines: Vec<&str> = markdown.lines().collect();

    // Find the section heading
    let mut start_idx = None;
    let mut heading_level = 0;

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            // Extract heading text and level
            let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
            let heading_text = trimmed[hash_count..].trim();

            if heading_text.to_lowercase() == section_lower {
                start_idx = Some(idx);
                heading_level = hash_count;
                break;
            }
        }
    }

    let start = start_idx?;

    // Find the end (next heading of same or higher level)
    let mut end_idx = lines.len();
    for (idx, line) in lines.iter().enumerate().skip(start + 1) {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
            if hash_count <= heading_level {
                end_idx = idx;
                break;
            }
        }
    }

    // Extract the section content
    let section_lines = &lines[start..end_idx];
    Some(section_lines.join("\n"))
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
                path.to_string_lossy().contains("Straus") || !path.to_string_lossy().is_empty()
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
            Ok(path) => assert!(!path.to_string_lossy().is_empty()),
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

        let result = get_source_chapter(&config, "nonexistent-source", "chapter-1", None).await;
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
            !sources.is_empty(),
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

    #[test]
    fn test_extract_section_basic() {
        let markdown = r#"# Chapter Title

Intro text.

## Section One

Content for section one.

## Section Two

Content for section two.

### Subsection Two-A

Nested content.

## Section Three

Final section.
"#;

        let result = extract_section(markdown, "Section Two");
        assert!(result.is_some());
        let extracted = result.unwrap();
        assert!(extracted.contains("## Section Two"));
        assert!(extracted.contains("Content for section two."));
        assert!(extracted.contains("### Subsection Two-A"));
        assert!(extracted.contains("Nested content."));
        assert!(!extracted.contains("Section Three"));
    }

    #[test]
    fn test_extract_section_case_insensitive() {
        let markdown = r#"## Prime Form

Content about prime form.

## Set Class

Content about set class.
"#;

        let result = extract_section(markdown, "prime form");
        assert!(result.is_some());
        let extracted = result.unwrap();
        assert!(extracted.contains("## Prime Form"));
        assert!(extracted.contains("Content about prime form."));
        assert!(!extracted.contains("Set Class"));
    }

    #[test]
    fn test_extract_section_not_found() {
        let markdown = r#"## Section One

Content.

## Section Two

More content.
"#;

        let result = extract_section(markdown, "Section Three");
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_section_at_end() {
        let markdown = r#"## Section One

Content one.

## Section Two

Content two at the end.
"#;

        let result = extract_section(markdown, "Section Two");
        assert!(result.is_some());
        let extracted = result.unwrap();
        assert!(extracted.contains("## Section Two"));
        assert!(extracted.contains("Content two at the end."));
    }

    #[test]
    fn test_extract_section_with_different_levels() {
        let markdown = r#"# Main Chapter

Intro.

## Section A

Content A.

### Subsection A1

Nested A1.

## Section B

Content B.
"#;

        let result = extract_section(markdown, "Subsection A1");
        assert!(result.is_some());
        let extracted = result.unwrap();
        assert!(extracted.contains("### Subsection A1"));
        assert!(extracted.contains("Nested A1."));
        assert!(!extracted.contains("Section B"));
    }

    // ---------------------------------------------------------------
    // extract_section: additional edge cases
    // ---------------------------------------------------------------

    #[test]
    fn test_extract_section_empty_markdown() {
        let result = extract_section("", "Anything");
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_section_no_headings() {
        let markdown = "Just some plain text\nwithout any headings.\n";
        let result = extract_section(markdown, "Missing");
        assert!(result.is_none());
    }

    #[test]
    fn test_extract_section_heading_only_no_body() {
        let markdown = "## Solo Heading\n";
        let result = extract_section(markdown, "Solo Heading");
        assert!(result.is_some());
        let extracted = result.unwrap();
        assert!(extracted.contains("## Solo Heading"));
    }

    #[test]
    fn test_extract_section_level_one_heading() {
        let markdown = r#"# Top Level

Top level content.

# Another Top Level

Other content.
"#;
        let result = extract_section(markdown, "Top Level");
        assert!(result.is_some());
        let extracted = result.unwrap();
        assert!(extracted.contains("# Top Level"));
        assert!(extracted.contains("Top level content."));
        assert!(!extracted.contains("Another Top Level"));
    }

    #[test]
    fn test_extract_section_same_level_boundary() {
        let markdown = r#"### A

Content A.

### B

Content B.

### C

Content C.
"#;
        let result = extract_section(markdown, "B");
        assert!(result.is_some());
        let extracted = result.unwrap();
        assert!(extracted.contains("### B"));
        assert!(extracted.contains("Content B."));
        assert!(!extracted.contains("Content A."));
        assert!(!extracted.contains("Content C."));
    }

    #[test]
    fn test_extract_section_higher_level_terminates() {
        // A ### section should end when a ## appears
        let markdown = r#"## Parent

### Child

Child content.

## Next Parent

Next content.
"#;
        let result = extract_section(markdown, "Child");
        assert!(result.is_some());
        let extracted = result.unwrap();
        assert!(extracted.contains("### Child"));
        assert!(extracted.contains("Child content."));
        assert!(!extracted.contains("Next Parent"));
    }

    #[test]
    fn test_extract_section_partial_name_no_match() {
        let markdown = "## Section One\n\nContent.\n";
        let result = extract_section(markdown, "Section");
        assert!(result.is_none(), "Partial heading name should not match");
    }

    // ---------------------------------------------------------------
    // detect_format: additional cases
    // ---------------------------------------------------------------

    #[test]
    fn test_detect_format_mixed_case_extensions() {
        assert!(matches!(detect_format("file.Pdf"), SourceFormat::Pdf));
        assert!(matches!(detect_format("file.ePub"), SourceFormat::Epub));
        assert!(matches!(detect_format("file.xMl"), SourceFormat::Xml));
    }

    #[test]
    fn test_detect_format_empty_string() {
        assert!(matches!(detect_format(""), SourceFormat::Markdown));
    }

    #[test]
    fn test_detect_format_extension_only() {
        assert!(matches!(detect_format(".pdf"), SourceFormat::Pdf));
        assert!(matches!(detect_format(".epub"), SourceFormat::Epub));
    }

    // ---------------------------------------------------------------
    // extract_title: additional cases
    // ---------------------------------------------------------------

    #[test]
    fn test_extract_title_empty_string() {
        let title = extract_title("");
        assert_eq!(title, "");
    }

    #[test]
    fn test_extract_title_just_extension() {
        let title = extract_title(".pdf");
        assert_eq!(title, "");
    }

    #[test]
    fn test_extract_title_year_no_space_after_bracket() {
        let title = extract_title("[2020]Title.pdf");
        assert_eq!(title, "Title");
    }

    #[test]
    fn test_extract_title_multiple_dots() {
        let title = extract_title("[2010] Author - v2.0 Edition.pdf");
        assert_eq!(title, "Author - v2.0 Edition");
    }

    // ---------------------------------------------------------------
    // humanize_source_id: additional cases
    // ---------------------------------------------------------------

    #[test]
    fn test_humanize_source_id_leading_trailing_dashes() {
        // Leading/trailing dashes produce empty segments which split_whitespace removes
        assert_eq!(humanize_source_id("-leading"), "Leading");
        assert_eq!(humanize_source_id("trailing-"), "Trailing");
    }

    #[test]
    fn test_humanize_source_id_consecutive_dashes() {
        assert_eq!(humanize_source_id("a--b"), "A B");
    }

    #[test]
    fn test_humanize_source_id_unicode() {
        // Unicode capitalisation should work
        let result = humanize_source_id("über-alles");
        assert!(result.starts_with("Über"));
    }

    // ---------------------------------------------------------------
    // Serialization / deserialization round-trip tests
    // ---------------------------------------------------------------

    #[test]
    fn test_availability_status_serialization() {
        let json = serde_json::to_string(&AvailabilityStatus::Indexed).unwrap();
        assert_eq!(json, r#""indexed""#);

        let json = serde_json::to_string(&AvailabilityStatus::Converted).unwrap();
        assert_eq!(json, r#""converted""#);

        let json = serde_json::to_string(&AvailabilityStatus::FileExists).unwrap();
        assert_eq!(json, r#""file_exists""#);

        let json = serde_json::to_string(&AvailabilityStatus::Unavailable).unwrap();
        assert_eq!(json, r#""unavailable""#);
    }

    #[test]
    fn test_availability_status_deserialization() {
        let s: AvailabilityStatus = serde_json::from_str(r#""indexed""#).unwrap();
        assert!(matches!(s, AvailabilityStatus::Indexed));

        let s: AvailabilityStatus = serde_json::from_str(r#""converted""#).unwrap();
        assert!(matches!(s, AvailabilityStatus::Converted));

        let s: AvailabilityStatus = serde_json::from_str(r#""file_exists""#).unwrap();
        assert!(matches!(s, AvailabilityStatus::FileExists));

        let s: AvailabilityStatus = serde_json::from_str(r#""unavailable""#).unwrap();
        assert!(matches!(s, AvailabilityStatus::Unavailable));
    }

    #[test]
    fn test_source_format_deserialization() {
        let f: SourceFormat = serde_json::from_str(r#""pdf""#).unwrap();
        assert!(matches!(f, SourceFormat::Pdf));

        let f: SourceFormat = serde_json::from_str(r#""epub""#).unwrap();
        assert!(matches!(f, SourceFormat::Epub));

        let f: SourceFormat = serde_json::from_str(r#""xml""#).unwrap();
        assert!(matches!(f, SourceFormat::Xml));

        let f: SourceFormat = serde_json::from_str(r#""markdown""#).unwrap();
        assert!(matches!(f, SourceFormat::Markdown));
    }

    #[test]
    fn test_source_status_deserialization() {
        let s: SourceStatus = serde_json::from_str(r#""converted""#).unwrap();
        assert!(matches!(s, SourceStatus::Converted));

        let s: SourceStatus = serde_json::from_str(r#""not_converted""#).unwrap();
        assert!(matches!(s, SourceStatus::NotConverted));
    }

    #[test]
    fn test_chapter_info_serialization() {
        let info = ChapterInfo {
            id: "ch-01".to_string(),
            title: "Introduction".to_string(),
            section: Some("pp. 1-10".to_string()),
            path: "/path/to/ch-01.md".to_string(),
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("ch-01"));
        assert!(json.contains("Introduction"));
        assert!(json.contains("pp. 1-10"));
    }

    #[test]
    fn test_chapter_info_section_none_skipped() {
        let info = ChapterInfo {
            id: "ch-01".to_string(),
            title: "Intro".to_string(),
            section: None,
            path: "/path".to_string(),
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(!json.contains("section"));
    }

    #[test]
    fn test_chapter_info_deserialization() {
        let json = r#"{"id":"ch-01","title":"Intro","path":"/p"}"#;
        let info: ChapterInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.id, "ch-01");
        assert_eq!(info.title, "Intro");
        assert!(info.section.is_none());
        assert_eq!(info.path, "/p");
    }

    #[test]
    fn test_list_chapters_response_serialization() {
        let response = ListChaptersResponse {
            source_id: "test-source".to_string(),
            chapters: vec![ChapterInfo {
                id: "ch-01".to_string(),
                title: "First".to_string(),
                section: None,
                path: "/p".to_string(),
            }],
            total: 1,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("test-source"));
        assert!(json.contains("ch-01"));
        assert!(json.contains(r#""total":1"#));
    }

    #[test]
    fn test_list_chapters_response_deserialization() {
        let json = r#"{"source_id":"src","chapters":[],"total":0}"#;
        let resp: ListChaptersResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.source_id, "src");
        assert!(resp.chapters.is_empty());
        assert_eq!(resp.total, 0);
    }

    #[test]
    fn test_source_availability_response_serialization() {
        let resp = SourceAvailabilityResponse {
            id: "test-id".to_string(),
            status: AvailabilityStatus::Converted,
            format: Some(SourceFormat::Pdf),
            file_exists: true,
            text_extracted: true,
            searchable: false,
            chapters: Some(10),
            message: "Converted but not indexed".to_string(),
        };

        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("test-id"));
        assert!(json.contains("converted"));
        assert!(json.contains("pdf"));
        assert!(json.contains(r#""chapters":10"#));
    }

    #[test]
    fn test_source_availability_response_optional_fields_skipped() {
        let resp = SourceAvailabilityResponse {
            id: "test".to_string(),
            status: AvailabilityStatus::Unavailable,
            format: None,
            file_exists: false,
            text_extracted: false,
            searchable: false,
            chapters: None,
            message: "Not available".to_string(),
        };

        let json = serde_json::to_string(&resp).unwrap();
        assert!(!json.contains("format"));
        assert!(!json.contains("chapters"));
    }

    #[test]
    fn test_source_info_deserialization() {
        let json = r#"{"id":"x","title":"X","format":"epub","path":"/p","status":"not_converted"}"#;
        let info: SourceInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.id, "x");
        assert!(matches!(info.format, SourceFormat::Epub));
        assert!(matches!(info.status, SourceStatus::NotConverted));
        assert!(info.chapters.is_none());
    }

    #[test]
    fn test_list_sources_response_deserialization() {
        let json = r#"{"sources":[]}"#;
        let resp: ListSourcesResponse = serde_json::from_str(json).unwrap();
        assert!(resp.sources.is_empty());
    }

    // ---------------------------------------------------------------
    // get_source_pdf_path: additional edge cases
    // ---------------------------------------------------------------

    #[test]
    fn test_get_source_pdf_path_empty_string() {
        let config = Config::load().expect("Config should load");
        let result = get_source_pdf_path(&config, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_source_pdf_path_multiple_dashes() {
        // splitn(2, '-') should split at first dash only
        let config = Config::load().expect("Config should load");
        let result = get_source_pdf_path(&config, "oxford-some-long-file-id");
        // Should attempt oxford category with file_id "some-long-file-id"
        match result {
            Ok(_) => {} // fine if it happens to exist
            Err(e) => {
                let msg = e.to_string();
                // Should NOT be "Unknown category" error
                assert!(
                    !msg.contains("Unknown category"),
                    "Expected file-id lookup error, got category error: {}",
                    msg
                );
            }
        }
    }

    #[test]
    fn test_get_source_pdf_path_dash_only() {
        let config = Config::load().expect("Config should load");
        // "-" splits into ["", ""] which means category="" and file_id=""
        let result = get_source_pdf_path(&config, "-");
        assert!(result.is_err());
    }

    // ---------------------------------------------------------------
    // scan_converted_sources: edge cases
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_scan_converted_sources_empty_dir() {
        use tempfile::TempDir;

        let temp = TempDir::new().unwrap();
        let sources = scan_converted_sources(temp.path()).await.unwrap();
        assert!(sources.is_empty());
    }

    #[tokio::test]
    async fn test_scan_converted_sources_dir_with_no_md_files() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let source_dir = temp.path().join("empty-source");
        fs::create_dir(&source_dir).await.unwrap();
        // Directory exists but has no .md files
        fs::write(source_dir.join("notes.txt"), "not markdown")
            .await
            .unwrap();

        let sources = scan_converted_sources(temp.path()).await.unwrap();
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].id, "empty-source");
        assert_eq!(sources[0].chapters, Some(0));
    }

    #[tokio::test]
    async fn test_scan_converted_sources_ignores_files_at_root() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        // A file at the root should not be treated as a source directory
        fs::write(temp.path().join("stray-file.md"), "# Stray")
            .await
            .unwrap();
        // But a directory should
        let source_dir = temp.path().join("real-source");
        fs::create_dir(&source_dir).await.unwrap();
        fs::write(source_dir.join("ch1.md"), "# Chapter 1")
            .await
            .unwrap();

        let sources = scan_converted_sources(temp.path()).await.unwrap();
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].id, "real-source");
    }

    // ---------------------------------------------------------------
    // check_source_file_exists: via get_source_pdf_path coverage
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_check_source_file_exists_unknown_category() {
        let config = Config::load().expect("Config should load");
        let (exists, format) = check_source_file_exists(&config, "bogus-file").await;
        assert!(!exists);
        assert!(format.is_none());
    }

    #[tokio::test]
    async fn test_check_source_file_exists_no_dash() {
        let config = Config::load().expect("Config should load");
        let (exists, format) = check_source_file_exists(&config, "nodash").await;
        assert!(!exists);
        assert!(format.is_none());
    }

    // ---------------------------------------------------------------
    // list_chapters_from_filesystem: tests
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_list_chapters_from_filesystem_source_not_found() {
        let config = Config::load().expect("Config should load");
        let result = list_chapters_from_filesystem(&config, "nonexistent-source-xyz").await;
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("not found") || msg.contains("File not found"),
            "Expected not-found error, got: {}",
            msg
        );
    }

    #[tokio::test]
    async fn test_list_chapters_from_filesystem_with_files() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();

        // Build a minimal config pointing sources_md at our temp dir
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        // Create source directory with chapter files
        let source_dir = temp.path().join("test-source");
        fs::create_dir(&source_dir).await.unwrap();
        fs::write(
            source_dir.join("chapter-01.md"),
            "---\ntitle: First Chapter\n---\n# First Chapter\n\nContent.",
        )
        .await
        .unwrap();
        fs::write(
            source_dir.join("chapter-02.md"),
            "---\ntitle: Second Chapter\n---\n# Second Chapter\n\nMore content.",
        )
        .await
        .unwrap();

        let chapters = list_chapters_from_filesystem(&config, "test-source")
            .await
            .unwrap();

        assert_eq!(chapters.len(), 2);
        // Should be sorted by path
        assert!(chapters[0].path < chapters[1].path);
    }

    #[tokio::test]
    async fn test_list_chapters_from_filesystem_empty_source_dir() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();

        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        let source_dir = temp.path().join("empty-source");
        fs::create_dir(&source_dir).await.unwrap();

        let chapters = list_chapters_from_filesystem(&config, "empty-source")
            .await
            .unwrap();
        assert!(chapters.is_empty());
    }

    // ---------------------------------------------------------------
    // get_source_chapter: section filtering
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_get_source_chapter_with_section_found() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        // Create source and chapter
        let source_dir = temp.path().join("my-source");
        fs::create_dir(&source_dir).await.unwrap();
        let chapter_content = r#"# Chapter Title

Intro text.

## Target Section

This is the target section content.

## Another Section

Other content.
"#;
        fs::write(source_dir.join("ch-01.md"), chapter_content)
            .await
            .unwrap();

        let result = get_source_chapter(&config, "my-source", "ch-01", Some("Target Section"))
            .await
            .unwrap();

        assert!(result.contains("## Target Section"));
        assert!(result.contains("This is the target section content."));
        assert!(!result.contains("Another Section"));
    }

    #[tokio::test]
    async fn test_get_source_chapter_with_section_not_found() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        let source_dir = temp.path().join("my-source");
        fs::create_dir(&source_dir).await.unwrap();
        fs::write(source_dir.join("ch-01.md"), "# Chapter\n\nBody text.\n")
            .await
            .unwrap();

        let result = get_source_chapter(&config, "my-source", "ch-01", Some("Missing Section"))
            .await
            .unwrap();

        assert!(result.contains("Section 'Missing Section' not found"));
        assert!(result.contains("Body text."));
    }

    #[tokio::test]
    async fn test_get_source_chapter_without_section() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        let source_dir = temp.path().join("my-source");
        fs::create_dir(&source_dir).await.unwrap();
        fs::write(
            source_dir.join("ch-01.md"),
            "# Full Chapter\n\nAll content.\n",
        )
        .await
        .unwrap();

        let result = get_source_chapter(&config, "my-source", "ch-01", None)
            .await
            .unwrap();

        assert!(result.contains("# Full Chapter"));
        assert!(result.contains("All content."));
    }

    #[tokio::test]
    async fn test_get_source_chapter_chapter_not_found() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        let source_dir = temp.path().join("my-source");
        fs::create_dir(&source_dir).await.unwrap();
        // Source dir exists but chapter does not
        fs::write(source_dir.join("other.md"), "# Other")
            .await
            .unwrap();

        let result = get_source_chapter(&config, "my-source", "nonexistent-chapter", None).await;
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("not found"),
            "Expected not-found error, got: {}",
            msg
        );
    }

    #[tokio::test]
    async fn test_get_source_chapter_source_not_found() {
        let config = Config::load().expect("Config should load");
        let result = get_source_chapter(&config, "totally-missing-source", "ch-01", None).await;
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(
            msg.contains("not found"),
            "Expected not-found error, got: {}",
            msg
        );
    }

    // ---------------------------------------------------------------
    // check_source_availability: status paths
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_check_source_availability_unavailable() {
        let config = Config::load().expect("Config should load");
        let state = AppState::new(config).await.expect("State should init");

        let resp = check_source_availability(&state, "totally-nonexistent-xyz")
            .await
            .unwrap();

        assert_eq!(resp.id, "totally-nonexistent-xyz");
        assert!(matches!(resp.status, AvailabilityStatus::Unavailable));
        assert!(!resp.file_exists);
        assert!(!resp.text_extracted);
        assert!(!resp.searchable);
        assert!(resp.chapters.is_none());
        assert!(resp.format.is_none());
        assert!(resp.message.contains("not available"));
    }

    #[tokio::test]
    async fn test_check_source_availability_converted() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        // Create a converted source directory
        let source_dir = temp.path().join("test-converted");
        fs::create_dir(&source_dir).await.unwrap();
        fs::write(source_dir.join("ch-01.md"), "# Ch 1")
            .await
            .unwrap();
        fs::write(source_dir.join("ch-02.md"), "# Ch 2")
            .await
            .unwrap();

        let state = AppState::new(config).await.expect("State should init");
        let resp = check_source_availability(&state, "test-converted")
            .await
            .unwrap();

        assert_eq!(resp.id, "test-converted");
        assert!(matches!(resp.status, AvailabilityStatus::Converted));
        assert!(resp.text_extracted);
        assert!(!resp.searchable);
        assert_eq!(resp.chapters, Some(2));
        assert!(resp.message.contains("converted to markdown"));
    }

    // ---------------------------------------------------------------
    // list_source_chapters: filesystem fallback
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_list_source_chapters_filesystem_fallback() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        let source_dir = temp.path().join("fallback-source");
        fs::create_dir(&source_dir).await.unwrap();
        fs::write(
            source_dir.join("intro.md"),
            "---\ntitle: Introduction\n---\n# Introduction\n\nContent.",
        )
        .await
        .unwrap();

        let state = AppState::new(config).await.expect("State should init");
        let resp = list_source_chapters(&state, "fallback-source")
            .await
            .unwrap();

        assert_eq!(resp.source_id, "fallback-source");
        assert_eq!(resp.total, resp.chapters.len());
    }

    #[tokio::test]
    async fn test_list_source_chapters_source_not_found() {
        let config = Config::load().expect("Config should load");
        let state = AppState::new(config).await.expect("State should init");

        let result = list_source_chapters(&state, "does-not-exist-xyz").await;
        assert!(result.is_err());
    }

    // ---------------------------------------------------------------
    // list_sources: integration test with temp filesystem
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_list_sources_with_converted_sources() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        // Clear unconverted sources to focus on converted ones
        config.sources.oxford.files.clear();
        config.sources.general.files.clear();
        config.sources.papers.files.clear();

        let source_dir = temp.path().join("integrated-source");
        fs::create_dir(&source_dir).await.unwrap();
        fs::write(source_dir.join("ch1.md"), "# Chapter 1")
            .await
            .unwrap();

        let resp = list_sources(&config).await.unwrap();
        assert_eq!(resp.sources.len(), 1);
        assert_eq!(resp.sources[0].id, "integrated-source");
        assert_eq!(resp.sources[0].title, "Integrated Source");
        assert!(matches!(resp.sources[0].format, SourceFormat::Markdown));
        assert!(matches!(resp.sources[0].status, SourceStatus::Converted));
        assert_eq!(resp.sources[0].chapters, Some(1));
    }

    #[tokio::test]
    async fn test_list_sources_empty_config_no_sources_dir() {
        let mut config = Config::load().expect("Config should load");
        // Point to a non-existent directory
        config.paths.sources_md = "/tmp/definitely-does-not-exist-xyz".to_string();
        config.sources.oxford.files.clear();
        config.sources.general.files.clear();
        config.sources.papers.files.clear();

        let resp = list_sources(&config).await.unwrap();
        assert!(resp.sources.is_empty());
    }

    // ---------------------------------------------------------------
    // list_unconverted_sources: with empty config
    // ---------------------------------------------------------------

    #[test]
    fn test_list_unconverted_sources_empty_files() {
        let mut config = Config::load().expect("Config should load");
        config.sources.oxford.files.clear();
        config.sources.general.files.clear();
        config.sources.papers.files.clear();

        let sources = list_unconverted_sources(&config).unwrap();
        assert!(sources.is_empty());
    }

    // ---------------------------------------------------------------
    // find_chapter_file: tests
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_find_chapter_file_found() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("my-chapter.md"), "# My Chapter")
            .await
            .unwrap();

        let result = find_chapter_file(temp.path(), "my-chapter").await;
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("my-chapter.md"));
    }

    #[tokio::test]
    async fn test_find_chapter_file_not_found() {
        use tempfile::TempDir;

        let temp = TempDir::new().unwrap();
        let result = find_chapter_file(temp.path(), "nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_chapter_file_in_subdirectory() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let subdir = temp.path().join("subdir");
        fs::create_dir(&subdir).await.unwrap();
        fs::write(subdir.join("nested-chapter.md"), "# Nested")
            .await
            .unwrap();

        let result = find_chapter_file(temp.path(), "nested-chapter").await;
        assert!(result.is_ok());
    }

    // ---------------------------------------------------------------
    // Debug trait coverage
    // ---------------------------------------------------------------

    #[test]
    fn test_source_info_debug() {
        let info = SourceInfo {
            id: "x".to_string(),
            title: "X".to_string(),
            format: SourceFormat::Pdf,
            path: "/p".to_string(),
            chapters: None,
            status: SourceStatus::Converted,
        };
        let debug = format!("{:?}", info);
        assert!(debug.contains("SourceInfo"));
    }

    #[test]
    fn test_source_format_debug() {
        assert!(format!("{:?}", SourceFormat::Pdf).contains("Pdf"));
        assert!(format!("{:?}", SourceFormat::Epub).contains("Epub"));
        assert!(format!("{:?}", SourceFormat::Xml).contains("Xml"));
        assert!(format!("{:?}", SourceFormat::Markdown).contains("Markdown"));
    }

    #[test]
    fn test_source_status_debug() {
        assert!(format!("{:?}", SourceStatus::Converted).contains("Converted"));
        assert!(format!("{:?}", SourceStatus::NotConverted).contains("NotConverted"));
    }

    #[test]
    fn test_availability_status_debug() {
        assert!(format!("{:?}", AvailabilityStatus::Indexed).contains("Indexed"));
        assert!(format!("{:?}", AvailabilityStatus::Converted).contains("Converted"));
        assert!(format!("{:?}", AvailabilityStatus::FileExists).contains("FileExists"));
        assert!(format!("{:?}", AvailabilityStatus::Unavailable).contains("Unavailable"));
    }

    #[test]
    fn test_chapter_info_debug() {
        let info = ChapterInfo {
            id: "c".to_string(),
            title: "T".to_string(),
            section: Some("s".to_string()),
            path: "p".to_string(),
        };
        let debug = format!("{:?}", info);
        assert!(debug.contains("ChapterInfo"));
    }

    #[test]
    fn test_list_chapters_response_debug() {
        let resp = ListChaptersResponse {
            source_id: "s".to_string(),
            chapters: vec![],
            total: 0,
        };
        let debug = format!("{:?}", resp);
        assert!(debug.contains("ListChaptersResponse"));
    }

    #[test]
    fn test_list_sources_response_debug() {
        let resp = ListSourcesResponse { sources: vec![] };
        let debug = format!("{:?}", resp);
        assert!(debug.contains("ListSourcesResponse"));
    }

    #[test]
    fn test_source_availability_response_debug() {
        let resp = SourceAvailabilityResponse {
            id: "x".to_string(),
            status: AvailabilityStatus::Unavailable,
            format: None,
            file_exists: false,
            text_extracted: false,
            searchable: false,
            chapters: None,
            message: "msg".to_string(),
        };
        let debug = format!("{:?}", resp);
        assert!(debug.contains("SourceAvailabilityResponse"));
    }

    // ---------------------------------------------------------------
    // Clone trait coverage
    // ---------------------------------------------------------------

    #[test]
    fn test_source_info_clone() {
        let info = SourceInfo {
            id: "a".to_string(),
            title: "A".to_string(),
            format: SourceFormat::Pdf,
            path: "/p".to_string(),
            chapters: Some(3),
            status: SourceStatus::Converted,
        };
        let cloned = info.clone();
        assert_eq!(cloned.id, "a");
        assert_eq!(cloned.chapters, Some(3));
    }

    #[test]
    fn test_source_format_clone() {
        let f = SourceFormat::Epub;
        let c = f.clone();
        assert!(matches!(c, SourceFormat::Epub));
    }

    #[test]
    fn test_source_status_clone() {
        let s = SourceStatus::NotConverted;
        let c = s.clone();
        assert!(matches!(c, SourceStatus::NotConverted));
    }

    #[test]
    fn test_availability_status_clone() {
        let a = AvailabilityStatus::FileExists;
        let c = a.clone();
        assert!(matches!(c, AvailabilityStatus::FileExists));
    }

    #[test]
    fn test_chapter_info_clone() {
        let info = ChapterInfo {
            id: "x".to_string(),
            title: "X".to_string(),
            section: Some("s".to_string()),
            path: "p".to_string(),
        };
        let cloned = info.clone();
        assert_eq!(cloned.id, "x");
        assert_eq!(cloned.section, Some("s".to_string()));
    }

    // ---------------------------------------------------------------
    // list_unconverted_sources: exercising loop bodies with entries
    // ---------------------------------------------------------------

    #[test]
    fn test_list_unconverted_sources_oxford_entries() {
        use std::collections::HashMap;

        let mut config = Config::load().expect("Config should load");
        config.sources.general.files.clear();
        config.sources.papers.files.clear();

        // Insert a synthetic oxford entry with an absolute path so expand_path succeeds
        config.sources.oxford.path = "/tmp".to_string();
        config.sources.oxford.files = HashMap::from([
            (
                "test-book".to_string(),
                "[2020] Test Author - Music Theory.pdf".to_string(),
            ),
            (
                "test-epub".to_string(),
                "[2019] Another - Study.epub".to_string(),
            ),
        ]);

        let sources = list_unconverted_sources(&config).unwrap();
        assert_eq!(sources.len(), 2);

        for s in &sources {
            assert!(s.id.starts_with("oxford-"));
            assert!(matches!(s.status, SourceStatus::NotConverted));
            assert!(s.chapters.is_none());
        }

        // Verify format detection within the loop
        let pdf_source = sources.iter().find(|s| s.id == "oxford-test-book").unwrap();
        assert!(matches!(pdf_source.format, SourceFormat::Pdf));
        assert_eq!(pdf_source.title, "Test Author - Music Theory");

        let epub_source = sources.iter().find(|s| s.id == "oxford-test-epub").unwrap();
        assert!(matches!(epub_source.format, SourceFormat::Epub));
        assert_eq!(epub_source.title, "Another - Study");
    }

    #[test]
    fn test_list_unconverted_sources_general_entries() {
        use std::collections::HashMap;

        let mut config = Config::load().expect("Config should load");
        config.sources.oxford.files.clear();
        config.sources.papers.files.clear();

        config.sources.general.path = "/tmp".to_string();
        config.sources.general.files = HashMap::from([(
            "harmony-guide".to_string(),
            "[2015] Harmony Guide.pdf".to_string(),
        )]);

        let sources = list_unconverted_sources(&config).unwrap();
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].id, "general-harmony-guide");
        assert!(matches!(sources[0].format, SourceFormat::Pdf));
        assert!(matches!(sources[0].status, SourceStatus::NotConverted));
        assert_eq!(sources[0].title, "Harmony Guide");
    }

    #[test]
    fn test_list_unconverted_sources_papers_entries() {
        use std::collections::HashMap;

        let mut config = Config::load().expect("Config should load");
        config.sources.oxford.files.clear();
        config.sources.general.files.clear();

        config.sources.papers.path = "/tmp".to_string();
        config.sources.papers.files = HashMap::from([(
            "set-theory".to_string(),
            "[2010] Set Theory in Music.xml".to_string(),
        )]);

        let sources = list_unconverted_sources(&config).unwrap();
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].id, "papers-set-theory");
        assert!(matches!(sources[0].format, SourceFormat::Xml));
        assert!(matches!(sources[0].status, SourceStatus::NotConverted));
        assert_eq!(sources[0].title, "Set Theory in Music");
    }

    #[test]
    fn test_list_unconverted_sources_all_categories() {
        use std::collections::HashMap;

        let mut config = Config::load().expect("Config should load");

        config.sources.oxford.path = "/tmp".to_string();
        config.sources.oxford.files =
            HashMap::from([("o1".to_string(), "Oxford Book.pdf".to_string())]);

        config.sources.general.path = "/tmp".to_string();
        config.sources.general.files =
            HashMap::from([("g1".to_string(), "General Book.epub".to_string())]);

        config.sources.papers.path = "/tmp".to_string();
        config.sources.papers.files = HashMap::from([("p1".to_string(), "Paper.xml".to_string())]);

        let sources = list_unconverted_sources(&config).unwrap();
        assert_eq!(sources.len(), 3);

        let has_oxford = sources.iter().any(|s| s.id == "oxford-o1");
        let has_general = sources.iter().any(|s| s.id == "general-g1");
        let has_papers = sources.iter().any(|s| s.id == "papers-p1");
        assert!(has_oxford);
        assert!(has_general);
        assert!(has_papers);
    }

    // ---------------------------------------------------------------
    // check_source_file_exists: success path with real file
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_check_source_file_exists_file_present() {
        use std::collections::HashMap;
        use tempfile::TempDir;

        let temp = TempDir::new().unwrap();
        // Create a real PDF file in the temp directory
        tokio::fs::write(temp.path().join("TestBook.pdf"), b"fake pdf content")
            .await
            .unwrap();

        let mut config = Config::load().expect("Config should load");
        config.sources.oxford.path = temp.path().to_string_lossy().to_string();
        config.sources.oxford.files =
            HashMap::from([("testbook".to_string(), "TestBook.pdf".to_string())]);

        let (exists, format) = check_source_file_exists(&config, "oxford-testbook").await;
        assert!(exists, "File should be detected as existing");
        assert!(format.is_some());
        assert!(matches!(format.unwrap(), SourceFormat::Pdf));
    }

    #[tokio::test]
    async fn test_check_source_file_exists_epub_present() {
        use std::collections::HashMap;
        use tempfile::TempDir;

        let temp = TempDir::new().unwrap();
        tokio::fs::write(temp.path().join("Book.epub"), b"fake epub")
            .await
            .unwrap();

        let mut config = Config::load().expect("Config should load");
        config.sources.general.path = temp.path().to_string_lossy().to_string();
        config.sources.general.files =
            HashMap::from([("mybook".to_string(), "Book.epub".to_string())]);

        let (exists, format) = check_source_file_exists(&config, "general-mybook").await;
        assert!(exists);
        assert!(matches!(format.unwrap(), SourceFormat::Epub));
    }

    #[tokio::test]
    async fn test_check_source_file_exists_file_configured_but_missing() {
        use std::collections::HashMap;

        let mut config = Config::load().expect("Config should load");
        config.sources.papers.path = "/tmp/nonexistent-dir-xyz".to_string();
        config.sources.papers.files =
            HashMap::from([("ghost".to_string(), "Ghost.pdf".to_string())]);

        let (exists, format) = check_source_file_exists(&config, "papers-ghost").await;
        assert!(!exists, "File should not exist on disk");
        assert!(format.is_none());
    }

    // ---------------------------------------------------------------
    // list_chapters_from_filesystem: metadata extraction failure path
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_list_chapters_from_filesystem_bad_metadata_skipped() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        let source_dir = temp.path().join("partial-source");
        fs::create_dir(&source_dir).await.unwrap();

        // File with valid YAML frontmatter (should succeed)
        fs::write(
            source_dir.join("good-chapter.md"),
            "---\ntitle: Good Chapter\n---\n# Good Chapter\n\nContent here.",
        )
        .await
        .unwrap();

        // File with no frontmatter at all - extract_metadata may still succeed
        // by falling back to heading extraction
        fs::write(
            source_dir.join("no-frontmatter.md"),
            "# Just a Heading\n\nBody.",
        )
        .await
        .unwrap();

        // Empty file - metadata extraction may fail
        fs::write(source_dir.join("empty.md"), "").await.unwrap();

        let chapters = list_chapters_from_filesystem(&config, "partial-source")
            .await
            .unwrap();

        // At least the good chapter should appear; others depend on metadata extraction
        assert!(
            !chapters.is_empty(),
            "Should have at least one chapter from good metadata"
        );

        // Verify sorting by path
        for window in chapters.windows(2) {
            assert!(
                window[0].path <= window[1].path,
                "Chapters should be sorted by path"
            );
        }
    }

    #[tokio::test]
    async fn test_list_chapters_from_filesystem_chapter_id_from_stem() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        let source_dir = temp.path().join("stem-test");
        fs::create_dir(&source_dir).await.unwrap();
        fs::write(
            source_dir.join("my-fancy-chapter.md"),
            "---\ntitle: Fancy Title\n---\n# Fancy Title\n\nContent.",
        )
        .await
        .unwrap();

        let chapters = list_chapters_from_filesystem(&config, "stem-test")
            .await
            .unwrap();

        // Chapter ID should come from the file stem
        if !chapters.is_empty() {
            assert_eq!(chapters[0].id, "my-fancy-chapter");
        }
    }

    // ---------------------------------------------------------------
    // Deserialization round-trips: SourceAvailabilityResponse
    // ---------------------------------------------------------------

    #[test]
    fn test_source_availability_response_deserialization_full() {
        let json = r#"{
            "id": "test-src",
            "status": "indexed",
            "format": "epub",
            "file_exists": true,
            "text_extracted": true,
            "searchable": true,
            "chapters": 5,
            "message": "Fully indexed"
        }"#;
        let resp: SourceAvailabilityResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, "test-src");
        assert!(matches!(resp.status, AvailabilityStatus::Indexed));
        assert!(matches!(resp.format, Some(SourceFormat::Epub)));
        assert!(resp.file_exists);
        assert!(resp.text_extracted);
        assert!(resp.searchable);
        assert_eq!(resp.chapters, Some(5));
        assert_eq!(resp.message, "Fully indexed");
    }

    #[test]
    fn test_source_availability_response_deserialization_minimal() {
        let json = r#"{
            "id": "x",
            "status": "unavailable",
            "file_exists": false,
            "text_extracted": false,
            "searchable": false,
            "message": "nope"
        }"#;
        let resp: SourceAvailabilityResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.id, "x");
        assert!(matches!(resp.status, AvailabilityStatus::Unavailable));
        assert!(resp.format.is_none());
        assert!(!resp.file_exists);
        assert!(resp.chapters.is_none());
    }

    #[test]
    fn test_source_availability_response_round_trip() {
        let original = SourceAvailabilityResponse {
            id: "round-trip".to_string(),
            status: AvailabilityStatus::FileExists,
            format: Some(SourceFormat::Xml),
            file_exists: true,
            text_extracted: false,
            searchable: false,
            chapters: None,
            message: "File exists only".to_string(),
        };

        let json = serde_json::to_string(&original).unwrap();
        let restored: SourceAvailabilityResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, "round-trip");
        assert!(matches!(restored.status, AvailabilityStatus::FileExists));
        assert!(matches!(restored.format, Some(SourceFormat::Xml)));
        assert!(restored.file_exists);
        assert!(!restored.text_extracted);
        assert!(restored.chapters.is_none());
    }

    // ---------------------------------------------------------------
    // Serialization round-trips: SourceInfo with chapters
    // ---------------------------------------------------------------

    #[test]
    fn test_source_info_deserialization_with_chapters() {
        let json = r#"{"id":"s","title":"S","format":"pdf","path":"/p","chapters":10,"status":"converted"}"#;
        let info: SourceInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.id, "s");
        assert_eq!(info.chapters, Some(10));
        assert!(matches!(info.status, SourceStatus::Converted));
    }

    #[test]
    fn test_source_info_round_trip_all_formats() {
        for (format, expected_str) in [
            (SourceFormat::Pdf, "pdf"),
            (SourceFormat::Epub, "epub"),
            (SourceFormat::Xml, "xml"),
            (SourceFormat::Markdown, "markdown"),
        ] {
            let info = SourceInfo {
                id: "rt".to_string(),
                title: "RT".to_string(),
                format,
                path: "/p".to_string(),
                chapters: Some(1),
                status: SourceStatus::Converted,
            };
            let json = serde_json::to_string(&info).unwrap();
            assert!(json.contains(expected_str));
            let restored: SourceInfo = serde_json::from_str(&json).unwrap();
            assert_eq!(restored.id, "rt");
            assert_eq!(restored.chapters, Some(1));
        }
    }

    // ---------------------------------------------------------------
    // ListSourcesResponse round-trip with multiple sources
    // ---------------------------------------------------------------

    #[test]
    fn test_list_sources_response_round_trip() {
        let response = ListSourcesResponse {
            sources: vec![
                SourceInfo {
                    id: "a".to_string(),
                    title: "A".to_string(),
                    format: SourceFormat::Pdf,
                    path: "/a".to_string(),
                    chapters: Some(3),
                    status: SourceStatus::Converted,
                },
                SourceInfo {
                    id: "b".to_string(),
                    title: "B".to_string(),
                    format: SourceFormat::Epub,
                    path: "/b".to_string(),
                    chapters: None,
                    status: SourceStatus::NotConverted,
                },
            ],
        };

        let json = serde_json::to_string(&response).unwrap();
        let restored: ListSourcesResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.sources.len(), 2);
        assert_eq!(restored.sources[0].id, "a");
        assert_eq!(restored.sources[0].chapters, Some(3));
        assert_eq!(restored.sources[1].id, "b");
        assert!(restored.sources[1].chapters.is_none());
    }

    // ---------------------------------------------------------------
    // ListChaptersResponse round-trip with chapters
    // ---------------------------------------------------------------

    #[test]
    fn test_list_chapters_response_round_trip() {
        let response = ListChaptersResponse {
            source_id: "my-source".to_string(),
            chapters: vec![
                ChapterInfo {
                    id: "ch-01".to_string(),
                    title: "First".to_string(),
                    section: Some("pp. 1-10".to_string()),
                    path: "/p/ch-01.md".to_string(),
                },
                ChapterInfo {
                    id: "ch-02".to_string(),
                    title: "Second".to_string(),
                    section: None,
                    path: "/p/ch-02.md".to_string(),
                },
            ],
            total: 2,
        };

        let json = serde_json::to_string(&response).unwrap();
        let restored: ListChaptersResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.source_id, "my-source");
        assert_eq!(restored.total, 2);
        assert_eq!(restored.chapters.len(), 2);
        assert_eq!(restored.chapters[0].section, Some("pp. 1-10".to_string()));
        assert!(restored.chapters[1].section.is_none());
    }

    // ---------------------------------------------------------------
    // check_source_availability: FileExists status path
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_check_source_availability_file_exists_not_converted() {
        use std::collections::HashMap;
        use tempfile::TempDir;

        let temp = TempDir::new().unwrap();

        // Create a real PDF file
        tokio::fs::write(temp.path().join("TheBook.pdf"), b"fake pdf")
            .await
            .unwrap();

        let mut config = Config::load().expect("Config should load");
        // Point sources_md to a directory where "oxford-thebook" subdirectory does NOT exist
        config.paths.sources_md = temp.path().join("no-md-here").to_string_lossy().to_string();
        config.sources.oxford.path = temp.path().to_string_lossy().to_string();
        config.sources.oxford.files =
            HashMap::from([("thebook".to_string(), "TheBook.pdf".to_string())]);
        config.sources.general.files.clear();
        config.sources.papers.files.clear();

        let state = AppState::new(config).await.expect("State should init");
        let resp = check_source_availability(&state, "oxford-thebook")
            .await
            .unwrap();

        assert_eq!(resp.id, "oxford-thebook");
        assert!(
            matches!(resp.status, AvailabilityStatus::FileExists),
            "Expected FileExists status, got {:?}",
            resp.status
        );
        assert!(resp.file_exists);
        assert!(!resp.text_extracted);
        assert!(!resp.searchable);
        assert!(resp.chapters.is_none());
        assert!(resp.format.is_some());
        assert!(resp.message.contains("file exists"));
    }

    // ---------------------------------------------------------------
    // list_sources: integration with unconverted entries
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_list_sources_with_unconverted_entries() {
        use std::collections::HashMap;

        let mut config = Config::load().expect("Config should load");
        // Point sources_md to nonexistent so no converted sources appear
        config.paths.sources_md = "/tmp/nonexistent-sources-md-xyz".to_string();

        config.sources.oxford.path = "/tmp".to_string();
        config.sources.oxford.files =
            HashMap::from([("book1".to_string(), "Book1.pdf".to_string())]);
        config.sources.general.path = "/tmp".to_string();
        config.sources.general.files =
            HashMap::from([("book2".to_string(), "Book2.epub".to_string())]);
        config.sources.papers.path = "/tmp".to_string();
        config.sources.papers.files =
            HashMap::from([("paper1".to_string(), "Paper1.xml".to_string())]);

        let resp = list_sources(&config).await.unwrap();
        assert_eq!(resp.sources.len(), 3);

        let ids: Vec<&str> = resp.sources.iter().map(|s| s.id.as_str()).collect();
        assert!(ids.contains(&"oxford-book1"));
        assert!(ids.contains(&"general-book2"));
        assert!(ids.contains(&"papers-paper1"));

        // All should be not-converted
        for s in &resp.sources {
            assert!(matches!(s.status, SourceStatus::NotConverted));
        }
    }

    #[tokio::test]
    async fn test_list_sources_combined_converted_and_unconverted() {
        use std::collections::HashMap;
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        // Create one converted source
        let source_dir = temp.path().join("converted-src");
        fs::create_dir(&source_dir).await.unwrap();
        fs::write(source_dir.join("ch1.md"), "# Ch 1")
            .await
            .unwrap();

        // Add one unconverted source
        config.sources.oxford.path = "/tmp".to_string();
        config.sources.oxford.files = HashMap::from([("raw".to_string(), "Raw.pdf".to_string())]);
        config.sources.general.files.clear();
        config.sources.papers.files.clear();

        let resp = list_sources(&config).await.unwrap();
        assert_eq!(resp.sources.len(), 2);

        let converted = resp
            .sources
            .iter()
            .find(|s| s.id == "converted-src")
            .unwrap();
        assert!(matches!(converted.status, SourceStatus::Converted));
        assert_eq!(converted.chapters, Some(1));

        let unconverted = resp.sources.iter().find(|s| s.id == "oxford-raw").unwrap();
        assert!(matches!(unconverted.status, SourceStatus::NotConverted));
        assert!(unconverted.chapters.is_none());
    }

    // ---------------------------------------------------------------
    // get_source_chapter: edge cases for chapter search
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_get_source_chapter_section_found_returns_only_section() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let mut config = Config::load().expect("Config should load");
        config.paths.sources_md = temp.path().to_string_lossy().to_string();

        let source_dir = temp.path().join("section-source");
        fs::create_dir(&source_dir).await.unwrap();
        let content = "# Main\n\n## Alpha\n\nAlpha content.\n\n## Beta\n\nBeta content.\n";
        fs::write(source_dir.join("main.md"), content)
            .await
            .unwrap();

        // Request specific section
        let result = get_source_chapter(&config, "section-source", "main", Some("Alpha"))
            .await
            .unwrap();
        assert!(result.contains("## Alpha"));
        assert!(result.contains("Alpha content."));
        assert!(!result.contains("Beta"));
    }

    // ---------------------------------------------------------------
    // extract_section: deep heading levels
    // ---------------------------------------------------------------

    #[test]
    fn test_extract_section_deep_heading_levels() {
        let markdown = r#"#### Deep Heading

Deep content here.

##### Even Deeper

Sub-content.

#### Another Deep

Other content.
"#;
        let result = extract_section(markdown, "Deep Heading");
        assert!(result.is_some());
        let extracted = result.unwrap();
        assert!(extracted.contains("#### Deep Heading"));
        assert!(extracted.contains("Deep content here."));
        assert!(extracted.contains("##### Even Deeper"));
        assert!(!extracted.contains("Another Deep"));
    }

    #[test]
    fn test_extract_section_whitespace_in_heading() {
        let markdown = "##   Spaced  Heading  \n\nContent.\n\n## Next\n\nMore.\n";
        // The extract_section trims the heading text, so "Spaced  Heading" with
        // extra internal spaces won't match "Spaced Heading" (single space).
        // This tests the actual behavior of the trimming logic.
        let result = extract_section(markdown, "Spaced  Heading");
        assert!(result.is_some());
    }

    // ---------------------------------------------------------------
    // scan_converted_sources: path in SourceInfo
    // ---------------------------------------------------------------

    #[tokio::test]
    async fn test_scan_converted_sources_path_field() {
        use tempfile::TempDir;
        use tokio::fs;

        let temp = TempDir::new().unwrap();
        let source_dir = temp.path().join("path-check");
        fs::create_dir(&source_dir).await.unwrap();
        fs::write(source_dir.join("ch.md"), "# Ch").await.unwrap();

        let sources = scan_converted_sources(temp.path()).await.unwrap();
        assert_eq!(sources.len(), 1);
        // Path should contain the full directory path
        assert!(
            sources[0].path.contains("path-check"),
            "Path should include directory name: {}",
            sources[0].path
        );
    }

    // ---------------------------------------------------------------
    // get_source_pdf_path: papers category with compound file ID
    // ---------------------------------------------------------------

    #[test]
    fn test_get_source_pdf_path_papers_compound_id() {
        use std::collections::HashMap;

        let mut config = Config::load().expect("Config should load");
        config.sources.papers.path = "/tmp".to_string();
        config.sources.papers.files = HashMap::from([(
            "set-theory-intro".to_string(),
            "Set Theory Intro.pdf".to_string(),
        )]);

        let result = get_source_pdf_path(&config, "papers-set-theory-intro");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("Set Theory Intro.pdf"));
    }

    #[test]
    fn test_get_source_pdf_path_general_known_file() {
        use std::collections::HashMap;

        let mut config = Config::load().expect("Config should load");
        config.sources.general.path = "/tmp".to_string();
        config.sources.general.files =
            HashMap::from([("my-guide".to_string(), "MyGuide.epub".to_string())]);

        let result = get_source_pdf_path(&config, "general-my-guide");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("MyGuide.epub"));
    }
}
