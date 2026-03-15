//! Simple search backend (linear scan) implementing fabryk's SearchBackend trait.
//!
//! This module provides `SimpleSearch`, a linear scan search backend that preserves
//! the original simple search functionality. It uses fabryk's `SearchDocument`
//! methods (`matches_query`, `relevance`, `extract_snippet`) for matching and
//! ranking, and returns results through fabryk's `SearchBackend` trait interface.

use async_trait::async_trait;

use fabryk::fts::{SearchBackend, SearchParams, SearchResult as FabrykSearchResult, SearchResults};

use crate::config::Config;
use crate::extractors::MusicTheoryDocumentExtractor;
use crate::util::files::{find_all_files, FindOptions};

/// Simple search backend using linear scan.
///
/// This backend scans all concept card files sequentially, parsing each with
/// `MusicTheoryDocumentExtractor` and using fabryk `SearchDocument` methods
/// for matching and relevance scoring.
///
/// It is suitable for small to medium collections (<500 documents) and
/// requires no index.
pub struct SimpleSearch {
    config: Config,
}

impl SimpleSearch {
    /// Create a new SimpleSearch backend.
    ///
    /// This is the fallback search backend for users who do not want to set up
    /// full-text search with Tantivy. It performs a linear scan of all concept cards.
    ///
    /// For better search quality with large collections, consider using the Tantivy
    /// backend by setting `backend = "tantivy"` in your configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration
    pub fn new(config: Config) -> Self {
        SimpleSearch { config }
    }
}

#[async_trait]
impl SearchBackend for SimpleSearch {
    async fn search(&self, params: SearchParams) -> fabryk::core::Result<SearchResults> {
        let concept_cards_path = self
            .config
            .paths
            .concept_cards_path()
            .map_err(|e| fabryk::core::Error::config(e.to_string()))?;

        if !crate::util::files::exists(&concept_cards_path).await {
            return Ok(SearchResults::empty(self.name()));
        }

        let extractor = MusicTheoryDocumentExtractor::new();
        let snippet_length = params.snippet_length.unwrap_or(200);
        let limit = params.limit.unwrap_or(10);

        let mut results = Vec::new();

        // Find all markdown files
        let files = find_all_files(&concept_cards_path, FindOptions::markdown())
            .await
            .map_err(|e| fabryk::core::Error::operation(e.to_string()))?;

        for file_info in files {
            let path = &file_info.path;

            // Read file content
            let content = match crate::util::files::read_file(path).await {
                Ok(c) => c,
                Err(_) => continue,
            };

            // Extract SearchDocument using the MusicTheoryDocumentExtractor
            let doc = match extractor.extract(path, &content) {
                Some(d) => d,
                None => continue,
            };

            // Check if document matches query (skip for wildcard queries)
            if !doc.matches_query(&params.query) {
                continue;
            }

            // Apply category filter
            if let Some(ref filter_category) = params.category {
                if !doc.matches_category(filter_category) {
                    continue;
                }
            }

            // Apply source filter
            if let Some(ref filter_source) = params.source {
                if !doc.matches_source(filter_source) {
                    continue;
                }
            }

            // Apply content_types filter
            if let Some(ref content_types) = params.content_types {
                let matches_any = content_types.iter().any(|ct| doc.matches_content_type(ct));
                if !matches_any {
                    continue;
                }
            }

            // Calculate relevance and extract snippet
            let relevance = doc.relevance(&params.query);
            let snippet = doc.extract_snippet(&params.query, snippet_length);

            results.push(FabrykSearchResult {
                id: doc.id.clone(),
                title: doc.title.clone(),
                description: doc.description.clone(),
                category: doc.category.clone(),
                source: doc.source.clone(),
                path: Some(doc.path.clone()),
                snippet,
                relevance,
                content_type: doc.content_type.clone(),
                section: doc.section.clone(),
                chapter: doc.chapter.clone(),
            });
        }

        // Sort by relevance (highest first)
        results.sort_by(|a, b| {
            b.relevance
                .partial_cmp(&a.relevance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Apply limit
        results.truncate(limit);

        let total = results.len();

        Ok(SearchResults {
            items: results,
            total,
            backend: self.name().to_string(),
        })
    }

    fn name(&self) -> &str {
        "simple"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{PathsConfig, SearchConfig, ServerConfig, SourcesConfig};

    fn test_config() -> Config {
        Config {
            server: ServerConfig {
                name: "test".to_string(),
                version: "0.1.0".to_string(),
            },
            paths: PathsConfig {
                base: ".".to_string(),
                sources_md: "sources-md".to_string(),
                concept_cards: "concept-cards".to_string(),
                concepts_unified: "concepts-unified".to_string(),
                guides: "guides".to_string(),
                skill_docs: ".".to_string(),
            },
            sources: SourcesConfig::default(),
            logging: twyg::OptsBuilder::new()
                .level(twyg::LogLevel::Info)
                .coloured(true)
                .output(twyg::Output::Stderr)
                .report_caller(false)
                .build()
                .unwrap(),
            search: SearchConfig {
                backend: "simple".to_string(),
                index_path: ".tantivy-index".to_string(),
                rebuild_on_startup: false,
                snippet_size: 200,
                fuzzy_search: false,
                fuzzy_distance: 2,
                query_mode: crate::config::QueryMode::Smart,
                minimum_match_percent: 0.6,
                enable_stopwords: true,
                custom_stopwords: vec![],
                stopword_allowlist: vec![],
                field_boost_title: 3.0,
                field_boost_description: 2.0,
                field_boost_content: 1.0,
            },
        }
    }

    #[test]
    fn test_simple_search_new() {
        let config = test_config();
        let backend = SimpleSearch::new(config);
        assert_eq!(backend.name(), "simple");
    }

    #[tokio::test]
    async fn test_simple_search_empty_directory() {
        let config = test_config();
        let backend = SimpleSearch::new(config);

        let params = SearchParams {
            query: "test".to_string(),
            limit: Some(10),
            ..Default::default()
        };

        let result = backend.search(params).await;
        assert!(result.is_ok());
        // Just verify search executes without error
    }

    #[tokio::test]
    async fn test_simple_search_is_ready() {
        let config = test_config();
        let backend = SimpleSearch::new(config);
        assert!(backend.is_ready());
    }

    #[tokio::test]
    async fn test_simple_search_with_matching_documents() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        // Create a concept card that matches "harmony"
        let card_content = r#"---
title: Harmony Basics
category: harmony
source: TestSource
description: An introduction to harmony
---

# Harmony Basics

Harmony is the combination of simultaneously sounded musical notes.
"#;
        fs::write(concept_cards_path.join("harmony-basics.md"), card_content)
            .expect("Failed to write test card");

        // Create a second card that does NOT match "harmony"
        let card2 = r#"---
title: Rhythm Patterns
category: rhythm
source: OtherSource
description: Rhythm fundamentals
---

# Rhythm Patterns

Rhythm is the pattern of sounds and silences in music.
"#;
        fs::write(concept_cards_path.join("rhythm-patterns.md"), card2)
            .expect("Failed to write test card");

        let mut config = test_config();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let backend = SimpleSearch::new(config);

        let params = SearchParams {
            query: "harmony".to_string(),
            limit: Some(10),
            ..Default::default()
        };

        let result = backend.search(params).await.expect("Search should succeed");
        assert_eq!(result.backend, "simple");
        assert!(
            result.total >= 1,
            "Should find at least one match for 'harmony'"
        );
        assert!(
            result.items.iter().any(|r| r.title == "Harmony Basics"),
            "Should find the harmony basics card"
        );
    }

    #[tokio::test]
    async fn test_simple_search_category_filter() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        // Two cards with different categories, both matching "music"
        let harmony_card = r#"---
title: Music Harmony
category: harmony
---

# Music Harmony

Music harmony concepts.
"#;
        let rhythm_card = r#"---
title: Music Rhythm
category: rhythm
---

# Music Rhythm

Music rhythm concepts.
"#;
        fs::write(concept_cards_path.join("music-harmony.md"), harmony_card)
            .expect("Failed to write card");
        fs::write(concept_cards_path.join("music-rhythm.md"), rhythm_card)
            .expect("Failed to write card");

        let mut config = test_config();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let backend = SimpleSearch::new(config);

        // Search with category filter
        let params = SearchParams {
            query: "music".to_string(),
            limit: Some(10),
            category: Some("harmony".to_string()),
            ..Default::default()
        };

        let result = backend.search(params).await.expect("Search should succeed");
        assert_eq!(
            result.total, 1,
            "Should find exactly one match with category filter"
        );
        assert_eq!(result.items[0].title, "Music Harmony");
    }

    #[tokio::test]
    async fn test_simple_search_source_filter() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card_a = r#"---
title: Note A
category: theory
source: BookA
---

# Note A

Notes about music theory from BookA.
"#;
        let card_b = r#"---
title: Note B
category: theory
source: BookB
---

# Note B

Notes about music theory from BookB.
"#;
        fs::write(concept_cards_path.join("note-a.md"), card_a).expect("Failed to write card");
        fs::write(concept_cards_path.join("note-b.md"), card_b).expect("Failed to write card");

        let mut config = test_config();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let backend = SimpleSearch::new(config);

        let params = SearchParams {
            query: "music theory".to_string(),
            limit: Some(10),
            source: Some("BookA".to_string()),
            ..Default::default()
        };

        let result = backend.search(params).await.expect("Search should succeed");
        assert_eq!(
            result.total, 1,
            "Should find exactly one match with source filter"
        );
        assert_eq!(result.items[0].title, "Note A");
    }

    #[tokio::test]
    async fn test_simple_search_content_types_filter() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card = r#"---
title: Chord Theory
category: chords
---

# Chord Theory

Understanding chord progressions.
"#;
        fs::write(concept_cards_path.join("chord-theory.md"), card).expect("Failed to write card");

        let mut config = test_config();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let backend = SimpleSearch::new(config);

        // Filter for a content type that won't match concept cards
        let params = SearchParams {
            query: "chord".to_string(),
            limit: Some(10),
            content_types: Some(vec!["nonexistent_type".to_string()]),
            ..Default::default()
        };

        let result = backend.search(params).await.expect("Search should succeed");
        assert_eq!(
            result.total, 0,
            "Should find no matches with mismatched content type filter"
        );
    }

    #[tokio::test]
    async fn test_simple_search_content_types_filter_matching() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card = r#"---
title: Chord Theory
category: chords
---

# Chord Theory

Understanding chord progressions.
"#;
        fs::write(concept_cards_path.join("chord-theory.md"), card).expect("Failed to write card");

        let mut config = test_config();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let backend = SimpleSearch::new(config);

        // Filter for "concept_card" content type (detected from concept-cards path)
        let params = SearchParams {
            query: "chord".to_string(),
            limit: Some(10),
            content_types: Some(vec!["concept_card".to_string()]),
            ..Default::default()
        };

        let result = backend.search(params).await.expect("Search should succeed");
        assert!(
            result.total >= 1,
            "Should find match with matching content type"
        );
    }

    #[tokio::test]
    async fn test_simple_search_limit_applied() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        // Create 5 cards all matching "music"
        for i in 0..5 {
            let card = format!(
                r#"---
title: Music Card {}
category: theory
---

# Music Card {}

Music theory content number {}.
"#,
                i, i, i
            );
            fs::write(concept_cards_path.join(format!("card-{}.md", i)), card)
                .expect("Failed to write card");
        }

        let mut config = test_config();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let backend = SimpleSearch::new(config);

        let params = SearchParams {
            query: "music".to_string(),
            limit: Some(2),
            ..Default::default()
        };

        let result = backend.search(params).await.expect("Search should succeed");
        assert!(
            result.total <= 2,
            "Should respect limit of 2, got {}",
            result.total
        );
    }

    #[tokio::test]
    async fn test_simple_search_default_snippet_length() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card = r#"---
title: Snippet Test
category: theory
---

# Snippet Test

Some content about melody and harmony.
"#;
        fs::write(concept_cards_path.join("snippet-test.md"), card).expect("Failed to write card");

        let mut config = test_config();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let backend = SimpleSearch::new(config);

        // No snippet_length or limit set -- test defaults
        let params = SearchParams {
            query: "melody".to_string(),
            snippet_length: None,
            limit: None,
            ..Default::default()
        };

        let result = backend.search(params).await.expect("Search should succeed");
        assert!(result.total >= 1);
    }

    #[tokio::test]
    async fn test_simple_search_no_match() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card = r#"---
title: Harmony Basics
category: harmony
---

# Harmony Basics

Harmony concepts.
"#;
        fs::write(concept_cards_path.join("harmony.md"), card).expect("Failed to write card");

        let mut config = test_config();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let backend = SimpleSearch::new(config);

        let params = SearchParams {
            query: "xyznonexistent".to_string(),
            limit: Some(10),
            ..Default::default()
        };

        let result = backend.search(params).await.expect("Search should succeed");
        assert_eq!(result.total, 0, "Should find no matches");
    }

    #[tokio::test]
    async fn test_simple_search_unreadable_file_skipped() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        // Create a valid card
        let card = r#"---
title: Valid Card
category: theory
---

# Valid Card

Valid content about music.
"#;
        fs::write(concept_cards_path.join("valid.md"), card).expect("Failed to write card");

        // Create a subdirectory with .md extension to cause a read error
        // (reading a directory as a file should fail)
        let fake_file = concept_cards_path.join("subdir");
        fs::create_dir_all(&fake_file).expect("Failed to create subdir");

        let mut config = test_config();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let backend = SimpleSearch::new(config);

        let params = SearchParams {
            query: "music".to_string(),
            limit: Some(10),
            ..Default::default()
        };

        // Should succeed, skipping any unreadable files
        let result = backend.search(params).await.expect("Search should succeed");
        assert!(result.total >= 1, "Should still find the valid card");
    }

    #[tokio::test]
    async fn test_simple_search_invalid_frontmatter_skipped() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        // Create a file with no frontmatter delimiters at all (will parse differently)
        let bad_card = "This has no frontmatter at all, just plain text about music.";
        fs::write(concept_cards_path.join("bad-card.md"), bad_card).expect("Failed to write card");

        // Create a valid card
        let good_card = r#"---
title: Good Card
category: theory
---

# Good Card

Good music content.
"#;
        fs::write(concept_cards_path.join("good-card.md"), good_card)
            .expect("Failed to write card");

        let mut config = test_config();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let backend = SimpleSearch::new(config);

        let params = SearchParams {
            query: "music".to_string(),
            limit: Some(10),
            ..Default::default()
        };

        // Should succeed even with a problematic file
        let result = backend.search(params).await.expect("Search should succeed");
        assert!(result.total >= 1);
    }

    #[tokio::test]
    async fn test_simple_search_results_sorted_by_relevance() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        // Card with "chord" in title and body (higher relevance)
        let high_relevance = r#"---
title: Chord Progressions
category: chords
description: All about chord progressions
---

# Chord Progressions

Chord progressions are sequences of chords. A chord is built from intervals.
Chord voicing and chord inversions are important chord concepts.
"#;

        // Card with "chord" only once (lower relevance)
        let low_relevance = r#"---
title: Scale Theory
category: scales
description: Scale fundamentals
---

# Scale Theory

Scales form the basis of melody. A chord can be built from a scale.
"#;

        fs::write(
            concept_cards_path.join("chord-progressions.md"),
            high_relevance,
        )
        .expect("Failed to write card");
        fs::write(concept_cards_path.join("scale-theory.md"), low_relevance)
            .expect("Failed to write card");

        let mut config = test_config();
        config.paths.concept_cards = concept_cards_path.to_string_lossy().to_string();

        let backend = SimpleSearch::new(config);

        let params = SearchParams {
            query: "chord".to_string(),
            limit: Some(10),
            ..Default::default()
        };

        let result = backend.search(params).await.expect("Search should succeed");
        assert!(result.total >= 2, "Should find both cards");
        // Results should be sorted by relevance (highest first)
        if result.items.len() >= 2 {
            assert!(
                result.items[0].relevance >= result.items[1].relevance,
                "Results should be sorted by relevance descending"
            );
        }
    }
}
