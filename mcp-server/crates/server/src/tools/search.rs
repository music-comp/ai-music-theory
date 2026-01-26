use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::Result;
use crate::search::create_search_backend;

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
}

/// Parameters for search_concepts tool.
#[derive(Debug, Deserialize)]
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
/// Uses the configured search backend (simple or tantivy).
pub async fn search_concepts(
    config: &Config,
    params: SearchConceptsParams,
) -> Result<SearchConceptsResponse> {
    // Create search backend based on configuration
    let backend = create_search_backend(config).await?;

    // Execute search (polymorphic dispatch)
    let results = backend.search(&params).await?;

    let total = results.len();

    Ok(SearchConceptsResponse {
        results,
        total,
        query: params.query,
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
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        assert!(json.contains("results"));
        assert!(json.contains("total"));
        assert!(json.contains("test query"));
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
