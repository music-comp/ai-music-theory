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
    /// Subcategory within the main category (v0.3.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    /// Knowledge tier (v0.3.0): "foundational", "intermediate", or "advanced".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Extraction confidence (v0.3.0): "high", "medium", or "low".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraction_confidence: Option<String>,
    /// Alternative names or aliases for the concept (v0.3.0).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
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
    /// Optional tier filter (v0.3.0) - only return results matching this tier.
    /// Valid values: "foundational", "intermediate", "advanced"
    #[serde(default)]
    pub tier: Option<String>,
    /// Optional subcategory filter (v0.3.0) - only return results from this subcategory.
    #[serde(default)]
    pub subcategory: Option<String>,
    /// Optional minimum extraction confidence filter (v0.3.0).
    /// Valid values: "high", "medium", "low"
    /// A value of "medium" includes "high" and "medium" but excludes "low".
    #[serde(default)]
    pub min_confidence: Option<String>,
}

fn default_limit() -> usize {
    10
}

/// Extract a prefixed value from a slice of tags.
///
/// For example, `extract_tag_value(&tags, "tier:")` returns `Some("foundational")`
/// if the tags contain `"tier:foundational"`.
fn extract_tag_value(tags: &[String], prefix: &str) -> Option<String> {
    tags.iter()
        .find(|t| t.starts_with(prefix))
        .map(|t| t[prefix.len()..].to_string())
}

/// Extract all alias values from tags.
///
/// Tags of the form `"alias:some-name"` are collected into a `Vec<String>`.
fn extract_aliases(tags: &[String]) -> Vec<String> {
    tags.iter()
        .filter_map(|t| t.strip_prefix("alias:").map(String::from))
        .collect()
}

/// Map a confidence level string to a numeric rank for ordering.
///
/// Returns `None` for unrecognized values.
fn confidence_rank(level: &str) -> Option<u8> {
    match level {
        "high" => Some(3),
        "medium" => Some(2),
        "low" => Some(1),
        _ => None,
    }
}

/// Check whether a result's confidence meets or exceeds the minimum.
///
/// If the result has no confidence, it does not meet any minimum.
fn meets_min_confidence(result_confidence: Option<&str>, min: &str) -> bool {
    let min_rank = match confidence_rank(min) {
        Some(r) => r,
        None => return false,
    };
    match result_confidence.and_then(confidence_rank) {
        Some(r) => r >= min_rank,
        None => false,
    }
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

    // Convert fabryk SearchResult -> project SearchResult.
    //
    // V3 concept card fields (tier, subcategory, extraction_confidence, aliases)
    // are encoded as prefixed tags on the SearchDocument (e.g. "tier:foundational",
    // "subcategory:consonance-dissonance", "confidence:high", "alias:tritone").
    // The fabryk SearchResult does not currently surface tags, so these fields
    // will be None / empty until the backend exposes them. The filtering logic
    // below is ready to work once the data flows through.
    let results: Vec<SearchResult> = fabryk_results
        .items
        .into_iter()
        .map(|r| {
            // When the backend result carries tags, extract v3 fields from them.
            // For now, tags are empty so these will be None / Vec::new().
            let tags: &[String] = &[];
            let subcategory = extract_tag_value(tags, "subcategory:");
            let tier = extract_tag_value(tags, "tier:");
            let extraction_confidence = extract_tag_value(tags, "confidence:");
            let aliases = extract_aliases(tags);

            SearchResult {
                id: r.id,
                title: r.title,
                category: r.category,
                source: r.source,
                path: r.path.unwrap_or_default(),
                snippet: r.snippet.unwrap_or_default(),
                relevance: r.relevance,
                content_type: r.content_type.unwrap_or_else(|| "concept_card".to_string()),
                section: r.section,
                subcategory,
                tier,
                extraction_confidence,
                aliases,
            }
        })
        .collect();

    // Post-search filtering for v3 params (tier, subcategory, min_confidence).
    let results: Vec<SearchResult> = results
        .into_iter()
        .filter(|r| {
            if let Some(ref tier_filter) = params.tier {
                match r.tier.as_deref() {
                    Some(t) if t == tier_filter.as_str() => {}
                    _ => return false,
                }
            }
            if let Some(ref sub_filter) = params.subcategory {
                match r.subcategory.as_deref() {
                    Some(s) if s == sub_filter.as_str() => {}
                    _ => return false,
                }
            }
            if let Some(ref min_conf) = params.min_confidence {
                if !meets_min_confidence(r.extraction_confidence.as_deref(), min_conf) {
                    return false;
                }
            }
            true
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
            subcategory: None,
            tier: None,
            extraction_confidence: None,
            aliases: vec![],
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
            subcategory: None,
            tier: None,
            extraction_confidence: None,
            aliases: vec![],
        };

        let json = serde_json::to_string(&result).expect("Should serialize");
        // None fields and empty aliases should be skipped
        assert!(!json.contains("\"source\""));
        assert!(!json.contains("\"section\""));
        assert!(!json.contains("\"subcategory\""));
        assert!(!json.contains("\"tier\""));
        assert!(!json.contains("\"extraction_confidence\""));
        assert!(!json.contains("\"aliases\""));
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

    #[test]
    fn test_search_concepts_params_with_v3_fields() {
        let json = r#"{"query":"tritone","tier":"foundational","subcategory":"consonance-dissonance","min_confidence":"medium"}"#;
        let params: SearchConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(params.query, "tritone");
        assert_eq!(params.tier, Some("foundational".to_string()));
        assert_eq!(
            params.subcategory,
            Some("consonance-dissonance".to_string())
        );
        assert_eq!(params.min_confidence, Some("medium".to_string()));
    }

    #[test]
    fn test_search_concepts_params_v3_fields_default_to_none() {
        let json = r#"{"query":"cadence"}"#;
        let params: SearchConceptsParams = serde_json::from_str(json).expect("Should deserialize");
        assert!(params.tier.is_none());
        assert!(params.subcategory.is_none());
        assert!(params.min_confidence.is_none());
    }

    #[test]
    fn test_search_result_v3_fields_serialized_when_present() {
        let result = SearchResult {
            id: "tritone".to_string(),
            title: "Tritone".to_string(),
            category: "harmony".to_string(),
            source: None,
            path: "/path".to_string(),
            snippet: "snippet".to_string(),
            relevance: 9.0,
            content_type: "concept_card".to_string(),
            section: None,
            subcategory: Some("consonance-dissonance".to_string()),
            tier: Some("foundational".to_string()),
            extraction_confidence: Some("high".to_string()),
            aliases: vec![
                "augmented fourth".to_string(),
                "diabolus in musica".to_string(),
            ],
        };

        let json = serde_json::to_string(&result).expect("Should serialize");
        assert!(json.contains("\"subcategory\":\"consonance-dissonance\""));
        assert!(json.contains("\"tier\":\"foundational\""));
        assert!(json.contains("\"extraction_confidence\":\"high\""));
        assert!(json.contains("\"aliases\":[\"augmented fourth\",\"diabolus in musica\"]"));
    }

    // --- Tag extraction helper tests ---

    #[test]
    fn test_extract_tag_value_found() {
        let tags = vec![
            "tier:foundational".to_string(),
            "subcategory:consonance-dissonance".to_string(),
            "confidence:high".to_string(),
        ];
        assert_eq!(
            extract_tag_value(&tags, "tier:"),
            Some("foundational".to_string())
        );
        assert_eq!(
            extract_tag_value(&tags, "subcategory:"),
            Some("consonance-dissonance".to_string())
        );
        assert_eq!(
            extract_tag_value(&tags, "confidence:"),
            Some("high".to_string())
        );
    }

    #[test]
    fn test_extract_tag_value_not_found() {
        let tags = vec!["tier:foundational".to_string()];
        assert_eq!(extract_tag_value(&tags, "subcategory:"), None);
    }

    #[test]
    fn test_extract_tag_value_empty_tags() {
        let tags: Vec<String> = vec![];
        assert_eq!(extract_tag_value(&tags, "tier:"), None);
    }

    #[test]
    fn test_extract_aliases() {
        let tags = vec![
            "tier:foundational".to_string(),
            "alias:augmented fourth".to_string(),
            "alias:diabolus in musica".to_string(),
            "confidence:high".to_string(),
        ];
        let aliases = extract_aliases(&tags);
        assert_eq!(aliases.len(), 2);
        assert_eq!(aliases[0], "augmented fourth");
        assert_eq!(aliases[1], "diabolus in musica");
    }

    #[test]
    fn test_extract_aliases_none() {
        let tags = vec!["tier:foundational".to_string()];
        let aliases = extract_aliases(&tags);
        assert!(aliases.is_empty());
    }

    // --- Confidence ranking tests ---

    #[test]
    fn test_confidence_rank_values() {
        assert_eq!(confidence_rank("high"), Some(3));
        assert_eq!(confidence_rank("medium"), Some(2));
        assert_eq!(confidence_rank("low"), Some(1));
        assert_eq!(confidence_rank("unknown"), None);
    }

    #[test]
    fn test_meets_min_confidence_high_threshold() {
        assert!(meets_min_confidence(Some("high"), "high"));
        assert!(!meets_min_confidence(Some("medium"), "high"));
        assert!(!meets_min_confidence(Some("low"), "high"));
    }

    #[test]
    fn test_meets_min_confidence_medium_threshold() {
        assert!(meets_min_confidence(Some("high"), "medium"));
        assert!(meets_min_confidence(Some("medium"), "medium"));
        assert!(!meets_min_confidence(Some("low"), "medium"));
    }

    #[test]
    fn test_meets_min_confidence_low_threshold() {
        assert!(meets_min_confidence(Some("high"), "low"));
        assert!(meets_min_confidence(Some("medium"), "low"));
        assert!(meets_min_confidence(Some("low"), "low"));
    }

    #[test]
    fn test_meets_min_confidence_none_result() {
        assert!(!meets_min_confidence(None, "low"));
        assert!(!meets_min_confidence(None, "medium"));
        assert!(!meets_min_confidence(None, "high"));
    }

    #[test]
    fn test_meets_min_confidence_invalid_min() {
        assert!(!meets_min_confidence(Some("high"), "invalid"));
    }

    // --- Post-search filtering tests ---

    /// Helper to create a SearchResult with v3 fields for filtering tests.
    fn make_result(
        id: &str,
        tier: Option<&str>,
        subcategory: Option<&str>,
        confidence: Option<&str>,
    ) -> SearchResult {
        SearchResult {
            id: id.to_string(),
            title: id.to_string(),
            category: "harmony".to_string(),
            source: None,
            path: "/path".to_string(),
            snippet: "snippet".to_string(),
            relevance: 1.0,
            content_type: "concept_card".to_string(),
            section: None,
            subcategory: subcategory.map(String::from),
            tier: tier.map(String::from),
            extraction_confidence: confidence.map(String::from),
            aliases: vec![],
        }
    }

    /// Apply the same filtering logic used in `search_concepts` for unit testing.
    fn apply_filters(
        results: Vec<SearchResult>,
        tier: Option<&str>,
        subcategory: Option<&str>,
        min_confidence: Option<&str>,
    ) -> Vec<SearchResult> {
        results
            .into_iter()
            .filter(|r| {
                if let Some(tier_filter) = tier {
                    match r.tier.as_deref() {
                        Some(t) if t == tier_filter => {}
                        _ => return false,
                    }
                }
                if let Some(sub_filter) = subcategory {
                    match r.subcategory.as_deref() {
                        Some(s) if s == sub_filter => {}
                        _ => return false,
                    }
                }
                if let Some(min_conf) = min_confidence {
                    if !meets_min_confidence(r.extraction_confidence.as_deref(), min_conf) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    #[test]
    fn test_filter_by_tier() {
        let results = vec![
            make_result("a", Some("foundational"), None, None),
            make_result("b", Some("advanced"), None, None),
            make_result("c", Some("foundational"), None, None),
            make_result("d", None, None, None),
        ];

        let filtered = apply_filters(results, Some("foundational"), None, None);
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].id, "a");
        assert_eq!(filtered[1].id, "c");
    }

    #[test]
    fn test_filter_by_subcategory() {
        let results = vec![
            make_result("a", None, Some("consonance-dissonance"), None),
            make_result("b", None, Some("chord-quality"), None),
            make_result("c", None, None, None),
        ];

        let filtered = apply_filters(results, None, Some("consonance-dissonance"), None);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "a");
    }

    #[test]
    fn test_filter_by_min_confidence_medium() {
        let results = vec![
            make_result("a", None, None, Some("high")),
            make_result("b", None, None, Some("medium")),
            make_result("c", None, None, Some("low")),
            make_result("d", None, None, None),
        ];

        let filtered = apply_filters(results, None, None, Some("medium"));
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].id, "a");
        assert_eq!(filtered[1].id, "b");
    }

    #[test]
    fn test_filter_combined_tier_and_confidence() {
        let results = vec![
            make_result("a", Some("foundational"), None, Some("high")),
            make_result("b", Some("foundational"), None, Some("low")),
            make_result("c", Some("advanced"), None, Some("high")),
        ];

        let filtered = apply_filters(results, Some("foundational"), None, Some("medium"));
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "a");
    }

    #[test]
    fn test_filter_no_filters_returns_all() {
        let results = vec![
            make_result("a", Some("foundational"), None, Some("high")),
            make_result("b", None, None, None),
        ];

        let filtered = apply_filters(results, None, None, None);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_all_excluded() {
        let results = vec![
            make_result("a", Some("foundational"), None, None),
            make_result("b", None, None, None),
        ];

        let filtered = apply_filters(results, Some("advanced"), None, None);
        assert!(filtered.is_empty());
    }
}
