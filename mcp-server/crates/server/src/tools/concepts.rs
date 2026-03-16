use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::error::Result;
use crate::markdown::extract_frontmatter;
use crate::metadata::extract_concept_metadata;
use crate::util::files::{find_all_files, find_file_by_id, read_file, FindOptions};

/// Information about a concept card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptInfo {
    pub id: String,
    pub title: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraction_confidence: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_page: Option<i32>,
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
    pub tier: Option<String>,
    #[serde(default)]
    pub subcategory: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
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

    // Apply filters if specified
    if let Some(params) = params {
        if let Some(category) = params.category {
            concepts.retain(|c| c.category == category);
        }
        if let Some(tier) = params.tier {
            concepts.retain(|c| c.tier.as_deref() == Some(tier.as_str()));
        }
        if let Some(subcategory) = params.subcategory {
            concepts.retain(|c| c.subcategory.as_deref() == Some(subcategory.as_str()));
        }
        if let Some(source) = params.source {
            concepts.retain(|c| c.source.as_deref() == Some(source.as_str()));
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
async fn scan_concept_cards(base_path: &std::path::Path) -> Result<Vec<ConceptInfo>> {
    let mut concepts = Vec::new();

    // Find all markdown files in the concept cards directory
    let files = find_all_files(base_path, FindOptions::markdown()).await?;

    for file_info in files {
        let path = &file_info.path;

        // Extract metadata from file
        let meta = extract_concept_metadata(base_path, path).await?;

        // Use description from metadata as preview
        let preview = meta.description.clone();

        // Parse frontmatter to extract v3 fields not yet in ConceptMetadata
        let content = read_file(path).await.unwrap_or_default();
        let (fm, _) = extract_frontmatter(&content).unwrap_or((None, ""));

        let (subcategory, tier, extraction_confidence, aliases, chapter_number, pdf_page) =
            if let Some(fm) = fm {
                (
                    fm.subcategory,
                    fm.tier,
                    fm.extraction_confidence,
                    fm.aliases,
                    fm.chapter_number,
                    fm.pdf_page,
                )
            } else {
                (None, None, None, Vec::new(), None, None)
            };

        concepts.push(ConceptInfo {
            id: meta.id,
            title: meta.title,
            category: meta.category,
            subcategory,
            tier,
            source: meta.source,
            chapter: meta.chapter,
            extraction_confidence,
            aliases,
            chapter_number,
            pdf_page,
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

/// Get a specific concept card by ID.
pub async fn get_concept(config: &Config, concept_id: &str) -> Result<String> {
    let concept_cards_path = config.paths.concept_cards_path()?;

    // Search for the concept file
    let concept_path = find_concept_file(&concept_cards_path, concept_id).await?;

    // Read and return the content
    let content = read_file(&concept_path).await?;
    Ok(content)
}

/// Information about a concept category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub name: String,
    pub count: usize,
}

/// Response for list_categories tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListCategoriesResponse {
    pub categories: Vec<CategoryInfo>,
    pub total: usize,
}

/// List all distinct concept categories with counts.
///
/// This tool scans all concept cards and returns a list of unique categories
/// with the number of concepts in each category.
pub async fn list_categories(config: &Config) -> Result<ListCategoriesResponse> {
    let concept_cards_path = config.paths.concept_cards_path()?;

    if !crate::util::files::exists(&concept_cards_path).await {
        return Ok(ListCategoriesResponse {
            categories: Vec::new(),
            total: 0,
        });
    }

    // Scan all concept files
    let files = find_all_files(&concept_cards_path, FindOptions::markdown()).await?;

    // Count by category
    let mut category_counts: HashMap<String, usize> = HashMap::new();

    for file_info in files {
        if let Ok(meta) = extract_concept_metadata(&concept_cards_path, &file_info.path).await {
            *category_counts.entry(meta.category).or_insert(0) += 1;
        }
    }

    // Convert to sorted list
    let mut categories: Vec<CategoryInfo> = category_counts
        .into_iter()
        .map(|(name, count)| CategoryInfo { name, count })
        .collect();

    categories.sort_by(|a, b| a.name.cmp(&b.name));
    let total = categories.len();

    Ok(ListCategoriesResponse { categories, total })
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
    use tempfile::TempDir;
    use tokio::fs;

    /// Helper to build a minimal `ConceptInfo` with sensible defaults.
    fn concept_info(id: &str, title: &str, category: &str, path: &str) -> ConceptInfo {
        ConceptInfo {
            id: id.to_string(),
            title: title.to_string(),
            category: category.to_string(),
            subcategory: None,
            tier: None,
            source: None,
            chapter: None,
            extraction_confidence: None,
            aliases: Vec::new(),
            chapter_number: None,
            pdf_page: None,
            path: path.to_string(),
            preview: None,
        }
    }

    /// Helper to build `ListConceptsParams` with all filters defaulting to `None`.
    fn list_params() -> ListConceptsParams {
        ListConceptsParams {
            category: None,
            tier: None,
            subcategory: None,
            source: None,
            limit: None,
        }
    }

    #[test]
    fn test_concept_info_serialization() {
        let mut info = concept_info(
            "test-concept",
            "Test Concept",
            "harmony",
            "/path/to/concept.md",
        );
        info.source = Some("Open Music Theory".to_string());
        info.chapter = Some("Test Chapter".to_string());
        info.preview = Some("This is a preview".to_string());

        let json = serde_json::to_string(&info).expect("Should serialize");
        assert!(json.contains("test-concept"));
        assert!(json.contains("Test Concept"));
        assert!(json.contains("harmony"));
        assert!(json.contains("Open Music Theory"));
        assert!(json.contains("This is a preview"));
    }

    #[test]
    fn test_concept_info_serialization_no_preview() {
        let info = concept_info("test", "Test", "test", "/path");

        let json = serde_json::to_string(&info).expect("Should serialize");
        // Optional None fields should be skipped
        assert!(!json.contains("preview"));
        assert!(!json.contains("source"));
        assert!(!json.contains("chapter"));
        assert!(!json.contains("subcategory"));
        assert!(!json.contains("tier"));
        assert!(!json.contains("extraction_confidence"));
        assert!(!json.contains("aliases"));
        assert!(!json.contains("chapter_number"));
        assert!(!json.contains("pdf_page"));
    }

    #[test]
    fn test_concept_info_serialization_v3_fields() {
        let mut info = concept_info(
            "acoustic-consonance",
            "Acoustic Consonance",
            "acoustics",
            "/path",
        );
        info.subcategory = Some("consonance-dissonance".to_string());
        info.tier = Some("foundational".to_string());
        info.extraction_confidence = Some("high".to_string());
        info.aliases = vec![
            "sensory consonance".to_string(),
            "tonal consonance".to_string(),
        ];
        info.chapter_number = Some(2);
        info.pdf_page = Some(45);

        let json = serde_json::to_string(&info).expect("Should serialize");
        assert!(json.contains("\"subcategory\":\"consonance-dissonance\""));
        assert!(json.contains("\"tier\":\"foundational\""));
        assert!(json.contains("\"extraction_confidence\":\"high\""));
        assert!(json.contains("sensory consonance"));
        assert!(json.contains("tonal consonance"));
        assert!(json.contains("\"chapter_number\":2"));
        assert!(json.contains("\"pdf_page\":45"));
    }

    #[test]
    fn test_list_concepts_response_serialization() {
        let response = ListConceptsResponse {
            concepts: vec![concept_info("test", "Test", "harmony", "/path")],
            total: 1,
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        assert!(json.contains("concepts"));
        assert!(json.contains("total"));
        assert!(json.contains("\"total\":1"));
    }

    #[test]
    fn test_list_concepts_params_default() {
        let json = "{}";
        let params: ListConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert!(params.category.is_none());
        assert!(params.tier.is_none());
        assert!(params.subcategory.is_none());
        assert!(params.source.is_none());
        assert!(params.limit.is_none());
    }

    #[test]
    fn test_list_concepts_params_with_category() {
        let json = r#"{"category":"harmony"}"#;
        let params: ListConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.category, Some("harmony".to_string()));
        assert!(params.limit.is_none());
    }

    #[test]
    fn test_list_concepts_params_with_limit() {
        let json = r#"{"limit":10}"#;
        let params: ListConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert!(params.category.is_none());
        assert_eq!(params.limit, Some(10));
    }

    #[test]
    fn test_list_concepts_params_with_both() {
        let json = r#"{"category":"rhythm","limit":5}"#;
        let params: ListConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.category, Some("rhythm".to_string()));
        assert_eq!(params.limit, Some(5));
    }

    #[test]
    fn test_list_concepts_params_with_tier() {
        let json = r#"{"tier":"foundational"}"#;
        let params: ListConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.tier, Some("foundational".to_string()));
        assert!(params.category.is_none());
    }

    #[test]
    fn test_list_concepts_params_with_subcategory() {
        let json = r#"{"subcategory":"consonance-dissonance"}"#;
        let params: ListConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(
            params.subcategory,
            Some("consonance-dissonance".to_string())
        );
    }

    #[test]
    fn test_list_concepts_params_with_source() {
        let json = r#"{"source":"Open Music Theory"}"#;
        let params: ListConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.source, Some("Open Music Theory".to_string()));
    }

    #[test]
    fn test_list_concepts_params_with_all_filters() {
        let json = r#"{"category":"harmony","tier":"advanced","subcategory":"chords","source":"OMT","limit":10}"#;
        let params: ListConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.category, Some("harmony".to_string()));
        assert_eq!(params.tier, Some("advanced".to_string()));
        assert_eq!(params.subcategory, Some("chords".to_string()));
        assert_eq!(params.source, Some("OMT".to_string()));
        assert_eq!(params.limit, Some(10));
    }

    #[tokio::test]
    async fn test_scan_concept_cards() {
        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path().join("concepts");
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        // Create test concept files with frontmatter
        fs::write(
            harmony_dir.join("triads.md"),
            "---\ntitle: Triads\ncategory: harmony\n---\n\n# Triads\n\nThree-note chords.",
        )
        .await
        .unwrap();
        fs::write(
            harmony_dir.join("seventh-chords.md"),
            "---\ntitle: Seventh Chords\ncategory: harmony\n---\n\n# Seventh Chords\n\nFour-note chords.",
        )
        .await
        .unwrap();

        let concepts = scan_concept_cards(&concepts_dir).await.unwrap();
        assert_eq!(concepts.len(), 2);
        assert_eq!(concepts[0].category, "harmony");
        assert_eq!(concepts[0].id, "seventh-chords");
        assert_eq!(concepts[0].title, "Seventh Chords");
        assert_eq!(concepts[1].id, "triads");
    }

    #[tokio::test]
    async fn test_scan_concept_cards_v3_fields() {
        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path().join("concepts");
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        // Create a v3 concept card with all fields
        let content = r#"---
title: Acoustic Consonance
category: acoustics
subcategory: consonance-dissonance
tier: foundational
source: Geometry of Music
extraction_confidence: high
aliases:
  - sensory consonance
  - tonal consonance
chapter_number: 2
pdf_page: 45
---
# Acoustic Consonance

Content here."#;

        fs::write(harmony_dir.join("acoustic-consonance.md"), content)
            .await
            .unwrap();

        let concepts = scan_concept_cards(&concepts_dir).await.unwrap();
        assert_eq!(concepts.len(), 1);
        let c = &concepts[0];
        assert_eq!(c.id, "acoustic-consonance");
        assert_eq!(c.subcategory.as_deref(), Some("consonance-dissonance"));
        assert_eq!(c.tier.as_deref(), Some("foundational"));
        assert_eq!(c.extraction_confidence.as_deref(), Some("high"));
        assert_eq!(c.aliases, vec!["sensory consonance", "tonal consonance"]);
        assert_eq!(c.chapter_number, Some(2));
        assert_eq!(c.pdf_page, Some(45));
    }

    #[tokio::test]
    async fn test_scan_concept_cards_v3_fields_absent() {
        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path().join("concepts");
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        // Create a v2-style card without v3 fields
        fs::write(
            harmony_dir.join("triads.md"),
            "---\ntitle: Triads\ncategory: harmony\n---\n\n# Triads",
        )
        .await
        .unwrap();

        let concepts = scan_concept_cards(&concepts_dir).await.unwrap();
        assert_eq!(concepts.len(), 1);
        let c = &concepts[0];
        assert!(c.subcategory.is_none());
        assert!(c.tier.is_none());
        assert!(c.extraction_confidence.is_none());
        assert!(c.aliases.is_empty());
        assert!(c.chapter_number.is_none());
        assert!(c.pdf_page.is_none());
    }

    #[tokio::test]
    async fn test_find_concept_file() {
        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path().join("concepts");
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        // Create concept file
        fs::write(harmony_dir.join("triads.md"), "# Triads")
            .await
            .unwrap();

        let found = find_concept_file(&concepts_dir, "triads").await.unwrap();
        assert!(found.ends_with("triads.md"));
    }

    #[tokio::test]
    async fn test_find_concept_file_readme() {
        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path().join("concepts");
        let triad_dir = concepts_dir.join("triad");
        fs::create_dir_all(&triad_dir).await.unwrap();

        // Create concept as README.md
        fs::write(triad_dir.join("README.md"), "# Triad")
            .await
            .unwrap();

        let found = find_concept_file(&concepts_dir, "triad").await.unwrap();
        assert!(found.ends_with("README.md"));
    }

    #[tokio::test]
    async fn test_list_concepts_empty_directory() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path().join("nonexistent");

        // Create a mock config pointing to nonexistent directory
        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        let response = list_concepts(&test_config, None).await.unwrap();
        assert_eq!(response.concepts.len(), 0);
        assert_eq!(response.total, 0);
    }

    #[tokio::test]
    async fn test_list_concepts_with_category_filter() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path();
        let harmony_dir = concepts_dir.join("harmony");
        let rhythm_dir = concepts_dir.join("rhythm");
        fs::create_dir_all(&harmony_dir).await.unwrap();
        fs::create_dir_all(&rhythm_dir).await.unwrap();

        // Create concept files
        fs::write(harmony_dir.join("triads.md"), "# Triads")
            .await
            .unwrap();
        fs::write(rhythm_dir.join("meter.md"), "# Meter")
            .await
            .unwrap();

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        // Filter by harmony category
        let mut params = list_params();
        params.category = Some("harmony".to_string());
        let response = list_concepts(&test_config, Some(params)).await.unwrap();
        assert_eq!(response.concepts.len(), 1);
        assert_eq!(response.concepts[0].category, "harmony");
    }

    #[tokio::test]
    async fn test_list_concepts_with_tier_filter() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path();
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        // Create concept files with different tiers
        fs::write(
            harmony_dir.join("triads.md"),
            "---\ntitle: Triads\ncategory: harmony\ntier: foundational\n---\n# Triads",
        )
        .await
        .unwrap();
        fs::write(
            harmony_dir.join("neo-riemannian.md"),
            "---\ntitle: Neo-Riemannian\ncategory: harmony\ntier: advanced\n---\n# Neo-Riemannian",
        )
        .await
        .unwrap();

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        // Filter by foundational tier
        let mut params = list_params();
        params.tier = Some("foundational".to_string());
        let response = list_concepts(&test_config, Some(params)).await.unwrap();
        assert_eq!(response.concepts.len(), 1);
        assert_eq!(response.concepts[0].id, "triads");
        assert_eq!(response.concepts[0].tier.as_deref(), Some("foundational"));
    }

    #[tokio::test]
    async fn test_list_concepts_with_subcategory_filter() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path();
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        fs::write(
            harmony_dir.join("triads.md"),
            "---\ntitle: Triads\ncategory: harmony\nsubcategory: chords\n---\n# Triads",
        )
        .await
        .unwrap();
        fs::write(
            harmony_dir.join("consonance.md"),
            "---\ntitle: Consonance\ncategory: harmony\nsubcategory: consonance-dissonance\n---\n# Consonance",
        )
        .await
        .unwrap();

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        let mut params = list_params();
        params.subcategory = Some("chords".to_string());
        let response = list_concepts(&test_config, Some(params)).await.unwrap();
        assert_eq!(response.concepts.len(), 1);
        assert_eq!(response.concepts[0].id, "triads");
    }

    #[tokio::test]
    async fn test_list_concepts_with_source_filter() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path();
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        fs::write(
            harmony_dir.join("triads.md"),
            "---\ntitle: Triads\ncategory: harmony\nsource: Open Music Theory\n---\n# Triads",
        )
        .await
        .unwrap();
        fs::write(
            harmony_dir.join("consonance.md"),
            "---\ntitle: Consonance\ncategory: harmony\nsource: Geometry of Music\n---\n# Consonance",
        )
        .await
        .unwrap();

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        let mut params = list_params();
        params.source = Some("Geometry of Music".to_string());
        let response = list_concepts(&test_config, Some(params)).await.unwrap();
        assert_eq!(response.concepts.len(), 1);
        assert_eq!(response.concepts[0].id, "consonance");
    }

    #[tokio::test]
    async fn test_list_concepts_with_combined_filters() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path();
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        fs::write(
            harmony_dir.join("triads.md"),
            "---\ntitle: Triads\ncategory: harmony\ntier: foundational\nsource: OMT\n---\n# Triads",
        )
        .await
        .unwrap();
        fs::write(
            harmony_dir.join("neo.md"),
            "---\ntitle: Neo\ncategory: harmony\ntier: advanced\nsource: OMT\n---\n# Neo",
        )
        .await
        .unwrap();
        fs::write(
            harmony_dir.join("consonance.md"),
            "---\ntitle: Consonance\ncategory: harmony\ntier: foundational\nsource: GoM\n---\n# Consonance",
        )
        .await
        .unwrap();

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        // Filter by category + tier + source: should match only triads
        let mut params = list_params();
        params.category = Some("harmony".to_string());
        params.tier = Some("foundational".to_string());
        params.source = Some("OMT".to_string());
        let response = list_concepts(&test_config, Some(params)).await.unwrap();
        assert_eq!(response.concepts.len(), 1);
        assert_eq!(response.concepts[0].id, "triads");
    }

    #[tokio::test]
    async fn test_list_concepts_filter_no_match() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path();
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        fs::write(
            harmony_dir.join("triads.md"),
            "---\ntitle: Triads\ncategory: harmony\ntier: foundational\n---\n# Triads",
        )
        .await
        .unwrap();

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        // Filter by tier that doesn't exist
        let mut params = list_params();
        params.tier = Some("expert".to_string());
        let response = list_concepts(&test_config, Some(params)).await.unwrap();
        assert_eq!(response.concepts.len(), 0);
        assert_eq!(response.total, 0);
    }

    #[tokio::test]
    async fn test_list_concepts_with_limit() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path();
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        // Create multiple concept files
        fs::write(harmony_dir.join("triads.md"), "# Triads")
            .await
            .unwrap();
        fs::write(harmony_dir.join("sevenths.md"), "# Sevenths")
            .await
            .unwrap();
        fs::write(harmony_dir.join("ninths.md"), "# Ninths")
            .await
            .unwrap();

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        // Limit to 2 results
        let mut params = list_params();
        params.limit = Some(2);
        let response = list_concepts(&test_config, Some(params)).await.unwrap();
        assert_eq!(response.concepts.len(), 2);
        assert_eq!(response.total, 2);
    }

    #[tokio::test]
    async fn test_get_concept() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path();
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        let content = "# Triads\n\nThree-note chords.";
        fs::write(harmony_dir.join("triads.md"), content)
            .await
            .unwrap();

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        let result = get_concept(&test_config, "triads").await.unwrap();
        assert_eq!(result, content);
    }

    #[test]
    fn test_category_info_serialization() {
        let info = CategoryInfo {
            name: "harmony".to_string(),
            count: 5,
        };

        let json = serde_json::to_string(&info).expect("Should serialize");
        assert!(json.contains("harmony"));
        assert!(json.contains("\"count\":5"));
    }

    #[test]
    fn test_list_categories_response_serialization() {
        let response = ListCategoriesResponse {
            categories: vec![
                CategoryInfo {
                    name: "harmony".to_string(),
                    count: 5,
                },
                CategoryInfo {
                    name: "rhythm".to_string(),
                    count: 3,
                },
            ],
            total: 2,
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        assert!(json.contains("categories"));
        assert!(json.contains("harmony"));
        assert!(json.contains("rhythm"));
        assert!(json.contains("\"total\":2"));
    }

    #[tokio::test]
    async fn test_list_categories() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path();
        let harmony_dir = concepts_dir.join("harmony");
        let rhythm_dir = concepts_dir.join("rhythm");
        fs::create_dir_all(&harmony_dir).await.unwrap();
        fs::create_dir_all(&rhythm_dir).await.unwrap();

        // Create concept files with categories in frontmatter
        fs::write(
            harmony_dir.join("triads.md"),
            "---\ncategory: harmony\n---\n# Triads",
        )
        .await
        .unwrap();
        fs::write(
            harmony_dir.join("sevenths.md"),
            "---\ncategory: harmony\n---\n# Sevenths",
        )
        .await
        .unwrap();
        fs::write(
            rhythm_dir.join("meter.md"),
            "---\ncategory: rhythm\n---\n# Meter",
        )
        .await
        .unwrap();

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        let response = list_categories(&test_config).await.unwrap();

        assert_eq!(response.total, 2);
        assert_eq!(response.categories.len(), 2);

        // Should be sorted alphabetically
        assert_eq!(response.categories[0].name, "harmony");
        assert_eq!(response.categories[0].count, 2);
        assert_eq!(response.categories[1].name, "rhythm");
        assert_eq!(response.categories[1].count, 1);
    }

    #[tokio::test]
    async fn test_list_categories_empty_directory() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path().join("nonexistent");

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        let response = list_categories(&test_config).await.unwrap();
        assert_eq!(response.categories.len(), 0);
        assert_eq!(response.total, 0);
    }

    #[tokio::test]
    async fn test_list_categories_fallback_to_directory() {
        use crate::config::Config;

        let temp = TempDir::new().unwrap();
        let concepts_dir = temp.path();
        let harmony_dir = concepts_dir.join("harmony");
        fs::create_dir_all(&harmony_dir).await.unwrap();

        // Create file without category in frontmatter
        fs::write(harmony_dir.join("test.md"), "# Test\n\nNo frontmatter")
            .await
            .unwrap();

        let config = Config::load().unwrap();
        let mut test_config = config.clone();
        test_config.paths.concept_cards = concepts_dir.to_string_lossy().to_string();

        let response = list_categories(&test_config).await.unwrap();

        assert_eq!(response.total, 1);
        // Should fall back to directory structure
        assert_eq!(response.categories[0].name, "harmony");
        assert_eq!(response.categories[0].count, 1);
    }
}
