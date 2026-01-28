//! Graph builder for extracting relationships from concept cards.
//!
//! This module provides the GraphBuilder which constructs a concept graph by:
//! 1. Loading source metadata from configuration
//! 2. Scanning and parsing concept cards
//! 3. Extracting relationships from "Related Concepts" sections
//! 4. Creating edges between nodes
//! 5. Merging manual edges from optional override file
//! 6. Deduplicating edges

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

use crate::config::Config;
use crate::error::{Error, Result};
use crate::metadata::extract_concept_metadata;
use crate::util::files::{find_all_files, FindOptions};

use super::parser::parse_related_concepts;
use super::types::{
    ConceptNode, Edge, EdgeOrigin, GraphData, GraphMetadata, Node, Relationship, SourceNode,
};

/// Builder for constructing the concept graph.
pub struct GraphBuilder {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    node_ids: HashSet<String>,
    source_title_to_id: HashMap<String, String>,
    warnings: Vec<String>,
}

impl GraphBuilder {
    /// Create a new GraphBuilder.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_ids: HashSet::new(),
            source_title_to_id: HashMap::new(),
            warnings: Vec::new(),
        }
    }

    /// Build the concept graph from concept cards and configuration.
    ///
    /// This performs the complete build pipeline:
    /// 1. Load sources from configuration
    /// 2. Scan concept cards directory
    /// 3. Extract relationships from cards
    /// 4. Create "introduces" edges from sources to concepts
    /// 5. Merge manual edges (if file exists)
    /// 6. Deduplicate edges
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration
    ///
    /// # Returns
    ///
    /// Returns tuple of (GraphData, warnings) where warnings contains any issues
    /// encountered during the build (e.g., broken references).
    ///
    /// # Errors
    ///
    /// Returns error if configuration loading, file scanning, or parsing fails.
    pub async fn build(config: &Config) -> Result<(GraphData, Vec<String>)> {
        let start = Instant::now();
        let mut builder = Self::new();

        // Step 1: Load source nodes from configuration
        log::info!("Loading source nodes from configuration");
        builder.add_source_nodes(config)?;

        // Step 2: Load concept cards and create concept nodes
        log::info!("Scanning concept cards directory");
        let cards = builder.load_concept_cards(config).await?;
        log::info!("Found {} concept cards", cards.len());

        for card in &cards {
            builder.add_concept_node(card);
        }

        // Step 3: Extract relationships and create edges
        log::info!("Extracting relationships from concept cards");
        for card in &cards {
            builder.extract_relationships(card);
        }

        // Step 4: Create "introduces" edges from sources to concepts
        log::info!("Creating source-to-concept edges");
        for card in &cards {
            builder.add_introduces_edge(card);
        }

        // Step 5: Merge manual edges (if file exists)
        let manual_edges_path = Path::new(&config.paths.base)
            .join("data")
            .join("graphs")
            .join("manual_edges.json");

        if manual_edges_path.exists() {
            log::info!("Merging manual edges from {}", manual_edges_path.display());
            builder.merge_manual_edges(&manual_edges_path)?;
        }

        // Step 6: Deduplicate edges
        log::info!("Deduplicating edges");
        builder.deduplicate_edges();

        let duration = start.elapsed();

        log::info!(
            "Graph build complete: {} nodes, {} edges, {} warnings, {}ms",
            builder.nodes.len(),
            builder.edges.len(),
            builder.warnings.len(),
            duration.as_millis()
        );

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

    /// Add source nodes from configuration.
    fn add_source_nodes(&mut self, config: &Config) -> Result<()> {
        // Process each source category (oxford, general, papers)
        for (category, source_cat) in [
            ("oxford", &config.sources.oxford),
            ("general", &config.sources.general),
            ("papers", &config.sources.papers),
        ] {
            for (file_id, filename) in &source_cat.files {
                let source_id = format!("{}-{}", category, file_id);

                // Extract author and year from filename
                let (author, year) = parse_source_filename(filename);

                // Check if source is converted (exists in sources-md directory)
                let sources_md_path = config.paths.sources_md_path()?;
                let converted_path = sources_md_path.join(&source_id);
                let is_converted = converted_path.exists() && converted_path.is_dir();

                let title = extract_title_from_filename(filename);

                let node = Node::Source(SourceNode {
                    id: source_id.clone(),
                    title: title.clone(),
                    author,
                    year,
                    is_converted,
                });

                self.node_ids.insert(source_id.clone());
                self.source_title_to_id.insert(title, source_id);
                self.nodes.push(node);
            }
        }

        Ok(())
    }

    /// Load concept cards from the configured directory.
    async fn load_concept_cards(&self, config: &Config) -> Result<Vec<ConceptCard>> {
        let base_path = config.paths.base_path()?;
        let cards_dir = config.paths.concept_cards_path()?;

        let mut cards = Vec::new();

        // Find all markdown files
        let file_infos = find_all_files(&cards_dir, FindOptions::markdown()).await?;

        for file_info in file_infos {
            // Extract metadata
            let metadata = extract_concept_metadata(&base_path, &file_info.path).await?;

            // Read file content to parse relationships
            let content = std::fs::read_to_string(&file_info.path)
                .map_err(|e| Error::io_with_path(e, &file_info.path))?;

            // Parse related concepts section
            let related_concepts = parse_related_concepts(&content);

            cards.push(ConceptCard {
                id: metadata.id,
                title: metadata.title,
                category: metadata.category,
                source: metadata.source.unwrap_or_else(|| "unknown".to_string()),
                related_concepts,
            });
        }

        Ok(cards)
    }

    /// Add a concept node to the graph.
    fn add_concept_node(&mut self, card: &ConceptCard) {
        let node = Node::Concept(ConceptNode {
            id: card.id.clone(),
            title: card.title.clone(),
            category: card.category.clone(),
            source_id: card.source.clone(),
            canonical_id: None,
            is_canonical: true, // For now, all are canonical
        });

        self.node_ids.insert(card.id.clone());
        self.nodes.push(node);
    }

    /// Extract relationships from a concept card and create edges.
    fn extract_relationships(&mut self, card: &ConceptCard) {
        let Some(related) = &card.related_concepts else {
            return;
        };

        // Prerequisites: these concepts are prerequisites FOR this card
        // Edge direction: from prereq → to card
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
        // Edge direction: from card → to leads_to
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
        // Edge direction: from card → to see_also
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

    /// Add an "introduces" edge from the source to the concept.
    fn add_introduces_edge(&mut self, card: &ConceptCard) {
        // Try to find source by ID first, then by title
        let source_id = if self.node_ids.contains(&card.source) {
            Some(card.source.clone())
        } else {
            self.source_title_to_id.get(&card.source).cloned()
        };

        if let Some(source_id) = source_id {
            self.edges.push(Edge {
                from: source_id,
                to: card.id.clone(),
                relationship: Relationship::Introduces,
                weight: 1.0,
                origin: EdgeOrigin::Extracted,
            });
        } else {
            // Only warn if source is not "unknown"
            if card.source != "unknown" {
                self.warnings.push(format!(
                    "Concept '{}' references unknown source '{}'",
                    card.id, card.source
                ));
            }
        }
    }

    /// Merge manual edges from a JSON file.
    ///
    /// Manual edges override any existing edges with the same (from, to, relationship) tuple.
    fn merge_manual_edges(&mut self, path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(path).map_err(|e| Error::io_with_path(e, path))?;

        let manual: ManualEdgesFile = serde_json::from_str(&content)
            .map_err(|e| Error::config(format!("Failed to parse manual_edges.json: {}", e)))?;

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

    /// Deduplicate edges, keeping the one with highest weight.
    fn deduplicate_edges(&mut self) {
        let mut seen: HashMap<(String, String, Relationship), usize> = HashMap::new();
        let mut to_remove: Vec<usize> = Vec::new();

        for (i, edge) in self.edges.iter().enumerate() {
            let key = (
                edge.from.clone(),
                edge.to.clone(),
                edge.relationship.clone(),
            );

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
        to_remove.dedup(); // Remove any duplicate indices from to_remove list
        for idx in to_remove {
            self.edges.remove(idx);
        }
    }
}

impl Default for GraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Intermediate representation of a concept card with relationships.
#[derive(Debug, Clone)]
struct ConceptCard {
    id: String,
    title: String,
    category: String,
    source: String,
    related_concepts: Option<crate::graph::parser::RelatedConcepts>,
}

/// Manual edges file format.
#[derive(Debug, serde::Deserialize)]
struct ManualEdgesFile {
    #[serde(default)]
    _description: String,
    edges: Vec<ManualEdge>,
}

/// A single manual edge entry.
#[derive(Debug, serde::Deserialize)]
struct ManualEdge {
    from: String,
    to: String,
    relationship: Relationship,
    weight: Option<f32>,
    #[serde(default)]
    _note: String,
}

/// Parse author and year from a source filename.
///
/// Expected format: `[YEAR] Author - Title.ext`
fn parse_source_filename(filename: &str) -> (String, Option<u16>) {
    // Extract year from [YEAR] prefix
    let year = if let Some(year_str) = filename.strip_prefix('[') {
        if let Some(end_bracket) = year_str.find(']') {
            year_str[..end_bracket].parse::<u16>().ok()
        } else {
            None
        }
    } else {
        None
    };

    // Extract author (between ] and -)
    let author = if let Some(start) = filename.find(']') {
        if let Some(dash) = filename[start..].find(" - ") {
            filename[start + 2..start + dash].trim().to_string()
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    };

    (author, year)
}

/// Extract title from a source filename.
///
/// Expected format: `[YEAR] Author - Title.ext`
fn extract_title_from_filename(filename: &str) -> String {
    // Extract title (after - and before .ext)
    if let Some(dash_pos) = filename.find(" - ") {
        if let Some(ext_pos) = filename.rfind('.') {
            filename[dash_pos + 3..ext_pos].trim().to_string()
        } else {
            filename[dash_pos + 3..].trim().to_string()
        }
    } else {
        // Fallback: use filename without extension
        if let Some(ext_pos) = filename.rfind('.') {
            filename[..ext_pos].to_string()
        } else {
            filename.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_source_filename() {
        let (author, year) = parse_source_filename(
            "[2011] Tymoczko - A Geometry of Music - Harmony and Counterpoint.pdf",
        );
        assert_eq!(author, "Tymoczko");
        assert_eq!(year, Some(2011));

        let (author, year) = parse_source_filename("[2022] Gotham - Open Music Theory.xml");
        assert_eq!(author, "Gotham");
        assert_eq!(year, Some(2022));

        let (author, year) = parse_source_filename("No Brackets - Title.pdf");
        assert_eq!(author, "Unknown");
        assert_eq!(year, None);
    }

    #[test]
    fn test_extract_title_from_filename() {
        assert_eq!(
            extract_title_from_filename(
                "[2011] Tymoczko - A Geometry of Music - Harmony and Counterpoint.pdf"
            ),
            "A Geometry of Music - Harmony and Counterpoint"
        );

        assert_eq!(
            extract_title_from_filename("[2022] Gotham - Open Music Theory.xml"),
            "Open Music Theory"
        );

        assert_eq!(extract_title_from_filename("NoAuthor.pdf"), "NoAuthor");
    }

    #[test]
    fn test_graph_builder_new() {
        let builder = GraphBuilder::new();
        assert_eq!(builder.nodes.len(), 0);
        assert_eq!(builder.edges.len(), 0);
        assert_eq!(builder.warnings.len(), 0);
    }

    #[test]
    fn test_graph_builder_default() {
        let builder = GraphBuilder::default();
        assert_eq!(builder.nodes.len(), 0);
    }

    #[test]
    fn test_manual_edge_deserialization() {
        let json = r#"{
            "description": "Test edges",
            "edges": [
                {
                    "from": "a",
                    "to": "b",
                    "relationship": "prerequisite",
                    "weight": 0.8,
                    "note": "Test note"
                }
            ]
        }"#;

        let manual: ManualEdgesFile = serde_json::from_str(json).unwrap();
        assert_eq!(manual.edges.len(), 1);
        assert_eq!(manual.edges[0].from, "a");
        assert_eq!(manual.edges[0].to, "b");
        assert_eq!(manual.edges[0].relationship, Relationship::Prerequisite);
        assert_eq!(manual.edges[0].weight, Some(0.8));
    }

    #[test]
    fn test_add_concept_node() {
        let mut builder = GraphBuilder::new();
        let card = ConceptCard {
            id: "test-concept".to_string(),
            title: "Test Concept".to_string(),
            category: "harmony".to_string(),
            source: "test-source".to_string(),
            related_concepts: None,
        };

        builder.add_concept_node(&card);

        assert_eq!(builder.nodes.len(), 1);
        assert!(builder.node_ids.contains("test-concept"));

        match &builder.nodes[0] {
            Node::Concept(c) => {
                assert_eq!(c.id, "test-concept");
                assert_eq!(c.title, "Test Concept");
                assert_eq!(c.category, "harmony");
                assert_eq!(c.source_id, "test-source");
                assert!(c.is_canonical);
                assert!(c.canonical_id.is_none());
            }
            _ => panic!("Expected Concept node"),
        }
    }

    #[test]
    fn test_extract_relationships_prerequisites() {
        let mut builder = GraphBuilder::new();

        // Add two concept nodes first
        builder.node_ids.insert("concept-a".to_string());
        builder.node_ids.insert("concept-b".to_string());

        let card = ConceptCard {
            id: "concept-b".to_string(),
            title: "Concept B".to_string(),
            category: "test".to_string(),
            source: "test".to_string(),
            related_concepts: Some(crate::graph::parser::RelatedConcepts {
                prerequisite: vec!["concept-a".to_string()],
                leads_to: vec![],
                see_also: vec![],
            }),
        };

        builder.extract_relationships(&card);

        assert_eq!(builder.edges.len(), 1);
        assert_eq!(builder.edges[0].from, "concept-a");
        assert_eq!(builder.edges[0].to, "concept-b");
        assert_eq!(builder.edges[0].relationship, Relationship::Prerequisite);
        assert_eq!(builder.edges[0].weight, 1.0);
        assert_eq!(builder.edges[0].origin, EdgeOrigin::Extracted);
        assert_eq!(builder.warnings.len(), 0);
    }

    #[test]
    fn test_extract_relationships_leads_to() {
        let mut builder = GraphBuilder::new();

        builder.node_ids.insert("concept-a".to_string());
        builder.node_ids.insert("concept-b".to_string());

        let card = ConceptCard {
            id: "concept-a".to_string(),
            title: "Concept A".to_string(),
            category: "test".to_string(),
            source: "test".to_string(),
            related_concepts: Some(crate::graph::parser::RelatedConcepts {
                prerequisite: vec![],
                leads_to: vec!["concept-b".to_string()],
                see_also: vec![],
            }),
        };

        builder.extract_relationships(&card);

        assert_eq!(builder.edges.len(), 1);
        assert_eq!(builder.edges[0].from, "concept-a");
        assert_eq!(builder.edges[0].to, "concept-b");
        assert_eq!(builder.edges[0].relationship, Relationship::Prerequisite);
    }

    #[test]
    fn test_extract_relationships_see_also() {
        let mut builder = GraphBuilder::new();

        builder.node_ids.insert("concept-a".to_string());
        builder.node_ids.insert("concept-b".to_string());

        let card = ConceptCard {
            id: "concept-a".to_string(),
            title: "Concept A".to_string(),
            category: "test".to_string(),
            source: "test".to_string(),
            related_concepts: Some(crate::graph::parser::RelatedConcepts {
                prerequisite: vec![],
                leads_to: vec![],
                see_also: vec!["concept-b".to_string()],
            }),
        };

        builder.extract_relationships(&card);

        assert_eq!(builder.edges.len(), 1);
        assert_eq!(builder.edges[0].from, "concept-a");
        assert_eq!(builder.edges[0].to, "concept-b");
        assert_eq!(builder.edges[0].relationship, Relationship::RelatesTo);
        assert_eq!(builder.edges[0].weight, 0.7);
    }

    #[test]
    fn test_extract_relationships_unknown_prerequisite_warning() {
        let mut builder = GraphBuilder::new();

        builder.node_ids.insert("concept-a".to_string());

        let card = ConceptCard {
            id: "concept-a".to_string(),
            title: "Concept A".to_string(),
            category: "test".to_string(),
            source: "test".to_string(),
            related_concepts: Some(crate::graph::parser::RelatedConcepts {
                prerequisite: vec!["unknown-concept".to_string()],
                leads_to: vec![],
                see_also: vec![],
            }),
        };

        builder.extract_relationships(&card);

        assert_eq!(builder.edges.len(), 0);
        assert_eq!(builder.warnings.len(), 1);
        assert!(builder.warnings[0].contains("unknown prerequisite"));
    }

    #[test]
    fn test_extract_relationships_no_warning_for_unknown_see_also() {
        let mut builder = GraphBuilder::new();

        builder.node_ids.insert("concept-a".to_string());

        let card = ConceptCard {
            id: "concept-a".to_string(),
            title: "Concept A".to_string(),
            category: "test".to_string(),
            source: "test".to_string(),
            related_concepts: Some(crate::graph::parser::RelatedConcepts {
                prerequisite: vec![],
                leads_to: vec![],
                see_also: vec!["unknown-concept".to_string()],
            }),
        };

        builder.extract_relationships(&card);

        // See also doesn't create edges or warnings for unknown references
        assert_eq!(builder.edges.len(), 0);
        assert_eq!(builder.warnings.len(), 0);
    }

    #[test]
    fn test_add_introduces_edge_by_id() {
        let mut builder = GraphBuilder::new();

        builder.node_ids.insert("test-source".to_string());
        builder.node_ids.insert("test-concept".to_string());

        let card = ConceptCard {
            id: "test-concept".to_string(),
            title: "Test Concept".to_string(),
            category: "test".to_string(),
            source: "test-source".to_string(),
            related_concepts: None,
        };

        builder.add_introduces_edge(&card);

        assert_eq!(builder.edges.len(), 1);
        assert_eq!(builder.edges[0].from, "test-source");
        assert_eq!(builder.edges[0].to, "test-concept");
        assert_eq!(builder.edges[0].relationship, Relationship::Introduces);
        assert_eq!(builder.edges[0].origin, EdgeOrigin::Extracted);
    }

    #[test]
    fn test_add_introduces_edge_by_title() {
        let mut builder = GraphBuilder::new();

        builder.node_ids.insert("test-source".to_string());
        builder.node_ids.insert("test-concept".to_string());
        builder
            .source_title_to_id
            .insert("Test Source".to_string(), "test-source".to_string());

        let card = ConceptCard {
            id: "test-concept".to_string(),
            title: "Test Concept".to_string(),
            category: "test".to_string(),
            source: "Test Source".to_string(), // Using title instead of ID
            related_concepts: None,
        };

        builder.add_introduces_edge(&card);

        assert_eq!(builder.edges.len(), 1);
        assert_eq!(builder.edges[0].from, "test-source");
    }

    #[test]
    fn test_add_introduces_edge_unknown_source_warning() {
        let mut builder = GraphBuilder::new();

        builder.node_ids.insert("test-concept".to_string());

        let card = ConceptCard {
            id: "test-concept".to_string(),
            title: "Test Concept".to_string(),
            category: "test".to_string(),
            source: "unknown-source".to_string(),
            related_concepts: None,
        };

        builder.add_introduces_edge(&card);

        assert_eq!(builder.edges.len(), 0);
        assert_eq!(builder.warnings.len(), 1);
        assert!(builder.warnings[0].contains("unknown source"));
    }

    #[test]
    fn test_add_introduces_edge_unknown_source_no_warning() {
        let mut builder = GraphBuilder::new();

        builder.node_ids.insert("test-concept".to_string());

        let card = ConceptCard {
            id: "test-concept".to_string(),
            title: "Test Concept".to_string(),
            category: "test".to_string(),
            source: "unknown".to_string(), // "unknown" doesn't generate warning
            related_concepts: None,
        };

        builder.add_introduces_edge(&card);

        assert_eq!(builder.edges.len(), 0);
        assert_eq!(builder.warnings.len(), 0);
    }

    #[test]
    fn test_deduplicate_edges_keeps_higher_weight() {
        let mut builder = GraphBuilder::new();

        // Add duplicate edges with different weights
        builder.edges.push(Edge {
            from: "a".to_string(),
            to: "b".to_string(),
            relationship: Relationship::Prerequisite,
            weight: 0.5,
            origin: EdgeOrigin::Extracted,
        });

        builder.edges.push(Edge {
            from: "a".to_string(),
            to: "b".to_string(),
            relationship: Relationship::Prerequisite,
            weight: 1.0, // Higher weight
            origin: EdgeOrigin::Manual,
        });

        builder.deduplicate_edges();

        assert_eq!(builder.edges.len(), 1);
        assert_eq!(builder.edges[0].weight, 1.0);
        assert_eq!(builder.edges[0].origin, EdgeOrigin::Manual);
    }

    #[test]
    fn test_deduplicate_edges_different_relationships_kept() {
        let mut builder = GraphBuilder::new();

        // Add edges with same nodes but different relationships
        builder.edges.push(Edge {
            from: "a".to_string(),
            to: "b".to_string(),
            relationship: Relationship::Prerequisite,
            weight: 1.0,
            origin: EdgeOrigin::Extracted,
        });

        builder.edges.push(Edge {
            from: "a".to_string(),
            to: "b".to_string(),
            relationship: Relationship::RelatesTo,
            weight: 0.7,
            origin: EdgeOrigin::Extracted,
        });

        builder.deduplicate_edges();

        // Both edges should be kept (different relationships)
        assert_eq!(builder.edges.len(), 2);
    }

    #[test]
    fn test_merge_manual_edges() {
        use tempfile::TempDir;

        let mut builder = GraphBuilder::new();

        // Set up nodes
        builder.node_ids.insert("concept-a".to_string());
        builder.node_ids.insert("concept-b".to_string());

        // Add existing edge that will be replaced
        builder.edges.push(Edge {
            from: "concept-a".to_string(),
            to: "concept-b".to_string(),
            relationship: Relationship::Prerequisite,
            weight: 1.0,
            origin: EdgeOrigin::Extracted,
        });

        // Create manual edges JSON file
        let temp_dir = TempDir::new().unwrap();
        let manual_edges_path = temp_dir.path().join("manual_edges.json");

        let manual_json = r#"{
            "description": "Test manual edges",
            "edges": [
                {
                    "from": "concept-a",
                    "to": "concept-b",
                    "relationship": "prerequisite",
                    "weight": 0.8,
                    "note": "Manual override"
                }
            ]
        }"#;

        std::fs::write(&manual_edges_path, manual_json).unwrap();

        builder.merge_manual_edges(&manual_edges_path).unwrap();

        // Should have replaced the existing edge
        assert_eq!(builder.edges.len(), 1);
        assert_eq!(builder.edges[0].weight, 0.8);
        assert_eq!(builder.edges[0].origin, EdgeOrigin::Manual);
    }

    #[test]
    fn test_merge_manual_edges_unknown_node_warning() {
        use tempfile::TempDir;

        let mut builder = GraphBuilder::new();

        builder.node_ids.insert("concept-a".to_string());

        let temp_dir = TempDir::new().unwrap();
        let manual_edges_path = temp_dir.path().join("manual_edges.json");

        let manual_json = r#"{
            "edges": [
                {
                    "from": "concept-a",
                    "to": "unknown-concept",
                    "relationship": "prerequisite"
                }
            ]
        }"#;

        std::fs::write(&manual_edges_path, manual_json).unwrap();

        builder.merge_manual_edges(&manual_edges_path).unwrap();

        assert_eq!(builder.edges.len(), 0);
        assert_eq!(builder.warnings.len(), 1);
        assert!(builder.warnings[0].contains("unknown 'to' node"));
    }
}
