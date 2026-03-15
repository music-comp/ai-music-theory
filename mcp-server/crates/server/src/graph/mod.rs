//! Graph module delegating to fabryk-graph.
//!
//! This module provides a thin wrapper around `fabryk::graph` types, adapting
//! them to the project's existing API surface. It replaces the original 11-file
//! hand-rolled graph module with delegation to the fabryk framework.
//!
//! # Architecture
//!
//! - **Re-exports**: Core types and algorithms from `fabryk::graph`.
//! - **Adapters**: `GraphStats`, `LoadedGraph`, and node discrimination helpers
//!   provide backward-compatible access patterns.
//! - **CLI handlers**: `handle_build`, `handle_validate`, `handle_stats`,
//!   `handle_compile` for the `graph` CLI subcommand.
//! - **query**: Project-specific MCP response types (JSON API contract).
//!
//! # Feature Gate
//!
//! This module is feature-gated with `#[cfg(feature = "graph")]` and only
//! compiles when the `graph` feature is enabled.

use std::path::Path;

use crate::config::Config;
use crate::error::{Error, Result};

// ============================================================================
// Re-exports from fabryk::graph — types
// ============================================================================

#[allow(unused_imports)]
pub use fabryk::graph::{
    Edge as FabrykEdge, EdgeOrigin as FabrykEdgeOrigin, GraphData, Node as FabrykNode, NodeType,
    Relationship as FabrykRelationship,
};

#[allow(unused_imports)]
pub use fabryk::graph::{BuildStats, GraphBuilder, GraphExtractor};

// ============================================================================
// Re-exports from fabryk::graph — algorithms
// ============================================================================

#[allow(unused_imports)]
pub use fabryk::graph::{
    calculate_centrality, find_bridges, get_related, neighborhood, prerequisites_sorted,
    shortest_path, validate_graph,
};

#[allow(unused_imports)]
pub use fabryk::graph::{compute_stats, quick_summary};

// ============================================================================
// Re-exports from fabryk::graph — result types
// ============================================================================

#[allow(unused_imports)]
pub use fabryk::graph::{
    CentralityScore, GraphStats as FabrykGraphStats, NeighborhoodResult, PathResult,
    PrerequisitesResult, ValidationIssue, ValidationResult,
};

// ============================================================================
// Re-exports from fabryk::graph — persistence
// ============================================================================

#[allow(unused_imports)]
pub use fabryk::graph::{load_graph, load_graph_from_str, save_graph};

// ============================================================================
// Query sub-module (MCP API contract types)
// ============================================================================

pub mod query;

#[allow(unused_imports)]
pub use query::{
    BridgeConceptsResponse, CentralConceptsResponse, ConceptPathResponse, ConceptSourcesResponse,
    ConceptVariantsResponse, DependentsResponse, NeighborhoodResponse, PrerequisitesResponse,
    RelatedConceptsResponse, SourceCoverageResponse,
};

// ============================================================================
// Backward-compatible adapter types
// ============================================================================

/// Statistics about the loaded graph (backward-compatible).
///
/// Provides concept/source counts computed from fabryk's flat `Node` type
/// using node discrimination helpers.
#[derive(Clone, Debug)]
pub struct GraphStats {
    /// Total number of nodes in the graph.
    pub node_count: u32,
    /// Total number of edges in the graph.
    pub edge_count: u32,
    /// Number of concept nodes (domain-type nodes).
    pub concept_count: u32,
    /// Number of source nodes (custom "source"-type nodes).
    pub source_count: u32,
}

/// Loaded graph with metadata.
///
/// Wraps fabryk's `GraphData` with a timestamp and backward-compatible
/// statistics. Used by `AppState` to hold the active graph instance.
#[derive(Clone, Debug)]
pub struct LoadedGraph {
    /// The underlying fabryk graph data.
    pub data: fabryk::graph::GraphData,
    /// When the graph was loaded.
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    /// Backward-compatible node/edge statistics.
    pub stats: GraphStats,
}

// ============================================================================
// Node discrimination helpers
// ============================================================================

/// Check if a fabryk `Node` is a concept node (Domain type).
///
/// In the fabryk graph model, concept nodes use the default `NodeType::Domain`.
pub fn is_concept_node(node: &fabryk::graph::Node) -> bool {
    matches!(node.node_type, NodeType::Domain)
}

/// Check if a fabryk `Node` is a source node.
///
/// Source nodes are stored with `NodeType::Custom("source")`.
pub fn is_source_node(node: &fabryk::graph::Node) -> bool {
    matches!(node.node_type, NodeType::Custom(ref s) if s == "source")
}

/// Get the node ID regardless of type.
pub fn node_id(node: &fabryk::graph::Node) -> &str {
    &node.id
}

/// Get the node title.
pub fn node_title(node: &fabryk::graph::Node) -> &str {
    &node.title
}

/// Get the node category (concepts have it in `.category`, sources may not).
pub fn node_category(node: &fabryk::graph::Node) -> &str {
    node.category.as_deref().unwrap_or("unknown")
}

/// Get the author from a source node's metadata.
pub fn source_author(node: &fabryk::graph::Node) -> &str {
    node.metadata
        .get("author")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown")
}

/// Get the year from a source node's metadata.
pub fn source_year(node: &fabryk::graph::Node) -> Option<u16> {
    node.metadata
        .get("year")
        .and_then(|v| v.as_u64())
        .map(|y| y as u16)
}

/// Check if a source has been converted to markdown.
pub fn source_is_converted(node: &fabryk::graph::Node) -> bool {
    node.metadata
        .get("is_converted")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

// ============================================================================
// Relationship mapping
// ============================================================================

/// Convert a project relationship name string to a fabryk `Relationship`.
///
/// Maps the project's 7 relationship types to fabryk variants:
/// - `Prerequisite`, `RelatesTo`, `Extends`, `Introduces`, `Covers` are direct
/// - `SameAs` and `Cites` become `Custom` variants
pub fn to_fabryk_relationship(name: &str) -> fabryk::graph::Relationship {
    match name.to_lowercase().as_str() {
        "prerequisite" => fabryk::graph::Relationship::Prerequisite,
        "relatesto" | "relates_to" => fabryk::graph::Relationship::RelatesTo,
        "extends" => fabryk::graph::Relationship::Extends,
        "introduces" => fabryk::graph::Relationship::Introduces,
        "covers" => fabryk::graph::Relationship::Covers,
        "sameas" | "same_as" => fabryk::graph::Relationship::Custom("same_as".to_string()),
        "cites" => fabryk::graph::Relationship::Custom("cites".to_string()),
        other => fabryk::graph::Relationship::Custom(other.to_string()),
    }
}

/// Convert a fabryk `Relationship` to a display string matching the old JSON output.
///
/// Returns PascalCase names for the project's 7 known relationships:
/// `"Prerequisite"`, `"RelatesTo"`, `"Extends"`, `"SameAs"`, `"Introduces"`,
/// `"Covers"`, `"Cites"`. Unknown `Custom` variants are returned as-is.
pub fn from_fabryk_relationship(rel: &fabryk::graph::Relationship) -> String {
    match rel {
        fabryk::graph::Relationship::Prerequisite => "Prerequisite".to_string(),
        fabryk::graph::Relationship::RelatesTo => "RelatesTo".to_string(),
        fabryk::graph::Relationship::Extends => "Extends".to_string(),
        fabryk::graph::Relationship::Introduces => "Introduces".to_string(),
        fabryk::graph::Relationship::Covers => "Covers".to_string(),
        fabryk::graph::Relationship::Custom(s) if s == "same_as" => "SameAs".to_string(),
        fabryk::graph::Relationship::Custom(s) if s == "cites" => "Cites".to_string(),
        fabryk::graph::Relationship::LeadsTo => "LeadsTo".to_string(),
        fabryk::graph::Relationship::VariantOf => "VariantOf".to_string(),
        fabryk::graph::Relationship::ContrastsWith => "ContrastsWith".to_string(),
        fabryk::graph::Relationship::AnswersQuestion => "AnswersQuestion".to_string(),
        fabryk::graph::Relationship::Custom(s) => s.clone(),
    }
}

// ============================================================================
// Graph loading
// ============================================================================

/// Load the concept graph for use in AppState.
///
/// Reads the graph from `data_dir/graphs/concept_graph.json`, parses it via
/// fabryk's `load_graph_from_str`, computes backward-compatible statistics,
/// and returns a `LoadedGraph` ready for use by the server.
///
/// # Arguments
///
/// * `data_dir` - Base data directory (contains `graphs/` subdirectory)
///
/// # Errors
///
/// Returns error if the graph file cannot be read or parsed.
pub async fn load_concept_graph(data_dir: &Path) -> Result<LoadedGraph> {
    let graph_path = data_dir.join("graphs/concept_graph.json");
    let json = tokio::fs::read_to_string(&graph_path)
        .await
        .map_err(|e| Error::io_with_path(e, &graph_path))?;

    let graph_data = fabryk::graph::load_graph_from_str(&json)
        .map_err(|e| Error::operation(format!("Failed to parse graph: {}", e)))?;

    let stats = compute_graph_stats(&graph_data);

    Ok(LoadedGraph {
        data: graph_data,
        loaded_at: chrono::Utc::now(),
        stats,
    })
}

/// Compute backward-compatible graph statistics from fabryk `GraphData`.
fn compute_graph_stats(data: &fabryk::graph::GraphData) -> GraphStats {
    let mut concept_count = 0u32;
    let mut source_count = 0u32;

    for node in data.nodes.values() {
        if is_concept_node(node) {
            concept_count += 1;
        } else if is_source_node(node) {
            source_count += 1;
        }
    }

    GraphStats {
        node_count: data.node_count() as u32,
        edge_count: data.edge_count() as u32,
        concept_count,
        source_count,
    }
}

// ============================================================================
// Graph building
// ============================================================================

/// Build the concept graph from content files and configuration.
///
/// Uses the fabryk `GraphBuilder` with the project's `MusicTheoryGraphExtractor`.
/// Optionally merges manual edges from `data/graphs/manual_edges.json`.
///
/// # Arguments
///
/// * `config` - Server configuration with content paths
///
/// # Returns
///
/// A tuple of `(GraphData, BuildStats)` on success.
///
/// # Errors
///
/// Returns error if path resolution, content discovery, or build fails.
pub async fn build_graph(config: &Config) -> Result<(fabryk::graph::GraphData, BuildStats)> {
    let concept_cards_path = config.paths.concept_cards_path()?;
    let data_dir = config.paths.base_path()?.join("data");

    let extractor = crate::extractors::MusicTheoryGraphExtractor::new();
    let mut builder =
        fabryk::graph::GraphBuilder::new(extractor).with_content_path(&concept_cards_path);

    // Add manual edges if available
    let manual_edges_path = data_dir.join("graphs/manual_edges.json");
    if manual_edges_path.exists() {
        builder = builder.with_manual_edges(&manual_edges_path);
    }

    let (graph_data, build_stats) = builder
        .build()
        .await
        .map_err(|e| Error::operation(format!("Graph build failed: {}", e)))?;

    Ok((graph_data, build_stats))
}

// ============================================================================
// CLI command handlers
// ============================================================================

/// Handle the `graph build` command.
///
/// Builds the concept graph from source content and saves it to disk.
///
/// # Arguments
///
/// * `config` - Server configuration
/// * `dry_run` - If true, build but do not write files
/// * `verbose` - If true, show detailed build output
pub async fn handle_build(config: &Config, dry_run: bool, verbose: bool) -> Result<()> {
    println!("Building concept graph from cards...");

    let (graph_data, build_stats) = build_graph(config).await?;

    // Show dangling references as warnings
    if !build_stats.dangling_refs.is_empty() {
        println!("\nWarnings:");
        for warning in &build_stats.dangling_refs {
            println!("  Warning: dangling reference: {}", warning);
        }
    }

    if verbose {
        println!("\nGraph structure:");
        println!("  Nodes: {}", graph_data.node_count());
        println!("  Edges: {}", graph_data.edge_count());
        println!("\nBuild statistics:");
        println!("  Files processed: {}", build_stats.files_processed);
        println!("  Files skipped: {}", build_stats.files_skipped);
        println!("  Manual edges loaded: {}", build_stats.manual_edges_loaded);
        println!("  Deduplicated edges: {}", build_stats.deduped_edges);
        if build_stats.from_cache {
            println!("  (loaded from cache)");
        }
    } else {
        println!("  Nodes: {}", graph_data.node_count());
        println!("  Edges: {}", graph_data.edge_count());
    }

    if dry_run {
        println!("\n(dry run - no files written)");
    } else {
        let data_dir = config.paths.base_path()?.join("data");
        let graphs_dir = data_dir.join("graphs");
        tokio::fs::create_dir_all(&graphs_dir)
            .await
            .map_err(|e| Error::io_with_path(e, &graphs_dir))?;

        let graph_path = graphs_dir.join("concept_graph.json");
        fabryk::graph::save_graph(&graph_data, &graph_path, None)
            .map_err(|e| Error::operation(format!("Failed to save graph: {}", e)))?;

        println!("\nWrote concept_graph.json");
    }

    Ok(())
}

/// Handle the `graph validate` command.
///
/// Loads the concept graph and runs validation checks for orphan nodes,
/// self-loops, duplicate edges, prerequisite cycles, and invalid canonical
/// references.
///
/// # Arguments
///
/// * `config` - Server configuration
pub async fn handle_validate(config: &Config) -> Result<()> {
    let data_dir = config.paths.base_path()?.join("data");
    let loaded = load_concept_graph(&data_dir).await?;

    println!("Validating concept graph...");

    let result = fabryk::graph::validate_graph(&loaded.data);

    if result.valid && result.warnings.is_empty() {
        println!("Graph is valid");
        println!(
            "  {} nodes, {} edges",
            loaded.data.node_count(),
            loaded.data.edge_count()
        );
    } else {
        if !result.errors.is_empty() {
            println!("Graph has errors:");
            for issue in &result.errors {
                println!("  ERROR [{}]: {}", issue.code, issue.message);
                for node in &issue.nodes {
                    println!("    node: {}", node);
                }
                for edge in &issue.edges {
                    println!("    edge: {}", edge);
                }
            }
        }

        if !result.warnings.is_empty() {
            println!("Graph has warnings:");
            for issue in &result.warnings {
                println!("  WARN [{}]: {}", issue.code, issue.message);
                for node in &issue.nodes {
                    println!("    node: {}", node);
                }
            }
        }

        println!(
            "\n  {} nodes, {} edges",
            loaded.data.node_count(),
            loaded.data.edge_count()
        );
    }

    Ok(())
}

/// Handle the `graph stats` command.
///
/// Loads the concept graph and prints comprehensive statistics including
/// node counts by type, edge counts by relationship, and category distribution.
///
/// # Arguments
///
/// * `config` - Server configuration
pub async fn handle_stats(config: &Config) -> Result<()> {
    let data_dir = config.paths.base_path()?.join("data");
    let loaded = load_concept_graph(&data_dir).await?;

    let fabryk_stats = fabryk::graph::compute_stats(&loaded.data);

    println!("Graph Statistics");
    println!("================");

    println!("Nodes:    {}", fabryk_stats.node_count);
    println!("  Concepts: {}", loaded.stats.concept_count);
    println!("  Sources:  {}", loaded.stats.source_count);
    println!("Edges:    {}", fabryk_stats.edge_count);

    // Relationship distribution
    if !fabryk_stats.relationship_distribution.is_empty() {
        println!("\nRelationship types:");
        let mut sorted_rels: Vec<_> = fabryk_stats.relationship_distribution.iter().collect();
        sorted_rels.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
        for (rel, count) in sorted_rels {
            println!("  {}: {}", rel, count);
        }
    }

    // Category distribution
    if !fabryk_stats.category_distribution.is_empty() {
        println!("\nConcept categories:");
        let mut sorted_cats: Vec<_> = fabryk_stats.category_distribution.iter().collect();
        sorted_cats.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
        for (cat, count) in sorted_cats {
            println!("  {}: {}", cat, count);
        }
    }

    // Degree analysis
    println!("\nDegree analysis:");
    println!("  Average degree: {:.2}", fabryk_stats.avg_degree);
    println!("  Max in-degree: {}", fabryk_stats.max_in_degree);
    println!("  Max out-degree: {}", fabryk_stats.max_out_degree);
    if let Some(ref node_id) = fabryk_stats.most_depended_on {
        println!("  Most depended on: {}", node_id);
    }
    if let Some(ref node_id) = fabryk_stats.most_dependencies {
        println!("  Most dependencies: {}", node_id);
    }
    if fabryk_stats.orphan_count > 0 {
        println!("  Orphan nodes: {}", fabryk_stats.orphan_count);
    }

    Ok(())
}

/// Handle the `graph compile` command.
///
/// Loads the concept graph from JSON and re-saves it (regenerating any caches).
///
/// # Arguments
///
/// * `config` - Server configuration
pub async fn handle_compile(config: &Config) -> Result<()> {
    println!("Rebuilding graph from JSON...");

    let data_dir = config.paths.base_path()?.join("data");
    let loaded = load_concept_graph(&data_dir).await?;

    // Re-save (which regenerates the JSON and any caches)
    let graph_path = data_dir.join("graphs/concept_graph.json");
    fabryk::graph::save_graph(&loaded.data, &graph_path, None)
        .map_err(|e| Error::operation(format!("Failed to save graph: {}", e)))?;

    println!("Cache rebuilt");
    println!("  Nodes: {}", loaded.data.node_count());
    println!("  Edges: {}", loaded.data.edge_count());

    Ok(())
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use fabryk::graph::{Edge, Node, NodeType, Relationship};

    // -----------------------------------------------------------------------
    // Node discrimination helpers
    // -----------------------------------------------------------------------

    #[test]
    fn test_is_concept_node_domain_type() {
        let node = Node::new("test", "Test");
        assert!(is_concept_node(&node));
    }

    #[test]
    fn test_is_concept_node_user_query_type() {
        let node = Node::new("q", "Query").with_node_type(NodeType::UserQuery);
        assert!(!is_concept_node(&node));
    }

    #[test]
    fn test_is_concept_node_custom_source_type() {
        let node =
            Node::new("src", "Source").with_node_type(NodeType::Custom("source".to_string()));
        assert!(!is_concept_node(&node));
    }

    #[test]
    fn test_is_source_node_true() {
        let node =
            Node::new("src", "Source").with_node_type(NodeType::Custom("source".to_string()));
        assert!(is_source_node(&node));
    }

    #[test]
    fn test_is_source_node_false_domain() {
        let node = Node::new("concept", "Concept");
        assert!(!is_source_node(&node));
    }

    #[test]
    fn test_is_source_node_false_other_custom() {
        let node = Node::new("x", "X").with_node_type(NodeType::Custom("other".to_string()));
        assert!(!is_source_node(&node));
    }

    #[test]
    fn test_node_id() {
        let node = Node::new("my-id", "My Title");
        assert_eq!(node_id(&node), "my-id");
    }

    #[test]
    fn test_node_title() {
        let node = Node::new("id", "My Title");
        assert_eq!(node_title(&node), "My Title");
    }

    #[test]
    fn test_node_category_present() {
        let node = Node::new("id", "Title").with_category("harmony");
        assert_eq!(node_category(&node), "harmony");
    }

    #[test]
    fn test_node_category_absent() {
        let node = Node::new("id", "Title");
        assert_eq!(node_category(&node), "unknown");
    }

    #[test]
    fn test_source_author_present() {
        let node = Node::new("src", "Source").with_metadata(
            "author",
            serde_json::Value::String("J. S. Bach".to_string()),
        );
        assert_eq!(source_author(&node), "J. S. Bach");
    }

    #[test]
    fn test_source_author_absent() {
        let node = Node::new("src", "Source");
        assert_eq!(source_author(&node), "Unknown");
    }

    #[test]
    fn test_source_year_present() {
        let node = Node::new("src", "Source").with_metadata("year", serde_json::json!(2024));
        assert_eq!(source_year(&node), Some(2024));
    }

    #[test]
    fn test_source_year_absent() {
        let node = Node::new("src", "Source");
        assert_eq!(source_year(&node), None);
    }

    #[test]
    fn test_source_is_converted_true() {
        let node =
            Node::new("src", "Source").with_metadata("is_converted", serde_json::json!(true));
        assert!(source_is_converted(&node));
    }

    #[test]
    fn test_source_is_converted_false() {
        let node =
            Node::new("src", "Source").with_metadata("is_converted", serde_json::json!(false));
        assert!(!source_is_converted(&node));
    }

    #[test]
    fn test_source_is_converted_absent() {
        let node = Node::new("src", "Source");
        assert!(!source_is_converted(&node));
    }

    // -----------------------------------------------------------------------
    // Relationship conversion
    // -----------------------------------------------------------------------

    #[test]
    fn test_to_fabryk_relationship_prerequisite() {
        assert_eq!(
            to_fabryk_relationship("Prerequisite"),
            Relationship::Prerequisite
        );
        assert_eq!(
            to_fabryk_relationship("prerequisite"),
            Relationship::Prerequisite
        );
    }

    #[test]
    fn test_to_fabryk_relationship_relates_to() {
        assert_eq!(to_fabryk_relationship("RelatesTo"), Relationship::RelatesTo);
        assert_eq!(
            to_fabryk_relationship("relates_to"),
            Relationship::RelatesTo
        );
    }

    #[test]
    fn test_to_fabryk_relationship_extends() {
        assert_eq!(to_fabryk_relationship("Extends"), Relationship::Extends);
    }

    #[test]
    fn test_to_fabryk_relationship_introduces() {
        assert_eq!(
            to_fabryk_relationship("Introduces"),
            Relationship::Introduces
        );
    }

    #[test]
    fn test_to_fabryk_relationship_covers() {
        assert_eq!(to_fabryk_relationship("Covers"), Relationship::Covers);
    }

    #[test]
    fn test_to_fabryk_relationship_same_as() {
        assert_eq!(
            to_fabryk_relationship("SameAs"),
            Relationship::Custom("same_as".to_string())
        );
        assert_eq!(
            to_fabryk_relationship("same_as"),
            Relationship::Custom("same_as".to_string())
        );
    }

    #[test]
    fn test_to_fabryk_relationship_cites() {
        assert_eq!(
            to_fabryk_relationship("Cites"),
            Relationship::Custom("cites".to_string())
        );
    }

    #[test]
    fn test_to_fabryk_relationship_unknown_custom() {
        assert_eq!(
            to_fabryk_relationship("unknown_rel"),
            Relationship::Custom("unknown_rel".to_string())
        );
    }

    #[test]
    fn test_from_fabryk_relationship_prerequisite() {
        assert_eq!(
            from_fabryk_relationship(&Relationship::Prerequisite),
            "Prerequisite"
        );
    }

    #[test]
    fn test_from_fabryk_relationship_relates_to() {
        assert_eq!(
            from_fabryk_relationship(&Relationship::RelatesTo),
            "RelatesTo"
        );
    }

    #[test]
    fn test_from_fabryk_relationship_extends() {
        assert_eq!(from_fabryk_relationship(&Relationship::Extends), "Extends");
    }

    #[test]
    fn test_from_fabryk_relationship_introduces() {
        assert_eq!(
            from_fabryk_relationship(&Relationship::Introduces),
            "Introduces"
        );
    }

    #[test]
    fn test_from_fabryk_relationship_covers() {
        assert_eq!(from_fabryk_relationship(&Relationship::Covers), "Covers");
    }

    #[test]
    fn test_from_fabryk_relationship_same_as() {
        let rel = Relationship::Custom("same_as".to_string());
        assert_eq!(from_fabryk_relationship(&rel), "SameAs");
    }

    #[test]
    fn test_from_fabryk_relationship_cites() {
        let rel = Relationship::Custom("cites".to_string());
        assert_eq!(from_fabryk_relationship(&rel), "Cites");
    }

    #[test]
    fn test_from_fabryk_relationship_leads_to() {
        assert_eq!(from_fabryk_relationship(&Relationship::LeadsTo), "LeadsTo");
    }

    #[test]
    fn test_from_fabryk_relationship_custom_passthrough() {
        let rel = Relationship::Custom("something_else".to_string());
        assert_eq!(from_fabryk_relationship(&rel), "something_else");
    }

    #[test]
    fn test_relationship_round_trip_prerequisite() {
        let original = "Prerequisite";
        let fabryk_rel = to_fabryk_relationship(original);
        let back = from_fabryk_relationship(&fabryk_rel);
        assert_eq!(back, original);
    }

    #[test]
    fn test_relationship_round_trip_relates_to() {
        let fabryk_rel = to_fabryk_relationship("RelatesTo");
        let back = from_fabryk_relationship(&fabryk_rel);
        assert_eq!(back, "RelatesTo");
    }

    #[test]
    fn test_relationship_round_trip_same_as() {
        let fabryk_rel = to_fabryk_relationship("SameAs");
        let back = from_fabryk_relationship(&fabryk_rel);
        assert_eq!(back, "SameAs");
    }

    #[test]
    fn test_relationship_round_trip_cites() {
        let fabryk_rel = to_fabryk_relationship("Cites");
        let back = from_fabryk_relationship(&fabryk_rel);
        assert_eq!(back, "Cites");
    }

    // -----------------------------------------------------------------------
    // GraphStats computation
    // -----------------------------------------------------------------------

    #[test]
    fn test_compute_graph_stats_empty() {
        let data = fabryk::graph::GraphData::new();
        let stats = compute_graph_stats(&data);

        assert_eq!(stats.node_count, 0);
        assert_eq!(stats.edge_count, 0);
        assert_eq!(stats.concept_count, 0);
        assert_eq!(stats.source_count, 0);
    }

    #[test]
    fn test_compute_graph_stats_concepts_only() {
        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A").with_category("harmony"));
        data.add_node(Node::new("b", "B").with_category("rhythm"));

        let stats = compute_graph_stats(&data);

        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.concept_count, 2);
        assert_eq!(stats.source_count, 0);
    }

    #[test]
    fn test_compute_graph_stats_mixed_nodes() {
        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("concept-1", "Concept 1").with_category("harmony"));
        data.add_node(
            Node::new("source-1", "Source 1")
                .with_node_type(NodeType::Custom("source".to_string())),
        );
        data.add_node(Node::new("concept-2", "Concept 2").with_category("rhythm"));

        data.add_edge(Edge::new(
            "concept-1",
            "concept-2",
            Relationship::Prerequisite,
        ))
        .expect("edge should be added");

        let stats = compute_graph_stats(&data);

        assert_eq!(stats.node_count, 3);
        assert_eq!(stats.edge_count, 1);
        assert_eq!(stats.concept_count, 2);
        assert_eq!(stats.source_count, 1);
    }

    // -----------------------------------------------------------------------
    // LoadedGraph creation
    // -----------------------------------------------------------------------

    #[test]
    fn test_loaded_graph_clone() {
        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("test", "Test"));

        let loaded = LoadedGraph {
            data,
            loaded_at: chrono::Utc::now(),
            stats: GraphStats {
                node_count: 1,
                edge_count: 0,
                concept_count: 1,
                source_count: 0,
            },
        };

        let cloned = loaded.clone();
        assert_eq!(cloned.stats.node_count, 1);
        assert_eq!(cloned.data.node_count(), 1);
    }

    #[test]
    fn test_graph_stats_debug() {
        let stats = GraphStats {
            node_count: 10,
            edge_count: 20,
            concept_count: 8,
            source_count: 2,
        };
        let debug_str = format!("{:?}", stats);
        assert!(debug_str.contains("node_count: 10"));
        assert!(debug_str.contains("concept_count: 8"));
    }

    // -----------------------------------------------------------------------
    // load_concept_graph (async)
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_load_concept_graph_success() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create graphs dir");

        // Build a minimal graph and save it via fabryk
        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("concept-a", "Concept A").with_category("test"));
        data.add_node(
            Node::new("source-1", "Source 1")
                .with_node_type(NodeType::Custom("source".to_string())),
        );
        data.add_edge(Edge::new("source-1", "concept-a", Relationship::Introduces))
            .expect("edge");

        let graph_path = graphs_dir.join("concept_graph.json");
        fabryk::graph::save_graph(&data, &graph_path, None).expect("save");

        let loaded = load_concept_graph(temp_dir.path()).await.expect("load");

        assert_eq!(loaded.stats.node_count, 2);
        assert_eq!(loaded.stats.edge_count, 1);
        assert_eq!(loaded.stats.concept_count, 1);
        assert_eq!(loaded.stats.source_count, 1);
        assert!(loaded.data.contains_node("concept-a"));
        assert!(loaded.data.contains_node("source-1"));
    }

    #[tokio::test]
    async fn test_load_concept_graph_missing_file() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let result = load_concept_graph(temp_dir.path()).await;
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // CLI handler smoke tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_handle_validate_valid_graph() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A").with_category("test"));
        data.add_node(Node::new("b", "B").with_category("test"));
        data.add_edge(Edge::new("a", "b", Relationship::Prerequisite))
            .expect("edge");

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        // Build a config pointing to our temp dir
        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_validate(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_stats_with_graph() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A").with_category("harmony"));
        data.add_node(
            Node::new("src", "Source").with_node_type(NodeType::Custom("source".to_string())),
        );
        data.add_edge(Edge::new("src", "a", Relationship::Introduces))
            .expect("edge");

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_stats(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_compile_with_graph() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A"));

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_compile(&config).await;
        assert!(result.is_ok());

        // Verify file was re-written
        assert!(graphs_dir.join("concept_graph.json").exists());
    }

    #[tokio::test]
    async fn test_handle_validate_missing_graph() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_validate(&config).await;
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // Additional from_fabryk_relationship coverage
    // -----------------------------------------------------------------------

    #[test]
    fn test_from_fabryk_relationship_variant_of() {
        assert_eq!(
            from_fabryk_relationship(&Relationship::VariantOf),
            "VariantOf"
        );
    }

    #[test]
    fn test_from_fabryk_relationship_contrasts_with() {
        assert_eq!(
            from_fabryk_relationship(&Relationship::ContrastsWith),
            "ContrastsWith"
        );
    }

    #[test]
    fn test_from_fabryk_relationship_answers_question() {
        assert_eq!(
            from_fabryk_relationship(&Relationship::AnswersQuestion),
            "AnswersQuestion"
        );
    }

    // -----------------------------------------------------------------------
    // Additional to_fabryk_relationship coverage
    // -----------------------------------------------------------------------

    #[test]
    fn test_to_fabryk_relationship_case_insensitive_extends() {
        assert_eq!(to_fabryk_relationship("EXTENDS"), Relationship::Extends);
    }

    #[test]
    fn test_to_fabryk_relationship_case_insensitive_introduces() {
        assert_eq!(
            to_fabryk_relationship("INTRODUCES"),
            Relationship::Introduces
        );
    }

    #[test]
    fn test_to_fabryk_relationship_case_insensitive_covers() {
        assert_eq!(to_fabryk_relationship("COVERS"), Relationship::Covers);
    }

    #[test]
    fn test_to_fabryk_relationship_case_insensitive_cites() {
        assert_eq!(
            to_fabryk_relationship("CITES"),
            Relationship::Custom("cites".to_string())
        );
    }

    // -----------------------------------------------------------------------
    // Relationship round-trip for additional variants
    // -----------------------------------------------------------------------

    #[test]
    fn test_relationship_round_trip_extends() {
        let fabryk_rel = to_fabryk_relationship("Extends");
        let back = from_fabryk_relationship(&fabryk_rel);
        assert_eq!(back, "Extends");
    }

    #[test]
    fn test_relationship_round_trip_introduces() {
        let fabryk_rel = to_fabryk_relationship("Introduces");
        let back = from_fabryk_relationship(&fabryk_rel);
        assert_eq!(back, "Introduces");
    }

    #[test]
    fn test_relationship_round_trip_covers() {
        let fabryk_rel = to_fabryk_relationship("Covers");
        let back = from_fabryk_relationship(&fabryk_rel);
        assert_eq!(back, "Covers");
    }

    // -----------------------------------------------------------------------
    // Node discrimination edge cases
    // -----------------------------------------------------------------------

    #[test]
    fn test_is_concept_node_custom_non_source() {
        let node = Node::new("x", "X").with_node_type(NodeType::Custom("other".to_string()));
        assert!(!is_concept_node(&node));
    }

    #[test]
    fn test_is_source_node_user_query_type() {
        let node = Node::new("q", "Query").with_node_type(NodeType::UserQuery);
        assert!(!is_source_node(&node));
    }

    #[test]
    fn test_node_id_with_special_characters() {
        let node = Node::new("concept@source-1/part", "Title");
        assert_eq!(node_id(&node), "concept@source-1/part");
    }

    #[test]
    fn test_node_title_unicode() {
        let node = Node::new("id", "Résumé des accords");
        assert_eq!(node_title(&node), "Résumé des accords");
    }

    // -----------------------------------------------------------------------
    // Source metadata edge cases
    // -----------------------------------------------------------------------

    #[test]
    fn test_source_author_non_string_metadata() {
        let node = Node::new("src", "Source").with_metadata("author", serde_json::json!(42));
        // Non-string value should fall back to "Unknown"
        assert_eq!(source_author(&node), "Unknown");
    }

    #[test]
    fn test_source_year_non_numeric_metadata() {
        let node =
            Node::new("src", "Source").with_metadata("year", serde_json::json!("not-a-number"));
        assert_eq!(source_year(&node), None);
    }

    #[test]
    fn test_source_year_float_metadata() {
        let node = Node::new("src", "Source").with_metadata("year", serde_json::json!(2024.5));
        // as_u64() on a float returns None in serde_json
        assert_eq!(source_year(&node), None);
    }

    #[test]
    fn test_source_is_converted_non_bool_metadata() {
        let node =
            Node::new("src", "Source").with_metadata("is_converted", serde_json::json!("yes"));
        // Non-bool should fall back to false
        assert!(!source_is_converted(&node));
    }

    #[test]
    fn test_source_is_converted_numeric_metadata() {
        let node = Node::new("src", "Source").with_metadata("is_converted", serde_json::json!(1));
        assert!(!source_is_converted(&node));
    }

    // -----------------------------------------------------------------------
    // compute_graph_stats with non-standard node types
    // -----------------------------------------------------------------------

    #[test]
    fn test_compute_graph_stats_user_query_nodes_not_counted() {
        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("c1", "C1").with_category("harmony"));
        data.add_node(Node::new("q1", "Query 1").with_node_type(NodeType::UserQuery));
        data.add_node(
            Node::new("src1", "Source 1").with_node_type(NodeType::Custom("source".to_string())),
        );
        data.add_node(
            Node::new("other1", "Other 1").with_node_type(NodeType::Custom("other".to_string())),
        );

        let stats = compute_graph_stats(&data);

        assert_eq!(stats.node_count, 4);
        assert_eq!(stats.concept_count, 1);
        assert_eq!(stats.source_count, 1);
        // user_query and other custom types are neither concept nor source
    }

    #[test]
    fn test_compute_graph_stats_sources_only() {
        let mut data = fabryk::graph::GraphData::new();
        data.add_node(
            Node::new("src1", "Source 1").with_node_type(NodeType::Custom("source".to_string())),
        );
        data.add_node(
            Node::new("src2", "Source 2").with_node_type(NodeType::Custom("source".to_string())),
        );

        let stats = compute_graph_stats(&data);

        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.concept_count, 0);
        assert_eq!(stats.source_count, 2);
    }

    #[test]
    fn test_compute_graph_stats_with_multiple_edges() {
        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A").with_category("test"));
        data.add_node(Node::new("b", "B").with_category("test"));
        data.add_node(Node::new("c", "C").with_category("test"));
        data.add_edge(Edge::new("a", "b", Relationship::Prerequisite))
            .expect("edge");
        data.add_edge(Edge::new("b", "c", Relationship::Prerequisite))
            .expect("edge");
        data.add_edge(Edge::new("a", "c", Relationship::RelatesTo))
            .expect("edge");

        let stats = compute_graph_stats(&data);

        assert_eq!(stats.node_count, 3);
        assert_eq!(stats.edge_count, 3);
        assert_eq!(stats.concept_count, 3);
        assert_eq!(stats.source_count, 0);
    }

    // -----------------------------------------------------------------------
    // LoadedGraph additional tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_loaded_graph_debug() {
        let data = fabryk::graph::GraphData::new();
        let loaded = LoadedGraph {
            data,
            loaded_at: chrono::Utc::now(),
            stats: GraphStats {
                node_count: 0,
                edge_count: 0,
                concept_count: 0,
                source_count: 0,
            },
        };
        let debug_str = format!("{:?}", loaded);
        assert!(debug_str.contains("LoadedGraph"));
        assert!(debug_str.contains("stats"));
    }

    #[test]
    fn test_graph_stats_clone() {
        let stats = GraphStats {
            node_count: 5,
            edge_count: 10,
            concept_count: 3,
            source_count: 2,
        };
        let cloned = stats.clone();
        assert_eq!(cloned.node_count, 5);
        assert_eq!(cloned.edge_count, 10);
        assert_eq!(cloned.concept_count, 3);
        assert_eq!(cloned.source_count, 2);
    }

    // -----------------------------------------------------------------------
    // load_concept_graph error paths
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_load_concept_graph_invalid_json() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create graphs dir");

        // Write invalid JSON
        let graph_path = graphs_dir.join("concept_graph.json");
        std::fs::write(&graph_path, "{ not valid json }").expect("write");

        let result = load_concept_graph(temp_dir.path()).await;
        assert!(result.is_err());
        let err_str = format!("{}", result.unwrap_err());
        assert!(
            err_str.contains("parse") || err_str.contains("Failed"),
            "Error should mention parsing failure: {}",
            err_str
        );
    }

    #[tokio::test]
    async fn test_load_concept_graph_empty_file() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create graphs dir");

        let graph_path = graphs_dir.join("concept_graph.json");
        std::fs::write(&graph_path, "").expect("write");

        let result = load_concept_graph(temp_dir.path()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_concept_graph_valid_json_wrong_structure() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create graphs dir");

        let graph_path = graphs_dir.join("concept_graph.json");
        std::fs::write(&graph_path, r#"{"key": "value"}"#).expect("write");

        let result = load_concept_graph(temp_dir.path()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_concept_graph_empty_graph() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create graphs dir");

        let data = fabryk::graph::GraphData::new();
        let graph_path = graphs_dir.join("concept_graph.json");
        fabryk::graph::save_graph(&data, &graph_path, None).expect("save");

        let loaded = load_concept_graph(temp_dir.path()).await.expect("load");
        assert_eq!(loaded.stats.node_count, 0);
        assert_eq!(loaded.stats.edge_count, 0);
        assert_eq!(loaded.stats.concept_count, 0);
        assert_eq!(loaded.stats.source_count, 0);
    }

    // -----------------------------------------------------------------------
    // CLI handlers: handle_validate with warnings/errors
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_handle_validate_graph_with_orphans() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        // Add orphan nodes (no edges)
        data.add_node(Node::new("orphan-1", "Orphan 1").with_category("test"));
        data.add_node(Node::new("orphan-2", "Orphan 2").with_category("test"));

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        // Should complete without error even if warnings are present
        let result = handle_validate(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_validate_graph_with_self_loop() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A").with_category("test"));
        data.add_node(Node::new("b", "B").with_category("test"));
        // Self-loop
        data.add_edge(Edge::new("a", "a", Relationship::RelatesTo))
            .expect("edge");
        data.add_edge(Edge::new("a", "b", Relationship::Prerequisite))
            .expect("edge");

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_validate(&config).await;
        assert!(result.is_ok());
    }

    // -----------------------------------------------------------------------
    // CLI handlers: handle_stats edge cases
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_handle_stats_empty_graph() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let data = fabryk::graph::GraphData::new();
        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_stats(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_stats_graph_with_multiple_categories() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A").with_category("harmony"));
        data.add_node(Node::new("b", "B").with_category("rhythm"));
        data.add_node(Node::new("c", "C").with_category("harmony"));
        data.add_node(
            Node::new("src", "Source").with_node_type(NodeType::Custom("source".to_string())),
        );
        data.add_edge(Edge::new("a", "b", Relationship::Prerequisite))
            .expect("edge");
        data.add_edge(Edge::new("a", "c", Relationship::RelatesTo))
            .expect("edge");
        data.add_edge(Edge::new("src", "a", Relationship::Introduces))
            .expect("edge");

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_stats(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_stats_missing_graph() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_stats(&config).await;
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // CLI handlers: handle_compile edge cases
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_handle_compile_missing_graph() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_compile(&config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_compile_preserves_data() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A").with_category("test"));
        data.add_node(Node::new("b", "B").with_category("test"));
        data.add_edge(Edge::new("a", "b", Relationship::Prerequisite))
            .expect("edge");

        let graph_path = graphs_dir.join("concept_graph.json");
        fabryk::graph::save_graph(&data, &graph_path, None).expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        handle_compile(&config).await.expect("compile");

        // Verify the graph can be reloaded after compile
        let data_dir = temp_dir.path().join("data");
        let reloaded = load_concept_graph(&data_dir).await.expect("reload");
        assert_eq!(reloaded.stats.node_count, 2);
        assert_eq!(reloaded.stats.edge_count, 1);
    }

    // -----------------------------------------------------------------------
    // load_concept_graph: large graph
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_load_concept_graph_multi_type_nodes() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create graphs dir");

        let mut data = fabryk::graph::GraphData::new();
        // Mix of types
        data.add_node(Node::new("c1", "C1").with_category("harmony"));
        data.add_node(Node::new("c2", "C2").with_category("rhythm"));
        data.add_node(
            Node::new("src1", "Source 1")
                .with_node_type(NodeType::Custom("source".to_string()))
                .with_metadata("author", serde_json::json!("Author A"))
                .with_metadata("year", serde_json::json!(2020)),
        );
        data.add_node(
            Node::new("src2", "Source 2")
                .with_node_type(NodeType::Custom("source".to_string()))
                .with_metadata("author", serde_json::json!("Author B")),
        );
        data.add_node(Node::new("q1", "Query 1").with_node_type(NodeType::UserQuery));

        data.add_edge(Edge::new("c1", "c2", Relationship::Prerequisite))
            .expect("edge");
        data.add_edge(Edge::new("src1", "c1", Relationship::Introduces))
            .expect("edge");
        data.add_edge(Edge::new("src2", "c2", Relationship::Covers))
            .expect("edge");

        let graph_path = graphs_dir.join("concept_graph.json");
        fabryk::graph::save_graph(&data, &graph_path, None).expect("save");

        let loaded = load_concept_graph(temp_dir.path()).await.expect("load");

        assert_eq!(loaded.stats.node_count, 5);
        assert_eq!(loaded.stats.edge_count, 3);
        assert_eq!(loaded.stats.concept_count, 2);
        assert_eq!(loaded.stats.source_count, 2);
    }

    // -----------------------------------------------------------------------
    // handle_stats: branches for orphans, most_depended_on, most_dependencies
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_handle_stats_graph_with_orphans() {
        // Exercises the `orphan_count > 0` println branch (line ~488-490)
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        // Connected pair
        data.add_node(Node::new("a", "A").with_category("harmony"));
        data.add_node(Node::new("b", "B").with_category("harmony"));
        data.add_edge(Edge::new("a", "b", Relationship::Prerequisite))
            .expect("edge");
        // Orphan (no edges)
        data.add_node(Node::new("orphan", "Orphan").with_category("misc"));

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_stats(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_stats_graph_with_high_degree_nodes() {
        // Exercises the `most_depended_on` and `most_dependencies` branches
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        // Hub node with many outgoing edges
        data.add_node(Node::new("hub", "Hub").with_category("core"));
        for i in 0..5 {
            let id = format!("dep-{}", i);
            let title = format!("Dep {}", i);
            data.add_node(Node::new(&id, &title).with_category("harmony"));
            data.add_edge(Edge::new("hub", &id, Relationship::Prerequisite))
                .expect("edge");
        }
        // Target node with many incoming edges
        data.add_node(Node::new("target", "Target").with_category("advanced"));
        for i in 0..5 {
            let id = format!("dep-{}", i);
            data.add_edge(Edge::new(&id, "target", Relationship::Prerequisite))
                .expect("edge");
        }

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_stats(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_stats_graph_with_no_relationships() {
        // Graph with only nodes and no edges: relationship distribution is empty
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A").with_category("harmony"));
        data.add_node(Node::new("b", "B").with_category("rhythm"));

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_stats(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_stats_graph_no_category_nodes() {
        // Graph where no node has a category: category distribution is empty
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A")); // No category
        data.add_node(Node::new("b", "B")); // No category
        data.add_edge(Edge::new("a", "b", Relationship::RelatesTo))
            .expect("edge");

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_stats(&config).await;
        assert!(result.is_ok());
    }

    // -----------------------------------------------------------------------
    // handle_validate: graphs that trigger errors and edge printing
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_handle_validate_graph_with_duplicate_edges() {
        // Duplicate edges should trigger validation warnings/errors
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("x", "X").with_category("test"));
        data.add_node(Node::new("y", "Y").with_category("test"));
        // Add the same edge twice (different relationship to allow it)
        data.add_edge(Edge::new("x", "y", Relationship::Prerequisite))
            .expect("edge");
        data.add_edge(Edge::new("x", "y", Relationship::RelatesTo))
            .expect("edge");

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_validate(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_validate_empty_graph() {
        // An empty graph is valid but exercises the valid+no-warnings branch
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let data = fabryk::graph::GraphData::new();
        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_validate(&config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_validate_complex_graph_with_many_relationships() {
        // A graph with many different relationship types exercises sorted output
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A").with_category("harmony"));
        data.add_node(Node::new("b", "B").with_category("rhythm"));
        data.add_node(Node::new("c", "C").with_category("melody"));
        data.add_node(Node::new("d", "D").with_category("form"));
        data.add_edge(Edge::new("a", "b", Relationship::Prerequisite))
            .expect("edge");
        data.add_edge(Edge::new("b", "c", Relationship::RelatesTo))
            .expect("edge");
        data.add_edge(Edge::new("c", "d", Relationship::Extends))
            .expect("edge");
        data.add_edge(Edge::new(
            "a",
            "c",
            Relationship::Custom("same_as".to_string()),
        ))
        .expect("edge");

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        let result = handle_validate(&config).await;
        assert!(result.is_ok());
    }

    // -----------------------------------------------------------------------
    // handle_compile: additional edge cases
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_handle_compile_large_graph() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("data/graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create dirs");

        let mut data = fabryk::graph::GraphData::new();
        for i in 0..20 {
            let id = format!("n{}", i);
            let title = format!("Node {}", i);
            data.add_node(Node::new(&id, &title).with_category("test"));
        }
        // Chain edges
        for i in 0..19 {
            let from = format!("n{}", i);
            let to = format!("n{}", i + 1);
            data.add_edge(Edge::new(&from, &to, Relationship::Prerequisite))
                .expect("edge");
        }

        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let mut config = Config::default();
        config.paths.base = temp_dir.path().to_string_lossy().to_string();

        handle_compile(&config).await.expect("compile");

        // Verify data integrity after compile
        let data_dir = temp_dir.path().join("data");
        let reloaded = load_concept_graph(&data_dir).await.expect("reload");
        assert_eq!(reloaded.stats.node_count, 20);
        assert_eq!(reloaded.stats.edge_count, 19);
    }

    // -----------------------------------------------------------------------
    // load_concept_graph: additional graph structures
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_load_concept_graph_only_sources() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create graphs dir");

        let mut data = fabryk::graph::GraphData::new();
        data.add_node(
            Node::new("src-a", "Source A")
                .with_node_type(NodeType::Custom("source".to_string()))
                .with_metadata("author", serde_json::json!("Author"))
                .with_metadata("year", serde_json::json!(2021))
                .with_metadata("is_converted", serde_json::json!(true)),
        );
        data.add_node(
            Node::new("src-b", "Source B").with_node_type(NodeType::Custom("source".to_string())),
        );

        let graph_path = graphs_dir.join("concept_graph.json");
        fabryk::graph::save_graph(&data, &graph_path, None).expect("save");

        let loaded = load_concept_graph(temp_dir.path()).await.expect("load");
        assert_eq!(loaded.stats.concept_count, 0);
        assert_eq!(loaded.stats.source_count, 2);
        assert_eq!(loaded.stats.node_count, 2);
    }

    #[tokio::test]
    async fn test_load_concept_graph_preserves_loaded_at_timestamp() {
        let temp_dir = tempfile::TempDir::new().expect("temp dir");
        let graphs_dir = temp_dir.path().join("graphs");
        std::fs::create_dir_all(&graphs_dir).expect("create graphs dir");

        let data = fabryk::graph::GraphData::new();
        fabryk::graph::save_graph(&data, graphs_dir.join("concept_graph.json"), None)
            .expect("save");

        let before = chrono::Utc::now();
        let loaded = load_concept_graph(temp_dir.path()).await.expect("load");
        let after = chrono::Utc::now();

        assert!(loaded.loaded_at >= before);
        assert!(loaded.loaded_at <= after);
    }

    // -----------------------------------------------------------------------
    // compute_graph_stats: edge cases with custom node types
    // -----------------------------------------------------------------------

    #[test]
    fn test_compute_graph_stats_only_custom_non_source_nodes() {
        // Nodes that are neither concept nor source
        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("x", "X").with_node_type(NodeType::Custom("widget".to_string())));
        data.add_node(Node::new("y", "Y").with_node_type(NodeType::Custom("gadget".to_string())));

        let stats = compute_graph_stats(&data);
        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.concept_count, 0);
        assert_eq!(stats.source_count, 0);
    }

    #[test]
    fn test_compute_graph_stats_all_types_represented() {
        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("c1", "Concept 1").with_category("theory"));
        data.add_node(Node::new("c2", "Concept 2").with_category("practice"));
        data.add_node(
            Node::new("s1", "Source 1").with_node_type(NodeType::Custom("source".to_string())),
        );
        data.add_node(
            Node::new("w1", "Widget 1").with_node_type(NodeType::Custom("widget".to_string())),
        );
        data.add_node(Node::new("q1", "Query 1").with_node_type(NodeType::UserQuery));
        data.add_edge(Edge::new("c1", "c2", Relationship::Prerequisite))
            .expect("edge");
        data.add_edge(Edge::new("s1", "c1", Relationship::Introduces))
            .expect("edge");
        data.add_edge(Edge::new(
            "c2",
            "s1",
            Relationship::Custom("cites".to_string()),
        ))
        .expect("edge");

        let stats = compute_graph_stats(&data);
        assert_eq!(stats.node_count, 5);
        assert_eq!(stats.edge_count, 3);
        assert_eq!(stats.concept_count, 2);
        assert_eq!(stats.source_count, 1);
    }

    // -----------------------------------------------------------------------
    // Source metadata: year edge cases
    // -----------------------------------------------------------------------

    #[test]
    fn test_source_year_zero() {
        let node = Node::new("src", "Source").with_metadata("year", serde_json::json!(0));
        assert_eq!(source_year(&node), Some(0));
    }

    #[test]
    fn test_source_year_large_value() {
        let node = Node::new("src", "Source").with_metadata("year", serde_json::json!(9999));
        assert_eq!(source_year(&node), Some(9999));
    }

    #[test]
    fn test_source_year_null_metadata() {
        let node = Node::new("src", "Source").with_metadata("year", serde_json::Value::Null);
        assert_eq!(source_year(&node), None);
    }

    #[test]
    fn test_source_author_empty_string() {
        let node = Node::new("src", "Source")
            .with_metadata("author", serde_json::Value::String(String::new()));
        assert_eq!(source_author(&node), "");
    }

    #[test]
    fn test_source_author_null_metadata() {
        let node = Node::new("src", "Source").with_metadata("author", serde_json::Value::Null);
        assert_eq!(source_author(&node), "Unknown");
    }

    // -----------------------------------------------------------------------
    // node_category: edge cases
    // -----------------------------------------------------------------------

    #[test]
    fn test_node_category_empty_string() {
        let node = Node::new("id", "Title").with_category("");
        assert_eq!(node_category(&node), "");
    }

    #[test]
    fn test_node_category_unicode() {
        let node = Node::new("id", "Title").with_category("théorie");
        assert_eq!(node_category(&node), "théorie");
    }

    // -----------------------------------------------------------------------
    // Relationship mapping: remaining unmapped variants
    // -----------------------------------------------------------------------

    #[test]
    fn test_to_fabryk_relationship_empty_string() {
        let rel = to_fabryk_relationship("");
        assert_eq!(rel, Relationship::Custom("".to_string()));
    }

    #[test]
    fn test_to_fabryk_relationship_mixed_case_same_as() {
        assert_eq!(
            to_fabryk_relationship("SAMEAS"),
            Relationship::Custom("same_as".to_string())
        );
        assert_eq!(
            to_fabryk_relationship("SAME_AS"),
            Relationship::Custom("same_as".to_string())
        );
    }

    #[test]
    fn test_to_fabryk_relationship_whitespace_is_custom() {
        let rel = to_fabryk_relationship("  ");
        assert_eq!(rel, Relationship::Custom("  ".to_string()));
    }

    #[test]
    fn test_from_fabryk_relationship_empty_custom() {
        let rel = Relationship::Custom(String::new());
        assert_eq!(from_fabryk_relationship(&rel), "");
    }

    // -----------------------------------------------------------------------
    // LoadedGraph: field access and constructed values
    // -----------------------------------------------------------------------

    #[test]
    fn test_loaded_graph_fields_accessible() {
        let mut data = fabryk::graph::GraphData::new();
        data.add_node(Node::new("a", "A").with_category("test"));
        data.add_node(Node::new("s", "S").with_node_type(NodeType::Custom("source".to_string())));
        data.add_edge(Edge::new("s", "a", Relationship::Introduces))
            .expect("edge");

        let stats = compute_graph_stats(&data);
        let loaded_at = chrono::Utc::now();

        let loaded = LoadedGraph {
            data,
            loaded_at,
            stats,
        };

        assert_eq!(loaded.stats.node_count, 2);
        assert_eq!(loaded.stats.edge_count, 1);
        assert_eq!(loaded.stats.concept_count, 1);
        assert_eq!(loaded.stats.source_count, 1);
        assert_eq!(loaded.loaded_at, loaded_at);
        assert!(loaded.data.contains_node("a"));
        assert!(loaded.data.contains_node("s"));
    }

    #[test]
    fn test_graph_stats_all_zeroes() {
        let stats = GraphStats {
            node_count: 0,
            edge_count: 0,
            concept_count: 0,
            source_count: 0,
        };
        let cloned = stats.clone();
        assert_eq!(cloned.node_count, 0);
        assert_eq!(cloned.edge_count, 0);
        let debug = format!("{:?}", stats);
        assert!(debug.contains("GraphStats"));
    }
}
