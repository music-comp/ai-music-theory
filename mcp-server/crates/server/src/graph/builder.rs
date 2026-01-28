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
    warnings: Vec<String>,
}

impl GraphBuilder {
    /// Create a new GraphBuilder.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_ids: HashSet::new(),
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
                let converted_path = Path::new(&config.paths.base)
                    .join(&config.paths.sources_md)
                    .join(&source_id);
                let is_converted = converted_path.exists() && converted_path.is_dir();

                let node = Node::Source(SourceNode {
                    id: source_id.clone(),
                    title: extract_title_from_filename(filename),
                    author,
                    year,
                    is_converted,
                });

                self.node_ids.insert(source_id);
                self.nodes.push(node);
            }
        }

        Ok(())
    }

    /// Load concept cards from the configured directory.
    async fn load_concept_cards(&self, config: &Config) -> Result<Vec<ConceptCard>> {
        let base_path = Path::new(&config.paths.base);
        let cards_dir = base_path.join(&config.paths.concept_cards);

        let mut cards = Vec::new();

        // Find all markdown files
        let file_infos = find_all_files(&cards_dir, FindOptions::markdown()).await?;

        for file_info in file_infos {
            // Extract metadata
            let metadata = extract_concept_metadata(base_path, &file_info.path).await?;

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
        if self.node_ids.contains(&card.source) {
            self.edges.push(Edge {
                from: card.source.clone(),
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
        let content =
            std::fs::read_to_string(path).map_err(|e| Error::io_with_path(e, path))?;

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
        to_remove.dedup();  // Remove any duplicate indices from to_remove list
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

        assert_eq!(
            extract_title_from_filename("NoAuthor.pdf"),
            "NoAuthor"
        );
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
}
