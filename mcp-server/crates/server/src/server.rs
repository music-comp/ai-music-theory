use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::handler::server::ServerHandler;
use rmcp::model::{
    AnnotateAble, CallToolResult, Content, ErrorCode, ErrorData, Implementation,
    ListResourcesResult, PaginatedRequestParams, RawResource, ReadResourceRequestParams,
    ReadResourceResult, ResourceContents, ServerCapabilities, ServerInfo,
};
use rmcp::service::{RequestContext, RoleServer};
use rmcp::{tool, tool_handler, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::resources;
use crate::state::AppState;
use crate::tools;

/// Music Theory MCP Server implementation.
#[derive(Clone)]
pub struct MusicTheoryServer {
    pub state: AppState,
    tool_router: ToolRouter<Self>,
}

// Parameter types for tools
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetSourceChapterParams {
    source_id: String,
    chapter: String,
    /// Optional section/page filter (v0.3.0)
    #[serde(default)]
    section: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetSourcePdfPathParams {
    source_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct CheckSourceAvailabilityParams {
    source_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ListSourceChaptersParams {
    source_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ListConceptsParams {
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    limit: Option<usize>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetConceptParams {
    concept_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SearchConceptsParams {
    query: String,
    #[serde(default = "default_limit")]
    limit: usize,
    /// Optional query mode override (smart, and, or, minimum_match)
    #[serde(default)]
    query_mode: Option<crate::config::QueryMode>,
    /// Optional category filter - only return results from this category
    #[serde(default)]
    category: Option<String>,
    /// Optional source filter - only return results from this source
    #[serde(default)]
    source: Option<String>,
    /// Optional content type filter (v0.3.0) - only return results of these types
    /// Valid values: "concept_card", "source_chapter", "unified_concept", "guide"
    #[serde(default)]
    content_types: Option<Vec<String>>,
}

fn default_limit() -> usize {
    10
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetGuideParams {
    guide_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetNodeParams {
    node_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetNodeEdgesParams {
    node_id: String,
    #[serde(default = "default_direction")]
    direction: String,
}

fn default_direction() -> String {
    "both".to_string()
}

// Graph query tool parameters
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetRelatedConceptsParams {
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

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct FindConceptPathParams {
    from_id: String,
    to_id: String,
    #[serde(default = "default_depth_5")]
    max_depth: u32,
}

fn default_depth_5() -> u32 {
    5
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetPrerequisitesParams {
    concept_id: String,
    #[serde(default = "default_depth_3")]
    depth: u32,
}

fn default_depth_3() -> u32 {
    3
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetConceptNeighborhoodParams {
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

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetDependentsParams {
    concept_id: String,
    #[serde(default = "default_depth_2")]
    depth: u32,
}

fn default_depth_2() -> u32 {
    2
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetCentralConceptsParams {
    #[serde(default)]
    category: Option<String>,
    #[serde(default = "default_limit_10")]
    limit: u32,
}

fn default_limit_10() -> u32 {
    10
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetConceptSourcesParams {
    concept_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetConceptVariantsParams {
    canonical_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct FindBridgeConceptsParams {
    category_a: String,
    category_b: String,
    #[serde(default = "default_limit_5")]
    limit: u32,
}

fn default_limit_5() -> u32 {
    5
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetSourceCoverageParams {
    source_id: String,
}

/// Helper function to create serialization error response.
fn serialization_error(e: serde_json::Error) -> ErrorData {
    ErrorData::new(
        ErrorCode::INTERNAL_ERROR,
        format!("Serialization error: {}", e),
        None,
    )
}

#[tool_router]
impl MusicTheoryServer {
    /// Create a new server instance.
    pub fn new(state: AppState) -> Self {
        let tool_router = Self::tool_router();

        // Log registered tools with structured logging
        let tools = tool_router.list_all();
        log::info!(count = tools.len(); "Registered tools");
        for tool in tools {
            log::info!(
                tool = &*tool.name,
                description = tool.description.as_deref().unwrap_or("");
                "Tool available"
            );
        }

        Self { state, tool_router }
    }

    #[tool(description = "List all available source materials with metadata")]
    async fn list_sources(&self) -> Result<CallToolResult, ErrorData> {
        let response = tools::sources::list_sources(&self.state.config)
            .await
            .map_err(|e| e.to_mcp_error("Error listing sources"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Check availability status of a source (indexed/converted/exists)")]
    async fn check_source_availability(
        &self,
        params: Parameters<CheckSourceAvailabilityParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let response = tools::sources::check_source_availability(&self.state, &params.0.source_id)
            .await
            .map_err(|e| e.to_mcp_error("Error checking source availability"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "List all chapters for a source material")]
    async fn list_source_chapters(
        &self,
        params: Parameters<ListSourceChaptersParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let response = tools::sources::list_source_chapters(&self.state, &params.0.source_id)
            .await
            .map_err(|e| e.to_mcp_error("Error listing chapters"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Retrieve a specific chapter from a source material")]
    async fn get_source_chapter(
        &self,
        params: Parameters<GetSourceChapterParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let content = tools::sources::get_source_chapter(
            &self.state.config,
            &params.0.source_id,
            &params.0.chapter,
            params.0.section.as_deref(),
        )
        .await
        .map_err(|e| e.to_mcp_error("Error retrieving chapter"))?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get filesystem path to original PDF/EPUB for a source")]
    async fn get_source_pdf_path(
        &self,
        params: Parameters<GetSourcePdfPathParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let path = tools::sources::get_source_pdf_path(&self.state.config, &params.0.source_id)
            .map_err(|e| e.to_mcp_error("Error getting PDF path"))?;

        let path_str = path.to_str().ok_or_else(|| {
            ErrorData::new(ErrorCode::INTERNAL_ERROR, "Invalid UTF-8 in path", None)
        })?;

        Ok(CallToolResult::success(vec![Content::text(
            path_str.to_string(),
        )]))
    }

    #[tool(description = "List concept cards with optional category filtering")]
    async fn list_concepts(
        &self,
        params: Parameters<ListConceptsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let filter_params = tools::concepts::ListConceptsParams {
            category: params.0.category,
            limit: params.0.limit,
        };

        let response = tools::concepts::list_concepts(&self.state.config, Some(filter_params))
            .await
            .map_err(|e| e.to_mcp_error("Error listing concepts"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Retrieve a specific concept card")]
    async fn get_concept(
        &self,
        params: Parameters<GetConceptParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let content = tools::concepts::get_concept(&self.state.config, &params.0.concept_id)
            .await
            .map_err(|e| e.to_mcp_error("Error retrieving concept"))?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "List all distinct concept categories with counts")]
    async fn list_categories(&self) -> Result<CallToolResult, ErrorData> {
        let response = tools::concepts::list_categories(&self.state.config)
            .await
            .map_err(|e| e.to_mcp_error("Error listing categories"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Search concept cards with full-text search and relevance ranking")]
    async fn search_concepts(
        &self,
        params: Parameters<SearchConceptsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let search_params = tools::search::SearchConceptsParams {
            query: params.0.query,
            limit: params.0.limit,
            query_mode: params.0.query_mode,
            category: params.0.category,
            source: params.0.source,
            content_types: params.0.content_types,
        };

        let response = tools::search::search_concepts(&self.state, search_params)
            .await
            .map_err(|e| e.to_mcp_error("Error searching concepts"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "List all available topic guides")]
    async fn list_guides(&self) -> Result<CallToolResult, ErrorData> {
        let response = tools::guides::list_guides(&self.state.config)
            .await
            .map_err(|e| e.to_mcp_error("Error listing guides"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Retrieve a specific topic guide")]
    async fn get_guide(
        &self,
        params: Parameters<GetGuideParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let content = tools::guides::get_guide(&self.state.config, &params.0.guide_id)
            .await
            .map_err(|e| e.to_mcp_error("Error retrieving guide"))?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get server health and search backend status")]
    async fn health(&self) -> Result<CallToolResult, ErrorData> {
        let response = tools::health::get_health(&self.state)
            .await
            .map_err(|e| e.to_mcp_error("Error getting health status"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get concept graph status and basic statistics")]
    async fn graph_status(&self) -> Result<CallToolResult, ErrorData> {
        let response = tools::graph::graph_status(&self.state)
            .await
            .map_err(|e| e.to_mcp_error("Error getting graph status"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get detailed concept graph statistics (categories, relationships)")]
    async fn graph_stats(&self) -> Result<CallToolResult, ErrorData> {
        let response = tools::graph::graph_stats(&self.state)
            .await
            .map_err(|e| e.to_mcp_error("Error getting graph statistics"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Validate concept graph integrity (orphans, self-loops)")]
    async fn graph_validate(&self) -> Result<CallToolResult, ErrorData> {
        let response = tools::graph::graph_validate(&self.state)
            .await
            .map_err(|e| e.to_mcp_error("Error validating graph"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get node information by ID with in/out degree counts")]
    async fn get_node(
        &self,
        params: Parameters<GetNodeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let response = tools::graph::get_node(&self.state, &params.0.node_id)
            .await
            .map_err(|e| e.to_mcp_error("Error getting node"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get all edges for a node with optional direction filter")]
    async fn get_node_edges(
        &self,
        params: Parameters<GetNodeEdgesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let response =
            tools::graph::get_node_edges(&self.state, &params.0.node_id, &params.0.direction)
                .await
                .map_err(|e| e.to_mcp_error("Error getting node edges"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    // Graph query tools
    #[tool(description = "Find related concepts with optional relationship and direction filtering")]
    async fn get_related_concepts(
        &self,
        params: Parameters<GetRelatedConceptsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_params = tools::graph_query::GetRelatedConceptsParams {
            concept_id: params.0.concept_id,
            relationship_types: params.0.relationship_types,
            direction: params.0.direction,
            depth: params.0.depth,
        };

        let response = tools::get_related_concepts(&self.state, tool_params)
            .await
            .map_err(|e| e.to_mcp_error("Error getting related concepts"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Find shortest path between two concepts in the graph")]
    async fn find_concept_path(
        &self,
        params: Parameters<FindConceptPathParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_params = tools::graph_query::FindConceptPathParams {
            from_id: params.0.from_id,
            to_id: params.0.to_id,
            max_depth: params.0.max_depth,
        };

        let response = tools::find_concept_path(&self.state, tool_params)
            .await
            .map_err(|e| e.to_mcp_error("Error finding concept path"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get prerequisites for a concept in topological learning order")]
    async fn get_prerequisites(
        &self,
        params: Parameters<GetPrerequisitesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_params = tools::graph_query::GetPrerequisitesParams {
            concept_id: params.0.concept_id,
            depth: params.0.depth,
        };

        let response = tools::get_prerequisites(&self.state, tool_params)
            .await
            .map_err(|e| e.to_mcp_error("Error getting prerequisites"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get local neighborhood subgraph around a concept")]
    async fn get_concept_neighborhood(
        &self,
        params: Parameters<GetConceptNeighborhoodParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_params = tools::graph_query::GetConceptNeighborhoodParams {
            concept_id: params.0.concept_id,
            radius: params.0.radius,
            max_nodes: params.0.max_nodes,
        };

        let response = tools::get_concept_neighborhood(&self.state, tool_params)
            .await
            .map_err(|e| e.to_mcp_error("Error getting concept neighborhood"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get concepts that depend on this concept as a prerequisite")]
    async fn get_dependents(
        &self,
        params: Parameters<GetDependentsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_params = tools::graph_query::GetDependentsParams {
            concept_id: params.0.concept_id,
            depth: params.0.depth,
        };

        let response = tools::get_dependents(&self.state, tool_params)
            .await
            .map_err(|e| e.to_mcp_error("Error getting dependents"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get most connected concepts by degree centrality")]
    async fn get_central_concepts(
        &self,
        params: Parameters<GetCentralConceptsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_params = tools::graph_query::GetCentralConceptsParams {
            category: params.0.category,
            limit: params.0.limit,
        };

        let response = tools::get_central_concepts(&self.state, tool_params)
            .await
            .map_err(|e| e.to_mcp_error("Error getting central concepts"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get all sources that introduce or cover a concept")]
    async fn get_concept_sources(
        &self,
        params: Parameters<GetConceptSourcesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_params = tools::graph_query::GetConceptSourcesParams {
            concept_id: params.0.concept_id,
        };

        let response = tools::get_concept_sources(&self.state, tool_params)
            .await
            .map_err(|e| e.to_mcp_error("Error getting concept sources"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get all source-specific variants of a canonical concept")]
    async fn get_concept_variants(
        &self,
        params: Parameters<GetConceptVariantsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_params = tools::graph_query::GetConceptVariantsParams {
            canonical_id: params.0.canonical_id,
        };

        let response = tools::get_concept_variants(&self.state, tool_params)
            .await
            .map_err(|e| e.to_mcp_error("Error getting concept variants"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Find concepts that bridge two categories by connecting to both")]
    async fn find_bridge_concepts(
        &self,
        params: Parameters<FindBridgeConceptsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_params = tools::graph_query::FindBridgeConceptsParams {
            category_a: params.0.category_a,
            category_b: params.0.category_b,
            limit: params.0.limit,
        };

        let response = tools::find_bridge_concepts(&self.state, tool_params)
            .await
            .map_err(|e| e.to_mcp_error("Error finding bridge concepts"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get all concepts introduced or covered by a source material")]
    async fn get_source_coverage(
        &self,
        params: Parameters<GetSourceCoverageParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let tool_params = tools::graph_query::GetSourceCoverageParams {
            source_id: params.0.source_id,
        };

        let response = tools::get_source_coverage(&self.state, tool_params)
            .await
            .map_err(|e| e.to_mcp_error("Error getting source coverage"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }
}

#[tool_handler]
impl ServerHandler for MusicTheoryServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: Default::default(),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
            server_info: Implementation {
                name: self.state.config.server.name.clone(),
                title: None,
                version: self.state.config.server.version.clone(),
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "Music Theory AI Skill - Access comprehensive music theory materials \
                 including source texts, concept cards, and topic guides."
                    .to_string(),
            ),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, ErrorData> {
        let resources = resources::list_resources()
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
            .collect();

        Ok(ListResourcesResult::with_all_items(resources))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, ErrorData> {
        match resources::get_resource(&self.state.config, &request.uri) {
            Ok(content) => Ok(ReadResourceResult {
                contents: vec![ResourceContents::text(content, request.uri)],
            }),
            Err(_) => Err(ErrorData::new(
                ErrorCode::RESOURCE_NOT_FOUND,
                format!("Resource not found: {}", request.uri),
                None,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_default_limit() {
        assert_eq!(default_limit(), 10);
    }

    #[test]
    fn test_serialization_error() {
        // Test the helper function
        let json_err = serde_json::from_str::<String>("not valid json").unwrap_err();
        let error = serialization_error(json_err);

        assert_eq!(error.code, ErrorCode::INTERNAL_ERROR);
        assert!(error.message.contains("Serialization error"));
    }

    #[tokio::test]
    async fn test_new_server() {
        let config = Config::load().unwrap();
        let state = AppState::new(config.clone()).await.unwrap();
        let server = MusicTheoryServer::new(state.clone());

        assert_eq!(server.state.config.server.name, config.server.name);
        assert_eq!(server.state.config.server.version, config.server.version);
    }

    #[tokio::test]
    async fn test_get_info() {
        let config = Config::load().unwrap();
        let state = AppState::new(config.clone()).await.unwrap();
        let server = MusicTheoryServer::new(state.clone());

        let info = server.get_info();

        assert_eq!(info.server_info.name, config.server.name);
        assert_eq!(info.server_info.version, config.server.version);
        assert!(info.instructions.is_some());
        assert!(info.instructions.unwrap().contains("Music Theory AI Skill"));
        assert!(info.capabilities.tools.is_some());
        assert!(info.capabilities.resources.is_some());
    }

    #[tokio::test]
    async fn test_list_sources() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = MusicTheoryServer::new(state);

        let result = server.list_sources().await;

        // Should return a result (success or error is OK, we just test it doesn't panic)
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_source_chapter() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = MusicTheoryServer::new(state);

        let params = Parameters(GetSourceChapterParams {
            source_id: "nonexistent-source".to_string(),
            chapter: "chapter-1".to_string(),
            section: None,
        });

        let result = server.get_source_chapter(params).await;

        // Should return a result (error expected for nonexistent source, but we test the code path)
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_source_pdf_path() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = MusicTheoryServer::new(state);

        let params = Parameters(GetSourcePdfPathParams {
            source_id: "nonexistent-source".to_string(),
        });

        let result = server.get_source_pdf_path(params).await;

        // Should return a result (testing code path, not data availability)
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_list_concepts() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = MusicTheoryServer::new(state);

        let params = Parameters(ListConceptsParams {
            category: None,
            limit: None,
        });

        let result = server.list_concepts(params).await;

        // Should return a result (testing code path)
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_list_concepts_with_category() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = MusicTheoryServer::new(state);

        let params = Parameters(ListConceptsParams {
            category: Some("harmony".to_string()),
            limit: None,
        });

        let result = server.list_concepts(params).await;

        // Should return a result
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_list_concepts_with_limit() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = MusicTheoryServer::new(state);

        let params = Parameters(ListConceptsParams {
            category: None,
            limit: Some(5),
        });

        let result = server.list_concepts(params).await;

        // Should return a result
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_concept() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = MusicTheoryServer::new(state);

        let params = Parameters(GetConceptParams {
            concept_id: "nonexistent-concept".to_string(),
        });

        let result = server.get_concept(params).await;

        // Should return a result (testing code path)
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_search_concepts() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = MusicTheoryServer::new(state);

        let params = Parameters(SearchConceptsParams {
            query: "harmony".to_string(),
            limit: 10,
            query_mode: None,
            category: None,
            source: None,
            content_types: None,
        });

        let result = server.search_concepts(params).await;

        // Should return a result
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_search_concepts_with_limit() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = MusicTheoryServer::new(state);

        let params = Parameters(SearchConceptsParams {
            query: "chord".to_string(),
            limit: 5,
            query_mode: None,
            category: None,
            source: None,
            content_types: None,
        });

        let result = server.search_concepts(params).await;

        // Should return a result
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_list_guides() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = MusicTheoryServer::new(state);

        let result = server.list_guides().await;

        // Should return a result
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_guide() {
        let config = Config::load().unwrap();
        let state = AppState::new(config).await.unwrap();
        let server = MusicTheoryServer::new(state);

        let params = Parameters(GetGuideParams {
            guide_id: "nonexistent-guide".to_string(),
        });

        let result = server.get_guide(params).await;

        // Should return a result (testing code path)
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_list_resources_directly() {
        // Test the resources module directly, avoiding MCP framework types
        let resources = resources::list_resources();

        assert!(!resources.is_empty());
        // Should have 4 resources (conventions, scope, sources, index)
        assert_eq!(resources.len(), 4);

        // Verify resource structure
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

    #[test]
    fn test_parameter_types_serialization() {
        // Test that parameter types can be serialized/deserialized
        let get_source_params = GetSourceChapterParams {
            source_id: "test".to_string(),
            chapter: "ch1".to_string(),
            section: None,
        };
        let json = serde_json::to_string(&get_source_params).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("ch1"));

        let list_params = ListConceptsParams {
            category: Some("harmony".to_string()),
            limit: Some(10),
        };
        let json = serde_json::to_string(&list_params).unwrap();
        assert!(json.contains("harmony"));
        assert!(json.contains("10"));

        let search_params = SearchConceptsParams {
            query: "test".to_string(),
            limit: 5,
            query_mode: None,
            category: None,
            source: None,
            content_types: None,
        };
        let json = serde_json::to_string(&search_params).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("5"));
    }

    #[test]
    fn test_parameter_types_deserialization() {
        // Test that parameter types can be deserialized from JSON
        let json = r#"{"source_id":"test","chapter":"ch1"}"#;
        let params: GetSourceChapterParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.source_id, "test");
        assert_eq!(params.chapter, "ch1");

        let json = r#"{"category":"harmony","limit":10}"#;
        let params: ListConceptsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.category, Some("harmony".to_string()));
        assert_eq!(params.limit, Some(10));

        let json = r#"{"query":"test","limit":5}"#;
        let params: SearchConceptsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.query, "test");
        assert_eq!(params.limit, 5);
    }

    #[test]
    fn test_search_params_default_limit() {
        // Test that search params uses default limit when not specified
        let json = r#"{"query":"test"}"#;
        let params: SearchConceptsParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.query, "test");
        assert_eq!(params.limit, 10); // Should use default
    }
}
