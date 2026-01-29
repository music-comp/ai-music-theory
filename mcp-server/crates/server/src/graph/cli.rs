//! CLI command handlers for graph management.
//!
//! Provides handlers for the graph subcommand:
//! - build: Build graph from concept cards
//! - validate: Check graph integrity
//! - stats: Show graph statistics
//! - compile: Rebuild rkyv cache

use crate::config::Config;
use crate::error::Result;

use super::builder::GraphBuilder;
use super::persistence::{load_graph, save_graph};

/// Handle the `graph build` command.
///
/// # Arguments
///
/// * `config` - Server configuration
/// * `dry_run` - If true, don't write files
/// * `verbose` - If true, show detailed output
pub async fn handle_build(config: &Config, dry_run: bool, verbose: bool) -> Result<()> {
    println!("Building concept graph from cards...");

    let (graph_data, warnings) = GraphBuilder::build(config).await?;

    // Show warnings
    if !warnings.is_empty() {
        println!("\nWarnings:");
        for warning in &warnings {
            println!("  ⚠️  {}", warning);
        }
    }

    if verbose {
        println!("\nGraph structure:");
        println!("  Nodes: {}", graph_data.nodes.len());
        println!("  Edges: {}", graph_data.edges.len());

        if let Some(meta) = &graph_data.metadata {
            println!("\nBuild metadata:");
            println!("  Source cards: {}", meta.source_cards_count);
            println!("  Build time: {}ms", meta.build_duration_ms);
            println!("  Built at: {}", meta.built_at);
        }
    } else {
        println!("  Nodes: {}", graph_data.nodes.len());
        println!("  Edges: {}", graph_data.edges.len());
    }

    if dry_run {
        println!("\n(dry run - no files written)");
    } else {
        let data_dir = config.paths.base_path()?.join("data");
        save_graph(&graph_data, &data_dir).await?;

        println!("\n✓ Wrote concept_graph.json");
        println!("✓ Generated rkyv cache");
    }

    Ok(())
}

/// Handle the `graph validate` command.
///
/// # Arguments
///
/// * `config` - Server configuration
pub async fn handle_validate(config: &Config) -> Result<()> {
    let data_dir = config.paths.base_path()?.join("data");
    let graph = load_graph(&data_dir).await?;

    println!("Validating concept graph...");

    let mut issues = Vec::new();
    let mut orphan_count = 0;
    let mut self_loop_count = 0;

    // Check for orphan nodes (no incoming or outgoing edges)
    for idx in graph.node_indices() {
        let in_degree = graph
            .edges_directed(idx, petgraph::Direction::Incoming)
            .count();
        let out_degree = graph
            .edges_directed(idx, petgraph::Direction::Outgoing)
            .count();

        if in_degree == 0 && out_degree == 0 {
            orphan_count += 1;
        }
    }

    // Check for self-loops
    use petgraph::visit::EdgeRef;
    for edge in graph.edge_references() {
        if edge.source() == edge.target() {
            self_loop_count += 1;
        }
    }

    if orphan_count > 0 {
        issues.push(format!(
            "Found {} orphan nodes (no relationships)",
            orphan_count
        ));
    }

    if self_loop_count > 0 {
        issues.push(format!("Found {} self-loops", self_loop_count));
    }

    if issues.is_empty() {
        println!("✓ Graph is valid");
        println!(
            "  {} nodes, {} edges",
            graph.node_count(),
            graph.edge_count()
        );
    } else {
        println!("✗ Graph has issues:");
        for issue in &issues {
            println!("  - {}", issue);
        }
    }

    Ok(())
}

/// Handle the `graph stats` command.
///
/// # Arguments
///
/// * `config` - Server configuration
pub async fn handle_stats(config: &Config) -> Result<()> {
    use super::types::Node;
    use std::collections::HashMap;

    let data_dir = config.paths.base_path()?.join("data");
    let json_path = data_dir.join("graphs").join("concept_graph.json");

    let content = tokio::fs::read_to_string(&json_path)
        .await
        .map_err(|e| crate::error::Error::io_with_path(e, &json_path))?;
    let graph_data: super::types::GraphData = serde_json::from_str(&content)
        .map_err(|e| crate::error::Error::config(format!("Failed to parse JSON: {}", e)))?;

    println!("Graph Statistics");
    println!("================");

    // Count nodes by type
    let mut concept_count = 0;
    let mut source_count = 0;

    for node in &graph_data.nodes {
        match node {
            Node::Concept(_) => concept_count += 1,
            Node::Source(_) => source_count += 1,
        }
    }

    println!("Nodes:    {}", graph_data.nodes.len());
    println!("  Concepts: {}", concept_count);
    println!("  Sources:  {}", source_count);
    println!("Edges:    {}", graph_data.edges.len());

    // Count by relationship type
    let mut rel_counts: HashMap<String, usize> = HashMap::new();
    for edge in &graph_data.edges {
        let rel_name = format!("{:?}", edge.relationship);
        *rel_counts.entry(rel_name).or_insert(0) += 1;
    }

    println!("\nRelationship types:");
    let mut sorted_rels: Vec<_> = rel_counts.iter().collect();
    sorted_rels.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
    for (rel, count) in sorted_rels {
        println!("  {}: {}", rel, count);
    }

    // Count by category (for concepts)
    let mut category_counts: HashMap<String, usize> = HashMap::new();
    for node in &graph_data.nodes {
        if let Node::Concept(c) = node {
            *category_counts.entry(c.category.clone()).or_insert(0) += 1;
        }
    }

    if !category_counts.is_empty() {
        println!("\nConcept categories:");
        let mut sorted_cats: Vec<_> = category_counts.iter().collect();
        sorted_cats.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
        for (cat, count) in sorted_cats {
            println!("  {}: {}", cat, count);
        }
    }

    // Show metadata
    if let Some(meta) = &graph_data.metadata {
        println!("\nBuild metadata:");
        println!("  Built: {}", meta.built_at);
        println!("  Source cards: {}", meta.source_cards_count);
        println!("  Build duration: {}ms", meta.build_duration_ms);
    }

    Ok(())
}

/// Handle the `graph compile` command.
///
/// # Arguments
///
/// * `config` - Server configuration
pub async fn handle_compile(config: &Config) -> Result<()> {
    println!("Rebuilding rkyv cache from JSON...");

    let data_dir = config.paths.base_path()?.join("data");
    let json_path = data_dir.join("graphs").join("concept_graph.json");

    // Load JSON
    let content = tokio::fs::read_to_string(&json_path)
        .await
        .map_err(|e| crate::error::Error::io_with_path(e, &json_path))?;
    let graph_data: super::types::GraphData = serde_json::from_str(&content)
        .map_err(|e| crate::error::Error::config(format!("Failed to parse JSON: {}", e)))?;

    // Save (which regenerates cache)
    save_graph(&graph_data, &data_dir).await?;

    println!("✓ Cache rebuilt");
    println!("  Nodes: {}", graph_data.nodes.len());
    println!("  Edges: {}", graph_data.edges.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::types::{
        ConceptNode, Edge, EdgeOrigin, GraphData, Node, Relationship, SourceNode,
    };
    use tempfile::TempDir;

    /// Create a test config pointing to a temporary directory with concept cards
    async fn create_test_config_with_cards() -> (Config, TempDir) {
        let temp_dir = TempDir::new().unwrap();

        // Create concept-cards directory structure
        let cards_dir = temp_dir.path().join("concept-cards").join("test-source");
        std::fs::create_dir_all(&cards_dir).unwrap();

        // Create a simple concept card
        let card_content = r#"---
concept: Test Concept
category: test
source: test-source
---

# Test Concept

## Related Concepts

- **Prerequisite**: none
- **Leads to**: none
- **See also**: none
"#;
        std::fs::write(cards_dir.join("test-concept.md"), card_content).unwrap();

        // Create sources-md directory (for conversion check)
        let sources_md = temp_dir
            .path()
            .join("sources-md")
            .join("general-test-source");
        std::fs::create_dir_all(&sources_md).unwrap();

        // Create a basic config.toml
        let config_dir = temp_dir.path().join("config");
        std::fs::create_dir_all(&config_dir).unwrap();

        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "concept-cards"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources.oxford]
path = "/tmp"
[sources.oxford.files]

[sources.general]
path = "/tmp"
[sources.general.files]
test-source = "[2024] Test - Test Source.md"

[sources.papers]
path = "/tmp"
[sources.papers.files]

[logging]
level = "error"
coloured = false
output = "stderr"

[search]
backend = "simple"
"#,
            temp_dir.path().display()
        );

        std::fs::write(config_dir.join("default.toml"), config_content).unwrap();

        // Just use the default config but override the base path
        // The CLI functions should work with paths relative to the config base
        let mut config = Config::load().unwrap();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        // Add test source file
        config.sources.general.files.insert(
            "test-source".to_string(),
            "[2024] Test - Test Source.md".to_string(),
        );

        (config, temp_dir)
    }

    /// Create a pre-built graph in the data directory
    async fn create_test_graph_file(temp_dir: &TempDir) {
        let data_dir = temp_dir.path().join("data");

        let graph_data = GraphData {
            version: "1.0".to_string(),
            nodes: vec![
                Node::Source(SourceNode {
                    id: "test-source".to_string(),
                    title: "Test Source".to_string(),
                    author: "Test Author".to_string(),
                    year: Some(2024),
                    is_converted: false,
                }),
                Node::Concept(ConceptNode {
                    id: "test-concept".to_string(),
                    title: "Test Concept".to_string(),
                    category: "test".to_string(),
                    source_id: "test-source".to_string(),
                    canonical_id: None,
                    is_canonical: true,
                }),
            ],
            edges: vec![Edge {
                from: "test-source".to_string(),
                to: "test-concept".to_string(),
                relationship: Relationship::Introduces,
                weight: 1.0,
                origin: EdgeOrigin::Extracted,
            }],
            metadata: None,
        };

        crate::graph::persistence::save_graph(&graph_data, &data_dir)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_handle_build_dry_run() {
        let (config, _temp_dir) = create_test_config_with_cards().await;

        let result = handle_build(&config, true, false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_build_creates_files() {
        let (config, temp_dir) = create_test_config_with_cards().await;

        let result = handle_build(&config, false, false).await;
        assert!(result.is_ok());

        // Verify files were created
        let json_path = temp_dir.path().join("data/graphs/concept_graph.json");
        assert!(json_path.exists());

        let cache_path = temp_dir.path().join("data/.cache/concept_graph.rkyv");
        assert!(cache_path.exists());
    }

    #[tokio::test]
    async fn test_handle_build_verbose() {
        let (config, _temp_dir) = create_test_config_with_cards().await;

        let result = handle_build(&config, true, true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_validate_valid_graph() {
        let (config, temp_dir) = create_test_config_with_cards().await;
        create_test_graph_file(&temp_dir).await;

        let result = handle_validate(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_validate_graph_not_found() {
        let (config, _temp_dir) = create_test_config_with_cards().await;

        // Don't create graph file
        let result = handle_validate(&config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_stats() {
        let (config, temp_dir) = create_test_config_with_cards().await;
        create_test_graph_file(&temp_dir).await;

        let result = handle_stats(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_stats_graph_not_found() {
        let (config, _temp_dir) = create_test_config_with_cards().await;

        let result = handle_stats(&config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_compile() {
        let (config, temp_dir) = create_test_config_with_cards().await;
        create_test_graph_file(&temp_dir).await;

        let result = handle_compile(&config).await;
        assert!(result.is_ok());

        // Verify cache was regenerated
        let cache_path = temp_dir.path().join("data/.cache/concept_graph.rkyv");
        assert!(cache_path.exists());
    }

    #[tokio::test]
    async fn test_handle_compile_graph_not_found() {
        let (config, _temp_dir) = create_test_config_with_cards().await;

        let result = handle_compile(&config).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_module_compiles() {
        // Just verify the module compiles
        assert!(true);
    }
}
