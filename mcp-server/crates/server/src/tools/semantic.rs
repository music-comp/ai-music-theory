//! Semantic search tool with vector, keyword, and hybrid modes.
//!
//! Provides [`semantic_search`] as the public entry point. When the `vector`
//! feature is enabled, all three [`SearchMode`] variants are available;
//! without it the function returns an error indicating the feature is required.

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::state::AppState;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Parameters accepted by the `semantic_search` tool.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[cfg_attr(not(feature = "vector"), allow(dead_code))]
pub struct SemanticSearchParams {
    /// The natural-language query string.
    pub query: String,

    /// Search mode: `vector`, `keyword`, or `hybrid` (default).
    #[serde(default = "default_mode")]
    pub mode: SearchMode,

    /// Optional category filter.
    #[serde(default)]
    pub category: Option<String>,

    /// Optional source filter.
    #[serde(default)]
    pub source: Option<String>,

    /// Maximum number of results to return (default 10).
    #[serde(default = "default_limit")]
    pub limit: usize,
}

/// The search strategy to use.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SearchMode {
    /// Pure vector similarity search.
    Vector,
    /// Keyword / full-text search only.
    Keyword,
    /// Reciprocal-rank fusion of vector + keyword results.
    #[default]
    Hybrid,
}

fn default_mode() -> SearchMode {
    SearchMode::Hybrid
}

fn default_limit() -> usize {
    10
}

/// Top-level response returned by [`semantic_search`].
#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticSearchResponse {
    /// Ranked result items.
    pub results: Vec<SemanticSearchResult>,
    /// Number of results returned.
    pub total: usize,
    /// Echo of the original query.
    pub query: String,
    /// The mode that was actually executed.
    pub mode: String,
    /// Human-readable backend description.
    pub backend: String,
}

/// A single search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticSearchResult {
    /// Unique identifier for the matched item.
    pub id: String,
    /// Relevance / similarity score.
    pub score: f32,
    /// Origin of this result: `"vector"`, `"keyword"`, or `"hybrid"`.
    pub source: String,
    /// Arbitrary metadata carried through from the backend.
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// Public entry point — vector feature enabled
// ---------------------------------------------------------------------------

/// Perform a semantic search across the music-theory knowledge base.
///
/// Dispatches to the requested [`SearchMode`]. Requires the `vector` Cargo
/// feature; without it, this function unconditionally returns an error.
#[cfg(feature = "vector")]
pub async fn semantic_search(
    state: &AppState,
    params: SemanticSearchParams,
) -> Result<SemanticSearchResponse> {
    // Reject whitespace-only queries (but allow truly empty queries to surface
    // validation errors downstream if desired).
    if !params.query.is_empty() && params.query.trim().is_empty() {
        return Err(crate::error::Error::operation(
            "Query must not be whitespace-only".to_string(),
        ));
    }

    match params.mode {
        SearchMode::Vector => search_vector(state, &params).await,
        SearchMode::Keyword => search_keyword(state, &params).await,
        SearchMode::Hybrid => search_hybrid(state, &params).await,
    }
}

// ---------------------------------------------------------------------------
// Public entry point — vector feature disabled
// ---------------------------------------------------------------------------

/// Stub returned when the `vector` feature is not compiled in.
#[cfg(not(feature = "vector"))]
pub async fn semantic_search(
    _state: &AppState,
    _params: SemanticSearchParams,
) -> Result<SemanticSearchResponse> {
    Err(crate::error::Error::operation(
        "Semantic search requires the 'vector' feature to be enabled".to_string(),
    ))
}

// ---------------------------------------------------------------------------
// Vector mode
// ---------------------------------------------------------------------------

#[cfg(feature = "vector")]
async fn search_vector(
    state: &AppState,
    params: &SemanticSearchParams,
) -> Result<SemanticSearchResponse> {
    use fabryk::vector::VectorSearchParams;

    // Extract the Arc and drop the guard in a sync block so the
    // RwLockReadGuard is never alive across an `.await` point.
    let vector_backend = {
        let guard = state.require_vector()?;
        guard
            .as_ref()
            .ok_or_else(|| {
                crate::error::Error::operation("Vector backend not available".to_string())
            })?
            .clone()
    };

    let mut search_params = VectorSearchParams::new(&params.query).with_limit(params.limit);

    if let Some(ref cat) = params.category {
        search_params = search_params.with_category(cat);
    }
    if let Some(ref source) = params.source {
        search_params = search_params.with_filter("source", source);
    }

    let results = vector_backend
        .search(search_params)
        .await
        .map_err(|e| crate::error::Error::operation(e.to_string()))?;

    // Capture the backend name before consuming `results.items`.
    let backend_name = results.backend.clone();

    let items: Vec<SemanticSearchResult> = results
        .items
        .into_iter()
        .map(|r| SemanticSearchResult {
            id: r.id,
            score: r.score,
            source: "vector".to_string(),
            metadata: r.metadata,
        })
        .collect();

    let total = items.len();
    Ok(SemanticSearchResponse {
        results: items,
        total,
        query: params.query.clone(),
        mode: "vector".to_string(),
        backend: backend_name,
    })
}

// ---------------------------------------------------------------------------
// Keyword mode
// ---------------------------------------------------------------------------

#[cfg(feature = "vector")]
async fn search_keyword(
    state: &AppState,
    params: &SemanticSearchParams,
) -> Result<SemanticSearchResponse> {
    let search_params = crate::tools::search::SearchConceptsParams {
        query: params.query.clone(),
        limit: params.limit,
        query_mode: None,
        category: params.category.clone(),
        source: params.source.clone(),
        content_types: None,
    };

    let response = crate::tools::search::search_concepts(state, search_params).await?;

    let items: Vec<SemanticSearchResult> = response
        .results
        .into_iter()
        .map(|r| {
            let mut metadata = HashMap::new();
            metadata.insert("title".to_string(), r.title);
            metadata.insert("category".to_string(), r.category);
            if let Some(source) = r.source {
                metadata.insert("source".to_string(), source);
            }
            metadata.insert("content_type".to_string(), r.content_type);
            SemanticSearchResult {
                id: r.id,
                score: r.relevance,
                source: "keyword".to_string(),
                metadata,
            }
        })
        .collect();

    let total = items.len();
    Ok(SemanticSearchResponse {
        results: items,
        total,
        query: params.query.clone(),
        mode: "keyword".to_string(),
        backend: response.backend,
    })
}

// ---------------------------------------------------------------------------
// Hybrid mode
// ---------------------------------------------------------------------------

#[cfg(feature = "vector")]
async fn search_hybrid(
    state: &AppState,
    params: &SemanticSearchParams,
) -> Result<SemanticSearchResponse> {
    use fabryk::vector::{reciprocal_rank_fusion, FtsResult, VectorBackend, VectorSearchParams};

    // Always run keyword search.
    let keyword_future = search_keyword(state, params);

    // Extract the Arc and drop the guard in a sync block — the `.await`
    // must live outside so the RwLockReadGuard is never alive across it.
    let maybe_backend: Option<std::sync::Arc<dyn VectorBackend>> = {
        match state.require_vector() {
            Ok(guard) => guard.as_ref().map(std::sync::Arc::clone),
            Err(_) => None,
        }
    };

    let vector_result = match maybe_backend {
        Some(vector_backend) => {
            let mut vp = VectorSearchParams::new(&params.query).with_limit(params.limit * 2);
            if let Some(ref cat) = params.category {
                vp = vp.with_category(cat);
            }
            if let Some(ref source) = params.source {
                vp = vp.with_filter("source", source);
            }
            Some(vector_backend.search(vp).await)
        }
        None => None,
    };

    let keyword_response = keyword_future.await?;

    // If vector search succeeded, fuse the two result sets with RRF.
    match vector_result {
        Some(Ok(vector_results)) => {
            let fts_results: Vec<FtsResult> = keyword_response
                .results
                .iter()
                .map(|r| FtsResult {
                    id: r.id.clone(),
                    score: r.score,
                    metadata: r.metadata.clone(),
                })
                .collect();

            let hybrid =
                reciprocal_rank_fusion(&vector_results.items, &fts_results, params.limit, 60);

            let items: Vec<SemanticSearchResult> = hybrid
                .into_iter()
                .map(|h| SemanticSearchResult {
                    id: h.id,
                    score: h.score,
                    source: h.source,
                    metadata: h.metadata,
                })
                .collect();

            let total = items.len();
            Ok(SemanticSearchResponse {
                results: items,
                total,
                query: params.query.clone(),
                mode: "hybrid".to_string(),
                backend: "hybrid (vector + keyword)".to_string(),
            })
        }
        _ => {
            // Vector search was unavailable or failed — degrade gracefully to
            // keyword-only results while still reporting hybrid mode.
            Ok(SemanticSearchResponse {
                results: keyword_response.results,
                total: keyword_response.total,
                query: params.query.clone(),
                mode: "hybrid".to_string(),
                backend: format!(
                    "{} (vector unavailable, keyword-only fallback)",
                    keyword_response.backend
                ),
            })
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mode() {
        assert_eq!(default_mode(), SearchMode::Hybrid);
    }

    #[test]
    fn test_default_limit() {
        assert_eq!(default_limit(), 10);
    }

    #[test]
    fn test_search_mode_default() {
        let mode = SearchMode::default();
        assert_eq!(mode, SearchMode::Hybrid);
    }

    #[test]
    fn test_search_mode_serialization() {
        let json = serde_json::to_string(&SearchMode::Vector).unwrap();
        assert_eq!(json, "\"vector\"");
        let json = serde_json::to_string(&SearchMode::Keyword).unwrap();
        assert_eq!(json, "\"keyword\"");
        let json = serde_json::to_string(&SearchMode::Hybrid).unwrap();
        assert_eq!(json, "\"hybrid\"");
    }

    #[test]
    fn test_search_mode_deserialization() {
        let mode: SearchMode = serde_json::from_str("\"vector\"").unwrap();
        assert_eq!(mode, SearchMode::Vector);
        let mode: SearchMode = serde_json::from_str("\"keyword\"").unwrap();
        assert_eq!(mode, SearchMode::Keyword);
        let mode: SearchMode = serde_json::from_str("\"hybrid\"").unwrap();
        assert_eq!(mode, SearchMode::Hybrid);
    }

    #[test]
    fn test_params_deserialization_defaults() {
        let json = r#"{"query":"harmony"}"#;
        let params: SemanticSearchParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.query, "harmony");
        assert_eq!(params.mode, SearchMode::Hybrid);
        assert_eq!(params.limit, 10);
        assert!(params.category.is_none());
        assert!(params.source.is_none());
    }

    #[test]
    fn test_params_deserialization_all_fields() {
        let json = r#"{"query":"cadence","mode":"vector","category":"harmony","source":"Open Music Theory","limit":5}"#;
        let params: SemanticSearchParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.query, "cadence");
        assert_eq!(params.mode, SearchMode::Vector);
        assert_eq!(params.limit, 5);
        assert_eq!(params.category, Some("harmony".to_string()));
        assert_eq!(params.source, Some("Open Music Theory".to_string()));
    }

    #[test]
    fn test_response_serialization() {
        let response = SemanticSearchResponse {
            results: vec![SemanticSearchResult {
                id: "test-1".to_string(),
                score: 0.95,
                source: "vector".to_string(),
                metadata: HashMap::from([("title".to_string(), "Test".to_string())]),
            }],
            total: 1,
            query: "test".to_string(),
            mode: "vector".to_string(),
            backend: "simple".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("test-1"));
        assert!(json.contains("0.95"));
        assert!(json.contains("vector"));
    }

    #[test]
    fn test_result_empty_metadata_skipped() {
        let result = SemanticSearchResult {
            id: "test".to_string(),
            score: 0.5,
            source: "keyword".to_string(),
            metadata: HashMap::new(),
        };
        let json = serde_json::to_string(&result).unwrap();
        // Empty metadata should be skipped due to `skip_serializing_if`.
        assert!(!json.contains("metadata"));
    }

    #[test]
    fn test_result_with_metadata_included() {
        let result = SemanticSearchResult {
            id: "test".to_string(),
            score: 0.5,
            source: "keyword".to_string(),
            metadata: HashMap::from([("key".to_string(), "value".to_string())]),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("metadata"));
        assert!(json.contains("key"));
    }
}
