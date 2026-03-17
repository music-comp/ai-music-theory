//! Integration tests for v0.3.0 multi-content-type search.
//!
//! Tests the full search pipeline with all 4 content types:
//! - concept_cards
//! - sources_md (source chapters)
//! - concepts_unified
//! - guides

#![cfg(feature = "fts")]

use std::sync::Arc;

use music_theory_mcp::config::{Config, PathsConfig, SearchConfig, ServerConfig, SourcesConfig};
use music_theory_mcp::metadata::{extract_metadata, ContentType};
use music_theory_mcp::search::build_index;
use music_theory_mcp::state::{initialize_fts, AppState};
use music_theory_mcp::tools::search::{search_concepts, SearchConceptsParams};
use music_theory_mcp::tools::sources::{
    check_source_availability, list_source_chapters, AvailabilityStatus,
};

use tempfile::TempDir;
use tokio::fs;

/// Create AppState with FTS initialized and ready.
/// Builds index, creates state, starts async FTS loading, and waits for ready.
async fn create_state_with_fts(config: Config) -> AppState {
    build_index(&config).await.expect("Index build failed");
    let state = AppState::new(config).await.expect("State creation failed");
    let state_arc = Arc::new(state.clone());
    initialize_fts(&state_arc).await.expect("FTS init failed");
    // Poll for FTS readiness
    for _ in 0..100 {
        if state.is_fts_ready() {
            return state;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    panic!("FTS did not become ready within 5 seconds");
}

/// Create a test config with temp directories.
async fn create_test_config(temp_dir: &TempDir) -> Config {
    let base = temp_dir.path();
    let index_path = base.join(".tantivy-index").to_string_lossy().to_string();

    Config {
        server: ServerConfig {
            name: "test".to_string(),
            version: "0.3.0".to_string(),
        },
        paths: PathsConfig {
            base: base.to_string_lossy().to_string(),
            sources_md: base.join("sources-md").to_string_lossy().to_string(),
            concept_cards: base.join("concept-cards").to_string_lossy().to_string(),
            concepts_unified: base.join("concepts-unified").to_string_lossy().to_string(),
            guides: base.join("guides").to_string_lossy().to_string(),
            skill_docs: base.to_string_lossy().to_string(),
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
            backend: "tantivy".to_string(),
            index_path,
            rebuild_on_startup: false,
            snippet_size: 200,
            fuzzy_search: false,
            fuzzy_distance: 2,
            query_mode: music_theory_mcp::config::QueryMode::Smart,
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

/// Create comprehensive test fixtures for all 4 content types.
async fn create_test_fixtures(temp_dir: &TempDir) {
    let base = temp_dir.path();

    // 1. Concept Cards
    let concept_cards = base.join("concept-cards/harmony");
    fs::create_dir_all(&concept_cards).await.unwrap();

    fs::write(
        concept_cards.join("cadence.md"),
        r#"---
title: Cadence
category: harmony
source: Test Music Theory
description: A melodic or harmonic progression that creates a sense of resolution
tags: [harmony, phrase, resolution]
---

# Cadence

A **cadence** is a melodic or harmonic progression that creates a sense of resolution or rest in music. Cadences are typically found at the ends of phrases.

## Types of Cadences

- **Authentic Cadence**: V-I progression
- **Plagal Cadence**: IV-I progression
- **Deceptive Cadence**: V-vi progression

Cadences are fundamental to understanding tonal harmony."#,
    )
    .await
    .unwrap();

    fs::write(
        concept_cards.join("triad.md"),
        r#"---
title: Triad
category: harmony
source: Test Music Theory
description: A three-note chord built from thirds
tags: [harmony, chord, tertian]
---

# Triad

A **triad** is a three-note chord built by stacking thirds. The most common triads are major and minor.

## Types of Triads

- **Major Triad**: Major third + minor third
- **Minor Triad**: Minor third + major third
- **Diminished Triad**: Minor third + minor third
- **Augmented Triad**: Major third + major third

Triads form the basis of Western tonal harmony."#,
    )
    .await
    .unwrap();

    // 2. Source Chapters
    let sources_md = base.join("sources-md/test-source");
    fs::create_dir_all(&sources_md).await.unwrap();

    fs::write(
        sources_md.join("chapter-1-introduction.md"),
        r#"---
source: Test Source
chapter: Chapter 1
section: pp. 1-15
category: fundamentals
title: Introduction to Music Theory
---

# Chapter 1: Introduction to Music Theory

This chapter introduces the fundamental concepts of music theory, including pitch, rhythm, and harmony.

## Section 1.1: Pitch and Intervals

Musical pitch is determined by the frequency of vibration. The relationship between two pitches is called an **interval**.

## Section 1.2: Scales and Keys

Scales are collections of pitches organized in ascending or descending order. The major scale is fundamental to Western music.

Cadences play an important role in establishing tonality."#,
    )
    .await
    .unwrap();

    fs::write(
        sources_md.join("chapter-2-harmony.md"),
        r#"---
source: Test Source
chapter: Chapter 2
section: pp. 16-30
category: harmony
title: Harmonic Fundamentals
---

# Chapter 2: Harmonic Fundamentals

This chapter explores harmonic theory, including triads, seventh chords, and voice leading.

## Section 2.1: Triads

The triad is the most basic tertian chord structure. Understanding triads is essential for harmonic analysis.

## Section 2.2: Harmonic Progressions

Common progressions include:
- I-IV-V-I
- I-vi-IV-V (50s progression)

Cadences provide closure to harmonic progressions."#,
    )
    .await
    .unwrap();

    // 3. Unified Concepts
    let concepts_unified = base.join("concepts-unified");
    fs::create_dir_all(&concepts_unified).await.unwrap();

    fs::write(
        concepts_unified.join("harmonic-function.md"),
        r#"---
title: Harmonic Function
category: harmony
sources: ["Test Source Book", "Test Music Theory"]
description: The role a chord plays in tonal harmony
tags: [harmony, function, tonic, dominant]
---

# Harmonic Function

**Harmonic function** describes the role a chord plays within a tonal context. The three primary functions are:

## Primary Functions

1. **Tonic (I)**: Provides stability and resolution
2. **Dominant (V)**: Creates tension and demands resolution
3. **Subdominant (IV)**: Provides contrast and preparation

Cadences articulate harmonic function through their progression patterns. The authentic cadence (V-I) is the strongest expression of dominant-to-tonic function.

## Sources

This synthesis draws from:
- Test Source Book (Chapters 1-2)
- Test Music Theory (cadence and triad concepts)"#,
    )
    .await
    .unwrap();

    // 4. Guides
    let guides = base.join("guides/fundamentals");
    fs::create_dir_all(&guides).await.unwrap();

    fs::write(
        guides.join("harmony-basics.md"),
        r#"---
title: Harmony Basics Guide
category: harmony
level: beginner
prerequisites: []
section: 1.0
description: Introduction to harmonic concepts for beginners
---

# Harmony Basics Guide

This guide introduces fundamental harmonic concepts for students beginning their study of music theory.

## Core Concepts

### 1. Triads

Start by understanding the triad - a three-note chord built from stacked thirds. The four types of triads are:
- Major
- Minor
- Diminished
- Augmented

### 2. Harmonic Progressions

Learn common chord progressions like I-IV-V-I.

### 3. Cadences

Understand how cadences provide phrase endings:
- Authentic cadence (V-I)
- Plagal cadence (IV-I)
- Deceptive cadence (V-vi)

## Learning Path

1. Master triad construction
2. Practice identifying harmonic progressions
3. Analyze cadences in musical examples

Cadences and triads form the foundation of tonal harmony."#,
    )
    .await
    .unwrap();
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_build_index_all_content_types() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_fixtures(&temp_dir).await;

    let config = create_test_config(&temp_dir).await;

    // Build index
    let stats = build_index(&config).await.expect("Index build failed");

    // Verify all content types were indexed
    assert_eq!(stats.concept_cards, 2, "Should index 2 concept cards");
    assert_eq!(stats.source_chapters, 2, "Should index 2 source chapters");
    assert_eq!(stats.unified_concepts, 1, "Should index 1 unified concept");
    assert_eq!(stats.guides, 1, "Should index 1 guide");
    assert_eq!(stats.indexed, 6, "Should index 6 total documents");
    assert_eq!(stats.errors, 0, "Should have no errors");
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_search_without_filter_returns_all_types() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_fixtures(&temp_dir).await;

    let config = create_test_config(&temp_dir).await;
    let state = create_state_with_fts(config).await;

    // Search for "cadence" - appears in all 4 content types
    let params = SearchConceptsParams {
        query: "cadence".to_string(),
        limit: 10,
        query_mode: None,
        category: None,
        source: None,
        content_types: None, // No filter - should return all types
        tier: None,
        subcategory: None,
        min_confidence: None,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("Search failed");

    // Should find results from multiple content types
    assert!(response.total >= 3, "Should find at least 3 results");

    // Verify we got different content types
    let mut content_types: Vec<String> = response
        .results
        .iter()
        .map(|r| r.content_type.clone())
        .collect();
    content_types.sort();
    content_types.dedup();

    assert!(
        content_types.len() >= 2,
        "Should have results from multiple content types"
    );
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_search_filter_by_content_type() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_fixtures(&temp_dir).await;

    let config = create_test_config(&temp_dir).await;
    let state = create_state_with_fts(config).await;

    // Search only concept cards
    let params = SearchConceptsParams {
        query: "triad".to_string(),
        limit: 10,
        query_mode: None,
        category: None,
        source: None,
        content_types: Some(vec!["concept_card".to_string()]),
        tier: None,
        subcategory: None,
        min_confidence: None,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("Search failed");

    // All results should be concept_card type
    assert!(response.total >= 1, "Should find concept card results");
    for result in &response.results {
        assert_eq!(
            result.content_type, "concept_card",
            "All results should be concept cards"
        );
    }
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_search_filter_by_multiple_content_types() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_fixtures(&temp_dir).await;

    let config = create_test_config(&temp_dir).await;
    let state = create_state_with_fts(config).await;

    // Search concept cards and guides
    let params = SearchConceptsParams {
        query: "harmony".to_string(),
        limit: 10,
        query_mode: None,
        category: None,
        source: None,
        content_types: Some(vec!["concept_card".to_string(), "guide".to_string()]),
        tier: None,
        subcategory: None,
        min_confidence: None,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("Search failed");

    // Should only get concept_card and guide results
    for result in &response.results {
        assert!(
            result.content_type == "concept_card" || result.content_type == "guide",
            "Results should only be concept_card or guide, got: {}",
            result.content_type
        );
    }
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_search_with_category_and_content_type_filters() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_fixtures(&temp_dir).await;

    let config = create_test_config(&temp_dir).await;
    let state = create_state_with_fts(config).await;

    // Search harmony category + source chapters only
    let params = SearchConceptsParams {
        query: "progression".to_string(),
        limit: 10,
        query_mode: None,
        category: Some("harmony".to_string()),
        source: None,
        content_types: Some(vec!["source_chapter".to_string()]),
        tier: None,
        subcategory: None,
        min_confidence: None,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("Search failed");

    // All results should be source_chapter + harmony category
    for result in &response.results {
        assert_eq!(
            result.content_type, "source_chapter",
            "Should only return source chapters"
        );
        assert_eq!(
            result.category, "harmony",
            "Should only return harmony category"
        );
    }
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_build_index_with_missing_directories() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    // Only create concept-cards, not other directories
    let concept_cards = temp_dir.path().join("concept-cards/harmony");
    fs::create_dir_all(&concept_cards).await.unwrap();

    fs::write(
        concept_cards.join("test.md"),
        r#"---
title: Test
category: harmony
---
# Test"#,
    )
    .await
    .unwrap();

    let config = create_test_config(&temp_dir).await;

    // Build index - should gracefully handle missing directories
    let stats = build_index(&config)
        .await
        .expect("Index build should succeed");

    assert_eq!(stats.concept_cards, 1, "Should index 1 concept card");
    assert_eq!(stats.source_chapters, 0, "Should have 0 source chapters");
    assert_eq!(stats.unified_concepts, 0, "Should have 0 unified concepts");
    assert_eq!(stats.guides, 0, "Should have 0 guides");
    assert_eq!(stats.indexed, 1, "Should index 1 document");
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_check_source_availability_indexed() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_fixtures(&temp_dir).await;

    let config = create_test_config(&temp_dir).await;
    let state = create_state_with_fts(config).await;

    // Check availability of indexed source
    let response = check_source_availability(&state, "test-source")
        .await
        .expect("Availability check failed");

    eprintln!("Response: {:?}", response);
    eprintln!("Status: {:?}", response.status);
    eprintln!("Message: {}", response.message);

    assert!(matches!(response.status, AvailabilityStatus::Indexed));
    assert!(response.searchable);
    assert!(response.text_extracted);
    assert_eq!(response.chapters, Some(2));
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_check_source_availability_converted_not_indexed() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_fixtures(&temp_dir).await;

    let config = create_test_config(&temp_dir).await;
    // Don't build index

    let state = AppState::new(config).await.expect("State creation failed");

    // Check availability - should show converted but not indexed
    let response = check_source_availability(&state, "test-source")
        .await
        .expect("Availability check failed");

    assert!(matches!(response.status, AvailabilityStatus::Converted));
    assert!(!response.searchable);
    assert!(response.text_extracted);
    assert_eq!(response.chapters, Some(2));
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_check_source_availability_unavailable() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    let config = create_test_config(&temp_dir).await;
    let state = AppState::new(config).await.expect("State creation failed");

    // Check availability of non-existent source
    let response = check_source_availability(&state, "nonexistent-source")
        .await
        .expect("Availability check failed");

    assert!(matches!(response.status, AvailabilityStatus::Unavailable));
    assert!(!response.searchable);
    assert!(!response.text_extracted);
    assert!(!response.file_exists);
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_list_source_chapters_from_index() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_fixtures(&temp_dir).await;

    let config = create_test_config(&temp_dir).await;
    let state = create_state_with_fts(config).await;

    // List chapters from indexed source
    let response = list_source_chapters(&state, "test-source")
        .await
        .expect("List chapters failed");

    assert_eq!(response.total, 2, "Should have 2 chapters");
    assert_eq!(response.chapters.len(), 2, "Should list 2 chapters");

    // Verify chapter metadata
    let has_intro = response
        .chapters
        .iter()
        .any(|c| c.title.contains("Introduction"));
    let has_harmony = response
        .chapters
        .iter()
        .any(|c| c.title.contains("Harmonic"));

    assert!(has_intro, "Should have Introduction chapter");
    assert!(has_harmony, "Should have Harmonic Fundamentals chapter");
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_cross_content_type_relevance_ranking() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_fixtures(&temp_dir).await;

    let config = create_test_config(&temp_dir).await;
    let state = create_state_with_fts(config).await;

    // Search for "cadence" - should rank by relevance across content types
    let params = SearchConceptsParams {
        query: "cadence".to_string(),
        limit: 10,
        query_mode: None,
        category: None,
        source: None,
        content_types: None,
        tier: None,
        subcategory: None,
        min_confidence: None,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("Search failed");

    // Verify results are sorted by relevance (descending)
    for i in 1..response.results.len() {
        assert!(
            response.results[i - 1].relevance >= response.results[i].relevance,
            "Results should be sorted by relevance"
        );
    }

    // The concept card about cadence should likely rank highest (title match)
    let top_result = &response.results[0];
    assert!(
        top_result.relevance > 5.0,
        "Top result should have high relevance"
    );
}

#[tokio::test]
async fn test_metadata_extraction_all_content_types() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_fixtures(&temp_dir).await;

    let base = temp_dir.path();

    // Test concept card extraction
    let concept_path = base.join("concept-cards/harmony/cadence.md");
    let concept_meta = extract_metadata(
        &base.join("concept-cards"),
        &concept_path,
        ContentType::ConceptCard,
    )
    .await
    .expect("Concept card extraction failed");

    assert_eq!(concept_meta.title, "Cadence");
    assert_eq!(concept_meta.category, "harmony");
    assert!(concept_meta.tags.contains(&"resolution".to_string()));

    // Test source chapter extraction
    let source_path = base.join("sources-md/test-source/chapter-1-introduction.md");
    let source_meta = extract_metadata(
        &base.join("sources-md/test-source"),
        &source_path,
        ContentType::SourceChapter,
    )
    .await
    .expect("Source chapter extraction failed");

    assert_eq!(source_meta.title, "Introduction to Music Theory");
    assert_eq!(source_meta.section, Some("pp. 1-15".to_string()));
    assert_eq!(source_meta.chapter, Some("Chapter 1".to_string()));

    // Test unified concept extraction
    let unified_path = base.join("concepts-unified/harmonic-function.md");
    let unified_meta = extract_metadata(
        &base.join("concepts-unified"),
        &unified_path,
        ContentType::UnifiedConcept,
    )
    .await
    .expect("Unified concept extraction failed");

    assert_eq!(unified_meta.title, "Harmonic Function");
    assert_eq!(unified_meta.category, "harmony");

    // Test guide extraction
    let guide_path = base.join("guides/fundamentals/harmony-basics.md");
    let guide_meta = extract_metadata(&base.join("guides"), &guide_path, ContentType::Guide)
        .await
        .expect("Guide extraction failed");

    assert_eq!(guide_meta.title, "Harmony Basics Guide");
    assert_eq!(guide_meta.section, Some("1.0".to_string()));
}

#[cfg(feature = "fts")]
#[tokio::test]
async fn test_search_document_creation_all_types() {
    use music_theory_mcp::extractors::MusicTheoryDocumentExtractor;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    create_test_fixtures(&temp_dir).await;

    let base = temp_dir.path();
    let extractor = MusicTheoryDocumentExtractor::new();

    // Test concept card document
    let concept_path = base.join("concept-cards/harmony/cadence.md");
    let concept_content = tokio::fs::read_to_string(&concept_path).await.unwrap();
    let concept_doc = extractor
        .extract(&concept_path, &concept_content)
        .expect("Document extraction failed");

    assert_eq!(concept_doc.content_type, Some("concept_card".to_string()));
    assert_eq!(concept_doc.title, "Cadence");
    assert!(concept_doc.section.is_none());

    // Test source chapter document
    let source_path = base.join("sources-md/test-source/chapter-1-introduction.md");
    let source_content = tokio::fs::read_to_string(&source_path).await.unwrap();
    let source_doc = extractor
        .extract(&source_path, &source_content)
        .expect("Document extraction failed");

    // MusicTheoryDocumentExtractor uses frontmatter category as content_type
    assert_eq!(source_doc.title, "Introduction to Music Theory");
    assert_eq!(source_doc.section, Some("pp. 1-15".to_string()));
}
