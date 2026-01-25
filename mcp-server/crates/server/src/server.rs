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

use crate::config::Config;
use crate::resources;
use crate::tools;

/// Music Theory MCP Server implementation.
#[derive(Clone)]
pub struct MusicTheoryServer {
    pub config: Config,
    tool_router: ToolRouter<Self>,
}

// Parameter types for tools
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetSourceChapterParams {
    source_id: String,
    chapter: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetSourcePdfPathParams {
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
}

fn default_limit() -> usize {
    10
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetGuideParams {
    guide_id: String,
}

#[tool_router]
impl MusicTheoryServer {
    /// Create a new server instance.
    pub fn new(config: Config) -> Self {
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

        Self {
            config,
            tool_router,
        }
    }

    #[tool(description = "List all available source materials with metadata")]
    async fn list_sources(&self) -> Result<CallToolResult, ErrorData> {
        let response = tools::sources::list_sources(&self.config).map_err(|e| {
            let (code, msg) = if e.is_not_found() {
                (ErrorCode::RESOURCE_NOT_FOUND, format!("Not found: {}", e))
            } else if e.is_config() {
                (
                    ErrorCode::INVALID_PARAMS,
                    format!("Configuration error: {}", e),
                )
            } else {
                (
                    ErrorCode::INTERNAL_ERROR,
                    format!("Error listing sources: {}", e),
                )
            };
            ErrorData::new(code, msg, None)
        })?;

        let content = serde_json::to_string_pretty(&response).map_err(|e| {
            ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Serialization error: {}", e),
                None,
            )
        })?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Retrieve a specific chapter from a source material")]
    async fn get_source_chapter(
        &self,
        params: Parameters<GetSourceChapterParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let content = tools::sources::get_source_chapter(
            &self.config,
            &params.0.source_id,
            &params.0.chapter,
        )
        .map_err(|e| {
            let (code, msg) = if e.is_not_found() {
                (ErrorCode::RESOURCE_NOT_FOUND, format!("Not found: {}", e))
            } else if e.is_config() {
                (
                    ErrorCode::INVALID_PARAMS,
                    format!("Configuration error: {}", e),
                )
            } else {
                (
                    ErrorCode::INTERNAL_ERROR,
                    format!("Error retrieving chapter: {}", e),
                )
            };
            ErrorData::new(code, msg, None)
        })?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get filesystem path to original PDF/EPUB for a source")]
    async fn get_source_pdf_path(
        &self,
        params: Parameters<GetSourcePdfPathParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let path = tools::sources::get_source_pdf_path(&self.config, &params.0.source_id).map_err(
            |e| {
                let (code, msg) = if e.is_not_found() {
                    (ErrorCode::RESOURCE_NOT_FOUND, format!("Not found: {}", e))
                } else if e.is_config() {
                    (
                        ErrorCode::INVALID_PARAMS,
                        format!("Configuration error: {}", e),
                    )
                } else {
                    (
                        ErrorCode::INTERNAL_ERROR,
                        format!("Error getting PDF path: {}", e),
                    )
                };
                ErrorData::new(code, msg, None)
            },
        )?;

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

        let response =
            tools::concepts::list_concepts(&self.config, Some(filter_params)).map_err(|e| {
                let (code, msg) = if e.is_not_found() {
                    (ErrorCode::RESOURCE_NOT_FOUND, format!("Not found: {}", e))
                } else if e.is_config() {
                    (
                        ErrorCode::INVALID_PARAMS,
                        format!("Configuration error: {}", e),
                    )
                } else {
                    (
                        ErrorCode::INTERNAL_ERROR,
                        format!("Error listing concepts: {}", e),
                    )
                };
                ErrorData::new(code, msg, None)
            })?;

        let content = serde_json::to_string_pretty(&response).map_err(|e| {
            ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Serialization error: {}", e),
                None,
            )
        })?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Retrieve a specific concept card")]
    async fn get_concept(
        &self,
        params: Parameters<GetConceptParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let content =
            tools::concepts::get_concept(&self.config, &params.0.concept_id).map_err(|e| {
                let (code, msg) = if e.is_not_found() {
                    (ErrorCode::RESOURCE_NOT_FOUND, format!("Not found: {}", e))
                } else if e.is_config() {
                    (
                        ErrorCode::INVALID_PARAMS,
                        format!("Configuration error: {}", e),
                    )
                } else {
                    (
                        ErrorCode::INTERNAL_ERROR,
                        format!("Error retrieving concept: {}", e),
                    )
                };
                ErrorData::new(code, msg, None)
            })?;

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
        };

        let response =
            tools::search::search_concepts(&self.config, search_params).map_err(|e| {
                let (code, msg) = if e.is_not_found() {
                    (ErrorCode::RESOURCE_NOT_FOUND, format!("Not found: {}", e))
                } else if e.is_config() {
                    (
                        ErrorCode::INVALID_PARAMS,
                        format!("Configuration error: {}", e),
                    )
                } else {
                    (
                        ErrorCode::INTERNAL_ERROR,
                        format!("Error searching concepts: {}", e),
                    )
                };
                ErrorData::new(code, msg, None)
            })?;

        let content = serde_json::to_string_pretty(&response).map_err(|e| {
            ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Serialization error: {}", e),
                None,
            )
        })?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "List all available topic guides")]
    async fn list_guides(&self) -> Result<CallToolResult, ErrorData> {
        let response = tools::guides::list_guides(&self.config).map_err(|e| {
            let (code, msg) = if e.is_not_found() {
                (ErrorCode::RESOURCE_NOT_FOUND, format!("Not found: {}", e))
            } else if e.is_config() {
                (
                    ErrorCode::INVALID_PARAMS,
                    format!("Configuration error: {}", e),
                )
            } else {
                (
                    ErrorCode::INTERNAL_ERROR,
                    format!("Error listing guides: {}", e),
                )
            };
            ErrorData::new(code, msg, None)
        })?;

        let content = serde_json::to_string_pretty(&response).map_err(|e| {
            ErrorData::new(
                ErrorCode::INTERNAL_ERROR,
                format!("Serialization error: {}", e),
                None,
            )
        })?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Retrieve a specific topic guide")]
    async fn get_guide(
        &self,
        params: Parameters<GetGuideParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let content = tools::guides::get_guide(&self.config, &params.0.guide_id).map_err(|e| {
            let (code, msg) = if e.is_not_found() {
                (ErrorCode::RESOURCE_NOT_FOUND, format!("Not found: {}", e))
            } else if e.is_config() {
                (
                    ErrorCode::INVALID_PARAMS,
                    format!("Configuration error: {}", e),
                )
            } else {
                (
                    ErrorCode::INTERNAL_ERROR,
                    format!("Error retrieving guide: {}", e),
                )
            };
            ErrorData::new(code, msg, None)
        })?;

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
                name: self.config.server.name.clone(),
                title: None,
                version: self.config.server.version.clone(),
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
        match resources::get_resource(&self.config, &request.uri) {
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
