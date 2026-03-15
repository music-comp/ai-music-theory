use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::QueryMode;
use crate::error::Result;
use crate::search::SearchParams;
use crate::state::AppState;

/// A search result item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub path: String,
    pub snippet: String,
    pub relevance: f32,
    /// Content type (v0.3.0): "concept_card" | "source_chapter" | "unified_concept" | "guide"
    pub content_type: String,
    /// Fine-grained location (v0.3.0): "pp. 23-28" or "Section 2.3"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
}

/// Response for search_concepts tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchConceptsResponse {
    pub results: Vec<SearchResult>,
    pub total: usize,
    pub query: String,
    /// The backend used for this search ("simple" or "tantivy")
    pub backend: String,
}

/// Parameters for search_concepts tool.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct SearchConceptsParams {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
    /// Optional query mode override (smart, and, or, minimum_match)
    /// Only used by TantivySearch backend (requires fts feature)
    #[serde(default)]
    pub query_mode: Option<QueryMode>,
    /// Optional category filter - only return results from this category
    #[serde(default)]
    pub category: Option<String>,
    /// Optional source filter - only return results from this source
    #[serde(default)]
    pub source: Option<String>,
    /// Optional content type filter (v0.3.0) - only return results of these types
    /// Valid values: "concept_card", "source_chapter", "unified_concept", "guide"
    /// If None, searches all content types
    #[serde(default)]
    pub content_types: Option<Vec<String>>,
}

fn default_limit() -> usize {
    10
}

/// Search across concept cards for a query.
///
/// Uses the currently active search backend from AppState.
/// Returns FTS results if ready, otherwise uses simple search.
///
/// Converts between the project's MCP-contract types (`SearchConceptsParams`,
/// `SearchResult`) and fabryk's types (`SearchParams`, `fabryk::fts::SearchResult`).
pub async fn search_concepts(
    state: &AppState,
    params: SearchConceptsParams,
) -> Result<SearchConceptsResponse> {
    // Reject whitespace-only queries (but allow empty string as wildcard/match-all)
    if !params.query.is_empty() && params.query.trim().is_empty() {
        return Err(crate::error::Error::operation(
            "Query must not be whitespace-only".to_string(),
        ));
    }

    // Get active backend from state (FTS if ready, else simple)
    let backend = state.search_backend();
    let backend_name = state.active_backend_name();

    // Convert project params -> fabryk SearchParams
    let fabryk_params = SearchParams {
        query: params.query.clone(),
        limit: Some(params.limit),
        category: params.category.clone(),
        source: params.source.clone(),
        content_types: params.content_types.clone(),
        query_mode: params
            .query_mode
            .as_ref()
            .map(crate::search::to_fabryk_query_mode),
        snippet_length: None,
    };

    // Execute search via fabryk's SearchBackend trait (owned params)
    let fabryk_results = backend
        .search(fabryk_params)
        .await
        .map_err(|e| crate::error::Error::operation(e.to_string()))?;

    // Convert fabryk SearchResult -> project SearchResult
    let results: Vec<SearchResult> = fabryk_results
        .items
        .into_iter()
        .map(|r| SearchResult {
            id: r.id,
            title: r.title,
            category: r.category,
            source: r.source,
            path: r.path.unwrap_or_default(),
            snippet: r.snippet.unwrap_or_default(),
            relevance: r.relevance,
            content_type: r.content_type.unwrap_or_else(|| "concept_card".to_string()),
            section: r.section,
        })
        .collect();

    let total = results.len();

    Ok(SearchConceptsResponse {
        results,
        total,
        query: params.query,
        backend: backend_name.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_limit() {
        assert_eq!(default_limit(), 10);
    }

    #[test]
    fn test_search_result_serialization() {
        let result = SearchResult {
            id: "test-concept".to_string(),
            title: "Test Concept".to_string(),
            category: "harmony".to_string(),
            source: Some("Open Music Theory".to_string()),
            path: "/path/to/concept.md".to_string(),
            snippet: "This is a test snippet".to_string(),
            relevance: 8.5,
            content_type: "concept_card".to_string(),
            section: None,
        };

        let json = serde_json::to_string(&result).expect("Should serialize");
        assert!(json.contains("test-concept"));
        assert!(json.contains("Test Concept"));
        assert!(json.contains("Open Music Theory"));
        assert!(json.contains("concept_card"));
        assert!(json.contains("8.5"));
    }

    #[test]
    fn test_search_result_serialization_no_source() {
        let result = SearchResult {
            id: "test".to_string(),
            title: "Test".to_string(),
            category: "harmony".to_string(),
            source: None,
            path: "/path".to_string(),
            snippet: "snippet".to_string(),
            relevance: 5.0,
            content_type: "concept_card".to_string(),
            section: None,
        };

        let json = serde_json::to_string(&result).expect("Should serialize");
        // source: None and section: None should be skipped
        assert!(!json.contains("\"source\""));
        assert!(!json.contains("\"section\""));
        assert!(json.contains("concept_card"));
    }

    #[test]
    fn test_search_concepts_response_serialization() {
        let response = SearchConceptsResponse {
            results: vec![],
            total: 0,
            query: "test query".to_string(),
            backend: "simple".to_string(),
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        assert!(json.contains("results"));
        assert!(json.contains("total"));
        assert!(json.contains("test query"));
        assert!(json.contains("simple"));
    }

    #[test]
    fn test_search_concepts_params_deserialization() {
        let json = r#"{"query":"harmony"}"#;
        let params: SearchConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.query, "harmony");
        assert_eq!(params.limit, 10); // default value
    }

    #[test]
    fn test_search_concepts_params_with_limit() {
        let json = r#"{"query":"harmony","limit":5}"#;
        let params: SearchConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.query, "harmony");
        assert_eq!(params.limit, 5);
    }

    #[test]
    fn test_search_concepts_params_with_query_mode() {
        let json = r#"{"query":"harmony","query_mode":"and"}"#;
        let params: SearchConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.query, "harmony");
        assert!(params.query_mode.is_some());
        assert_eq!(params.query_mode.unwrap(), QueryMode::And);
    }

    #[test]
    fn test_search_concepts_params_without_query_mode() {
        let json = r#"{"query":"harmony"}"#;
        let params: SearchConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.query, "harmony");
        assert!(params.query_mode.is_none());
    }

    #[test]
    fn test_search_concepts_params_with_category() {
        let json = r#"{"query":"cadence","category":"harmony"}"#;
        let params: SearchConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.query, "cadence");
        assert_eq!(params.category, Some("harmony".to_string()));
    }

    #[test]
    fn test_search_concepts_params_without_category() {
        let json = r#"{"query":"cadence"}"#;
        let params: SearchConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.query, "cadence");
        assert!(params.category.is_none());
    }

    #[test]
    fn test_search_concepts_params_all_fields() {
        let json =
            r#"{"query":"suspension","limit":20,"query_mode":"and","category":"voice-leading"}"#;
        let params: SearchConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.query, "suspension");
        assert_eq!(params.limit, 20);
        assert_eq!(params.query_mode, Some(QueryMode::And));
        assert_eq!(params.category, Some("voice-leading".to_string()));
    }
}
