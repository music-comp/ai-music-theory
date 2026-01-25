use serde::{Deserialize, Serialize};
use std::fs;
use walkdir::WalkDir;

use crate::config::Config;
use crate::error::Result;

/// A search result item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub category: String,
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
pub fn search_concepts(config: &Config, params: SearchConceptsParams) -> Result<SearchConceptsResponse> {
    let concept_cards_path = config.paths.concept_cards_path()?;

    if !concept_cards_path.exists() {
        return Ok(SearchConceptsResponse {
            results: Vec::new(),
            total: 0,
            query: params.query.clone(),
        });
    }

    let query_lower = params.query.to_lowercase();
    let mut results = Vec::new();

    // Walk through all markdown files
    for entry in WalkDir::new(&concept_cards_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                == Some("md")
        })
    {
        let path = entry.path();

        // Read file content
        if let Ok(content) = fs::read_to_string(path) {
            let content_lower = content.to_lowercase();

            // Check if query matches
            if content_lower.contains(&query_lower) {
                // Extract metadata
                let (title, _) = extract_metadata(&content);

                // Extract category
                let category = path
                    .parent()
                    .and_then(|p| p.strip_prefix(&concept_cards_path).ok())
                    .and_then(|p| p.components().next())
                    .and_then(|c| c.as_os_str().to_str())
                    .unwrap_or("uncategorized")
                    .to_string();

                // Extract snippet around match
                let snippet = extract_snippet(&content, &params.query);

                // Calculate relevance score
                let relevance = calculate_relevance(&content_lower, &title.to_lowercase(), &query_lower);

                let concept_id = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                results.push(SearchResult {
                    id: concept_id,
                    title,
                    category,
                    path: path.to_string_lossy().to_string(),
                    snippet,
                    relevance,
                });
            }
        }
    }

    // Sort by relevance (highest first)
    results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());

    let total = results.len();

    // Apply limit
    results.truncate(params.limit);

    Ok(SearchConceptsResponse {
        results,
        total,
        query: params.query,
    })
}

/// Extract title from markdown content.
fn extract_metadata(content: &str) -> (String, Option<String>) {
    let mut title = String::new();
    let mut in_frontmatter = false;
    let mut frontmatter_count = 0;

    for line in content.lines().take(20) {
        if line.trim() == "---" {
            frontmatter_count += 1;
            if frontmatter_count == 1 {
                in_frontmatter = true;
                continue;
            } else if frontmatter_count == 2 {
                in_frontmatter = false;
                continue;
            }
        }

        if in_frontmatter && line.starts_with("title:") {
            title = line[6..].trim().trim_matches('"').to_string();
            continue;
        }

        if !in_frontmatter && title.is_empty() && line.starts_with('#') {
            title = line.trim_start_matches('#').trim().to_string();
            break;
        }
    }

    if title.is_empty() {
        title = "Untitled".to_string();
    }

    (title, None)
}

/// Extract a snippet around the search query match.
fn extract_snippet(content: &str, query: &str) -> String {
    let query_lower = query.to_lowercase();
    let content_lower = content.to_lowercase();

    if let Some(pos) = content_lower.find(&query_lower) {
        let start = pos.saturating_sub(100);
        let end = (pos + query.len() + 100).min(content.len());

        let mut snippet = content[start..end].to_string();

        // Clean up snippet
        snippet = snippet.replace('\n', " ");
        snippet = snippet.trim().to_string();

        // Add ellipsis if truncated
        if start > 0 {
            snippet = format!("...{}", snippet);
        }
        if end < content.len() {
            snippet = format!("{}...", snippet);
        }

        snippet
    } else {
        // Fallback to first 200 characters
        content
            .chars()
            .take(200)
            .collect::<String>()
            .replace('\n', " ")
            .trim()
            .to_string()
    }
}

/// Calculate relevance score for a search result.
fn calculate_relevance(content_lower: &str, title_lower: &str, query_lower: &str) -> f32 {
    let mut score = 0.0;

    // Title match is worth more
    if title_lower.contains(query_lower) {
        score += 10.0;
    }

    // Count occurrences in content
    let occurrences = content_lower.matches(query_lower).count() as f32;
    score += occurrences;

    // Boost for exact word match (not just substring)
    let words: Vec<&str> = content_lower.split_whitespace().collect();
    let exact_matches = words.iter().filter(|&&w| w == query_lower).count() as f32;
    score += exact_matches * 2.0;

    // Normalize by content length to favor shorter, more focused documents
    let content_len = content_lower.len() as f32;
    if content_len > 0.0 {
        score = score * 1000.0 / content_len;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_snippet() {
        let content = "This is a test document about harmony. Harmony is an important concept in music theory. We will explore various aspects of harmony.";
        let snippet = extract_snippet(content, "harmony");
        assert!(snippet.contains("harmony"));
        assert!(snippet.len() <= 250);
    }

    #[test]
    fn test_calculate_relevance() {
        let content = "harmony is key. studying harmony helps. harmony harmony harmony.";
        let title = "harmony basics";
        let query = "harmony";

        let score = calculate_relevance(content, title, query);
        assert!(score > 0.0);
    }

    #[test]
    fn test_calculate_relevance_title_boost() {
        let content1 = "document about music";
        let title1 = "harmony guide";
        let content2 = "harmony harmony harmony";
        let title2 = "random title";

        let score1 = calculate_relevance(content1, title1, "harmony");
        let score2 = calculate_relevance(content2, title2, "harmony");

        // Title match should boost score significantly
        assert!(score1 > 5.0);
    }
}
