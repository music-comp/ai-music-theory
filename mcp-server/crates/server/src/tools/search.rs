use serde::{Deserialize, Serialize};

use crate::error::Result;
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
#[derive(Debug, Clone, Deserialize)]
pub struct SearchConceptsParams {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}

/// Search across concept cards for a query.
///
/// Uses the currently active search backend from AppState.
/// Returns FTS results if ready, otherwise uses simple search.
pub async fn search_concepts(
    state: &AppState,
    params: SearchConceptsParams,
) -> Result<SearchConceptsResponse> {
    // Get active backend from state (FTS if ready, else simple)
    let backend = state.search_backend();
    let backend_name = state.active_backend_name();

    // Execute search (polymorphic dispatch)
    let results = backend.search(&params).await?;

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
        };

        let json = serde_json::to_string(&result).expect("Should serialize");
        assert!(json.contains("test-concept"));
        assert!(json.contains("Test Concept"));
        assert!(json.contains("Open Music Theory"));
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
        };

        let json = serde_json::to_string(&result).expect("Should serialize");
        // source: None should be skipped
        assert!(!json.contains("source"));
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
}
