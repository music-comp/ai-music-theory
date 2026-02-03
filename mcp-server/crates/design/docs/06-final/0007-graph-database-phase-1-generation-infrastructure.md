---
number: 7
title: "Graph Database Phase 1 - Generation & Infrastructure"
author: "String,"
component: All
tags: [change-me]
created: 2026-01-27
updated: 2026-02-03
state: Final
supersedes: null
superseded-by: null
version: 1.0
---

# Graph Database Phase 1 - Generation & Infrastructure

## Overview

This phase implements the foundation for the concept graph: data model, build pipeline, persistence layer, and basic inspection tools. Phase 2 (separate spec) will add query algorithms and conversational tools.

**Binary name**: `music-theory-mcp`

---

## Goals

1. Define the graph data model (nodes, edges, relationships)
2. Build pipeline to extract relationships from concept cards
3. Persist graph as JSON (source of truth) + rkyv (fast cache)
4. Load graph async on server startup
5. Provide basic inspection/management MCP tools
6. Add CLI commands for graph management

---

## Data Model

### Design: Hybrid Canonical + Source-Specific

We use a two-layer approach:

```
┌─────────────────────────────────────────────────────┐
│                 CANONICAL LAYER                     │
│                                                     │
│   "suspension" (harmonized canonical concept)       │
│                                                     │
└─────────────────────┬───────────────────────────────┘
                      │ same_as edges
        ┌─────────────┼─────────────┐
        ▼             ▼             ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ suspension   │ │ suspension   │ │ suspension   │
│ @omt         │ │ @tymoczko    │ │ @lewin       │
│ SOURCE LAYER │ │ SOURCE LAYER │ │ SOURCE LAYER │
└──────────────┘ └──────────────┘ └──────────────┘
```

- **Canonical concepts**: ~500 harmonized concepts for general queries
- **Source-specific variants**: Preserve how each source frames a concept
- **`SameAs` edges**: Link variants to canonical concepts

### Rust Types

```rust
// ============================================================
// NODE TYPES
// ============================================================

use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

/// A node in the concept graph
#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[derive(SerdeSerialize, SerdeDeserialize)]
#[serde(tag = "type")]
pub enum Node {
    Concept(ConceptNode),
    Source(SourceNode),
}

/// A concept node (either canonical or source-specific)
#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[derive(SerdeSerialize, SerdeDeserialize)]
pub struct ConceptNode {
    /// Unique identifier (e.g., "suspension" or "suspension@lewin")
    pub id: String,

    /// Human-readable title
    pub title: String,

    /// Category (harmony, counterpoint, form, etc.)
    pub category: String,

    /// Source that introduced/defines this concept
    pub source_id: String,

    /// If this is a source-specific variant, the canonical concept ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_id: Option<String>,

    /// Is this a canonical (harmonized) concept?
    #[serde(default)]
    pub is_canonical: bool,
}

/// A source node (book, paper, textbook)
#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[derive(SerdeSerialize, SerdeDeserialize)]
pub struct SourceNode {
    /// Unique identifier (e.g., "oxford-lewin-gmit")
    pub id: String,

    /// Full title
    pub title: String,

    /// Author(s)
    pub author: String,

    /// Publication year
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,

    /// Is this source converted/indexed?
    #[serde(default)]
    pub is_converted: bool,
}

// ============================================================
// EDGE TYPES
// ============================================================

/// An edge in the concept graph
#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[derive(SerdeSerialize, SerdeDeserialize)]
pub struct Edge {
    /// Source node ID
    pub from: String,

    /// Target node ID
    pub to: String,

    /// Type of relationship
    pub relationship: Relationship,

    /// Strength/relevance (0.0 - 1.0)
    #[serde(default = "default_weight")]
    pub weight: f32,

    /// How was this edge created?
    #[serde(default)]
    pub origin: EdgeOrigin,
}

fn default_weight() -> f32 { 1.0 }

/// Types of relationships between nodes
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(SerdeSerialize, SerdeDeserialize)]
#[serde(rename_all = "snake_case")]
pub enum Relationship {
    // Concept-to-Concept
    /// General conceptual relationship
    RelatesTo,
    /// Must understand A before B (A is prerequisite for B)
    Prerequisite,
    /// B builds on / extends A
    Extends,
    /// A and B are the same concept (different names or source variants)
    SameAs,

    // Source-to-Concept
    /// Source introduces/defines this concept
    Introduces,
    /// Source covers/discusses this concept
    Covers,

    // Source-to-Source
    /// Source A cites Source B
    Cites,
}

/// How an edge was created
#[derive(Archive, Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
#[derive(SerdeSerialize, SerdeDeserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeOrigin {
    /// Extracted from concept card "Related Concepts" section
    #[default]
    Extracted,
    /// Human-curated (manual_edges.json)
    Manual,
    /// Auto-inferred (future feature)
    Inferred,
}

// ============================================================
// GRAPH DATA (for serialization)
// ============================================================

/// Complete graph data for serialization
#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[derive(SerdeSerialize, SerdeDeserialize)]
pub struct GraphData {
    /// Schema version
    pub version: String,

    /// All nodes
    pub nodes: Vec<Node>,

    /// All edges
    pub edges: Vec<Edge>,

    /// Build metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<GraphMetadata>,
}

#[derive(Archive, Serialize, Deserialize, Clone, Debug)]
#[derive(SerdeSerialize, SerdeDeserialize)]
pub struct GraphMetadata {
    pub built_at: String,  // ISO 8601
    pub source_cards_count: u32,
    pub build_duration_ms: u64,
}
```

### JSON Schema

```json
{
  "version": "1.0",
  "metadata": {
    "built_at": "2025-01-27T15:30:00Z",
    "source_cards_count": 175,
    "build_duration_ms": 1250
  },
  "nodes": [
    {
      "type": "Concept",
      "id": "suspension",
      "title": "Suspension",
      "category": "harmony",
      "source_id": "open-music-theory",
      "is_canonical": true
    },
    {
      "type": "Concept",
      "id": "suspension@tymoczko",
      "title": "Suspension",
      "category": "voice-leading",
      "source_id": "oxford-tymoczko-geometry",
      "canonical_id": "suspension",
      "is_canonical": false
    },
    {
      "type": "Source",
      "id": "open-music-theory",
      "title": "Open Music Theory",
      "author": "Various",
      "year": 2022,
      "is_converted": true
    }
  ],
  "edges": [
    {
      "from": "fourth-species",
      "to": "suspension",
      "relationship": "introduces",
      "weight": 1.0,
      "origin": "extracted"
    },
    {
      "from": "suspension",
      "to": "dissonance",
      "relationship": "relates_to",
      "weight": 0.8,
      "origin": "extracted"
    },
    {
      "from": "suspension@tymoczko",
      "to": "suspension",
      "relationship": "same_as",
      "weight": 1.0,
      "origin": "extracted"
    }
  ]
}
```

---

## File Structure

```
data/
├── graphs/
│   ├── concept_graph.json      # Source of truth (git-tracked)
│   └── manual_edges.json       # Human-curated overrides (git-tracked, optional)
├── .cache/
│   ├── concept_graph.rkyv      # Compiled binary cache (git-ignored)
│   └── graph_hash              # Cache invalidation hash (git-ignored)
├── indexes/
│   └── tantivy/                # Existing search index
├── concept-cards/
│   └── ...                     # Existing concept cards
└── sources/
    └── ...                     # Converted source texts
```

Add to `.gitignore`:

```
data/.cache/
```

---

## Build Pipeline

### Overview

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Concept Cards  │     │ Sources Config  │     │  Manual Edges   │
│     (.md)       │     │                 │     │    (.json)      │
└────────┬────────┘     └────────┬────────┘     └────────┬────────┘
         │                       │                       │
         │ parse                 │ load                  │ load
         ▼                       ▼                       ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Build Pipeline                           │
│                                                                 │
│  1. Create concept nodes from cards                             │
│  2. Create source nodes from config                             │
│  3. Extract edges from "Related Concepts" sections              │
│  4. Create source→concept "introduces" edges                    │
│  5. Merge manual edges (override duplicates)                    │
│  6. Validate references                                         │
│  7. Deduplicate edges                                           │
│                                                                 │
└────────────────────────────┬────────────────────────────────────┘
                             │
                             ▼
                 ┌───────────────────────┐
                 │  concept_graph.json   │
                 │   (source of truth)   │
                 └───────────────────────┘
```

### Implementation

```rust
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

pub struct GraphBuilder {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    node_ids: HashSet<String>,
    warnings: Vec<String>,
}

impl GraphBuilder {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_ids: HashSet::new(),
            warnings: Vec::new(),
        }
    }

    /// Build graph from concept cards and sources
    pub fn build(data_dir: &Path) -> Result<(GraphData, Vec<String>)> {
        let start = Instant::now();
        let mut builder = Self::new();

        // 1. Load source configuration
        let sources = load_sources_config(data_dir)?;
        for source in &sources {
            builder.add_source_node(source);
        }

        // 2. Load and process concept cards
        let cards = load_concept_cards(data_dir)?;
        for card in &cards {
            builder.add_concept_node(card);
        }

        // 3. Extract relationships from concept cards
        for card in &cards {
            builder.extract_relationships(card);
        }

        // 4. Create source→concept "introduces" edges
        for card in &cards {
            builder.add_introduces_edge(card);
        }

        // 5. Load and merge manual edges
        let manual_path = data_dir.join("graphs/manual_edges.json");
        if manual_path.exists() {
            builder.merge_manual_edges(&manual_path)?;
        }

        // 6. Deduplicate edges
        builder.deduplicate_edges();

        let duration = start.elapsed();

        let graph_data = GraphData {
            version: "1.0".to_string(),
            nodes: builder.nodes,
            edges: builder.edges,
            metadata: Some(GraphMetadata {
                built_at: chrono::Utc::now().to_rfc3339(),
                source_cards_count: cards.len() as u32,
                build_duration_ms: duration.as_millis() as u64,
            }),
        };

        Ok((graph_data, builder.warnings))
    }

    fn add_source_node(&mut self, source: &SourceConfig) {
        let node = Node::Source(SourceNode {
            id: source.id.clone(),
            title: source.title.clone(),
            author: source.author.clone().unwrap_or_default(),
            year: source.year,
            is_converted: source.status == "converted",
        });
        self.node_ids.insert(source.id.clone());
        self.nodes.push(node);
    }

    fn add_concept_node(&mut self, card: &ConceptCard) {
        let node = Node::Concept(ConceptNode {
            id: card.id.clone(),
            title: card.title.clone(),
            category: card.category.clone(),
            source_id: card.source.clone(),
            canonical_id: None,
            is_canonical: true,  // For now, all are canonical
        });
        self.node_ids.insert(card.id.clone());
        self.nodes.push(node);
    }

    fn extract_relationships(&mut self, card: &ConceptCard) {
        let Some(related) = &card.related_concepts else { return };

        // Prerequisites: these concepts are prerequisites FOR this card
        for prereq_id in &related.prerequisite {
            if self.node_ids.contains(prereq_id) {
                self.edges.push(Edge {
                    from: prereq_id.clone(),
                    to: card.id.clone(),
                    relationship: Relationship::Prerequisite,
                    weight: 1.0,
                    origin: EdgeOrigin::Extracted,
                });
            } else {
                self.warnings.push(format!(
                    "Concept '{}' references unknown prerequisite '{}'",
                    card.id, prereq_id
                ));
            }
        }

        // Leads to: this card is prerequisite FOR these concepts
        for leads_to_id in &related.leads_to {
            if self.node_ids.contains(leads_to_id) {
                self.edges.push(Edge {
                    from: card.id.clone(),
                    to: leads_to_id.clone(),
                    relationship: Relationship::Prerequisite,
                    weight: 1.0,
                    origin: EdgeOrigin::Extracted,
                });
            } else {
                self.warnings.push(format!(
                    "Concept '{}' references unknown leads_to '{}'",
                    card.id, leads_to_id
                ));
            }
        }

        // See also: general relationship (lower weight)
        for see_also_id in &related.see_also {
            if self.node_ids.contains(see_also_id) {
                self.edges.push(Edge {
                    from: card.id.clone(),
                    to: see_also_id.clone(),
                    relationship: Relationship::RelatesTo,
                    weight: 0.7,
                    origin: EdgeOrigin::Extracted,
                });
            }
            // Don't warn for see_also - these are softer references
        }
    }

    fn add_introduces_edge(&mut self, card: &ConceptCard) {
        if self.node_ids.contains(&card.source) {
            self.edges.push(Edge {
                from: card.source.clone(),
                to: card.id.clone(),
                relationship: Relationship::Introduces,
                weight: 1.0,
                origin: EdgeOrigin::Extracted,
            });
        }
    }

    fn merge_manual_edges(&mut self, path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        let manual: ManualEdgesFile = serde_json::from_str(&content)?;

        for edge in manual.edges {
            // Validate references
            if !self.node_ids.contains(&edge.from) {
                self.warnings.push(format!(
                    "Manual edge references unknown 'from' node: '{}'",
                    edge.from
                ));
                continue;
            }
            if !self.node_ids.contains(&edge.to) {
                self.warnings.push(format!(
                    "Manual edge references unknown 'to' node: '{}'",
                    edge.to
                ));
                continue;
            }

            // Remove any existing edge between these nodes with same relationship
            self.edges.retain(|e| {
                !(e.from == edge.from && e.to == edge.to && e.relationship == edge.relationship)
            });

            // Add manual edge
            self.edges.push(Edge {
                from: edge.from,
                to: edge.to,
                relationship: edge.relationship,
                weight: edge.weight.unwrap_or(1.0),
                origin: EdgeOrigin::Manual,
            });
        }

        Ok(())
    }

    fn deduplicate_edges(&mut self) {
        // Keep highest weight if duplicate edges exist
        let mut seen: HashMap<(String, String, Relationship), usize> = HashMap::new();
        let mut to_remove: Vec<usize> = Vec::new();

        for (i, edge) in self.edges.iter().enumerate() {
            let key = (edge.from.clone(), edge.to.clone(), edge.relationship.clone());

            if let Some(&existing_idx) = seen.get(&key) {
                // Keep the one with higher weight
                if edge.weight > self.edges[existing_idx].weight {
                    to_remove.push(existing_idx);
                    seen.insert(key, i);
                } else {
                    to_remove.push(i);
                }
            } else {
                seen.insert(key, i);
            }
        }

        // Remove duplicates (in reverse order to preserve indices)
        to_remove.sort_by(|a, b| b.cmp(a));
        for idx in to_remove {
            self.edges.remove(idx);
        }
    }
}

/// Manual edges file format
#[derive(SerdeDeserialize)]
struct ManualEdgesFile {
    #[serde(default)]
    description: String,
    edges: Vec<ManualEdge>,
}

#[derive(SerdeDeserialize)]
struct ManualEdge {
    from: String,
    to: String,
    relationship: Relationship,
    weight: Option<f32>,
    #[serde(default)]
    note: String,
}
```

---

## Persistence Layer

### rkyv + mmap Caching

```rust
use blake3::Hasher;
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

/// Load graph, using cache if valid
pub async fn load_concept_graph(data_dir: &Path) -> Result<ConceptGraph> {
    let json_path = data_dir.join("graphs/concept_graph.json");
    let cache_dir = data_dir.join(".cache");
    let cache_path = cache_dir.join("concept_graph.rkyv");
    let hash_path = cache_dir.join("graph_hash");

    // Ensure cache directory exists
    tokio::fs::create_dir_all(&cache_dir).await?;

    // Check if source JSON exists
    if !json_path.exists() {
        return Err(anyhow::anyhow!(
            "concept_graph.json not found. Run `music-theory-mcp graph build` first."
        ));
    }

    // Hash the source JSON
    let json_bytes = tokio::fs::read(&json_path).await?;
    let current_hash = blake3::hash(&json_bytes);
    let current_hash_hex = current_hash.to_hex().to_string();

    // Check cache validity
    let cache_valid = is_cache_valid(&cache_path, &hash_path, &current_hash_hex).await;

    if cache_valid {
        tracing::info!("Graph cache valid, loading via mmap...");
        match load_from_rkyv(&cache_path).await {
            Ok(graph) => return Ok(graph),
            Err(e) => {
                tracing::warn!("Failed to load cache, rebuilding: {}", e);
            }
        }
    }

    // Cache miss or invalid - parse JSON
    tracing::info!("Building graph from JSON...");
    let graph_data: GraphData = serde_json::from_slice(&json_bytes)?;
    let graph = graph_data.to_petgraph();

    // Write new cache
    if let Err(e) = save_cache(&graph_data, &cache_path, &hash_path, &current_hash_hex).await {
        tracing::warn!("Failed to save cache: {}", e);
    }

    Ok(graph)
}

async fn is_cache_valid(cache_path: &Path, hash_path: &Path, expected_hash: &str) -> bool {
    if !cache_path.exists() || !hash_path.exists() {
        return false;
    }

    match tokio::fs::read_to_string(hash_path).await {
        Ok(stored_hash) => stored_hash.trim() == expected_hash,
        Err(_) => false,
    }
}

async fn load_from_rkyv(path: &Path) -> Result<ConceptGraph> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };

    // Validate and access archived data
    let archived = unsafe {
        rkyv::archived_root::<GraphData>(&mmap)
    };

    // Convert to petgraph
    Ok(archived_to_petgraph(archived))
}

async fn save_cache(
    graph_data: &GraphData,
    cache_path: &Path,
    hash_path: &Path,
    hash: &str,
) -> Result<()> {
    // Serialize to rkyv
    let bytes = rkyv::to_bytes::<_, 4096>(graph_data)
        .map_err(|e| anyhow::anyhow!("rkyv serialization failed: {}", e))?;

    // Write atomically
    let tmp_cache = cache_path.with_extension("rkyv.tmp");
    tokio::fs::write(&tmp_cache, &bytes).await?;
    tokio::fs::rename(&tmp_cache, cache_path).await?;

    // Write hash
    tokio::fs::write(hash_path, hash).await?;

    tracing::info!("Graph cache saved ({} bytes)", bytes.len());
    Ok(())
}

/// Convert archived graph data to petgraph (zero-copy where possible)
fn archived_to_petgraph(archived: &ArchivedGraphData) -> ConceptGraph {
    use petgraph::graph::DiGraph;

    let mut graph = DiGraph::new();
    let mut node_indices: HashMap<String, petgraph::graph::NodeIndex> = HashMap::new();

    // Add nodes
    for node in archived.nodes.iter() {
        let owned_node: Node = node.deserialize(&mut rkyv::Infallible).unwrap();
        let id = match &owned_node {
            Node::Concept(c) => c.id.clone(),
            Node::Source(s) => s.id.clone(),
        };
        let idx = graph.add_node(owned_node);
        node_indices.insert(id, idx);
    }

    // Add edges
    for edge in archived.edges.iter() {
        let owned_edge: Edge = edge.deserialize(&mut rkyv::Infallible).unwrap();
        if let (Some(&from_idx), Some(&to_idx)) = (
            node_indices.get(&owned_edge.from),
            node_indices.get(&owned_edge.to),
        ) {
            graph.add_edge(from_idx, to_idx, owned_edge);
        }
    }

    graph
}
```

---

## Async Loading

### AppState Integration

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use petgraph::graph::DiGraph;

pub type ConceptGraph = DiGraph<Node, Edge>;

pub struct AppState {
    // Existing fields
    pub concepts: ConceptIndex,
    pub search_index: RwLock<Option<TantivyIndex>>,

    // New: concept graph
    #[cfg(feature = "graph")]
    pub graph: RwLock<GraphState>,
}

#[cfg(feature = "graph")]
pub enum GraphState {
    /// Graph not yet loaded
    NotLoaded,
    /// Graph is currently loading
    Loading,
    /// Graph loaded successfully
    Loaded(LoadedGraph),
    /// Graph failed to load
    Failed(String),
}

#[cfg(feature = "graph")]
pub struct LoadedGraph {
    pub graph: ConceptGraph,
    pub node_index: HashMap<String, petgraph::graph::NodeIndex>,
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    pub stats: GraphStats,
}

#[cfg(feature = "graph")]
pub struct GraphStats {
    pub node_count: u32,
    pub edge_count: u32,
    pub concept_count: u32,
    pub source_count: u32,
}

impl AppState {
    pub async fn initialize(data_dir: PathBuf) -> Arc<Self> {
        let state = Arc::new(Self {
            concepts: ConceptIndex::load(&data_dir),
            search_index: RwLock::new(None),
            #[cfg(feature = "graph")]
            graph: RwLock::new(GraphState::NotLoaded),
        });

        // Spawn async loaders
        #[cfg(feature = "search")]
        {
            let s = state.clone();
            let d = data_dir.clone();
            tokio::spawn(async move {
                let index = load_tantivy_index(&d).await;
                *s.search_index.write().await = Some(index);
                tracing::info!("Search index ready");
            });
        }

        #[cfg(feature = "graph")]
        {
            let s = state.clone();
            let d = data_dir.clone();
            tokio::spawn(async move {
                *s.graph.write().await = GraphState::Loading;

                match load_concept_graph(&d).await {
                    Ok(graph) => {
                        let stats = compute_graph_stats(&graph);
                        let node_index = build_node_index(&graph);
                        *s.graph.write().await = GraphState::Loaded(LoadedGraph {
                            graph,
                            node_index,
                            loaded_at: chrono::Utc::now(),
                            stats,
                        });
                        tracing::info!("Concept graph ready");
                    }
                    Err(e) => {
                        tracing::error!("Failed to load concept graph: {}", e);
                        *s.graph.write().await = GraphState::Failed(e.to_string());
                    }
                }
            });
        }

        state
    }
}

fn build_node_index(graph: &ConceptGraph) -> HashMap<String, petgraph::graph::NodeIndex> {
    let mut index = HashMap::new();
    for idx in graph.node_indices() {
        let id = match &graph[idx] {
            Node::Concept(c) => c.id.clone(),
            Node::Source(s) => s.id.clone(),
        };
        index.insert(id, idx);
    }
    index
}

fn compute_graph_stats(graph: &ConceptGraph) -> GraphStats {
    let mut concept_count = 0;
    let mut source_count = 0;

    for node in graph.node_weights() {
        match node {
            Node::Concept(_) => concept_count += 1,
            Node::Source(_) => source_count += 1,
        }
    }

    GraphStats {
        node_count: graph.node_count() as u32,
        edge_count: graph.edge_count() as u32,
        concept_count,
        source_count,
    }
}
```

---

## CLI Commands

Add to the existing CLI:

```rust
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    // Existing commands...

    /// Graph database management
    #[cfg(feature = "graph")]
    Graph(GraphCommands),
}

#[cfg(feature = "graph")]
#[derive(Parser)]
pub struct GraphCommands {
    #[command(subcommand)]
    command: GraphSubcommand,
}

#[cfg(feature = "graph")]
#[derive(Subcommand)]
pub enum GraphSubcommand {
    /// Build graph from concept cards
    Build {
        /// Show what would change without writing
        #[arg(long)]
        dry_run: bool,

        /// Show detailed output
        #[arg(long, short)]
        verbose: bool,
    },

    /// Validate graph integrity
    Validate,

    /// Show graph statistics
    Stats,

    /// Rebuild rkyv cache from JSON
    Compile,
}

#[cfg(feature = "graph")]
pub async fn handle_graph_command(cmd: GraphSubcommand, data_dir: &Path) -> Result<()> {
    match cmd {
        GraphSubcommand::Build { dry_run, verbose } => {
            println!("Building concept graph from cards...");

            let (graph_data, warnings) = GraphBuilder::build(data_dir)?;

            // Show warnings
            for warning in &warnings {
                println!("  ⚠️  {}", warning);
            }

            if verbose {
                println!("\nNodes: {}", graph_data.nodes.len());
                println!("Edges: {}", graph_data.edges.len());
            }

            if dry_run {
                println!("\n(dry run - no files written)");
            } else {
                let json_path = data_dir.join("graphs/concept_graph.json");
                std::fs::create_dir_all(json_path.parent().unwrap())?;

                let json = serde_json::to_string_pretty(&graph_data)?;
                std::fs::write(&json_path, &json)?;

                println!("\n✓ Wrote {}", json_path.display());

                // Invalidate cache
                let cache_path = data_dir.join(".cache/concept_graph.rkyv");
                if cache_path.exists() {
                    std::fs::remove_file(&cache_path)?;
                    println!("✓ Invalidated cache");
                }
            }

            Ok(())
        }

        GraphSubcommand::Validate => {
            let json_path = data_dir.join("graphs/concept_graph.json");
            let content = std::fs::read_to_string(&json_path)?;
            let graph_data: GraphData = serde_json::from_str(&content)?;

            let validation = validate_graph(&graph_data);

            if validation.is_valid {
                println!("✓ Graph is valid");
            } else {
                println!("✗ Graph has issues:");
            }

            if !validation.orphan_nodes.is_empty() {
                println!("\n  Orphan nodes (no relationships):");
                for id in &validation.orphan_nodes {
                    println!("    - {}", id);
                }
            }

            if !validation.broken_references.is_empty() {
                println!("\n  Broken references:");
                for br in &validation.broken_references {
                    println!("    - {} -> {} ({})", br.from, br.to, br.error);
                }
            }

            if validation.self_loops > 0 {
                println!("\n  Self-loops: {}", validation.self_loops);
            }

            Ok(())
        }

        GraphSubcommand::Stats => {
            let json_path = data_dir.join("graphs/concept_graph.json");
            let content = std::fs::read_to_string(&json_path)?;
            let graph_data: GraphData = serde_json::from_str(&content)?;

            let stats = compute_stats(&graph_data);

            println!("Graph Statistics");
            println!("================");
            println!("Nodes:    {}", stats.total_nodes);
            println!("  Concepts: {}", stats.concept_count);
            println!("  Sources:  {}", stats.source_count);
            println!("Edges:    {}", stats.total_edges);
            println!("\nRelationship types:");
            for (rel, count) in &stats.relationships {
                println!("  {:?}: {}", rel, count);
            }
            println!("\nCategories:");
            for (cat, count) in &stats.categories {
                println!("  {}: {}", cat, count);
            }

            if let Some(meta) = &graph_data.metadata {
                println!("\nBuilt: {}", meta.built_at);
            }

            Ok(())
        }

        GraphSubcommand::Compile => {
            println!("Rebuilding rkyv cache...");

            let json_path = data_dir.join("graphs/concept_graph.json");
            let content = std::fs::read_to_string(&json_path)?;
            let graph_data: GraphData = serde_json::from_str(&content)?;

            let cache_dir = data_dir.join(".cache");
            std::fs::create_dir_all(&cache_dir)?;

            let cache_path = cache_dir.join("concept_graph.rkyv");
            let hash_path = cache_dir.join("graph_hash");

            let bytes = rkyv::to_bytes::<_, 4096>(&graph_data)?;
            std::fs::write(&cache_path, &bytes)?;

            let hash = blake3::hash(content.as_bytes());
            std::fs::write(&hash_path, hash.to_hex().as_str())?;

            println!("✓ Cache rebuilt ({} bytes)", bytes.len());

            Ok(())
        }
    }
}
```

---

## MCP Tools (Phase 1)

### Tool: `graph_status`

```rust
/// Check graph loading status and basic info
#[tool(
    name = "graph_status",
    description = "Check if the concept graph is loaded and get basic status information"
)]
pub async fn graph_status(
    state: Arc<AppState>,
) -> Result<GraphStatusResponse, McpError> {
    let graph_state = state.graph.read().await;

    match &*graph_state {
        GraphState::NotLoaded => Ok(GraphStatusResponse {
            status: "not_loaded".into(),
            loaded: false,
            loading: false,
            node_count: None,
            edge_count: None,
            loaded_at: None,
            message: Some("Graph has not been loaded yet".into()),
        }),

        GraphState::Loading => Ok(GraphStatusResponse {
            status: "loading".into(),
            loaded: false,
            loading: true,
            node_count: None,
            edge_count: None,
            loaded_at: None,
            message: Some("Graph is currently loading...".into()),
        }),

        GraphState::Loaded(lg) => Ok(GraphStatusResponse {
            status: "loaded".into(),
            loaded: true,
            loading: false,
            node_count: Some(lg.stats.node_count),
            edge_count: Some(lg.stats.edge_count),
            loaded_at: Some(lg.loaded_at.to_rfc3339()),
            message: None,
        }),

        GraphState::Failed(err) => Ok(GraphStatusResponse {
            status: "failed".into(),
            loaded: false,
            loading: false,
            node_count: None,
            edge_count: None,
            loaded_at: None,
            message: Some(format!("Failed to load: {}", err)),
        }),
    }
}

#[derive(Serialize)]
pub struct GraphStatusResponse {
    status: String,
    loaded: bool,
    loading: bool,
    node_count: Option<u32>,
    edge_count: Option<u32>,
    loaded_at: Option<String>,
    message: Option<String>,
}
```

### Tool: `graph_stats`

```rust
/// Get detailed graph statistics
#[tool(
    name = "graph_stats",
    description = "Get detailed statistics about the concept graph"
)]
pub async fn graph_stats(
    state: Arc<AppState>,
) -> Result<GraphStatsResponse, McpError> {
    let graph_state = state.graph.read().await;

    let GraphState::Loaded(lg) = &*graph_state else {
        return Err(McpError::ServiceUnavailable {
            message: "Graph not loaded".into(),
        });
    };

    // Count by category
    let mut categories: HashMap<String, u32> = HashMap::new();
    let mut relationships: HashMap<String, u32> = HashMap::new();
    let mut orphan_count = 0;

    for idx in lg.graph.node_indices() {
        if let Node::Concept(c) = &lg.graph[idx] {
            *categories.entry(c.category.clone()).or_insert(0) += 1;
        }

        // Check for orphans (no edges)
        if lg.graph.edges(idx).count() == 0 {
            orphan_count += 1;
        }
    }

    for edge in lg.graph.edge_weights() {
        let rel_name = format!("{:?}", edge.relationship);
        *relationships.entry(rel_name).or_insert(0) += 1;
    }

    Ok(GraphStatsResponse {
        total_nodes: lg.stats.node_count,
        total_edges: lg.stats.edge_count,
        concept_count: lg.stats.concept_count,
        source_count: lg.stats.source_count,
        categories,
        relationships,
        orphan_nodes: orphan_count,
        loaded_at: lg.loaded_at.to_rfc3339(),
    })
}

#[derive(Serialize)]
pub struct GraphStatsResponse {
    total_nodes: u32,
    total_edges: u32,
    concept_count: u32,
    source_count: u32,
    categories: HashMap<String, u32>,
    relationships: HashMap<String, u32>,
    orphan_nodes: u32,
    loaded_at: String,
}
```

### Tool: `graph_validate`

```rust
/// Validate graph integrity
#[tool(
    name = "graph_validate",
    description = "Check graph for broken references, orphans, and other issues"
)]
pub async fn graph_validate(
    state: Arc<AppState>,
) -> Result<GraphValidateResponse, McpError> {
    let graph_state = state.graph.read().await;

    let GraphState::Loaded(lg) = &*graph_state else {
        return Err(McpError::ServiceUnavailable {
            message: "Graph not loaded".into(),
        });
    };

    let mut orphan_nodes = Vec::new();
    let mut self_loops = 0;

    for idx in lg.graph.node_indices() {
        // Check for orphans
        let in_degree = lg.graph.edges_directed(idx, petgraph::Direction::Incoming).count();
        let out_degree = lg.graph.edges_directed(idx, petgraph::Direction::Outgoing).count();

        if in_degree == 0 && out_degree == 0 {
            let id = match &lg.graph[idx] {
                Node::Concept(c) => c.id.clone(),
                Node::Source(s) => s.id.clone(),
            };
            orphan_nodes.push(id);
        }
    }

    // Check for self-loops
    for edge in lg.graph.edge_references() {
        if edge.source() == edge.target() {
            self_loops += 1;
        }
    }

    let is_valid = orphan_nodes.is_empty() && self_loops == 0;

    Ok(GraphValidateResponse {
        valid: is_valid,
        orphan_nodes,
        self_loops,
        warnings: Vec::new(),
    })
}

#[derive(Serialize)]
pub struct GraphValidateResponse {
    valid: bool,
    orphan_nodes: Vec<String>,
    self_loops: u32,
    warnings: Vec<String>,
}
```

### Tool: `get_node`

```rust
/// Get raw node data by ID
#[tool(
    name = "get_node",
    description = "Get detailed information about a specific node (concept or source)"
)]
pub async fn get_node(
    state: Arc<AppState>,
    #[arg(description = "Node ID to look up")]
    node_id: String,
) -> Result<GetNodeResponse, McpError> {
    let graph_state = state.graph.read().await;

    let GraphState::Loaded(lg) = &*graph_state else {
        return Err(McpError::ServiceUnavailable {
            message: "Graph not loaded".into(),
        });
    };

    let Some(&idx) = lg.node_index.get(&node_id) else {
        return Ok(GetNodeResponse {
            found: false,
            node: None,
            in_degree: 0,
            out_degree: 0,
        });
    };

    let node = lg.graph[idx].clone();
    let in_degree = lg.graph.edges_directed(idx, petgraph::Direction::Incoming).count() as u32;
    let out_degree = lg.graph.edges_directed(idx, petgraph::Direction::Outgoing).count() as u32;

    Ok(GetNodeResponse {
        found: true,
        node: Some(node),
        in_degree,
        out_degree,
    })
}

#[derive(Serialize)]
pub struct GetNodeResponse {
    found: bool,
    node: Option<Node>,
    in_degree: u32,
    out_degree: u32,
}
```

### Tool: `get_node_edges`

```rust
/// Get all edges for a specific node
#[tool(
    name = "get_node_edges",
    description = "Get all edges connected to a specific node"
)]
pub async fn get_node_edges(
    state: Arc<AppState>,
    #[arg(description = "Node ID to get edges for")]
    node_id: String,
    #[arg(description = "Filter by direction: incoming, outgoing, or both (default: both)")]
    direction: Option<String>,
) -> Result<GetNodeEdgesResponse, McpError> {
    let graph_state = state.graph.read().await;

    let GraphState::Loaded(lg) = &*graph_state else {
        return Err(McpError::ServiceUnavailable {
            message: "Graph not loaded".into(),
        });
    };

    let Some(&idx) = lg.node_index.get(&node_id) else {
        return Ok(GetNodeEdgesResponse {
            found: false,
            node_id,
            edges: Vec::new(),
        });
    };

    let mut edges = Vec::new();
    let dir = direction.as_deref().unwrap_or("both");

    // Helper to get node ID from index
    let get_id = |i: petgraph::graph::NodeIndex| -> String {
        match &lg.graph[i] {
            Node::Concept(c) => c.id.clone(),
            Node::Source(s) => s.id.clone(),
        }
    };

    let get_title = |i: petgraph::graph::NodeIndex| -> String {
        match &lg.graph[i] {
            Node::Concept(c) => c.title.clone(),
            Node::Source(s) => s.title.clone(),
        }
    };

    if dir == "incoming" || dir == "both" {
        for edge in lg.graph.edges_directed(idx, petgraph::Direction::Incoming) {
            edges.push(EdgeDetail {
                from_id: get_id(edge.source()),
                from_title: get_title(edge.source()),
                to_id: get_id(edge.target()),
                to_title: get_title(edge.target()),
                relationship: edge.weight().relationship.clone(),
                weight: edge.weight().weight,
                origin: edge.weight().origin.clone(),
                direction: "incoming".into(),
            });
        }
    }

    if dir == "outgoing" || dir == "both" {
        for edge in lg.graph.edges_directed(idx, petgraph::Direction::Outgoing) {
            edges.push(EdgeDetail {
                from_id: get_id(edge.source()),
                from_title: get_title(edge.source()),
                to_id: get_id(edge.target()),
                to_title: get_title(edge.target()),
                relationship: edge.weight().relationship.clone(),
                weight: edge.weight().weight,
                origin: edge.weight().origin.clone(),
                direction: "outgoing".into(),
            });
        }
    }

    Ok(GetNodeEdgesResponse {
        found: true,
        node_id,
        edges,
    })
}

#[derive(Serialize)]
pub struct GetNodeEdgesResponse {
    found: bool,
    node_id: String,
    edges: Vec<EdgeDetail>,
}

#[derive(Serialize)]
pub struct EdgeDetail {
    from_id: String,
    from_title: String,
    to_id: String,
    to_title: String,
    relationship: Relationship,
    weight: f32,
    origin: EdgeOrigin,
    direction: String,
}
```

---

## Feature Gate

```toml
# Cargo.toml
[features]
default = ["search"]
search = ["tantivy"]
graph = ["dep:petgraph", "dep:rkyv", "dep:memmap2", "dep:blake3"]
full = ["search", "graph"]

[dependencies]
petgraph = { version = "0.6", optional = true }
rkyv = { version = "0.7", features = ["validation"], optional = true }
memmap2 = { version = "0.9", optional = true }
blake3 = { version = "1.5", optional = true }

# Always needed
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
anyhow = "1.0"
```

---

## Success Criteria

### Must Have (Phase 1 Complete)

- [ ] Data model defined (Node, Edge, Relationship types)
- [ ] `music-theory-mcp graph build` creates `concept_graph.json` from concept cards
- [ ] Graph loads async on server startup with rkyv caching
- [ ] Cache invalidation works (hash-based)
- [ ] `graph_status` tool returns accurate loading state
- [ ] `graph_stats` tool shows node/edge counts and categories
- [ ] `get_node` and `get_node_edges` work for inspection
- [ ] Feature-gated with `--features graph`

### Should Have

- [ ] `graph_validate` detects broken references and orphans
- [ ] `music-theory-mcp graph validate` CLI command
- [ ] `music-theory-mcp graph stats` CLI command
- [ ] Warnings for broken references during build
- [ ] Manual edges override support (`manual_edges.json`)

### Nice to Have

- [ ] `music-theory-mcp graph compile` to manually rebuild cache
- [ ] Dry-run mode for build command
- [ ] Verbose output showing extracted relationships
