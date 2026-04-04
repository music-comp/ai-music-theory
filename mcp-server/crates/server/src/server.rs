//! Music Theory MCP Server — registry-based implementation.
//!
//! Replaces the monolithic `#[tool_router]` approach with composable
//! [`ToolRegistry`] implementations grouped by domain, served by
//! [`FabrykMcpServer`].

use std::sync::Arc;

use serde::Deserialize;
use serde_json::{json, Value};

use fabryk_mcp::{
    model::{
        AnnotateAble, CallToolResult, Content, ErrorCode, ErrorData, RawResource, Resource,
        ResourceContents, Tool,
    },
    CompositeRegistry, FabrykMcpServer, ResourceFuture, ResourceRegistry, ToolRegistry, ToolResult,
};

use crate::config::Config;
use crate::error::McpErrorContextExt;
use crate::resources;
use crate::state::AppState;
use crate::tools;

// ============================================================================
// Helper functions
// ============================================================================

/// Convert a JSON value to the `Map<String, Value>` that `Tool::new` expects.
fn json_schema(value: Value) -> serde_json::Map<String, Value> {
    match value {
        Value::Object(map) => map,
        _ => serde_json::Map::new(),
    }
}

/// Convenience wrapper to build a `Tool` with an input schema.
fn make_tool(name: &str, description: &str, schema: Value) -> Tool {
    Tool::new(
        name.to_string(),
        description.to_string(),
        json_schema(schema),
    )
}

/// Convenience wrapper to build a `Tool` with no parameters.
fn make_tool_no_params(name: &str, description: &str) -> Tool {
    Tool::new(
        name.to_string(),
        description.to_string(),
        json_schema(json!({"type": "object"})),
    )
}

/// Serialize a value to pretty JSON and wrap it in a successful `CallToolResult`.
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

/// Convert a project error to an MCP `ErrorData` using the `McpErrorContextExt` trait.
fn to_mcp_error(e: crate::error::Error, context: &str) -> ErrorData {
    e.to_mcp_error(context)
}

// ============================================================================
// Argument types (deserialized from MCP tool call arguments)
// ============================================================================

fn default_limit() -> usize {
    10
}

#[derive(Deserialize)]
struct SemanticSearchArgs {
    query: String,
    #[serde(default = "default_hybrid_mode")]
    mode: tools::semantic::SearchMode,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    source: Option<String>,
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_hybrid_mode() -> tools::semantic::SearchMode {
    tools::semantic::SearchMode::Hybrid
}

#[derive(Deserialize)]
struct SearchByQuestionArgs {
    question: String,
    #[serde(default = "default_limit")]
    limit: usize,
}

// ============================================================================
// SemanticSearchRegistry (1 tool) — domain-specific, feature-gated
// ============================================================================

struct SemanticSearchRegistry {
    state: AppState,
}

impl SemanticSearchRegistry {
    fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl ToolRegistry for SemanticSearchRegistry {
    fn tools(&self) -> Vec<Tool> {
        vec![make_tool(
            "semantic_search",
            "Semantic search using vector embeddings, keyword search, or hybrid mode (vector+keyword via reciprocal rank fusion)",
            json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Natural language search query"
                    },
                    "mode": {
                        "type": "string",
                        "description": "Search mode: \"vector\", \"keyword\", or \"hybrid\" (default)",
                        "enum": ["vector", "keyword", "hybrid"],
                        "default": "hybrid"
                    },
                    "category": {
                        "type": "string",
                        "description": "Optional category filter"
                    },
                    "source": {
                        "type": "string",
                        "description": "Optional source filter"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum results (default: 10)",
                        "default": 10
                    }
                },
                "required": ["query"]
            }),
        )]
    }

    fn call(&self, name: &str, args: Value) -> Option<ToolResult> {
        if name != "semantic_search" {
            return None;
        }

        let state = self.state.clone();
        Some(Box::pin(async move {
            let args: SemanticSearchArgs = serde_json::from_value(args).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                    None,
                )
            })?;
            let tool_params = tools::semantic::SemanticSearchParams {
                query: args.query,
                mode: args.mode,
                category: args.category,
                source: args.source,
                limit: args.limit,
            };
            let response = tools::semantic::semantic_search(&state, tool_params)
                .await
                .map_err(|e| to_mcp_error(e, "Error in semantic search"))?;
            serialize_response(&response)
        }))
    }
}

// ============================================================================
// QuestionSearchRegistry (1 tool) — standalone for search_by_question
// ============================================================================

struct QuestionSearchRegistry {
    config: Config,
}

impl QuestionSearchRegistry {
    fn new(config: Config) -> Self {
        Self { config }
    }
}

impl ToolRegistry for QuestionSearchRegistry {
    fn tools(&self) -> Vec<Tool> {
        vec![make_tool(
            "search_by_question",
            "Find concept cards that answer a specific question. Matches against competency questions declared in card metadata.",
            json!({
                "type": "object",
                "properties": {
                    "question": {
                        "type": "string",
                        "description": "The question to search for"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum results (default: 10)",
                        "default": 10
                    }
                },
                "required": ["question"]
            }),
        )]
    }

    fn call(&self, name: &str, args: Value) -> Option<ToolResult> {
        if name != "search_by_question" {
            return None;
        }

        let config = self.config.clone();
        Some(Box::pin(async move {
            let args: SearchByQuestionArgs = serde_json::from_value(args).map_err(|e| {
                ErrorData::new(
                    ErrorCode::INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                    None,
                )
            })?;
            let params = tools::questions::SearchByQuestionParams {
                question: args.question,
                limit: args.limit,
            };
            let response = tools::questions::search_by_question(&config, params)
                .await
                .map_err(|e| to_mcp_error(e, "Error searching by question"))?;
            serialize_response(&response)
        }))
    }
}

// ============================================================================
// HealthToolsRegistry (1 tool)
// ============================================================================

struct HealthToolsRegistry {
    state: AppState,
}

impl HealthToolsRegistry {
    fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl ToolRegistry for HealthToolsRegistry {
    fn tools(&self) -> Vec<Tool> {
        vec![make_tool_no_params(
            "health",
            "Get server health and search backend status",
        )]
    }

    fn call(&self, name: &str, _args: Value) -> Option<ToolResult> {
        if name != "health" {
            return None;
        }

        let state = self.state.clone();
        Some(Box::pin(async move {
            let response = tools::health::get_health(&state)
                .await
                .map_err(|e| to_mcp_error(e, "Error getting health status"))?;
            serialize_response(&response)
        }))
    }
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
// MusicTheoryResources (ResourceRegistry implementation)
// ============================================================================

struct MusicTheoryResources {
    config: Config,
}

impl MusicTheoryResources {
    fn new(config: Config) -> Self {
        Self { config }
    }
}

impl ResourceRegistry for MusicTheoryResources {
    fn resources(&self) -> Vec<Resource> {
        resources::list_resources()
            .into_iter()
            .map(|info| {
                RawResource {
                    uri: info.uri,
                    name: info.name,
                    title: None,
                    description: Some(info.description),
                    mime_type: Some(info.mime_type),
                    size: None,
                    icons: None,
                    meta: None,
                }
                .no_annotation()
            })
            .collect()
    }

    fn read(&self, uri: &str) -> Option<ResourceFuture> {
        // Clone values needed for the async block
        let config = self.config.clone();
        let uri_owned = uri.to_string();

        // Check if this URI is one we handle before creating the future
        let known_uris = [
            "skill://conventions",
            "skill://scope",
            "skill://sources",
            "skill://index",
        ];
        if !known_uris.contains(&uri) {
            return None;
        }

        Some(Box::pin(async move {
            match resources::get_resource(&config, &uri_owned) {
                Ok(content) => Ok(vec![ResourceContents::text(content, uri_owned)]),
                Err(_) => Err(ErrorData::new(
                    ErrorCode::RESOURCE_NOT_FOUND,
                    format!("Resource not found: {}", uri_owned),
                    None,
                )),
            }
        }))
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
    json!({
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
    use fabryk_mcp::content::{ContentTools, GuideTools, SourceTools};
    use fabryk_mcp::fts::FtsTools;
    #[cfg(feature = "graph")]
    use fabryk_mcp::graph::GraphTools;
    use std::collections::HashMap;

    use crate::content::{
        MusicTheoryContentProvider, MusicTheoryGuideProvider, MusicTheorySourceProvider,
    };

    let content_provider = Arc::new(MusicTheoryContentProvider::new(state.config.clone()));
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

    let source_provider = Arc::new(MusicTheorySourceProvider::new(state.clone()));
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

    let guide_provider = MusicTheoryGuideProvider::new(state.config.clone());
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

    let semantic_search = SemanticSearchRegistry::new(state.clone());
    let question_search = QuestionSearchRegistry::new(state.config.clone());
    let health_tools = HealthToolsRegistry::new(state.clone());
    let music_theory_tools = MusicTheoryToolsRegistry;

    #[allow(unused_mut)]
    let mut registry = CompositeRegistry::new()
        .add(concept_tools)
        .add(guide_tools)
        .add(source_tools)
        .add(search_tools)
        .add(semantic_search)
        .add(question_search)
        .add(graph_tools)
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
        let node_filter = Arc::new(crate::graph::MusicTheoryNodeFilter);
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

    let resources = MusicTheoryResources::new(state.config.clone());

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

    #[test]
    fn test_default_limit() {
        assert_eq!(default_limit(), 10);
    }

    #[test]
    fn test_serialize_response() {
        let data = serde_json::json!({"key": "value"});
        let result = serialize_response(&data);
        assert!(result.is_ok());
        let call_result = result.unwrap();
        assert_eq!(call_result.is_error, Some(false));
    }

    #[test]
    fn test_json_schema_with_object() {
        let schema = json!({"type": "object", "properties": {}});
        let map = json_schema(schema);
        assert!(map.contains_key("type"));
    }

    #[test]
    fn test_json_schema_with_non_object() {
        let schema = json!("not an object");
        let map = json_schema(schema);
        assert!(map.is_empty());
    }

    #[test]
    fn test_make_tool() {
        let tool = make_tool("test_tool", "A test tool", json!({"type": "object"}));
        assert_eq!(tool.name, "test_tool");
        assert_eq!(tool.description.as_deref(), Some("A test tool"));
    }

    #[test]
    fn test_make_tool_no_params() {
        let tool = make_tool_no_params("test_tool", "A test tool");
        assert_eq!(tool.name, "test_tool");
    }

    // --- Registry tool counts ---

    #[tokio::test]
    async fn test_semantic_search_registry_tool_count() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let registry = SemanticSearchRegistry::new(state);
        assert_eq!(registry.tools().len(), 1);
    }

    #[tokio::test]
    async fn test_health_tools_registry_tool_count() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let registry = HealthToolsRegistry::new(state);
        assert_eq!(registry.tools().len(), 1);
    }

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

    // --- Argument deserialization ---

    #[test]
    fn test_semantic_search_args_defaults() {
        let json = r#"{"query":"test"}"#;
        let args: SemanticSearchArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.query, "test");
        assert_eq!(args.mode, tools::semantic::SearchMode::Hybrid);
        assert_eq!(args.limit, 10);
        assert!(args.category.is_none());
        assert!(args.source.is_none());
    }

    #[test]
    fn test_search_by_question_args_defaults() {
        let json = r#"{"question":"What is harmony?"}"#;
        let args: SearchByQuestionArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.question, "What is harmony?");
        assert_eq!(args.limit, 10);
    }

    #[test]
    fn test_search_by_question_args_with_limit() {
        let json = r#"{"question":"What is a chord?","limit":5}"#;
        let args: SearchByQuestionArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.question, "What is a chord?");
        assert_eq!(args.limit, 5);
    }

    // --- Resource registry ---

    #[test]
    fn test_music_theory_resources_list() {
        let config = Config::load().unwrap();
        let resources = MusicTheoryResources::new(config);
        let list = resources.resources();
        assert_eq!(list.len(), 4);
    }

    #[test]
    fn test_music_theory_resources_read_known() {
        let config = Config::load().unwrap();
        let resources = MusicTheoryResources::new(config);
        assert!(resources.read("skill://conventions").is_some());
        assert!(resources.read("skill://scope").is_some());
        assert!(resources.read("skill://sources").is_some());
        assert!(resources.read("skill://index").is_some());
    }

    #[test]
    fn test_music_theory_resources_read_unknown() {
        let config = Config::load().unwrap();
        let resources = MusicTheoryResources::new(config);
        assert!(resources.read("skill://nonexistent").is_none());
    }

    // --- Tool call dispatch ---

    #[tokio::test]
    async fn test_semantic_search_registry_unknown_tool_returns_none() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let registry = SemanticSearchRegistry::new(state);
        assert!(registry.call("nonexistent", json!({})).is_none());
    }

    #[tokio::test]
    async fn test_health_registry_unknown_tool_returns_none() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let registry = HealthToolsRegistry::new(state);
        assert!(registry.call("nonexistent", json!({})).is_none());
    }

    // --- Resource content tests (directly via resources module) ---

    #[test]
    fn test_list_resources_directly() {
        let resources = resources::list_resources();
        assert!(!resources.is_empty());
        assert_eq!(resources.len(), 4);

        let first_resource = &resources[0];
        assert!(first_resource.uri.starts_with("skill://"));
        assert!(!first_resource.name.is_empty());
        assert!(!first_resource.description.is_empty());
        assert_eq!(first_resource.mime_type, "text/markdown");
    }

    #[test]
    fn test_read_resource_conventions() {
        let config = Config::load().unwrap();
        let result = resources::get_resource(&config, "skill://conventions");
        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(!content.is_empty());
        assert!(content.contains("Music Theory") || content.contains("Notation"));
    }

    #[test]
    fn test_read_resource_scope() {
        let config = Config::load().unwrap();
        let result = resources::get_resource(&config, "skill://scope");
        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_read_resource_sources() {
        let config = Config::load().unwrap();
        let result = resources::get_resource(&config, "skill://sources");
        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_read_resource_index() {
        let config = Config::load().unwrap();
        let result = resources::get_resource(&config, "skill://index");
        assert!(result.is_ok());
        let content = result.unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_read_resource_not_found() {
        let config = Config::load().unwrap();
        let result = resources::get_resource(&config, "skill://nonexistent");
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.is_not_found());
    }
}
