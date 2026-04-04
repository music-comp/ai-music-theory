//! Content layer: provider implementations for music theory content.
//!
//! This module wraps existing tool functionality behind fabryk's content traits,
//! enabling domain-agnostic MCP tools to operate on music theory data.
//!
//! - [`MusicTheoryContentProvider`] implements [`ContentItemProvider`] for concept cards,
//!   delegating to `tools::concepts`.
//! - [`MusicTheorySourceProvider`] implements [`SourceProvider`] for reference materials
//!   (books, papers), delegating to `tools::sources`.
//!
//! Both providers delegate to existing functions rather than reimplementing logic.
//! This allows incremental migration: the old tool handlers continue to work
//! while new trait-based handlers can be introduced alongside them.

use async_trait::async_trait;
use fabryk::core::Result;
use fabryk_mcp::content::{
    CategoryInfo, ChapterInfo as FabrykChapterInfo, ContentItemProvider, FilterMap, GuideProvider,
    QuestionMatch, QuestionSearchProvider, QuestionSearchResponse, SourceProvider,
};
use serde::Serialize;
use std::path::PathBuf;

use crate::config::Config;
use crate::state::AppState;
use crate::tools::concepts::{ListCategoriesResponse, ListConceptsParams};

/// Summary information for a music theory concept card.
///
/// This is the `ItemSummary` associated type for `ContentItemProvider`.
/// It mirrors the fields from `tools::concepts::ConceptInfo` that are
/// relevant for listing operations.
#[derive(Debug, Clone, Serialize)]
pub struct ConceptSummary {
    /// Concept identifier (e.g., "major-triad").
    pub id: String,
    /// Human-readable title (e.g., "Major Triad").
    pub title: String,
    /// Category the concept belongs to (e.g., "harmony").
    pub category: String,
    /// Source text, if any (e.g., "Open Music Theory").
    pub source: Option<String>,
    /// Short description or preview text.
    pub preview: Option<String>,
    // V3 fields
    /// Finer classification within category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    /// Prerequisite depth: foundational, intermediate, advanced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Extraction quality: high, medium, low.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraction_confidence: Option<String>,
    /// Alternative names, abbreviations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
}

/// Content provider for music theory concept cards.
///
/// Wraps the existing concept card infrastructure, delegating to
/// `tools::concepts::list_concepts`, `tools::concepts::get_concept`,
/// and `tools::concepts::list_categories`.
#[derive(Debug)]
pub struct MusicTheoryContentProvider {
    config: Config,
}

impl MusicTheoryContentProvider {
    /// Create a new content provider with the given configuration.
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ContentItemProvider for MusicTheoryContentProvider {
    type ItemSummary = ConceptSummary;
    type ItemDetail = String;

    async fn list_items(
        &self,
        category: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<Self::ItemSummary>> {
        let params = if category.is_some() || limit.is_some() {
            Some(ListConceptsParams {
                category: category.map(String::from),
                limit,
                tier: None,
                subcategory: None,
                source: None,
            })
        } else {
            None
        };

        let response = crate::tools::concepts::list_concepts(&self.config, params).await?;

        let summaries = response
            .concepts
            .into_iter()
            .map(|c| ConceptSummary {
                id: c.id,
                title: c.title,
                category: c.category,
                source: c.source,
                preview: c.preview,
                subcategory: c.subcategory,
                tier: c.tier,
                extraction_confidence: c.extraction_confidence,
                aliases: c.aliases,
            })
            .collect();

        Ok(summaries)
    }

    async fn list_items_filtered(
        &self,
        category: Option<&str>,
        limit: Option<usize>,
        extra_filters: &FilterMap,
    ) -> Result<Vec<Self::ItemSummary>> {
        let tier = extra_filters.get("tier").and_then(|v| v.as_str());
        let subcategory = extra_filters.get("subcategory").and_then(|v| v.as_str());
        let source = extra_filters.get("source").and_then(|v| v.as_str());

        let has_filters = category.is_some()
            || limit.is_some()
            || tier.is_some()
            || subcategory.is_some()
            || source.is_some();

        let params = if has_filters {
            Some(ListConceptsParams {
                category: category.map(String::from),
                limit,
                tier: tier.map(String::from),
                subcategory: subcategory.map(String::from),
                source: source.map(String::from),
            })
        } else {
            None
        };

        let response = crate::tools::concepts::list_concepts(&self.config, params).await?;

        let summaries = response
            .concepts
            .into_iter()
            .map(|c| ConceptSummary {
                id: c.id,
                title: c.title,
                category: c.category,
                source: c.source,
                preview: c.preview,
                subcategory: c.subcategory,
                tier: c.tier,
                extraction_confidence: c.extraction_confidence,
                aliases: c.aliases,
            })
            .collect();

        Ok(summaries)
    }

    async fn get_item(&self, id: &str) -> Result<Self::ItemDetail> {
        crate::tools::concepts::get_concept(&self.config, id).await
    }

    async fn list_categories(&self) -> Result<Vec<CategoryInfo>> {
        let response: ListCategoriesResponse =
            crate::tools::concepts::list_categories(&self.config).await?;

        let categories = response
            .categories
            .into_iter()
            .map(|c| CategoryInfo {
                id: c.name.clone(),
                name: c.name,
                count: c.count,
                description: None,
            })
            .collect();

        Ok(categories)
    }

    fn content_type_name(&self) -> &str {
        "concept"
    }

    fn content_type_name_plural(&self) -> &str {
        "concepts"
    }
}

// ============================================================================
// Source Provider
// ============================================================================

/// Summary information for a source material.
///
/// This is the `SourceSummary` associated type for [`SourceProvider`].
/// It mirrors the fields from [`crate::tools::sources::SourceInfo`] that are
/// relevant for listing operations.
#[derive(Debug, Clone, Serialize)]
pub struct SourceSummary {
    /// Source identifier (e.g., "open-music-theory").
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Format of the source file.
    pub format: String,
    /// Filesystem path to the source.
    pub path: String,
    /// Number of chapters, if converted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapters: Option<usize>,
    /// Conversion status.
    pub status: String,
}

/// Source provider for music theory reference materials.
///
/// Wraps the existing source infrastructure, delegating to
/// [`crate::tools::sources::list_sources`],
/// [`crate::tools::sources::get_source_chapter`],
/// [`crate::tools::sources::list_source_chapters`], and
/// [`crate::tools::sources::get_source_pdf_path`].
#[derive(Clone)]
pub struct MusicTheorySourceProvider {
    state: AppState,
}

impl std::fmt::Debug for MusicTheorySourceProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MusicTheorySourceProvider")
            .finish_non_exhaustive()
    }
}

impl MusicTheorySourceProvider {
    /// Create a new source provider with the given application state.
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl SourceProvider for MusicTheorySourceProvider {
    type SourceSummary = SourceSummary;

    async fn list_sources(&self) -> Result<Vec<Self::SourceSummary>> {
        let response = crate::tools::sources::list_sources(&self.state.config).await?;

        let summaries = response
            .sources
            .into_iter()
            .map(|s| SourceSummary {
                id: s.id,
                title: s.title,
                format: format!("{:?}", s.format).to_lowercase(),
                path: s.path,
                chapters: s.chapters,
                status: format!("{:?}", s.status),
            })
            .collect();

        Ok(summaries)
    }

    async fn get_chapter(
        &self,
        source_id: &str,
        chapter: &str,
        section: Option<&str>,
    ) -> Result<String> {
        crate::tools::sources::get_source_chapter(
            &self.state.config,
            source_id,
            chapter,
            section,
        )
        .await
    }

    async fn list_chapters(&self, source_id: &str) -> Result<Vec<FabrykChapterInfo>> {
        let response =
            crate::tools::sources::list_source_chapters(&self.state, source_id).await?;

        let chapters = response
            .chapters
            .into_iter()
            .map(|c| FabrykChapterInfo {
                id: c.id,
                title: c.title,
                number: c.section,
                available: true,
            })
            .collect();

        Ok(chapters)
    }

    async fn get_source_path(&self, source_id: &str) -> Result<Option<PathBuf>> {
        match crate::tools::sources::get_source_pdf_path(&self.state.config, source_id) {
            Ok(path) => {
                if path.exists() {
                    Ok(Some(path))
                } else {
                    Ok(None)
                }
            }
            Err(_) => Ok(None),
        }
    }

    async fn is_available(&self, source_id: &str) -> Result<bool> {
        let response =
            crate::tools::sources::check_source_availability(&self.state, source_id).await?;
        Ok(!matches!(
            response.status,
            crate::tools::sources::AvailabilityStatus::Unavailable
        ))
    }
}

// ============================================================================
// Guide Provider
// ============================================================================

/// Summary information for a topic guide.
///
/// This is the `GuideSummary` associated type for [`GuideProvider`].
/// It mirrors the fields from [`crate::tools::guides::GuideInfo`].
#[derive(Debug, Clone, Serialize)]
pub struct GuideSummary {
    /// Guide identifier (e.g., "intervals").
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Topic area the guide covers.
    pub topic: String,
    /// Filesystem path to the guide.
    pub path: String,
    /// Short description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Guide provider for music theory topic guides.
///
/// Wraps the existing guide infrastructure, delegating to
/// [`crate::tools::guides::list_guides`] and
/// [`crate::tools::guides::get_guide`].
#[derive(Debug)]
pub struct MusicTheoryGuideProvider {
    config: Config,
}

impl MusicTheoryGuideProvider {
    /// Create a new guide provider with the given configuration.
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl GuideProvider for MusicTheoryGuideProvider {
    type GuideSummary = GuideSummary;

    async fn list_guides(&self) -> Result<Vec<Self::GuideSummary>> {
        let response = crate::tools::guides::list_guides(&self.config).await?;

        let summaries = response
            .guides
            .into_iter()
            .map(|g| GuideSummary {
                id: g.id,
                title: g.title,
                topic: g.topic,
                path: g.path,
                description: g.description,
            })
            .collect();

        Ok(summaries)
    }

    async fn get_guide(&self, id: &str) -> Result<String> {
        crate::tools::guides::get_guide(&self.config, id).await
    }
}

// ============================================================================
// Question Search Provider
// ============================================================================

/// Question search provider for music theory concept cards.
///
/// Wraps the existing question search infrastructure, delegating to
/// [`crate::tools::questions::search_by_question`].
/// Converts the domain-specific `QuestionMatch` fields (`concept_id`,
/// `concept_title`) into fabryk's generic `QuestionMatch` fields
/// (`item_id`, `item_title`).
#[derive(Debug)]
pub struct MusicTheoryQuestionProvider {
    config: Config,
}

impl MusicTheoryQuestionProvider {
    /// Create a new question search provider with the given configuration.
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[async_trait]
impl QuestionSearchProvider for MusicTheoryQuestionProvider {
    async fn search_by_question(
        &self,
        question: &str,
        limit: usize,
    ) -> Result<QuestionSearchResponse> {
        let params = crate::tools::questions::SearchByQuestionParams {
            question: question.to_string(),
            limit,
        };
        let response = crate::tools::questions::search_by_question(&self.config, params).await?;

        Ok(QuestionSearchResponse {
            matches: response
                .matches
                .into_iter()
                .map(|m| QuestionMatch {
                    item_id: m.concept_id,
                    item_title: m.concept_title,
                    matched_question: m.matched_question,
                    category: m.category,
                    tier: m.tier,
                    similarity: m.similarity,
                })
                .collect(),
            total: response.total,
            query: response.query,
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concept_summary_serialization() {
        let summary = ConceptSummary {
            id: "major-triad".to_string(),
            title: "Major Triad".to_string(),
            category: "harmony".to_string(),
            source: Some("Open Music Theory".to_string()),
            preview: Some("A three-note chord".to_string()),
            subcategory: None,
            tier: None,
            extraction_confidence: None,
            aliases: vec![],
        };

        let json = serde_json::to_string(&summary).expect("should serialize");
        assert!(json.contains("major-triad"));
        assert!(json.contains("Major Triad"));
        assert!(json.contains("harmony"));
        assert!(json.contains("Open Music Theory"));
        assert!(json.contains("A three-note chord"));
    }

    #[test]
    fn test_concept_summary_serialization_no_optionals() {
        let summary = ConceptSummary {
            id: "test".to_string(),
            title: "Test".to_string(),
            category: "test".to_string(),
            source: None,
            preview: None,
            subcategory: None,
            tier: None,
            extraction_confidence: None,
            aliases: vec![],
        };

        let json = serde_json::to_string(&summary).expect("should serialize");
        assert!(json.contains("\"id\":\"test\""));
        assert!(json.contains("null"));
    }

    #[test]
    fn test_music_theory_content_provider_new() {
        let config = Config::load().unwrap();
        let provider = MusicTheoryContentProvider::new(config);
        assert_eq!(provider.content_type_name(), "concept");
        assert_eq!(provider.content_type_name_plural(), "concepts");
    }

    #[tokio::test]
    async fn test_list_items_empty_directory() {
        let mut config = Config::load().unwrap();
        config.paths.concept_cards = "/nonexistent/path/for/testing".to_string();

        let provider = MusicTheoryContentProvider::new(config);
        let items = provider.list_items(None, None).await.unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn test_list_categories_empty_directory() {
        let mut config = Config::load().unwrap();
        config.paths.concept_cards = "/nonexistent/path/for/testing".to_string();

        let provider = MusicTheoryContentProvider::new(config);
        let categories = provider.list_categories().await.unwrap();
        assert!(categories.is_empty());
    }

    #[tokio::test]
    async fn test_list_items_with_temp_content() {
        let temp = tempfile::TempDir::new().unwrap();
        let harmony_dir = temp.path().join("harmony");
        tokio::fs::create_dir_all(&harmony_dir).await.unwrap();

        tokio::fs::write(
            harmony_dir.join("triads.md"),
            "---\ntitle: Triads\ncategory: harmony\n---\n\n# Triads\n\nThree-note chords.",
        )
        .await
        .unwrap();

        let mut config = Config::load().unwrap();
        config.paths.concept_cards = temp.path().to_string_lossy().to_string();

        let provider = MusicTheoryContentProvider::new(config);

        // List all
        let items = provider.list_items(None, None).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "triads");
        assert_eq!(items[0].title, "Triads");
        assert_eq!(items[0].category, "harmony");

        // Filter by category (match)
        let items = provider.list_items(Some("harmony"), None).await.unwrap();
        assert_eq!(items.len(), 1);

        // Filter by category (no match)
        let items = provider.list_items(Some("rhythm"), None).await.unwrap();
        assert!(items.is_empty());

        // With limit
        let items = provider.list_items(None, Some(0)).await.unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn test_get_item_with_temp_content() {
        let temp = tempfile::TempDir::new().unwrap();
        let harmony_dir = temp.path().join("harmony");
        tokio::fs::create_dir_all(&harmony_dir).await.unwrap();

        let content =
            "---\ntitle: Triads\ncategory: harmony\n---\n\n# Triads\n\nThree-note chords.";
        tokio::fs::write(harmony_dir.join("triads.md"), content)
            .await
            .unwrap();

        let mut config = Config::load().unwrap();
        config.paths.concept_cards = temp.path().to_string_lossy().to_string();

        let provider = MusicTheoryContentProvider::new(config);
        let detail = provider.get_item("triads").await.unwrap();
        assert_eq!(detail, content);
    }

    #[tokio::test]
    async fn test_list_categories_with_temp_content() {
        let temp = tempfile::TempDir::new().unwrap();
        let harmony_dir = temp.path().join("harmony");
        let rhythm_dir = temp.path().join("rhythm");
        tokio::fs::create_dir_all(&harmony_dir).await.unwrap();
        tokio::fs::create_dir_all(&rhythm_dir).await.unwrap();

        tokio::fs::write(
            harmony_dir.join("triads.md"),
            "---\ncategory: harmony\n---\n# Triads",
        )
        .await
        .unwrap();
        tokio::fs::write(
            rhythm_dir.join("meter.md"),
            "---\ncategory: rhythm\n---\n# Meter",
        )
        .await
        .unwrap();

        let mut config = Config::load().unwrap();
        config.paths.concept_cards = temp.path().to_string_lossy().to_string();

        let provider = MusicTheoryContentProvider::new(config);
        let categories = provider.list_categories().await.unwrap();

        assert_eq!(categories.len(), 2);
        // Should be sorted alphabetically (from underlying list_categories)
        assert_eq!(categories[0].id, "harmony");
        assert_eq!(categories[0].name, "harmony");
        assert_eq!(categories[0].count, 1);
        assert_eq!(categories[1].id, "rhythm");
        assert_eq!(categories[1].name, "rhythm");
        assert_eq!(categories[1].count, 1);
        // description is always None since underlying doesn't provide it
        assert!(categories[0].description.is_none());
    }

    #[tokio::test]
    async fn test_count_delegates_to_list_items() {
        let mut config = Config::load().unwrap();
        config.paths.concept_cards = "/nonexistent/path".to_string();

        let provider = MusicTheoryContentProvider::new(config);
        let count = provider.count().await.unwrap();
        assert_eq!(count, 0);
    }

    // ========================================================================
    // MusicTheorySourceProvider tests
    // ========================================================================

    #[tokio::test]
    async fn test_source_provider_new() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let _provider = MusicTheorySourceProvider::new(state);
        // Construction succeeds — provider is ready for use.
    }

    #[test]
    fn test_source_summary_serialization() {
        let summary = SourceSummary {
            id: "open-music-theory".to_string(),
            title: "Open Music Theory".to_string(),
            format: "markdown".to_string(),
            path: "/some/path".to_string(),
            chapters: Some(12),
            status: "Converted".to_string(),
        };

        let json = serde_json::to_string(&summary).expect("should serialize");
        assert!(json.contains("open-music-theory"));
        assert!(json.contains("Open Music Theory"));
        assert!(json.contains("markdown"));
        assert!(json.contains("12"));
    }

    #[test]
    fn test_source_summary_serialization_no_chapters() {
        let summary = SourceSummary {
            id: "test".to_string(),
            title: "Test".to_string(),
            format: "pdf".to_string(),
            path: "/test".to_string(),
            chapters: None,
            status: "NotConverted".to_string(),
        };

        let json = serde_json::to_string(&summary).expect("should serialize");
        // chapters field should be skipped when None
        assert!(!json.contains("chapters"));
    }

    #[tokio::test]
    async fn test_source_provider_list_sources_empty() {
        let mut config = Config::load().unwrap();
        config.paths.sources_md = "/nonexistent/path/for/testing".to_string();
        // Clear configured source files so list_unconverted_sources returns nothing
        config.sources.oxford.files.clear();
        config.sources.general.files.clear();
        config.sources.papers.files.clear();
        let state = AppState::new(config).await.unwrap();

        let provider = MusicTheorySourceProvider::new(state);
        let sources = provider.list_sources().await.unwrap();
        assert!(sources.is_empty());
    }

    #[tokio::test]
    async fn test_source_provider_get_chapter_not_found() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();

        let provider = MusicTheorySourceProvider::new(state);
        let result = provider
            .get_chapter("nonexistent-source", "chapter-1", None)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_source_provider_get_source_path_unknown() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();

        let provider = MusicTheorySourceProvider::new(state);
        let result = provider.get_source_path("unknown-source").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_source_provider_is_available_nonexistent() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();

        let provider = MusicTheorySourceProvider::new(state);
        let available = provider.is_available("nonexistent-source").await.unwrap();
        assert!(!available);
    }

    #[tokio::test]
    async fn test_source_provider_list_chapters_with_temp_content() {
        let temp = tempfile::TempDir::new().unwrap();
        let source_dir = temp.path().join("my-source");
        tokio::fs::create_dir_all(&source_dir).await.unwrap();

        tokio::fs::write(
            source_dir.join("ch-01.md"),
            "---\ntitle: Introduction\nsection: pp. 1-10\n---\n\n# Introduction",
        )
        .await
        .unwrap();

        let mut config = Config::load().unwrap();
        config.paths.sources_md = temp.path().to_string_lossy().to_string();
        let state = AppState::new(config).await.unwrap();

        let provider = MusicTheorySourceProvider::new(state);
        let chapters = provider.list_chapters("my-source").await.unwrap();
        assert_eq!(chapters.len(), 1);
        assert_eq!(chapters[0].id, "ch-01");
        assert_eq!(chapters[0].title, "Introduction");
        assert!(chapters[0].available);
    }

    #[tokio::test]
    async fn test_source_provider_get_chapter_with_temp_content() {
        let temp = tempfile::TempDir::new().unwrap();
        let source_dir = temp.path().join("my-source");
        tokio::fs::create_dir_all(&source_dir).await.unwrap();

        let content = "---\ntitle: Introduction\n---\n\n# Introduction\n\nWelcome.";
        tokio::fs::write(source_dir.join("ch-01.md"), content)
            .await
            .unwrap();

        let mut config = Config::load().unwrap();
        config.paths.sources_md = temp.path().to_string_lossy().to_string();
        let state = AppState::new(config).await.unwrap();

        let provider = MusicTheorySourceProvider::new(state);
        let result = provider
            .get_chapter("my-source", "ch-01", None)
            .await
            .unwrap();
        assert_eq!(result, content);
    }

    // ========================================================================
    // MusicTheoryGuideProvider tests
    // ========================================================================

    #[test]
    fn test_guide_summary_serialization() {
        let summary = GuideSummary {
            id: "intervals".to_string(),
            title: "Intervals".to_string(),
            topic: "harmony".to_string(),
            path: "/some/path".to_string(),
            description: Some("A guide about intervals".to_string()),
        };

        let json = serde_json::to_string(&summary).expect("should serialize");
        assert!(json.contains("intervals"));
        assert!(json.contains("Intervals"));
        assert!(json.contains("harmony"));
        assert!(json.contains("A guide about intervals"));
    }

    #[test]
    fn test_guide_summary_serialization_no_description() {
        let summary = GuideSummary {
            id: "test".to_string(),
            title: "Test".to_string(),
            topic: "general".to_string(),
            path: "/test".to_string(),
            description: None,
        };

        let json = serde_json::to_string(&summary).expect("should serialize");
        // description field should be skipped when None
        assert!(!json.contains("description"));
    }

    #[test]
    fn test_guide_provider_new() {
        let config = Config::load().unwrap();
        let provider = MusicTheoryGuideProvider::new(config);
        assert_eq!(provider.guide_type_name(), "guide");
        assert_eq!(provider.guide_type_name_plural(), "guides");
    }

    #[tokio::test]
    async fn test_guide_provider_list_guides_empty() {
        let mut config = Config::load().unwrap();
        config.paths.guides = "/nonexistent/path/for/testing".to_string();

        let provider = MusicTheoryGuideProvider::new(config);
        let guides = provider.list_guides().await.unwrap();
        assert!(guides.is_empty());
    }

    #[tokio::test]
    async fn test_guide_provider_list_guides_with_temp_content() {
        let temp = tempfile::TempDir::new().unwrap();
        let topic_dir = temp.path().join("harmony");
        tokio::fs::create_dir_all(&topic_dir).await.unwrap();

        tokio::fs::write(
            topic_dir.join("intervals.md"),
            "---\ntitle: Intervals\n---\n\n# Intervals\n\nAn interval is the distance between two pitches.",
        )
        .await
        .unwrap();

        let mut config = Config::load().unwrap();
        config.paths.guides = temp.path().to_string_lossy().to_string();

        let provider = MusicTheoryGuideProvider::new(config);
        let guides = provider.list_guides().await.unwrap();
        assert_eq!(guides.len(), 1);
        assert_eq!(guides[0].id, "intervals");
        assert_eq!(guides[0].title, "Intervals");
        assert_eq!(guides[0].topic, "harmony");
    }

    #[tokio::test]
    async fn test_guide_provider_get_guide_with_temp_content() {
        let temp = tempfile::TempDir::new().unwrap();
        let topic_dir = temp.path().join("harmony");
        tokio::fs::create_dir_all(&topic_dir).await.unwrap();

        let content = "---\ntitle: Intervals\n---\n\n# Intervals\n\nAn interval is the distance between two pitches.";
        tokio::fs::write(topic_dir.join("intervals.md"), content)
            .await
            .unwrap();

        let mut config = Config::load().unwrap();
        config.paths.guides = temp.path().to_string_lossy().to_string();

        let provider = MusicTheoryGuideProvider::new(config);
        let result = provider.get_guide("intervals").await.unwrap();
        assert_eq!(result, content);
    }

    #[tokio::test]
    async fn test_guide_provider_get_guide_not_found() {
        let mut config = Config::load().unwrap();
        config.paths.guides = "/nonexistent/path/for/testing".to_string();

        let provider = MusicTheoryGuideProvider::new(config);
        let result = provider.get_guide("nonexistent").await;
        assert!(result.is_err());
    }

    // ========================================================================
    // MusicTheoryQuestionProvider tests
    // ========================================================================

    #[test]
    fn test_question_provider_new() {
        let config = Config::load().unwrap();
        let _provider = MusicTheoryQuestionProvider::new(config);
        // Construction succeeds.
    }

    #[tokio::test]
    async fn test_question_provider_search_empty_directory() {
        let temp = tempfile::TempDir::new().unwrap();
        let mut config = Config::load().unwrap();
        config.paths.concept_cards = temp.path().to_string_lossy().to_string();

        let provider = MusicTheoryQuestionProvider::new(config);
        let response = provider.search_by_question("anything", 10).await.unwrap();
        assert!(response.matches.is_empty());
        assert_eq!(response.total, 0);
    }

    #[tokio::test]
    async fn test_question_provider_search_with_temp_content() {
        let temp = tempfile::TempDir::new().unwrap();

        let card = r#"---
title: "Circle of Fifths"
slug: "circle-of-fifths"
category: "harmony"
tier: "intermediate"
answers_questions:
  - "What is the circle of fifths?"
---
# Circle of Fifths

Content about the circle of fifths."#;

        tokio::fs::write(temp.path().join("circle-of-fifths.md"), card)
            .await
            .unwrap();

        let mut config = Config::load().unwrap();
        config.paths.concept_cards = temp.path().to_string_lossy().to_string();

        let provider = MusicTheoryQuestionProvider::new(config);
        let response = provider
            .search_by_question("What is the circle of fifths?", 10)
            .await
            .unwrap();

        assert!(!response.matches.is_empty());
        // Field mapping: concept_id -> item_id, concept_title -> item_title
        assert_eq!(response.matches[0].item_id, "circle-of-fifths");
        assert_eq!(response.matches[0].item_title, "Circle of Fifths");
        assert_eq!(response.matches[0].category, "harmony");
        assert_eq!(response.matches[0].tier, Some("intermediate".to_string()));
        assert!(response.matches[0].similarity > 0.9);
    }

    #[tokio::test]
    async fn test_question_provider_respects_limit() {
        let temp = tempfile::TempDir::new().unwrap();

        let card = r#"---
title: "Test Card"
slug: "test-card"
category: "test"
answers_questions:
  - "What is this?"
  - "How does this work?"
  - "Why is this important?"
---
# Test"#;

        tokio::fs::write(temp.path().join("test-card.md"), card)
            .await
            .unwrap();

        let mut config = Config::load().unwrap();
        config.paths.concept_cards = temp.path().to_string_lossy().to_string();

        let provider = MusicTheoryQuestionProvider::new(config);
        let response = provider.search_by_question("what", 1).await.unwrap();

        assert!(response.matches.len() <= 1);
    }
}
