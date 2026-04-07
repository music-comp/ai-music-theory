//! Music Theory MCP Server — registry-based implementation.
//!
//! Replaces the monolithic `#[tool_router]` approach with composable
//! [`ToolRegistry`] implementations grouped by domain, served by
//! [`FabrykMcpServer`].

use std::sync::Arc;

use serde_json::{json, Value};

use fabryk_mcp::{
    model::{CallToolResult, Content, ErrorCode, ErrorData, Tool},
    CompositeRegistry, FabrykMcpServer, ToolRegistry, ToolResult,
};

use crate::error::McpErrorContextExt;
use crate::state::AppState;
use crate::tools;

// ============================================================================
// Helper functions (for MusicTheoryToolsRegistry)
// ============================================================================

fn make_tool(name: &str, description: &str, schema: Value) -> Tool {
    let input_schema = match schema {
        Value::Object(map) => map,
        _ => serde_json::Map::new(),
    };
    Tool::new(name.to_string(), description.to_string(), input_schema)
}

fn serialize_response<T: serde::Serialize>(value: &T) -> Result<CallToolResult, ErrorData> {
    let json = serde_json::to_string_pretty(value).map_err(|e| {
        ErrorData::new(
            ErrorCode::INTERNAL_ERROR,
            format!("Serialization error: {}", e),
            None,
        )
    })?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}

fn to_mcp_error(e: crate::error::Error, context: &str) -> ErrorData {
    e.to_mcp_error(context)
}

// ============================================================================
// MusicTheoryToolsRegistry (9 tools — pure computation, no state needed)
// ============================================================================

struct MusicTheoryToolsRegistry;

impl ToolRegistry for MusicTheoryToolsRegistry {
    fn tools(&self) -> Vec<Tool> {
        vec![
            make_tool(
                "get_scale_notes",
                "Compute the notes of a musical scale given a tonic and mode",
                json!({
                    "type": "object",
                    "properties": {
                        "tonic": {
                            "type": "string",
                            "description": "Root note (e.g. 'C', 'F#', 'Bb')"
                        },
                        "mode": {
                            "type": "string",
                            "description": "Scale mode (e.g. 'Major', 'Dorian', 'Harmonic Minor', 'Pentatonic Major')"
                        },
                        "direction": {
                            "type": "string",
                            "enum": ["ascending", "descending"],
                            "description": "Scale direction (default: ascending)"
                        }
                    },
                    "required": ["tonic", "mode"]
                }),
            ),
            make_tool(
                "get_chord_notes",
                "Compute the notes of a chord given root, quality, and optional number/inversion",
                json!({
                    "type": "object",
                    "properties": {
                        "root": {
                            "type": "string",
                            "description": "Root note (e.g. 'C', 'F#', 'Bb')"
                        },
                        "quality": {
                            "type": "string",
                            "description": "Chord quality (e.g. 'Major', 'Minor', 'Diminished', 'Dominant')"
                        },
                        "number": {
                            "type": "string",
                            "description": "Chord number (e.g. 'Seventh', 'Ninth', 'Triad')"
                        },
                        "inversion": {
                            "type": "integer",
                            "description": "Inversion number (0 = root position, 1 = first inversion, etc.)"
                        }
                    },
                    "required": ["root", "quality"]
                }),
            ),
            make_tool(
                "get_interval",
                "Calculate the interval between two notes",
                json!({
                    "type": "object",
                    "properties": {
                        "from": {
                            "type": "string",
                            "description": "Starting note (e.g. 'C', 'F#')"
                        },
                        "to": {
                            "type": "string",
                            "description": "Ending note (e.g. 'G', 'Bb')"
                        }
                    },
                    "required": ["from", "to"]
                }),
            ),
            make_tool(
                "transpose",
                "Transpose a list of notes up or down by a number of semitones",
                json!({
                    "type": "object",
                    "properties": {
                        "notes": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Notes to transpose (e.g. ['C', 'E', 'G'])"
                        },
                        "semitones": {
                            "type": "integer",
                            "description": "Number of semitones to transpose"
                        },
                        "direction": {
                            "type": "string",
                            "enum": ["up", "down"],
                            "description": "Transpose direction (default: up)"
                        }
                    },
                    "required": ["notes", "semitones"]
                }),
            ),
            make_tool(
                "get_diatonic_chords",
                "Get all diatonic chords (triads or sevenths) for a key",
                json!({
                    "type": "object",
                    "properties": {
                        "tonic": {
                            "type": "string",
                            "description": "Key tonic (e.g. 'C', 'G', 'Bb')"
                        },
                        "mode": {
                            "type": "string",
                            "description": "Scale mode (e.g. 'Ionian', 'Dorian', 'Aeolian')"
                        },
                        "chord_type": {
                            "type": "string",
                            "enum": ["triad", "seventh"],
                            "description": "Whether to build triads or seventh chords (default: triad)"
                        }
                    },
                    "required": ["tonic", "mode"]
                }),
            ),
            make_tool(
                "identify_chord",
                "Identify possible chords from a set of notes",
                json!({
                    "type": "object",
                    "properties": {
                        "notes": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Notes to identify (e.g. ['C', 'E', 'G'])"
                        }
                    },
                    "required": ["notes"]
                }),
            ),
            make_tool(
                "identify_scale",
                "Identify possible scales/modes from a set of notes",
                json!({
                    "type": "object",
                    "properties": {
                        "notes": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Notes to identify (e.g. ['C', 'D', 'E', 'F', 'G', 'A', 'B'])"
                        }
                    },
                    "required": ["notes"]
                }),
            ),
            make_tool(
                "check_enharmonic",
                "Check if two notes are enharmonically equivalent",
                json!({
                    "type": "object",
                    "properties": {
                        "note_a": {
                            "type": "string",
                            "description": "First note (e.g. 'C#')"
                        },
                        "note_b": {
                            "type": "string",
                            "description": "Second note (e.g. 'Db')"
                        }
                    },
                    "required": ["note_a", "note_b"]
                }),
            ),
            make_tool(
                "analyze_roman_numerals",
                "Analyze chords in a key context, returning Roman numeral labels",
                json!({
                    "type": "object",
                    "properties": {
                        "key_tonic": {
                            "type": "string",
                            "description": "Key tonic (e.g. 'C', 'G')"
                        },
                        "key_mode": {
                            "type": "string",
                            "description": "Key mode (e.g. 'Major', 'Ionian', 'Aeolian')"
                        },
                        "chords": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Chords to analyze (e.g. ['C Major', 'G Major', 'A Minor'])"
                        }
                    },
                    "required": ["key_tonic", "key_mode", "chords"]
                }),
            ),
        ]
    }

    fn call(&self, name: &str, args: Value) -> Option<ToolResult> {
        match name {
            "get_scale_notes" => Some(Box::pin(async move {
                let args: tools::music_theory::GetScaleNotesArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::get_scale_notes(args)
                    .map_err(|e| to_mcp_error(e, "Error computing scale"))?;
                serialize_response(&response)
            })),
            "get_chord_notes" => Some(Box::pin(async move {
                let args: tools::music_theory::GetChordNotesArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::get_chord_notes(args)
                    .map_err(|e| to_mcp_error(e, "Error computing chord"))?;
                serialize_response(&response)
            })),
            "get_interval" => Some(Box::pin(async move {
                let args: tools::music_theory::GetIntervalArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::get_interval(args)
                    .map_err(|e| to_mcp_error(e, "Error computing interval"))?;
                serialize_response(&response)
            })),
            "transpose" => Some(Box::pin(async move {
                let args: tools::music_theory::TransposeArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::transpose(args)
                    .map_err(|e| to_mcp_error(e, "Error transposing"))?;
                serialize_response(&response)
            })),
            "get_diatonic_chords" => Some(Box::pin(async move {
                let args: tools::music_theory::GetDiatonicChordsArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::get_diatonic_chords(args)
                    .map_err(|e| to_mcp_error(e, "Error computing diatonic chords"))?;
                serialize_response(&response)
            })),
            "identify_chord" => Some(Box::pin(async move {
                let args: tools::music_theory::IdentifyChordArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::identify_chord(args)
                    .map_err(|e| to_mcp_error(e, "Error identifying chord"))?;
                serialize_response(&response)
            })),
            "identify_scale" => Some(Box::pin(async move {
                let args: tools::music_theory::IdentifyScaleArgs = serde_json::from_value(args)
                    .map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::identify_scale(args)
                    .map_err(|e| to_mcp_error(e, "Error identifying scale"))?;
                serialize_response(&response)
            })),
            "check_enharmonic" => Some(Box::pin(async move {
                let args: tools::music_theory::CheckEnharmonicArgs = serde_json::from_value(args)
                    .map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let response = tools::music_theory::check_enharmonic(args)
                    .map_err(|e| to_mcp_error(e, "Error checking enharmonic"))?;
                serialize_response(&response)
            })),
            "analyze_roman_numerals" => Some(Box::pin(async move {
                let args: tools::music_theory::AnalyzeRomanNumeralsArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::music_theory::analyze_roman_numerals(args)
                    .map_err(|e| to_mcp_error(e, "Error analyzing roman numerals"))?;
                serialize_response(&response)
            })),
            _ => None,
        }
    }
}

// ============================================================================
// Server builder
// ============================================================================

#[cfg(feature = "graph")]
/// Extra JSON schema properties for music-theory-specific graph tool filters.
///
/// Used by `GraphTools::with_extra_schema` to add `tier` and `min_confidence`
/// parameters to graph query tools that support domain-specific filtering.
fn tier_confidence_schema() -> serde_json::Value {
    serde_json::json!({
        "tier": {
            "type": "string",
            "enum": ["foundational", "intermediate", "advanced"],
            "description": "Filter results by prerequisite depth tier"
        },
        "min_confidence": {
            "type": "string",
            "enum": ["high", "medium", "low"],
            "description": "Minimum extraction confidence threshold"
        }
    })
}

/// Build the Music Theory MCP server from application state.
///
/// Composes domain-specific tool registries and resources into a single
/// [`FabrykMcpServer`] ready for stdio or HTTP transport.
///
/// # Graph tools
///
/// Graph tools are provided by fabryk's [`GraphTools`](fabryk_mcp::graph::GraphTools)
/// with custom tool names (via `.with_names()`), a domain-specific node filter
/// (via `.with_node_filter()`), and extra schema properties for tier/confidence
/// filtering (via `.with_extra_schema()`).
///
/// The 16 original tool names are preserved:
/// `graph_status`, `graph_stats`, `graph_validate`, `get_node`, `get_node_edges`,
/// `get_related_concepts`, `find_concept_path`, `get_prerequisites`,
/// `get_concept_neighborhood`, `get_dependents`, `get_central_concepts`,
/// `get_concept_sources`, `get_concept_variants`, `find_bridge_concepts`,
/// `get_source_coverage`, `get_learning_path`.
///
/// Fabryk's `graph_bridges` tool is also exposed (an additional 17th tool that
/// finds bridge nodes by betweenness centrality).
pub fn build_server(state: AppState) -> FabrykMcpServer {
    use fabryk_mcp::content::{
        ContentTools, FsContentItemProvider, FsGuideProvider, FsQuestionSearchProvider,
        FsSourceProvider, GuideTools, SourceTools,
    };
    use fabryk_mcp::fts::FtsTools;
    #[cfg(feature = "graph")]
    use fabryk_mcp::graph::GraphTools;
    use std::collections::HashMap;

    let concept_cards_path = state.config.paths.concept_cards_path().unwrap_or_default();
    let content_provider = Arc::new(
        FsContentItemProvider::new(&concept_cards_path)
            .with_content_type_name("concept", "concepts"),
    );
    let concept_tools = ContentTools::with_shared(content_provider)
        .with_names(HashMap::from([
            ("list".to_string(), "list_concepts".to_string()),
            ("get".to_string(), "get_concept".to_string()),
            ("categories".to_string(), "list_categories".to_string()),
        ]))
        .with_descriptions(HashMap::from([
            (
                "list".to_string(),
                "List concept cards with optional filtering by category, tier, subcategory, or source".to_string(),
            ),
            (
                "get".to_string(),
                "Retrieve a specific concept card".to_string(),
            ),
            (
                "categories".to_string(),
                "List all distinct concept categories with counts".to_string(),
            ),
        ]))
        .with_get_id_field("concept_id")
        .with_extra_list_schema(serde_json::json!({
            "tier": {
                "type": "string",
                "enum": ["foundational", "intermediate", "advanced"],
                "description": "Filter by prerequisite depth tier"
            },
            "subcategory": {
                "type": "string",
                "description": "Filter by subcategory"
            },
            "source": {
                "type": "string",
                "description": "Filter by source text"
            }
        }));

    let sources_md_path = state.config.paths.sources_md_path().unwrap_or_default();
    let source_provider = Arc::new(FsSourceProvider::new(&sources_md_path));
    let source_tools = SourceTools::with_shared(source_provider)
        .with_names(HashMap::from([
            ("sources_list".to_string(), "list_sources".to_string()),
            (
                "sources_chapters".to_string(),
                "list_source_chapters".to_string(),
            ),
            (
                "sources_get_chapter".to_string(),
                "get_source_chapter".to_string(),
            ),
            (
                "sources_check_availability".to_string(),
                "check_source_availability".to_string(),
            ),
            (
                "sources_get_path".to_string(),
                "get_source_pdf_path".to_string(),
            ),
        ]))
        .with_descriptions(HashMap::from([
            (
                "sources_list".to_string(),
                "List all available source materials with metadata".to_string(),
            ),
            (
                "sources_check_availability".to_string(),
                "Check availability status of a source (indexed/converted/exists)".to_string(),
            ),
            (
                "sources_chapters".to_string(),
                "List all chapters for a source material".to_string(),
            ),
            (
                "sources_get_chapter".to_string(),
                "Retrieve a specific chapter from a source material".to_string(),
            ),
            (
                "sources_get_path".to_string(),
                "Get filesystem path to original PDF/EPUB for a source".to_string(),
            ),
        ]));

    let guides_path = state.config.paths.guides_path().unwrap_or_default();
    let guide_provider = FsGuideProvider::new(&guides_path);
    let guide_tools = GuideTools::new(guide_provider);

    let search_backend = state.search_backend();
    let search_tools = FtsTools::with_shared(search_backend)
        .with_names(HashMap::from([(
            "search".to_string(),
            "search_concepts".to_string(),
        )]))
        .with_descriptions(HashMap::from([(
            "search".to_string(),
            "Search concept cards with full-text search and relevance ranking".to_string(),
        )]))
        .with_extra_search_schema(serde_json::json!({
            "tier": {
                "type": "string",
                "enum": ["foundational", "intermediate", "advanced"],
                "description": "Filter by prerequisite depth tier"
            },
            "subcategory": {
                "type": "string",
                "description": "Filter by subcategory"
            },
            "min_confidence": {
                "type": "string",
                "enum": ["high", "medium", "low"],
                "description": "Minimum extraction confidence (high includes only high; medium includes high+medium; low includes all)"
            },
            "content_types": {
                "type": "array",
                "items": {"type": "string"},
                "description": "Optional content type filter (concept_card, source_chapter, unified_concept, guide)"
            }
        }));

    // Fabryk's SemanticSearchTools handles keyword/vector/hybrid dispatch with
    // RRF fusion. The vector_slot is a tokio::sync::RwLock that starts empty and
    // is populated when the background vector build completes.
    #[cfg(feature = "vector")]
    let semantic_search = {
        use fabryk_mcp::semantic::SemanticSearchTools;
        SemanticSearchTools::with_vector_slot(
            state.search_backend(),
            Arc::clone(&state.vector_slot),
        )
    };
    #[cfg(not(feature = "vector"))]
    let semantic_search = {
        use fabryk_mcp::semantic::SemanticSearchTools;
        SemanticSearchTools::new(state.search_backend(), None)
    };
    let question_provider = FsQuestionSearchProvider::new(&concept_cards_path);
    let question_search = fabryk_mcp::content::QuestionSearchTools::new(question_provider);
    let music_theory_tools = MusicTheoryToolsRegistry;

    // Build backend probes for health diagnostics
    let search_backend_for_probe = state.search_backend();
    #[allow(unused_mut)]
    let mut probes: Vec<Arc<dyn fabryk::core::BackendProbe>> =
        vec![fabryk::fts::search_probe(search_backend_for_probe)];

    #[cfg(feature = "vector")]
    {
        if let Ok(guard) = state.vector_backend.read() {
            if let Some(ref backend) = *guard {
                probes.push(fabryk::vector::vector_probe(Arc::clone(backend)));
            }
        }
    }

    let health_tools = fabryk_mcp::HealthTools::new(
        &state.config.server.name,
        &state.config.server.version,
        0, // legacy field; CompositeRegistry handles actual count
    )
    .with_backends(probes)
    .with_search_config(fabryk_mcp::SearchConfigInfo {
        query_mode: format!("{:?}", state.config.search.query_mode).to_lowercase(),
        stopwords_enabled: state.config.search.enable_stopwords,
        fuzzy_search: state.config.search.fuzzy_search,
        field_boosts: Some(fabryk_mcp::FieldBoosts {
            title: state.config.search.field_boost_title,
            description: state.config.search.field_boost_description,
            content: state.config.search.field_boost_content,
        }),
    });

    #[allow(unused_mut)]
    let mut registry = CompositeRegistry::new()
        .add(concept_tools)
        .add(guide_tools)
        .add(source_tools)
        .add(search_tools)
        .add(semantic_search)
        .add(question_search)
        .add(health_tools)
        .add(music_theory_tools);

    // ---- Graph tools (17 tools, replacing the former 16-tool GraphToolsRegistry) ----
    //
    // The shared_graph field provides an Arc<tokio::sync::RwLock<GraphData>> that
    // starts empty and is populated when the graph finishes loading asynchronously.
    // GraphTools reads from this lock on every tool call, so it transparently
    // picks up the graph data once available.
    #[cfg(feature = "graph")]
    {
        let node_filter = Arc::new(
            fabryk_mcp::graph::MetadataNodeFilter::new()
                .with_exact("tier", "tier")
                .with_ordered(
                    "min_confidence",
                    "extraction_confidence",
                    &["low", "medium", "high"],
                ),
        );
        let extra = tier_confidence_schema();

        let graph_tools = GraphTools::with_shared(Arc::clone(&state.shared_graph))
            .with_node_filter(node_filter)
            .with_names(HashMap::from([
                // Slot key -> exposed tool name
                (GraphTools::SLOT_GET_NODE.into(), "get_node".into()),
                (GraphTools::SLOT_GET_NODE_EDGES.into(), "get_node_edges".into()),
                (GraphTools::SLOT_RELATED.into(), "get_related_concepts".into()),
                (GraphTools::SLOT_PATH.into(), "find_concept_path".into()),
                (GraphTools::SLOT_PREREQUISITES.into(), "get_prerequisites".into()),
                (GraphTools::SLOT_NEIGHBORHOOD.into(), "get_concept_neighborhood".into()),
                (GraphTools::SLOT_DEPENDENTS.into(), "get_dependents".into()),
                (GraphTools::SLOT_CENTRALITY.into(), "get_central_concepts".into()),
                (GraphTools::SLOT_CONCEPT_SOURCES.into(), "get_concept_sources".into()),
                (GraphTools::SLOT_CONCEPT_VARIANTS.into(), "get_concept_variants".into()),
                (GraphTools::SLOT_BRIDGE_CATEGORIES.into(), "find_bridge_concepts".into()),
                (GraphTools::SLOT_SOURCE_COVERAGE.into(), "get_source_coverage".into()),
                (GraphTools::SLOT_LEARNING_PATH.into(), "get_learning_path".into()),
                (GraphTools::SLOT_STATUS.into(), "graph_status".into()),
                (GraphTools::SLOT_INFO.into(), "graph_stats".into()),
                (GraphTools::SLOT_VALIDATE.into(), "graph_validate".into()),
                // graph_bridges keeps its default name (additive 17th tool)
            ]))
            .with_descriptions(HashMap::from([
                (
                    GraphTools::SLOT_STATUS.into(),
                    "Get concept graph status and basic statistics".into(),
                ),
                (
                    GraphTools::SLOT_INFO.into(),
                    "Get detailed concept graph statistics (categories, relationships)".into(),
                ),
                (
                    GraphTools::SLOT_VALIDATE.into(),
                    "Validate concept graph integrity (orphans, self-loops)".into(),
                ),
                (
                    GraphTools::SLOT_GET_NODE.into(),
                    "Get node information by ID with in/out degree counts".into(),
                ),
                (
                    GraphTools::SLOT_GET_NODE_EDGES.into(),
                    "Get all edges for a node with optional direction filter".into(),
                ),
                (
                    GraphTools::SLOT_RELATED.into(),
                    "Find related concepts with optional relationship and direction filtering".into(),
                ),
                (
                    GraphTools::SLOT_PATH.into(),
                    "Find shortest path between two concepts in the graph".into(),
                ),
                (
                    GraphTools::SLOT_PREREQUISITES.into(),
                    "Get prerequisites for a concept in topological learning order".into(),
                ),
                (
                    GraphTools::SLOT_NEIGHBORHOOD.into(),
                    "Get local neighborhood subgraph around a concept".into(),
                ),
                (
                    GraphTools::SLOT_DEPENDENTS.into(),
                    "Get concepts that depend on this concept as a prerequisite".into(),
                ),
                (
                    GraphTools::SLOT_CENTRALITY.into(),
                    "Get most connected concepts by degree centrality".into(),
                ),
                (
                    GraphTools::SLOT_CONCEPT_SOURCES.into(),
                    "Get all sources that introduce or cover a concept".into(),
                ),
                (
                    GraphTools::SLOT_CONCEPT_VARIANTS.into(),
                    "Get all source-specific variants of a canonical concept".into(),
                ),
                (
                    GraphTools::SLOT_BRIDGE_CATEGORIES.into(),
                    "Find concepts that bridge two categories by connecting to both".into(),
                ),
                (
                    GraphTools::SLOT_SOURCE_COVERAGE.into(),
                    "Get all concepts introduced or covered by a source material".into(),
                ),
                (
                    GraphTools::SLOT_LEARNING_PATH.into(),
                    "Get topologically sorted learning path of prerequisites for a target concept, with tier annotations".into(),
                ),
            ]))
            // Add tier/confidence filter schema to tools that support it
            .with_extra_schema(GraphTools::SLOT_RELATED, extra.clone())
            .with_extra_schema(GraphTools::SLOT_PREREQUISITES, extra.clone())
            .with_extra_schema(GraphTools::SLOT_NEIGHBORHOOD, extra.clone())
            .with_extra_schema(GraphTools::SLOT_CENTRALITY, extra.clone())
            .with_extra_schema(GraphTools::SLOT_LEARNING_PATH, extra);

        registry = registry.add(graph_tools);
    }

    let skill_docs_path = state
        .config
        .paths
        .skill_docs_path()
        .unwrap_or_else(|_| std::path::PathBuf::from("."));

    let resources = fabryk_mcp::StaticResources::new(skill_docs_path)
        .with_resource(fabryk_mcp::StaticResourceDef {
            uri: "skill://conventions".into(),
            name: "Music Theory Conventions".into(),
            description: "Notation conventions and terminology used in this skill".into(),
            mime_type: "text/markdown".into(),
            filename: "CONVENTIONS.md".into(),
            fallback: Some(crate::resources::default_conventions()),
        })
        .with_resource(fabryk_mcp::StaticResourceDef {
            uri: "skill://scope".into(),
            name: "Skill Scope".into(),
            description: "Topics covered and learning objectives of this skill".into(),
            mime_type: "text/markdown".into(),
            filename: "SCOPE.md".into(),
            fallback: Some(crate::resources::default_scope()),
        })
        .with_resource(fabryk_mcp::StaticResourceDef {
            uri: "skill://sources".into(),
            name: "Source Materials".into(),
            description: "Bibliography and source attribution".into(),
            mime_type: "text/markdown".into(),
            filename: "SOURCES.md".into(),
            fallback: Some(crate::resources::default_sources()),
        })
        .with_resource(fabryk_mcp::StaticResourceDef {
            uri: "skill://index".into(),
            name: "Skill Index".into(),
            description: "Complete index of concepts, topics, and materials".into(),
            mime_type: "text/markdown".into(),
            filename: "INDEX.md".into(),
            fallback: Some(crate::resources::default_index()),
        });

    FabrykMcpServer::new(registry)
        .with_name(&state.config.server.name)
        .with_version(&state.config.server.version)
        .with_description(
            "Music Theory AI Skill - Access comprehensive music theory materials \
             including source texts, concept cards, and topic guides.",
        )
        .with_resources(resources)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    // --- Build server ---

    #[tokio::test]
    async fn test_build_server() {
        let config = Config::load().unwrap();
        let state = AppState::new(config.clone()).await.unwrap();
        let server = build_server(state);

        assert_eq!(server.config().name, config.server.name);
        assert_eq!(server.config().version, config.server.version);
        assert!(server
            .config()
            .description
            .as_ref()
            .unwrap()
            .contains("Music Theory AI Skill"));
    }

    #[tokio::test]
    async fn test_build_server_total_tool_count() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = build_server(state);

        // Without graph feature:
        //   3 concept + 2 guide + 5 source + 2 fts + 1 semantic + 1 question + 1 health + 9 music theory = 24
        // With graph feature:
        //   24 + 17 graph = 41
        #[cfg(feature = "graph")]
        assert_eq!(server.registry().tool_count(), 41);
        #[cfg(not(feature = "graph"))]
        assert_eq!(server.registry().tool_count(), 24);
    }

    #[tokio::test]
    async fn test_build_server_has_all_tools() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = build_server(state);
        let registry = server.registry();

        let expected_tools = [
            "list_concepts",
            "get_concept",
            "list_categories",
            "list_guides",
            "get_guide",
            "list_sources",
            "check_source_availability",
            "list_source_chapters",
            "get_source_chapter",
            "get_source_pdf_path",
            "search_concepts",
            "search_status",
            "semantic_search",
            "search_by_question",
            "health",
        ];

        for tool_name in &expected_tools {
            assert!(registry.has_tool(tool_name), "Missing tool: {}", tool_name);
        }
    }

    #[cfg(feature = "graph")]
    #[tokio::test]
    async fn test_build_server_has_graph_tools() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = build_server(state);
        let registry = server.registry();

        let expected_graph_tools = [
            "graph_status",
            "graph_stats",
            "graph_validate",
            "get_node",
            "get_node_edges",
            "get_related_concepts",
            "find_concept_path",
            "get_prerequisites",
            "get_concept_neighborhood",
            "get_dependents",
            "get_central_concepts",
            "get_concept_sources",
            "get_concept_variants",
            "find_bridge_concepts",
            "get_source_coverage",
            "get_learning_path",
            "graph_bridges", // fabryk's bridge-node-by-centrality tool
            // music theory computation tools
            "get_scale_notes",
            "get_chord_notes",
            "get_interval",
            "transpose",
            "get_diatonic_chords",
            "identify_chord",
            "identify_scale",
            "check_enharmonic",
            "analyze_roman_numerals",
        ];

        for tool_name in &expected_graph_tools {
            assert!(
                registry.has_tool(tool_name),
                "Missing graph tool: {}",
                tool_name
            );
        }
    }
}
