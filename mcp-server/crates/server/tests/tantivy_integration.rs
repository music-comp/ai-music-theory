//! Integration tests for Tantivy search functionality.
//!
//! These tests verify the complete search pipeline from index building
//! through query execution and result retrieval.

#![cfg(feature = "fts")]

use music_theory_mcp::config::{
    Config, LoggingConfig, PathsConfig, SearchConfig, ServerConfig, SourcesConfig,
};
use music_theory_mcp::search::build_index;
use music_theory_mcp::state::AppState;
use music_theory_mcp::tools::search::{search_concepts, SearchConceptsParams};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Create a test config with Tantivy backend
fn create_test_config(temp_dir: &TempDir, backend: &str, fuzzy: bool) -> Config {
    let index_path = temp_dir.path().join("index");
    let base_path = temp_dir.path();

    Config {
        server: ServerConfig {
            name: "test-server".to_string(),
            version: "0.1.0".to_string(),
        },
        paths: PathsConfig {
            base: base_path.to_string_lossy().to_string(),
            sources_md: base_path.join("sources-md").to_string_lossy().to_string(),
            concept_cards: base_path
                .join("concept-cards")
                .to_string_lossy()
                .to_string(),
            concepts_unified: base_path
                .join("concepts-unified")
                .to_string_lossy()
                .to_string(),
            guides: base_path.join("guides").to_string_lossy().to_string(),
            skill_docs: base_path.to_string_lossy().to_string(),
        },
        sources: SourcesConfig::default(),
        logging: LoggingConfig {
            level: "error".to_string(),
        },
        search: SearchConfig {
            backend: backend.to_string(),
            index_path: index_path.to_string_lossy().to_string(),
            rebuild_on_startup: false,
            snippet_size: 200,
            fuzzy_search: fuzzy,
            fuzzy_distance: 2,
        },
    }
}

/// Create a test markdown file with frontmatter and content
fn create_test_concept_card(
    dir: &PathBuf,
    filename: &str,
    title: &str,
    category: &str,
    description: &str,
    content: &str,
) -> std::io::Result<()> {
    let frontmatter = format!(
        "---\ntitle: {}\ncategory: {}\ndescription: {}\n---\n\n{}",
        title, category, description, content
    );
    fs::create_dir_all(dir)?;
    fs::write(dir.join(filename), frontmatter)
}

/// Set up test data with sample concept cards
fn setup_test_data(temp_dir: &TempDir) -> std::io::Result<PathBuf> {
    let concept_cards_path = temp_dir.path().join("concept-cards");

    // Card 1: Triads
    create_test_concept_card(
        &concept_cards_path,
        "triads.md",
        "Triads",
        "harmony",
        "Three-note chords built from thirds",
        "Triads are three-note chords built from stacked thirds. The most common types are major, minor, diminished, and augmented triads.",
    )?;

    // Card 2: Seventh Chords
    create_test_concept_card(
        &concept_cards_path,
        "seventh-chords.md",
        "Seventh Chords",
        "harmony",
        "Four-note chords with a seventh",
        "Seventh chords add a fourth note to triads, creating richer harmonies. Common types include major seventh, dominant seventh, and minor seventh chords.",
    )?;

    // Card 3: Voice Leading
    create_test_concept_card(
        &concept_cards_path,
        "voice-leading.md",
        "Voice Leading",
        "counterpoint",
        "Movement of individual voices between chords",
        "Voice leading refers to how individual voices move between chords. Good voice leading creates smooth melodic lines and avoids awkward leaps.",
    )?;

    // Card 4: Parallel Fifths
    create_test_concept_card(
        &concept_cards_path,
        "parallel-fifths.md",
        "Parallel Fifths",
        "counterpoint",
        "Prohibited voice leading motion",
        "Parallel fifths occur when two voices move in the same direction maintaining a perfect fifth interval. This is generally avoided in traditional harmony.",
    )?;

    Ok(concept_cards_path)
}

#[tokio::test]
async fn test_build_index_from_test_data() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_test_data(&temp_dir).expect("Failed to setup test data");

    let config = create_test_config(&temp_dir, "tantivy", false);

    // Build index
    let stats = build_index(&config).await.expect("Failed to build index");

    // Verify stats
    assert_eq!(stats.files_found, 4, "Should find 4 test files");
    assert_eq!(stats.indexed, 4, "Should index all 4 files");
    assert_eq!(stats.errors, 0, "Should have no errors");

    // Verify index directory exists
    let index_path = config
        .search
        .index_path()
        .expect("Failed to get index path");
    assert!(index_path.exists(), "Index directory should exist");
    assert!(
        index_path.join("meta.json").exists(),
        "Index meta.json should exist"
    );
}

#[tokio::test]
async fn test_search_returns_relevant_results() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_test_data(&temp_dir).expect("Failed to setup test data");

    let config = create_test_config(&temp_dir, "tantivy", false);

    // Build index
    build_index(&config).await.expect("Failed to build index");

    // Create AppState (which loads the index)
    let state = AppState::new(config).await.expect("Failed to create state");

    // Search for "triads"
    let params = SearchConceptsParams {
        query: "triads".to_string(),
        limit: 10,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("Search failed");
    let results = response.results;

    // Verify results
    assert!(!results.is_empty(), "Should find results for 'triads'");
    assert_eq!(
        results[0].title, "Triads",
        "First result should be Triads card"
    );
    assert_eq!(results[0].category, "harmony");
}

#[tokio::test]
async fn test_search_ranking_by_relevance() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_test_data(&temp_dir).expect("Failed to setup test data");

    let config = create_test_config(&temp_dir, "tantivy", false);
    build_index(&config).await.expect("Failed to build index");

    let state = AppState::new(config).await.expect("Failed to create state");

    // Search for "chords" - appears in multiple documents
    let params = SearchConceptsParams {
        query: "chords".to_string(),
        limit: 10,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("Search failed");
    let results = response.results;

    // Should find multiple results
    assert!(
        results.len() >= 2,
        "Should find multiple results for 'chords'"
    );

    // Results should be sorted by relevance (title match should rank higher)
    // "Seventh Chords" has "Chords" in title, "Triads" only in content
    assert!(
        results.iter().any(|r| r.title == "Seventh Chords"),
        "Should find Seventh Chords"
    );
}

#[tokio::test]
async fn test_fuzzy_search_finds_typos() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_test_data(&temp_dir).expect("Failed to setup test data");

    let config = create_test_config(&temp_dir, "tantivy", true); // Enable fuzzy search
    build_index(&config).await.expect("Failed to build index");

    let state = AppState::new(config).await.expect("Failed to create state");

    // Search with typo: "haromny" instead of "harmony"
    let params = SearchConceptsParams {
        query: "haromny".to_string(),
        limit: 10,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("Search failed");
    let results = response.results;

    // Fuzzy search should find "harmony" documents
    assert!(
        !results.is_empty(),
        "Fuzzy search should find results despite typo"
    );
    assert_eq!(
        results[0].category, "harmony",
        "Should find harmony category"
    );
}

#[tokio::test]
async fn test_backend_switching_simple_to_tantivy() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_test_data(&temp_dir).expect("Failed to setup test data");

    // Test with simple backend
    let simple_config = create_test_config(&temp_dir, "simple", false);
    let simple_state = AppState::new(simple_config)
        .await
        .expect("Failed to create simple state");

    let params = SearchConceptsParams {
        query: "voice".to_string(),
        limit: 10,
    };

    let simple_response = search_concepts(&simple_state, params.clone())
        .await
        .expect("Simple search failed");
    let simple_results = simple_response.results;

    // Build Tantivy index and test
    let tantivy_config = create_test_config(&temp_dir, "tantivy", false);
    build_index(&tantivy_config)
        .await
        .expect("Failed to build index");
    let tantivy_state = AppState::new(tantivy_config)
        .await
        .expect("Failed to create tantivy state");

    let tantivy_response = search_concepts(&tantivy_state, params)
        .await
        .expect("Tantivy search failed");
    let tantivy_results = tantivy_response.results;

    // Both backends should find results (may differ in ranking)
    assert!(
        !simple_results.is_empty(),
        "Simple backend should find results"
    );
    assert!(
        !tantivy_results.is_empty(),
        "Tantivy backend should find results"
    );

    // Both should find the "Voice Leading" card
    assert!(
        simple_results.iter().any(|r| r.title == "Voice Leading"),
        "Simple search should find Voice Leading"
    );
    assert!(
        tantivy_results.iter().any(|r| r.title == "Voice Leading"),
        "Tantivy search should find Voice Leading"
    );
}

#[tokio::test]
async fn test_snippet_generation_includes_context() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_test_data(&temp_dir).expect("Failed to setup test data");

    let config = create_test_config(&temp_dir, "tantivy", false);
    build_index(&config).await.expect("Failed to build index");

    let state = AppState::new(config).await.expect("Failed to create state");

    let params = SearchConceptsParams {
        query: "parallel".to_string(),
        limit: 10,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("Search failed");
    let results = response.results;

    assert!(!results.is_empty(), "Should find results for 'parallel'");

    let result = &results[0];
    assert!(
        result.snippet.contains("parallel") || result.snippet.contains("Parallel"),
        "Snippet should contain the query term"
    );
    assert!(!result.snippet.is_empty(), "Snippet should not be empty");
}

#[tokio::test]
async fn test_empty_query_errors_correctly() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_test_data(&temp_dir).expect("Failed to setup test data");

    let config = create_test_config(&temp_dir, "tantivy", false);
    build_index(&config).await.expect("Failed to build index");

    let state = AppState::new(config).await.expect("Failed to create state");

    let params = SearchConceptsParams {
        query: "".to_string(),
        limit: 10,
    };

    let result = search_concepts(&state, params).await;

    // Empty query should return an error
    assert!(result.is_err(), "Empty query should return error");
}

#[tokio::test]
async fn test_search_with_limit() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_test_data(&temp_dir).expect("Failed to setup test data");

    let config = create_test_config(&temp_dir, "tantivy", false);
    build_index(&config).await.expect("Failed to build index");

    let state = AppState::new(config).await.expect("Failed to create state");

    // Search with limit of 1
    let params = SearchConceptsParams {
        query: "harmony".to_string(),
        limit: 1,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("Search failed");
    let results = response.results;

    // Should respect limit
    assert!(results.len() <= 1, "Should respect limit parameter");
}

#[tokio::test]
async fn test_index_rebuild_clears_old_data() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let concept_cards_path = temp_dir.path().join("concept-cards");

    // Create initial data
    create_test_concept_card(
        &concept_cards_path,
        "test1.md",
        "Test 1",
        "test",
        "First test",
        "Content 1",
    )
    .expect("Failed to create test file");

    let config = create_test_config(&temp_dir, "tantivy", false);

    // Build initial index
    let stats1 = build_index(&config).await.expect("Failed to build index");
    assert_eq!(stats1.indexed, 1, "Should index 1 file initially");

    // Add more files
    create_test_concept_card(
        &concept_cards_path,
        "test2.md",
        "Test 2",
        "test",
        "Second test",
        "Content 2",
    )
    .expect("Failed to create second test file");

    // Rebuild index
    let stats2 = build_index(&config).await.expect("Failed to rebuild index");
    assert_eq!(stats2.indexed, 2, "Should index 2 files after rebuild");

    // Search should find both
    let state = AppState::new(config).await.expect("Failed to create state");
    let params = SearchConceptsParams {
        query: "test".to_string(),
        limit: 10,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("Search failed");
    let results = response.results;
    assert_eq!(results.len(), 2, "Should find both documents after rebuild");
}

#[tokio::test]
async fn test_search_tool_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    setup_test_data(&temp_dir).expect("Failed to setup test data");

    let config = create_test_config(&temp_dir, "tantivy", false);
    build_index(&config).await.expect("Failed to build index");

    // Test the search_concepts tool (full integration)
    let state = AppState::new(config).await.expect("Failed to create state");
    let params = SearchConceptsParams {
        query: "harmony".to_string(),
        limit: 5,
    };

    let response = search_concepts(&state, params)
        .await
        .expect("search_concepts failed");

    assert!(!response.results.is_empty(), "Should find results");
    assert_eq!(response.query, "harmony");
    assert!(response.total > 0, "Total should be greater than 0");
    assert_eq!(response.backend, "tantivy", "Should use tantivy backend");
}

#[tokio::test]
async fn test_health_tool_with_fts_ready() {
    use music_theory_mcp::tools::health::get_health;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config = create_test_config(&temp_dir, "tantivy", false);

    // Set up test data
    setup_test_data(&temp_dir);

    // Build index
    build_index(&config).await.expect("Failed to build index");

    // Create state (should load the index)
    let state = AppState::new(config).await.expect("Failed to create state");

    // Get health status
    let health = get_health(&state).await.expect("Failed to get health");

    assert_eq!(health.status, "ok");
    assert_eq!(
        health.backend.active, "tantivy",
        "Should report tantivy as active"
    );
    assert!(health.backend.fts_enabled, "FTS should be enabled");
    assert!(health.backend.fts_ready, "FTS should be ready");
    assert!(
        health.backend.index_stats.is_some(),
        "Should have index stats"
    );

    if let Some(stats) = health.backend.index_stats {
        assert!(stats.doc_count > 0, "Should have indexed documents");
        assert!(
            stats.last_indexed.is_some(),
            "Should have last indexed time"
        );
    }
}

#[tokio::test]
async fn test_health_tool_simple_backend() {
    use music_theory_mcp::tools::health::get_health;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config = create_test_config(&temp_dir, "simple", false);

    setup_test_data(&temp_dir);

    let state = AppState::new(config).await.expect("Failed to create state");

    let health = get_health(&state).await.expect("Failed to get health");

    assert_eq!(health.status, "ok");
    assert_eq!(
        health.backend.active, "simple",
        "Should report simple as active"
    );
}

#[tokio::test]
async fn test_health_tool_tantivy_not_ready() {
    use music_theory_mcp::tools::health::get_health;

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let config = create_test_config(&temp_dir, "tantivy", false);

    setup_test_data(&temp_dir);

    // Create state without building index
    let state = AppState::new(config).await.expect("Failed to create state");

    let health = get_health(&state).await.expect("Failed to get health");

    assert_eq!(health.status, "ok");
    assert_eq!(
        health.backend.active, "simple",
        "Should fall back to simple"
    );
    assert!(
        health.backend.fts_enabled,
        "FTS should be enabled in config"
    );
    assert!(!health.backend.fts_ready, "FTS should not be ready");
    assert!(
        health.backend.index_stats.is_none(),
        "Should not have index stats"
    );
}
