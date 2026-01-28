//! Graph persistence layer with rkyv caching.
//!
//! This module handles saving and loading the concept graph, with an efficient
//! caching layer using rkyv + mmap for fast startup times.
//!
//! # Cache Strategy
//!
//! - JSON file is the source of truth
//! - rkyv binary cache is generated from JSON
//! - blake3 hash validates cache freshness
//! - mmap allows zero-copy deserialization

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use memmap2::Mmap;
use petgraph::graph::{DiGraph, NodeIndex};

use crate::error::{Error, Result};

use super::types::{Edge, GraphData, Node};

/// Type alias for the concept graph structure.
///
/// A directed graph with Node (vertices) and Edge (edges).
pub type ConceptGraph = DiGraph<Node, Edge>;

/// Save graph data to JSON and rkyv cache.
///
/// This performs atomic writes to ensure data integrity:
/// 1. Serialize to JSON and write to `concept_graph.json`
/// 2. Compute blake3 hash of JSON
/// 3. Serialize to rkyv binary format
/// 4. Write rkyv to temp file, then atomically rename
/// 5. Write hash to validation file
///
/// # Arguments
///
/// * `graph_data` - The graph to save
/// * `data_dir` - Base data directory (contains graphs/ and .cache/)
///
/// # Errors
///
/// Returns error if serialization or file I/O fails.
pub async fn save_graph(graph_data: &GraphData, data_dir: &Path) -> Result<()> {
    // Ensure directories exist
    let graphs_dir = data_dir.join("graphs");
    let cache_dir = data_dir.join(".cache");

    tokio::fs::create_dir_all(&graphs_dir).await
        .map_err(|e| Error::io_with_path(e, &graphs_dir))?;
    tokio::fs::create_dir_all(&cache_dir).await
        .map_err(|e| Error::io_with_path(e, &cache_dir))?;

    // Step 1: Serialize to JSON
    let json = serde_json::to_string_pretty(graph_data)
        .map_err(|e| Error::config(format!("Failed to serialize graph to JSON: {}", e)))?;

    // Step 2: Write JSON (source of truth)
    let json_path = graphs_dir.join("concept_graph.json");
    tokio::fs::write(&json_path, &json).await
        .map_err(|e| Error::io_with_path(e, &json_path))?;

    log::info!("Wrote graph to {}", json_path.display());

    // Step 3: Compute hash
    let hash = blake3::hash(json.as_bytes());
    let hash_hex = hash.to_hex().to_string();

    // Step 4: Serialize to rkyv (this is synchronous, but fast)
    let rkyv_bytes = rkyv::to_bytes::<_, 4096>(graph_data)
        .map_err(|e| Error::config(format!("rkyv serialization failed: {}", e)))?;

    // Step 5: Write rkyv cache atomically
    let cache_path = cache_dir.join("concept_graph.rkyv");
    let temp_cache = cache_path.with_extension("rkyv.tmp");

    tokio::fs::write(&temp_cache, &rkyv_bytes).await
        .map_err(|e| Error::io_with_path(e, &temp_cache))?;
    tokio::fs::rename(&temp_cache, &cache_path).await
        .map_err(|e| Error::io_with_path(e, &cache_path))?;

    // Step 6: Write hash
    let hash_path = cache_dir.join("graph_hash");
    tokio::fs::write(&hash_path, hash_hex).await
        .map_err(|e| Error::io_with_path(e, &hash_path))?;

    log::info!("Saved rkyv cache ({} bytes) with hash", rkyv_bytes.len());

    Ok(())
}

/// Load graph from cache or JSON.
///
/// This attempts to use the cached rkyv version for fast loading.
/// If the cache is invalid or missing, it falls back to parsing JSON.
///
/// # Cache Validation
///
/// The cache is valid if:
/// - rkyv file exists
/// - hash file exists
/// - hash matches current JSON hash
///
/// # Arguments
///
/// * `data_dir` - Base data directory
///
/// # Returns
///
/// Returns the loaded ConceptGraph (petgraph DiGraph).
///
/// # Errors
///
/// Returns error if JSON doesn't exist or can't be parsed.
pub async fn load_graph(data_dir: &Path) -> Result<ConceptGraph> {
    let json_path = data_dir.join("graphs").join("concept_graph.json");
    let cache_path = data_dir.join(".cache").join("concept_graph.rkyv");
    let hash_path = data_dir.join(".cache").join("graph_hash");

    // Check if JSON exists
    if !json_path.exists() {
        return Err(Error::not_found_msg(
            "concept_graph.json not found. Run `music-theory-mcp graph build` first."
        ));
    }

    // Read JSON and compute hash
    let json_bytes = tokio::fs::read(&json_path).await
        .map_err(|e| Error::io_with_path(e, &json_path))?;
    let current_hash = blake3::hash(&json_bytes);
    let current_hash_hex = current_hash.to_hex().to_string();

    // Check cache validity
    let cache_valid = is_cache_valid(&cache_path, &hash_path, &current_hash_hex).await;

    if cache_valid {
        log::info!("Graph cache valid, loading via mmap");
        match load_from_rkyv(&cache_path).await {
            Ok(graph) => {
                log::info!("Loaded graph from cache: {} nodes, {} edges",
                    graph.node_count(), graph.edge_count());
                return Ok(graph);
            }
            Err(e) => {
                log::warn!("Failed to load cache, rebuilding: {}", e);
            }
        }
    } else {
        log::info!("Cache invalid or missing, loading from JSON");
    }

    // Cache miss or invalid - parse JSON
    let graph_data: GraphData = serde_json::from_slice(&json_bytes)
        .map_err(|e| Error::config(format!("Failed to parse concept_graph.json: {}", e)))?;

    let graph = to_petgraph(&graph_data);

    log::info!("Loaded graph from JSON: {} nodes, {} edges",
        graph.node_count(), graph.edge_count());

    // Save cache for next time
    if let Err(e) = save_cache(&graph_data, &cache_path, &hash_path, &current_hash_hex).await {
        log::warn!("Failed to save cache: {}", e);
    }

    Ok(graph)
}

/// Check if cache is valid.
async fn is_cache_valid(cache_path: &Path, hash_path: &Path, expected_hash: &str) -> bool {
    if !cache_path.exists() || !hash_path.exists() {
        return false;
    }

    match tokio::fs::read_to_string(hash_path).await {
        Ok(stored_hash) => stored_hash.trim() == expected_hash,
        Err(_) => false,
    }
}

/// Load graph from rkyv cache using mmap.
async fn load_from_rkyv(path: &Path) -> Result<ConceptGraph> {
    // Open file (sync operation, but fast)
    let file = File::open(path)
        .map_err(|e| Error::io_with_path(e, path))?;

    // Memory-map the file
    let mmap = unsafe {
        Mmap::map(&file)
            .map_err(|e| Error::io(e))?
    };

    // Access archived data (zero-copy)
    let archived = unsafe {
        rkyv::archived_root::<GraphData>(&mmap)
    };

    // Convert to petgraph
    Ok(archived_to_petgraph(archived))
}

/// Save cache after loading from JSON.
async fn save_cache(
    graph_data: &GraphData,
    cache_path: &Path,
    hash_path: &Path,
    hash: &str,
) -> Result<()> {
    // Serialize to rkyv
    let bytes = rkyv::to_bytes::<_, 4096>(graph_data)
        .map_err(|e| Error::config(format!("rkyv serialization failed: {}", e)))?;

    // Write atomically
    let tmp_cache = cache_path.with_extension("rkyv.tmp");
    tokio::fs::write(&tmp_cache, &bytes).await
        .map_err(|e| Error::io_with_path(e, &tmp_cache))?;
    tokio::fs::rename(&tmp_cache, cache_path).await
        .map_err(|e| Error::io_with_path(e, cache_path))?;

    // Write hash
    tokio::fs::write(hash_path, hash).await
        .map_err(|e| Error::io_with_path(e, hash_path))?;

    log::info!("Saved graph cache ({} bytes)", bytes.len());
    Ok(())
}

/// Convert GraphData to petgraph DiGraph.
pub fn to_petgraph(graph_data: &GraphData) -> ConceptGraph {
    let mut graph = DiGraph::new();
    let mut node_indices: HashMap<String, NodeIndex> = HashMap::new();

    // Add all nodes first
    for node in &graph_data.nodes {
        let id = match node {
            Node::Concept(c) => c.id.clone(),
            Node::Source(s) => s.id.clone(),
        };
        let idx = graph.add_node(node.clone());
        node_indices.insert(id, idx);
    }

    // Add edges
    for edge in &graph_data.edges {
        if let (Some(&from_idx), Some(&to_idx)) = (
            node_indices.get(&edge.from),
            node_indices.get(&edge.to),
        ) {
            graph.add_edge(from_idx, to_idx, edge.clone());
        }
    }

    graph
}

/// Convert archived GraphData to petgraph (from rkyv mmap).
fn archived_to_petgraph(archived: &rkyv::Archived<GraphData>) -> ConceptGraph {
    let mut graph = DiGraph::new();
    let mut node_indices: HashMap<String, NodeIndex> = HashMap::new();

    // Add nodes
    for node in archived.nodes.iter() {
        // Deserialize node from archived format
        let owned_node: Node = rkyv::Deserialize::deserialize(node, &mut rkyv::Infallible)
            .expect("Failed to deserialize node");

        let id = match &owned_node {
            Node::Concept(c) => c.id.clone(),
            Node::Source(s) => s.id.clone(),
        };
        let idx = graph.add_node(owned_node);
        node_indices.insert(id, idx);
    }

    // Add edges
    for edge in archived.edges.iter() {
        let owned_edge: Edge = rkyv::Deserialize::deserialize(edge, &mut rkyv::Infallible)
            .expect("Failed to deserialize edge");

        if let (Some(&from_idx), Some(&to_idx)) = (
            node_indices.get(&owned_edge.from),
            node_indices.get(&owned_edge.to),
        ) {
            graph.add_edge(from_idx, to_idx, owned_edge);
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::graph::types::{ConceptNode, SourceNode, GraphMetadata, Relationship, EdgeOrigin};

    fn create_test_graph() -> GraphData {
        let concept = Node::Concept(ConceptNode {
            id: "test-concept".to_string(),
            title: "Test Concept".to_string(),
            category: "test".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        });

        let source = Node::Source(SourceNode {
            id: "test-source".to_string(),
            title: "Test Source".to_string(),
            author: "Test Author".to_string(),
            year: Some(2024),
            is_converted: false,
        });

        let edge = Edge {
            from: "test-source".to_string(),
            to: "test-concept".to_string(),
            relationship: Relationship::Introduces,
            weight: 1.0,
            origin: EdgeOrigin::Extracted,
        };

        GraphData {
            version: "1.0".to_string(),
            nodes: vec![concept, source],
            edges: vec![edge],
            metadata: Some(GraphMetadata {
                built_at: "2024-01-01T00:00:00Z".to_string(),
                source_cards_count: 1,
                build_duration_ms: 100,
            }),
        }
    }

    #[tokio::test]
    async fn test_save_and_load_graph() {
        let temp_dir = TempDir::new().unwrap();
        let graph_data = create_test_graph();

        // Save
        save_graph(&graph_data, temp_dir.path()).await.unwrap();

        // Verify files exist
        assert!(temp_dir.path().join("graphs/concept_graph.json").exists());
        assert!(temp_dir.path().join(".cache/concept_graph.rkyv").exists());
        assert!(temp_dir.path().join(".cache/graph_hash").exists());

        // Load
        let graph = load_graph(temp_dir.path()).await.unwrap();

        // Verify structure
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[tokio::test]
    async fn test_cache_hit() {
        let temp_dir = TempDir::new().unwrap();
        let graph_data = create_test_graph();

        // First save
        save_graph(&graph_data, temp_dir.path()).await.unwrap();

        // First load (cache miss, creates cache)
        let _graph1 = load_graph(temp_dir.path()).await.unwrap();

        // Second load (cache hit - should be faster)
        let graph2 = load_graph(temp_dir.path()).await.unwrap();

        assert_eq!(graph2.node_count(), 2);
        assert_eq!(graph2.edge_count(), 1);
    }

    #[tokio::test]
    async fn test_cache_invalidation() {
        let temp_dir = TempDir::new().unwrap();
        let mut graph_data = create_test_graph();

        // Save initial graph
        save_graph(&graph_data, temp_dir.path()).await.unwrap();
        let _graph1 = load_graph(temp_dir.path()).await.unwrap();

        // Modify graph
        graph_data.nodes.push(Node::Concept(ConceptNode {
            id: "new-concept".to_string(),
            title: "New Concept".to_string(),
            category: "test".to_string(),
            source_id: "test-source".to_string(),
            canonical_id: None,
            is_canonical: true,
        }));

        // Save modified graph (should invalidate cache)
        save_graph(&graph_data, temp_dir.path()).await.unwrap();

        // Load (should detect cache invalidation)
        let graph2 = load_graph(temp_dir.path()).await.unwrap();

        assert_eq!(graph2.node_count(), 3); // New node added
    }

    #[tokio::test]
    async fn test_load_without_json() {
        let temp_dir = TempDir::new().unwrap();

        let result = load_graph(temp_dir.path()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_to_petgraph() {
        let graph_data = create_test_graph();
        let graph = to_petgraph(&graph_data);

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);

        // Verify we can access nodes
        for node in graph.node_weights() {
            match node {
                Node::Concept(c) => assert_eq!(c.id, "test-concept"),
                Node::Source(s) => assert_eq!(s.id, "test-source"),
            }
        }
    }

    #[tokio::test]
    async fn test_atomic_write() {
        let temp_dir = TempDir::new().unwrap();
        let graph_data = create_test_graph();

        // Save
        save_graph(&graph_data, temp_dir.path()).await.unwrap();

        // Verify no temp files left behind
        let cache_dir = temp_dir.path().join(".cache");
        let entries: Vec<_> = std::fs::read_dir(&cache_dir).unwrap().collect();

        for entry in entries {
            let entry = entry.unwrap();
            let filename = entry.file_name();
            let filename_str = filename.to_str().unwrap();
            // No .tmp files should remain
            assert!(!filename_str.ends_with(".tmp"));
        }
    }
}
