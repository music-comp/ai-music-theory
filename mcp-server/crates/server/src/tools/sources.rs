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
}
