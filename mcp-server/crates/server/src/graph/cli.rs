//! CLI command handlers for graph management.
//!
//! Provides handlers for the graph subcommand:
//! - build: Build graph from concept cards
//! - validate: Check graph integrity
//! - stats: Show graph statistics
//! - compile: Rebuild rkyv cache

use std::path::Path;

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
        let data_dir = Path::new(&config.paths.base).join("data");
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
    let data_dir = Path::new(&config.paths.base).join("data");
    let graph = load_graph(&data_dir).await?;

    println!("Validating concept graph...");

    let mut issues = Vec::new();
    let mut orphan_count = 0;
    let mut self_loop_count = 0;

    // Check for orphan nodes (no incoming or outgoing edges)
    for idx in graph.node_indices() {
        let in_degree = graph.edges_directed(idx, petgraph::Direction::Incoming).count();
        let out_degree = graph.edges_directed(idx, petgraph::Direction::Outgoing).count();

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
        issues.push(format!("Found {} orphan nodes (no relationships)", orphan_count));
    }

    if self_loop_count > 0 {
        issues.push(format!("Found {} self-loops", self_loop_count));
    }

    if issues.is_empty() {
        println!("✓ Graph is valid");
        println!("  {} nodes, {} edges", graph.node_count(), graph.edge_count());
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
    use std::collections::HashMap;
    use super::types::Node;

    let data_dir = Path::new(&config.paths.base).join("data");
    let json_path = data_dir.join("graphs").join("concept_graph.json");

    let content = tokio::fs::read_to_string(&json_path).await
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

    let data_dir = Path::new(&config.paths.base).join("data");
    let json_path = data_dir.join("graphs").join("concept_graph.json");

    // Load JSON
    let content = tokio::fs::read_to_string(&json_path).await
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
    #[test]
    fn test_module_compiles() {
        // Just verify the module compiles
        assert!(true);
    }
}
