use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::config::Config;
use crate::error::Result;
use crate::markdown::{extract_first_heading, extract_frontmatter};
use crate::util::files::{find_all_files, read_file, FindOptions};

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
pub async fn search_concepts(
    config: &Config,
    params: SearchConceptsParams,
) -> Result<SearchConceptsResponse> {
    let concept_cards_path = config.paths.concept_cards_path()?;

    if !crate::util::files::exists(&concept_cards_path).await {
        return Ok(SearchConceptsResponse {
            results: Vec::new(),
            total: 0,
            query: params.query.clone(),
        });
    }

    let query_lower = params.query.to_lowercase();
    let mut results = Vec::new();

    // Find all markdown files
    let files = find_all_files(&concept_cards_path, FindOptions::markdown()).await?;

    for file_info in files {
        let path = &file_info.path;

        // Read file content
        if let Ok(content) = read_file(path).await {
            let content_lower = content.to_lowercase();

            // Check if query matches
            if content_lower.contains(&query_lower) {
                // Extract metadata
                let (title, _) = extract_metadata(&content);

                // Extract category
                let category = extract_category(&concept_cards_path, path);

                // Extract snippet around match
                let snippet = extract_snippet(&content, &params.query);

                // Calculate relevance score
                let relevance =
                    calculate_relevance(&content_lower, &title.to_lowercase(), &query_lower);

                let concept_id = file_info.stem.clone();

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
    // Safety: Use unwrap_or to handle potential NaN values gracefully
    results.sort_by(|a, b| {
        b.relevance
            .partial_cmp(&a.relevance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let total = results.len();

    // Apply limit
    results.truncate(params.limit);

    Ok(SearchConceptsResponse {
        results,
        total,
        query: params.query,
    })
}

/// Extract category from the path relative to base.
fn extract_category(base: &Path, file_path: &Path) -> String {
    // Safety: unwrap_or provides sensible display fallback if category extraction fails
    file_path
        .parent()
        .and_then(|parent| parent.strip_prefix(base).ok())
        .and_then(|relative| relative.components().next())
        .and_then(|component| component.as_os_str().to_str())
        .unwrap_or("uncategorized")
        .to_string()
}

/// Extract title from markdown content.
fn extract_metadata(content: &str) -> (String, Option<String>) {
    // Extract frontmatter
    let (frontmatter, body) = extract_frontmatter(content).unwrap_or((None, content));

    // Get title from frontmatter or first heading
    let title = frontmatter
        .and_then(|fm| fm.title)
        .or_else(|| extract_first_heading(body).map(|(_, text)| text))
        .unwrap_or_else(|| "Untitled".to_string());

    (title, None)
}

/// Extract a snippet around the search query match.
fn extract_snippet(content: &str, query: &str) -> String {
    let query_lower = query.to_lowercase();
    let content_lower = content.to_lowercase();

    if let Some(pos) = content_lower.find(&query_lower) {
        let start = pos.saturating_sub(100);
        let end = (pos + query.len() + 100).min(content.len());

        // Pre-allocate with estimated capacity to avoid reallocations
        let mut snippet = String::with_capacity(end - start + 6); // +6 for possible "..." on both ends

        // Add leading ellipsis if truncated
        if start > 0 {
            snippet.push_str("...");
        }

        // Extract and clean the content in one pass
        let slice = &content[start..end];
        for ch in slice.chars() {
            if ch == '\n' {
                snippet.push(' ');
            } else {
                snippet.push(ch);
            }
        }

        // Add trailing ellipsis if truncated
        if end < content.len() {
            snippet.push_str("...");
        }

        // Trim whitespace
        snippet.trim().to_string()
    } else {
        // Fallback to first 200 characters
        let mut snippet = String::with_capacity(200);
        let char_count = content.chars().take(200).count();

        for ch in content.chars().take(char_count) {
            if ch == '\n' {
                snippet.push(' ');
            } else {
                snippet.push(ch);
            }
        }

        snippet.trim().to_string()
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
        let _score2 = calculate_relevance(content2, title2, "harmony");

        // Title match should boost score significantly
        assert!(score1 > 5.0);
    }
}
