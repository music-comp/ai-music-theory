//! Music Theory MCP Server — registry-based implementation.
//!
//! Replaces the monolithic `#[tool_router]` approach with composable
//! [`ToolRegistry`] implementations grouped by domain, served by
//! [`FabrykMcpServer`].

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

#[derive(Deserialize)]
struct ListConceptsArgs {
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    limit: Option<usize>,
}

#[derive(Deserialize)]
struct GetConceptArgs {
    concept_id: String,
}

#[derive(Deserialize)]
struct GetGuideArgs {
    guide_id: String,
}

#[derive(Deserialize)]
struct ListSourceChaptersArgs {
    source_id: String,
}

#[derive(Deserialize)]
struct CheckSourceAvailabilityArgs {
    source_id: String,
}

#[derive(Deserialize)]
struct GetSourceChapterArgs {
    source_id: String,
    chapter: String,
    #[serde(default)]
    section: Option<String>,
}

#[derive(Deserialize)]
struct GetSourcePdfPathArgs {
    source_id: String,
}

#[derive(Deserialize)]
struct SearchConceptsArgs {
    query: String,
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default)]
    query_mode: Option<crate::config::QueryMode>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    content_types: Option<Vec<String>>,
}

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
struct GetNodeArgs {
    node_id: String,
}

#[derive(Deserialize)]
struct GetNodeEdgesArgs {
    node_id: String,
    #[serde(default = "default_direction")]
    direction: String,
}

fn default_direction() -> String {
    "both".to_string()
}

#[derive(Deserialize)]
struct GetRelatedConceptsArgs {
    concept_id: String,
    #[serde(default)]
    relationship_types: Option<String>,
    #[serde(default = "default_direction")]
    direction: String,
    #[serde(default = "default_depth_1")]
    depth: u32,
}

fn default_depth_1() -> u32 {
    1
}

#[derive(Deserialize)]
struct FindConceptPathArgs {
    from_id: String,
    to_id: String,
    #[serde(default = "default_depth_5")]
    max_depth: u32,
}

fn default_depth_5() -> u32 {
    5
}

#[derive(Deserialize)]
struct GetPrerequisitesArgs {
    concept_id: String,
    #[serde(default = "default_depth_3")]
    depth: u32,
}

fn default_depth_3() -> u32 {
    3
}

#[derive(Deserialize)]
struct GetConceptNeighborhoodArgs {
    concept_id: String,
    #[serde(default = "default_radius")]
    radius: u32,
    #[serde(default = "default_max_nodes")]
    max_nodes: u32,
}

fn default_radius() -> u32 {
    2
}

fn default_max_nodes() -> u32 {
    30
}

#[derive(Deserialize)]
struct GetDependentsArgs {
    concept_id: String,
    #[serde(default = "default_depth_2")]
    depth: u32,
}

fn default_depth_2() -> u32 {
    2
}

#[derive(Deserialize)]
struct GetCentralConceptsArgs {
    #[serde(default)]
    category: Option<String>,
    #[serde(default = "default_limit_10")]
    limit: u32,
}

fn default_limit_10() -> u32 {
    10
}

#[derive(Deserialize)]
struct GetConceptSourcesArgs {
    concept_id: String,
}

#[derive(Deserialize)]
struct GetConceptVariantsArgs {
    canonical_id: String,
}

#[derive(Deserialize)]
struct FindBridgeConceptsArgs {
    category_a: String,
    category_b: String,
    #[serde(default = "default_limit_5")]
    limit: u32,
}

fn default_limit_5() -> u32 {
    5
}

#[derive(Deserialize)]
struct GetSourceCoverageArgs {
    source_id: String,
}

// ============================================================================
// ConceptToolsRegistry (3 tools)
// ============================================================================

struct ConceptToolsRegistry {
    config: Config,
}

impl ConceptToolsRegistry {
    fn new(config: Config) -> Self {
        Self { config }
    }
}

impl ToolRegistry for ConceptToolsRegistry {
    fn tools(&self) -> Vec<Tool> {
        vec![
            make_tool(
                "list_concepts",
                "List concept cards with optional category filtering",
                json!({
                    "type": "object",
                    "properties": {
                        "category": {
                            "type": "string",
                            "description": "Filter by category"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum results"
                        }
                    }
                }),
            ),
            make_tool(
                "get_concept",
                "Retrieve a specific concept card",
                json!({
                    "type": "object",
                    "properties": {
                        "concept_id": {
                            "type": "string",
                            "description": "Concept identifier"
                        }
                    },
                    "required": ["concept_id"]
                }),
            ),
            make_tool_no_params(
                "list_categories",
                "List all distinct concept categories with counts",
            ),
        ]
    }

    fn call(&self, name: &str, args: Value) -> Option<ToolResult> {
        let config = self.config.clone();

        if name == "list_concepts" {
            return Some(Box::pin(async move {
                let args: ListConceptsArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let params = tools::concepts::ListConceptsParams {
                    category: args.category,
                    limit: args.limit,
                };
                let response = tools::concepts::list_concepts(&config, Some(params))
                    .await
                    .map_err(|e| to_mcp_error(e, "Error listing concepts"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_concept" {
            return Some(Box::pin(async move {
                let args: GetConceptArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let content = tools::concepts::get_concept(&config, &args.concept_id)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error retrieving concept"))?;
                Ok(CallToolResult::success(vec![Content::text(content)]))
            }));
        }

        if name == "list_categories" {
            return Some(Box::pin(async move {
                let response = tools::concepts::list_categories(&config)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error listing categories"))?;
                serialize_response(&response)
            }));
        }

        None
    }
}

// ============================================================================
// GuideToolsRegistry (2 tools)
// ============================================================================

struct GuideToolsRegistry {
    config: Config,
}

impl GuideToolsRegistry {
    fn new(config: Config) -> Self {
        Self { config }
    }
}

impl ToolRegistry for GuideToolsRegistry {
    fn tools(&self) -> Vec<Tool> {
        vec![
            make_tool_no_params("list_guides", "List all available topic guides"),
            make_tool(
                "get_guide",
                "Retrieve a specific topic guide",
                json!({
                    "type": "object",
                    "properties": {
                        "guide_id": {
                            "type": "string",
                            "description": "Guide identifier"
                        }
                    },
                    "required": ["guide_id"]
                }),
            ),
        ]
    }

    fn call(&self, name: &str, args: Value) -> Option<ToolResult> {
        let config = self.config.clone();

        if name == "list_guides" {
            return Some(Box::pin(async move {
                let response = tools::guides::list_guides(&config)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error listing guides"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_guide" {
            return Some(Box::pin(async move {
                let args: GetGuideArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let content = tools::guides::get_guide(&config, &args.guide_id)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error retrieving guide"))?;
                Ok(CallToolResult::success(vec![Content::text(content)]))
            }));
        }

        None
    }
}

// ============================================================================
// SourceToolsRegistry (5 tools)
// ============================================================================

struct SourceToolsRegistry {
    state: AppState,
}

impl SourceToolsRegistry {
    fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl ToolRegistry for SourceToolsRegistry {
    fn tools(&self) -> Vec<Tool> {
        vec![
            make_tool_no_params(
                "list_sources",
                "List all available source materials with metadata",
            ),
            make_tool(
                "check_source_availability",
                "Check availability status of a source (indexed/converted/exists)",
                json!({
                    "type": "object",
                    "properties": {
                        "source_id": {
                            "type": "string",
                            "description": "Source identifier"
                        }
                    },
                    "required": ["source_id"]
                }),
            ),
            make_tool(
                "list_source_chapters",
                "List all chapters for a source material",
                json!({
                    "type": "object",
                    "properties": {
                        "source_id": {
                            "type": "string",
                            "description": "Source identifier"
                        }
                    },
                    "required": ["source_id"]
                }),
            ),
            make_tool(
                "get_source_chapter",
                "Retrieve a specific chapter from a source material",
                json!({
                    "type": "object",
                    "properties": {
                        "source_id": {
                            "type": "string",
                            "description": "Source identifier"
                        },
                        "chapter": {
                            "type": "string",
                            "description": "Chapter identifier"
                        },
                        "section": {
                            "type": "string",
                            "description": "Optional section/page filter (v0.3.0)"
                        }
                    },
                    "required": ["source_id", "chapter"]
                }),
            ),
            make_tool(
                "get_source_pdf_path",
                "Get filesystem path to original PDF/EPUB for a source",
                json!({
                    "type": "object",
                    "properties": {
                        "source_id": {
                            "type": "string",
                            "description": "Source identifier"
                        }
                    },
                    "required": ["source_id"]
                }),
            ),
        ]
    }

    fn call(&self, name: &str, args: Value) -> Option<ToolResult> {
        let state = self.state.clone();

        if name == "list_sources" {
            return Some(Box::pin(async move {
                let response = tools::sources::list_sources(&state.config)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error listing sources"))?;
                serialize_response(&response)
            }));
        }

        if name == "check_source_availability" {
            return Some(Box::pin(async move {
                let args: CheckSourceAvailabilityArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let response = tools::sources::check_source_availability(&state, &args.source_id)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error checking source availability"))?;
                serialize_response(&response)
            }));
        }

        if name == "list_source_chapters" {
            return Some(Box::pin(async move {
                let args: ListSourceChaptersArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let response = tools::sources::list_source_chapters(&state, &args.source_id)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error listing chapters"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_source_chapter" {
            return Some(Box::pin(async move {
                let args: GetSourceChapterArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let content = tools::sources::get_source_chapter(
                    &state.config,
                    &args.source_id,
                    &args.chapter,
                    args.section.as_deref(),
                )
                .await
                .map_err(|e| to_mcp_error(e, "Error retrieving chapter"))?;
                Ok(CallToolResult::success(vec![Content::text(content)]))
            }));
        }

        if name == "get_source_pdf_path" {
            return Some(Box::pin(async move {
                let args: GetSourcePdfPathArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let path = tools::sources::get_source_pdf_path(&state.config, &args.source_id)
                    .map_err(|e| to_mcp_error(e, "Error getting PDF path"))?;
                let path_str = path.to_str().ok_or_else(|| {
                    ErrorData::new(ErrorCode::INTERNAL_ERROR, "Invalid UTF-8 in path", None)
                })?;
                Ok(CallToolResult::success(vec![Content::text(
                    path_str.to_string(),
                )]))
            }));
        }

        None
    }
}

// ============================================================================
// SearchToolsRegistry (2 tools)
// ============================================================================

struct SearchToolsRegistry {
    state: AppState,
}

impl SearchToolsRegistry {
    fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl ToolRegistry for SearchToolsRegistry {
    fn tools(&self) -> Vec<Tool> {
        vec![
            make_tool(
                "search_concepts",
                "Search concept cards with full-text search and relevance ranking",
                json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Search query"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum results (default: 10)",
                            "default": 10
                        },
                        "query_mode": {
                            "type": "string",
                            "description": "Optional query mode override (smart, and, or, minimum_match)",
                            "enum": ["smart", "and", "or", "minimum_match"]
                        },
                        "category": {
                            "type": "string",
                            "description": "Optional category filter - only return results from this category"
                        },
                        "source": {
                            "type": "string",
                            "description": "Optional source filter - only return results from this source"
                        },
                        "content_types": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Optional content type filter (concept_card, source_chapter, unified_concept, guide)"
                        }
                    },
                    "required": ["query"]
                }),
            ),
            make_tool(
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
            ),
        ]
    }

    fn call(&self, name: &str, args: Value) -> Option<ToolResult> {
        let state = self.state.clone();

        if name == "search_concepts" {
            return Some(Box::pin(async move {
                let args: SearchConceptsArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let search_params = tools::search::SearchConceptsParams {
                    query: args.query,
                    limit: args.limit,
                    query_mode: args.query_mode,
                    category: args.category,
                    source: args.source,
                    content_types: args.content_types,
                };
                let response = tools::search::search_concepts(&state, search_params)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error searching concepts"))?;
                serialize_response(&response)
            }));
        }

        if name == "semantic_search" {
            return Some(Box::pin(async move {
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
            }));
        }

        None
    }
}

// ============================================================================
// GraphToolsRegistry (15 tools)
// ============================================================================

struct GraphToolsRegistry {
    state: AppState,
}

impl GraphToolsRegistry {
    fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl ToolRegistry for GraphToolsRegistry {
    fn tools(&self) -> Vec<Tool> {
        vec![
            // Core graph tools
            make_tool_no_params(
                "graph_status",
                "Get concept graph status and basic statistics",
            ),
            make_tool_no_params(
                "graph_stats",
                "Get detailed concept graph statistics (categories, relationships)",
            ),
            make_tool_no_params(
                "graph_validate",
                "Validate concept graph integrity (orphans, self-loops)",
            ),
            make_tool(
                "get_node",
                "Get node information by ID with in/out degree counts",
                json!({
                    "type": "object",
                    "properties": {
                        "node_id": {
                            "type": "string",
                            "description": "Node identifier"
                        }
                    },
                    "required": ["node_id"]
                }),
            ),
            make_tool(
                "get_node_edges",
                "Get all edges for a node with optional direction filter",
                json!({
                    "type": "object",
                    "properties": {
                        "node_id": {
                            "type": "string",
                            "description": "Node identifier"
                        },
                        "direction": {
                            "type": "string",
                            "description": "Direction filter: incoming, outgoing, both (default: both)",
                            "enum": ["incoming", "outgoing", "both"],
                            "default": "both"
                        }
                    },
                    "required": ["node_id"]
                }),
            ),
            // Graph query tools
            make_tool(
                "get_related_concepts",
                "Find related concepts with optional relationship and direction filtering",
                json!({
                    "type": "object",
                    "properties": {
                        "concept_id": {
                            "type": "string",
                            "description": "Concept identifier"
                        },
                        "relationship_types": {
                            "type": "string",
                            "description": "Optional relationship types filter (comma-separated: \"prerequisite,relates_to\")"
                        },
                        "direction": {
                            "type": "string",
                            "description": "Direction filter: incoming, outgoing, both (default: both)",
                            "enum": ["incoming", "outgoing", "both"],
                            "default": "both"
                        },
                        "depth": {
                            "type": "integer",
                            "description": "Depth to traverse (default: 1, max: 3)",
                            "default": 1
                        }
                    },
                    "required": ["concept_id"]
                }),
            ),
            make_tool(
                "find_concept_path",
                "Find shortest path between two concepts in the graph",
                json!({
                    "type": "object",
                    "properties": {
                        "from_id": {
                            "type": "string",
                            "description": "Starting concept ID"
                        },
                        "to_id": {
                            "type": "string",
                            "description": "Ending concept ID"
                        },
                        "max_depth": {
                            "type": "integer",
                            "description": "Maximum depth to search (default: 5, max: 8)",
                            "default": 5
                        }
                    },
                    "required": ["from_id", "to_id"]
                }),
            ),
            make_tool(
                "get_prerequisites",
                "Get prerequisites for a concept in topological learning order",
                json!({
                    "type": "object",
                    "properties": {
                        "concept_id": {
                            "type": "string",
                            "description": "Concept identifier"
                        },
                        "depth": {
                            "type": "integer",
                            "description": "Depth to traverse (default: 3, max: 5)",
                            "default": 3
                        }
                    },
                    "required": ["concept_id"]
                }),
            ),
            make_tool(
                "get_concept_neighborhood",
                "Get local neighborhood subgraph around a concept",
                json!({
                    "type": "object",
                    "properties": {
                        "concept_id": {
                            "type": "string",
                            "description": "Concept identifier"
                        },
                        "radius": {
                            "type": "integer",
                            "description": "Radius in hops (default: 2, max: 3)",
                            "default": 2
                        },
                        "max_nodes": {
                            "type": "integer",
                            "description": "Maximum nodes to return (default: 30, max: 50)",
                            "default": 30
                        }
                    },
                    "required": ["concept_id"]
                }),
            ),
            make_tool(
                "get_dependents",
                "Get concepts that depend on this concept as a prerequisite",
                json!({
                    "type": "object",
                    "properties": {
                        "concept_id": {
                            "type": "string",
                            "description": "Concept identifier"
                        },
                        "depth": {
                            "type": "integer",
                            "description": "Depth to traverse (default: 2, max: 4)",
                            "default": 2
                        }
                    },
                    "required": ["concept_id"]
                }),
            ),
            make_tool(
                "get_central_concepts",
                "Get most connected concepts by degree centrality",
                json!({
                    "type": "object",
                    "properties": {
                        "category": {
                            "type": "string",
                            "description": "Optional category filter"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Limit number of results (default: 10, max: 25)",
                            "default": 10
                        }
                    }
                }),
            ),
            make_tool(
                "get_concept_sources",
                "Get all sources that introduce or cover a concept",
                json!({
                    "type": "object",
                    "properties": {
                        "concept_id": {
                            "type": "string",
                            "description": "Concept identifier"
                        }
                    },
                    "required": ["concept_id"]
                }),
            ),
            make_tool(
                "get_concept_variants",
                "Get all source-specific variants of a canonical concept",
                json!({
                    "type": "object",
                    "properties": {
                        "canonical_id": {
                            "type": "string",
                            "description": "Canonical concept identifier"
                        }
                    },
                    "required": ["canonical_id"]
                }),
            ),
            make_tool(
                "find_bridge_concepts",
                "Find concepts that bridge two categories by connecting to both",
                json!({
                    "type": "object",
                    "properties": {
                        "category_a": {
                            "type": "string",
                            "description": "First category"
                        },
                        "category_b": {
                            "type": "string",
                            "description": "Second category"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Limit number of results (default: 5, max: 15)",
                            "default": 5
                        }
                    },
                    "required": ["category_a", "category_b"]
                }),
            ),
            make_tool(
                "get_source_coverage",
                "Get all concepts introduced or covered by a source material",
                json!({
                    "type": "object",
                    "properties": {
                        "source_id": {
                            "type": "string",
                            "description": "Source identifier"
                        }
                    },
                    "required": ["source_id"]
                }),
            ),
        ]
    }

    fn call(&self, name: &str, args: Value) -> Option<ToolResult> {
        let state = self.state.clone();

        // --- Core graph tools ---

        if name == "graph_status" {
            return Some(Box::pin(async move {
                let response = tools::graph::graph_status(&state)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting graph status"))?;
                serialize_response(&response)
            }));
        }

        if name == "graph_stats" {
            return Some(Box::pin(async move {
                let response = tools::graph::graph_stats(&state)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting graph statistics"))?;
                serialize_response(&response)
            }));
        }

        if name == "graph_validate" {
            return Some(Box::pin(async move {
                let response = tools::graph::graph_validate(&state)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error validating graph"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_node" {
            return Some(Box::pin(async move {
                let args: GetNodeArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let response = tools::graph::get_node(&state, &args.node_id)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting node"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_node_edges" {
            return Some(Box::pin(async move {
                let args: GetNodeEdgesArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let response = tools::graph::get_node_edges(&state, &args.node_id, &args.direction)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting node edges"))?;
                serialize_response(&response)
            }));
        }

        // --- Graph query tools ---

        if name == "get_related_concepts" {
            return Some(Box::pin(async move {
                let args: GetRelatedConceptsArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let tool_params = tools::graph_query::GetRelatedConceptsParams {
                    concept_id: args.concept_id,
                    relationship_types: args.relationship_types,
                    direction: args.direction,
                    depth: args.depth,
                };
                let response = tools::get_related_concepts(&state, tool_params)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting related concepts"))?;
                serialize_response(&response)
            }));
        }

        if name == "find_concept_path" {
            return Some(Box::pin(async move {
                let args: FindConceptPathArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let tool_params = tools::graph_query::FindConceptPathParams {
                    from_id: args.from_id,
                    to_id: args.to_id,
                    max_depth: args.max_depth,
                };
                let response = tools::find_concept_path(&state, tool_params)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error finding concept path"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_prerequisites" {
            return Some(Box::pin(async move {
                let args: GetPrerequisitesArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let tool_params = tools::graph_query::GetPrerequisitesParams {
                    concept_id: args.concept_id,
                    depth: args.depth,
                };
                let response = tools::get_prerequisites(&state, tool_params)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting prerequisites"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_concept_neighborhood" {
            return Some(Box::pin(async move {
                let args: GetConceptNeighborhoodArgs =
                    serde_json::from_value(args).map_err(|e| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            format!("Invalid parameters: {}", e),
                            None,
                        )
                    })?;
                let tool_params = tools::graph_query::GetConceptNeighborhoodParams {
                    concept_id: args.concept_id,
                    radius: args.radius,
                    max_nodes: args.max_nodes,
                };
                let response = tools::get_concept_neighborhood(&state, tool_params)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting concept neighborhood"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_dependents" {
            return Some(Box::pin(async move {
                let args: GetDependentsArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let tool_params = tools::graph_query::GetDependentsParams {
                    concept_id: args.concept_id,
                    depth: args.depth,
                };
                let response = tools::get_dependents(&state, tool_params)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting dependents"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_central_concepts" {
            return Some(Box::pin(async move {
                let args: GetCentralConceptsArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let tool_params = tools::graph_query::GetCentralConceptsParams {
                    category: args.category,
                    limit: args.limit,
                };
                let response = tools::get_central_concepts(&state, tool_params)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting central concepts"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_concept_sources" {
            return Some(Box::pin(async move {
                let args: GetConceptSourcesArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let tool_params = tools::graph_query::GetConceptSourcesParams {
                    concept_id: args.concept_id,
                };
                let response = tools::get_concept_sources(&state, tool_params)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting concept sources"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_concept_variants" {
            return Some(Box::pin(async move {
                let args: GetConceptVariantsArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let tool_params = tools::graph_query::GetConceptVariantsParams {
                    canonical_id: args.canonical_id,
                };
                let response = tools::get_concept_variants(&state, tool_params)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting concept variants"))?;
                serialize_response(&response)
            }));
        }

        if name == "find_bridge_concepts" {
            return Some(Box::pin(async move {
                let args: FindBridgeConceptsArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let tool_params = tools::graph_query::FindBridgeConceptsParams {
                    category_a: args.category_a,
                    category_b: args.category_b,
                    limit: args.limit,
                };
                let response = tools::find_bridge_concepts(&state, tool_params)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error finding bridge concepts"))?;
                serialize_response(&response)
            }));
        }

        if name == "get_source_coverage" {
            return Some(Box::pin(async move {
                let args: GetSourceCoverageArgs = serde_json::from_value(args).map_err(|e| {
                    ErrorData::new(
                        ErrorCode::INVALID_PARAMS,
                        format!("Invalid parameters: {}", e),
                        None,
                    )
                })?;
                let tool_params = tools::graph_query::GetSourceCoverageParams {
                    source_id: args.source_id,
                };
                let response = tools::get_source_coverage(&state, tool_params)
                    .await
                    .map_err(|e| to_mcp_error(e, "Error getting source coverage"))?;
                serialize_response(&response)
            }));
        }

        None
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

/// Build the Music Theory MCP server from application state.
///
/// Composes domain-specific tool registries and resources into a single
/// [`FabrykMcpServer`] ready for stdio or HTTP transport.
pub fn build_server(state: AppState) -> FabrykMcpServer {
    let concept_tools = ConceptToolsRegistry::new(state.config.clone());
    let guide_tools = GuideToolsRegistry::new(state.config.clone());
    let source_tools = SourceToolsRegistry::new(state.clone());
    let search_tools = SearchToolsRegistry::new(state.clone());
    let graph_tools = GraphToolsRegistry::new(state.clone());
    let health_tools = HealthToolsRegistry::new(state.clone());

    let registry = CompositeRegistry::new()
        .add(concept_tools)
        .add(guide_tools)
        .add(source_tools)
        .add(search_tools)
        .add(graph_tools)
        .add(health_tools);

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
    fn test_default_direction() {
        assert_eq!(default_direction(), "both");
    }

    #[test]
    fn test_default_depth_values() {
        assert_eq!(default_depth_1(), 1);
        assert_eq!(default_depth_2(), 2);
        assert_eq!(default_depth_3(), 3);
        assert_eq!(default_depth_5(), 5);
    }

    #[test]
    fn test_default_neighborhood_values() {
        assert_eq!(default_radius(), 2);
        assert_eq!(default_max_nodes(), 30);
    }

    #[test]
    fn test_default_limit_values() {
        assert_eq!(default_limit_5(), 5);
        assert_eq!(default_limit_10(), 10);
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

    #[test]
    fn test_concept_tools_registry_tool_count() {
        let config = Config::load().unwrap();
        let registry = ConceptToolsRegistry::new(config);
        assert_eq!(registry.tools().len(), 3);
    }

    #[test]
    fn test_guide_tools_registry_tool_count() {
        let config = Config::load().unwrap();
        let registry = GuideToolsRegistry::new(config);
        assert_eq!(registry.tools().len(), 2);
    }

    #[tokio::test]
    async fn test_source_tools_registry_tool_count() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let registry = SourceToolsRegistry::new(state);
        assert_eq!(registry.tools().len(), 5);
    }

    #[tokio::test]
    async fn test_search_tools_registry_tool_count() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let registry = SearchToolsRegistry::new(state);
        assert_eq!(registry.tools().len(), 2);
    }

    #[tokio::test]
    async fn test_graph_tools_registry_tool_count() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let registry = GraphToolsRegistry::new(state);
        assert_eq!(registry.tools().len(), 15);
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

        // 3 concept + 2 guide + 5 source + 2 search + 15 graph + 1 health = 28
        assert_eq!(server.registry().tool_count(), 28);
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
            "semantic_search",
            "health",
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
        ];

        for tool_name in &expected_tools {
            assert!(registry.has_tool(tool_name), "Missing tool: {}", tool_name);
        }
    }

    // --- Argument deserialization ---

    #[test]
    fn test_list_concepts_args_deserialization() {
        let json = r#"{"category":"harmony","limit":10}"#;
        let args: ListConceptsArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.category, Some("harmony".to_string()));
        assert_eq!(args.limit, Some(10));
    }

    #[test]
    fn test_list_concepts_args_defaults() {
        let json = r#"{}"#;
        let args: ListConceptsArgs = serde_json::from_str(json).unwrap();
        assert!(args.category.is_none());
        assert!(args.limit.is_none());
    }

    #[test]
    fn test_search_concepts_args_deserialization() {
        let json = r#"{"query":"harmony"}"#;
        let args: SearchConceptsArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.query, "harmony");
        assert_eq!(args.limit, 10); // default
    }

    #[test]
    fn test_search_concepts_args_with_limit() {
        let json = r#"{"query":"chord","limit":5}"#;
        let args: SearchConceptsArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.query, "chord");
        assert_eq!(args.limit, 5);
    }

    #[test]
    fn test_get_source_chapter_args_deserialization() {
        let json = r#"{"source_id":"test","chapter":"ch1"}"#;
        let args: GetSourceChapterArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.source_id, "test");
        assert_eq!(args.chapter, "ch1");
        assert!(args.section.is_none());
    }

    #[test]
    fn test_get_source_chapter_args_with_section() {
        let json = r#"{"source_id":"test","chapter":"ch1","section":"pp. 23-28"}"#;
        let args: GetSourceChapterArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.section, Some("pp. 23-28".to_string()));
    }

    #[test]
    fn test_get_related_concepts_args_defaults() {
        let json = r#"{"concept_id":"test"}"#;
        let args: GetRelatedConceptsArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.concept_id, "test");
        assert_eq!(args.direction, "both");
        assert_eq!(args.depth, 1);
        assert!(args.relationship_types.is_none());
    }

    #[test]
    fn test_find_concept_path_args_defaults() {
        let json = r#"{"from_id":"a","to_id":"b"}"#;
        let args: FindConceptPathArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.from_id, "a");
        assert_eq!(args.to_id, "b");
        assert_eq!(args.max_depth, 5);
    }

    #[test]
    fn test_get_concept_neighborhood_args_defaults() {
        let json = r#"{"concept_id":"test"}"#;
        let args: GetConceptNeighborhoodArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.concept_id, "test");
        assert_eq!(args.radius, 2);
        assert_eq!(args.max_nodes, 30);
    }

    #[test]
    fn test_find_bridge_concepts_args_defaults() {
        let json = r#"{"category_a":"harmony","category_b":"rhythm"}"#;
        let args: FindBridgeConceptsArgs = serde_json::from_str(json).unwrap();
        assert_eq!(args.category_a, "harmony");
        assert_eq!(args.category_b, "rhythm");
        assert_eq!(args.limit, 5);
    }

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

    #[test]
    fn test_concept_registry_unknown_tool_returns_none() {
        let config = Config::load().unwrap();
        let registry = ConceptToolsRegistry::new(config);
        assert!(registry.call("nonexistent", json!({})).is_none());
    }

    #[test]
    fn test_guide_registry_unknown_tool_returns_none() {
        let config = Config::load().unwrap();
        let registry = GuideToolsRegistry::new(config);
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
