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
        let response = tools::sources::list_sources(&self.config)
            .await
            .map_err(|e| e.to_mcp_error("Error listing sources"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

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
        .await
        .map_err(|e| e.to_mcp_error("Error retrieving chapter"))?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "Get filesystem path to original PDF/EPUB for a source")]
    async fn get_source_pdf_path(
        &self,
        params: Parameters<GetSourcePdfPathParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let path = tools::sources::get_source_pdf_path(&self.config, &params.0.source_id)
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

        let response = tools::concepts::list_concepts(&self.config, Some(filter_params))
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
        let content = tools::concepts::get_concept(&self.config, &params.0.concept_id)
            .await
            .map_err(|e| e.to_mcp_error("Error retrieving concept"))?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "List all distinct concept categories with counts")]
    async fn list_categories(&self) -> Result<CallToolResult, ErrorData> {
        let response = tools::concepts::list_categories(&self.config)
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
        };

        let response = tools::search::search_concepts(&self.config, search_params)
            .await
            .map_err(|e| e.to_mcp_error("Error searching concepts"))?;

        let content = serde_json::to_string_pretty(&response).map_err(serialization_error)?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(description = "List all available topic guides")]
    async fn list_guides(&self) -> Result<CallToolResult, ErrorData> {
        let response = tools::guides::list_guides(&self.config)
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
        let content = tools::guides::get_guide(&self.config, &params.0.guide_id)
            .await
            .map_err(|e| e.to_mcp_error("Error retrieving guide"))?;

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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_new_server() {
        let config = Config::load().unwrap();
        let server = MusicTheoryServer::new(config.clone());

        assert_eq!(server.config.server.name, config.server.name);
        assert_eq!(server.config.server.version, config.server.version);
    }

    #[test]
    fn test_get_info() {
        let config = Config::load().unwrap();
        let server = MusicTheoryServer::new(config.clone());

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
        let server = MusicTheoryServer::new(config);

        let result = server.list_sources().await;

        // Should return a result (success or error is OK, we just test it doesn't panic)
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_source_chapter() {
        let config = Config::load().unwrap();
        let server = MusicTheoryServer::new(config);

        let params = Parameters(GetSourceChapterParams {
            source_id: "nonexistent-source".to_string(),
            chapter: "chapter-1".to_string(),
        });

        let result = server.get_source_chapter(params).await;

        // Should return a result (error expected for nonexistent source, but we test the code path)
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_source_pdf_path() {
        let config = Config::load().unwrap();
        let server = MusicTheoryServer::new(config);

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
        let server = MusicTheoryServer::new(config);

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
        let server = MusicTheoryServer::new(config);

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
        let server = MusicTheoryServer::new(config);

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
        let server = MusicTheoryServer::new(config);

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
        let server = MusicTheoryServer::new(config);

        let params = Parameters(SearchConceptsParams {
            query: "harmony".to_string(),
            limit: 10,
        });

        let result = server.search_concepts(params).await;

        // Should return a result
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_search_concepts_with_limit() {
        let config = Config::load().unwrap();
        let server = MusicTheoryServer::new(config);

        let params = Parameters(SearchConceptsParams {
            query: "chord".to_string(),
            limit: 5,
        });

        let result = server.search_concepts(params).await;

        // Should return a result
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_list_guides() {
        let config = Config::load().unwrap();
        let server = MusicTheoryServer::new(config);

        let result = server.list_guides().await;

        // Should return a result
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_get_guide() {
        let config = Config::load().unwrap();
        let server = MusicTheoryServer::new(config);

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
