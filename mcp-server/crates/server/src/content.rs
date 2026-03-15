//! Content layer: `ContentItemProvider` implementation for music theory concepts.
//!
//! This module wraps the existing `tools::concepts` functionality behind the
//! `fabryk_mcp::content::ContentItemProvider` trait, enabling domain-agnostic
//! MCP tools to operate on music theory concept cards.
//!
//! The provider delegates to existing functions in `tools::concepts` rather
//! than reimplementing the logic. This allows incremental migration: the old
//! tool handlers continue to work while new trait-based handlers can be
//! introduced alongside them.

use async_trait::async_trait;
use fabryk::core::Result;
use fabryk_mcp::content::{CategoryInfo, ContentItemProvider};
use serde::Serialize;

use crate::config::Config;
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
}
