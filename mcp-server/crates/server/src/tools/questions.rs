//! Search concept cards by competency questions.
//!
//! Provides fuzzy string matching against the `answers_questions` frontmatter
//! field found in V3 concept cards. Uses normalized Damerau-Levenshtein
//! distance for similarity scoring, with a substring-match boost.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::Result;
use crate::util::files::{find_all_files, read_file, FindOptions};

/// Parameters for searching concept cards by competency question.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct SearchByQuestionParams {
    /// The question to search for.
    pub question: String,
    /// Maximum number of results to return (default: 10).
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}

/// A single matching concept card with its similarity score.
#[derive(Debug, Clone, Serialize)]
pub struct QuestionMatch {
    /// Machine-readable concept identifier (file stem).
    pub concept_id: String,
    /// Human-readable title from the card's frontmatter.
    pub concept_title: String,
    /// The specific competency question that matched.
    pub matched_question: String,
    /// Thematic category of the concept card.
    pub category: String,
    /// Prerequisite depth tier, if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// Similarity score between 0.0 and 1.0 (higher is better).
    pub similarity: f64,
}

/// Response from `search_by_question`.
#[derive(Debug, Serialize)]
pub struct SearchByQuestionResponse {
    /// Matching concept cards sorted by descending similarity.
    pub matches: Vec<QuestionMatch>,
    /// Total number of matches found (before applying `limit`).
    pub total: usize,
    /// The original query string.
    pub query: String,
}

/// Minimum similarity threshold for a match to be included.
const SIMILARITY_THRESHOLD: f64 = 0.3;

/// Bonus added when either string is a case-insensitive substring of the other.
const SUBSTRING_BOOST: f64 = 0.3;

/// Search concept cards whose competency questions match the given query.
///
/// For each card that has an `answers_questions` field, every question is
/// compared against the query using normalized Damerau-Levenshtein distance.
/// A substring-match bonus is applied when the query appears within a question
/// (or vice versa). Results are sorted by descending similarity and capped at
/// `params.limit`.
///
/// # Errors
///
/// Returns an error if the concept cards directory cannot be resolved or
/// if file I/O fails.
pub async fn search_by_question(
    config: &Config,
    params: SearchByQuestionParams,
) -> Result<SearchByQuestionResponse> {
    let cards_path = config.paths.concept_cards_path()?;
    let files = find_all_files(&cards_path, FindOptions::markdown()).await?;

    let query_lower = params.question.to_lowercase();
    let mut all_matches: Vec<QuestionMatch> = Vec::new();

    for file_info in &files {
        let content = match read_file(&file_info.path).await {
            Ok(c) => c,
            Err(_) => continue,
        };

        let (frontmatter, _body) =
            match crate::markdown::extract_frontmatter_with_path(&content, &file_info.path) {
            Ok((Some(fm), body)) => (fm, body),
            _ => continue,
        };

        if frontmatter.answers_questions.is_empty() {
            continue;
        }

        let concept_id = frontmatter
            .slug
            .clone()
            .unwrap_or_else(|| file_info.stem.clone());
        let concept_title = frontmatter
            .title
            .clone()
            .or_else(|| frontmatter.concept.clone())
            .unwrap_or_else(|| file_info.stem.clone());
        let category = frontmatter
            .category
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        let tier = frontmatter.tier.clone();

        for question in &frontmatter.answers_questions {
            let question_lower = question.to_lowercase();

            let mut similarity =
                strsim::normalized_damerau_levenshtein(&query_lower, &question_lower);

            // Boost if either is a substring of the other
            if query_lower.contains(&question_lower) || question_lower.contains(&query_lower) {
                similarity = (similarity + SUBSTRING_BOOST).min(1.0);
            }

            if similarity > SIMILARITY_THRESHOLD {
                all_matches.push(QuestionMatch {
                    concept_id: concept_id.clone(),
                    concept_title: concept_title.clone(),
                    matched_question: question.clone(),
                    category: category.clone(),
                    tier: tier.clone(),
                    similarity,
                });
            }
        }
    }

    // Sort by similarity descending
    all_matches.sort_by(|a, b| {
        b.similarity
            .partial_cmp(&a.similarity)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let total = all_matches.len();
    all_matches.truncate(params.limit);

    Ok(SearchByQuestionResponse {
        matches: all_matches,
        total,
        query: params.question,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use tokio::fs;

    /// Create a temporary directory with concept card files for testing.
    async fn setup_test_cards(dir: &std::path::Path) {
        let card1 = r#"---
title: "Acoustic Consonance"
concept: "Acoustic Consonance"
slug: "acoustic-consonance"
category: "acoustics"
tier: "foundational"
answers_questions:
  - "What makes two tones sound consonant?"
  - "How does frequency ratio affect consonance?"
---
# Acoustic Consonance

Content about acoustic consonance."#;

        let card2 = r#"---
title: "Circle of Fifths"
concept: "Circle of Fifths"
slug: "circle-of-fifths"
category: "harmony"
tier: "intermediate"
answers_questions:
  - "How are keys related by fifths?"
  - "What is the circle of fifths?"
---
# Circle of Fifths

Content about the circle of fifths."#;

        let card3 = r#"---
title: "Pitch Class"
concept: "Pitch Class"
slug: "pitch-class"
category: "fundamentals"
answers_questions:
  - "What is a pitch class?"
  - "How do octave equivalence and pitch class relate?"
---
# Pitch Class

Content about pitch class."#;

        let card_no_questions = r#"---
title: "No Questions Card"
category: "other"
---
# No Questions

This card has no competency questions."#;

        fs::write(dir.join("acoustic-consonance.md"), card1)
            .await
            .unwrap();
        fs::write(dir.join("circle-of-fifths.md"), card2)
            .await
            .unwrap();
        fs::write(dir.join("pitch-class.md"), card3).await.unwrap();
        fs::write(dir.join("no-questions.md"), card_no_questions)
            .await
            .unwrap();
    }

    fn make_test_config(path: &std::path::Path) -> Config {
        let mut config = Config::default();
        config.paths.concept_cards = path.to_string_lossy().to_string();
        config
    }

    #[tokio::test]
    async fn test_search_by_question_exact_match() {
        let temp = TempDir::new().unwrap();
        setup_test_cards(temp.path()).await;
        let config = make_test_config(temp.path());

        let params = SearchByQuestionParams {
            question: "What is a pitch class?".to_string(),
            limit: 10,
        };
        let response = search_by_question(&config, params).await.unwrap();

        assert!(!response.matches.is_empty());
        assert_eq!(response.matches[0].concept_id, "pitch-class");
        assert_eq!(
            response.matches[0].matched_question,
            "What is a pitch class?"
        );
        assert!(response.matches[0].similarity > 0.9);
    }

    #[tokio::test]
    async fn test_search_by_question_fuzzy_match() {
        let temp = TempDir::new().unwrap();
        setup_test_cards(temp.path()).await;
        let config = make_test_config(temp.path());

        let params = SearchByQuestionParams {
            question: "what is the circle of fifths".to_string(),
            limit: 10,
        };
        let response = search_by_question(&config, params).await.unwrap();

        assert!(!response.matches.is_empty());
        // The top match should be the circle-of-fifths card
        let top = &response.matches[0];
        assert_eq!(top.concept_id, "circle-of-fifths");
        assert!(top.similarity > 0.5);
    }

    #[tokio::test]
    async fn test_search_by_question_case_insensitive() {
        let temp = TempDir::new().unwrap();
        setup_test_cards(temp.path()).await;
        let config = make_test_config(temp.path());

        let params = SearchByQuestionParams {
            question: "WHAT IS A PITCH CLASS?".to_string(),
            limit: 10,
        };
        let response = search_by_question(&config, params).await.unwrap();

        assert!(!response.matches.is_empty());
        assert_eq!(response.matches[0].concept_id, "pitch-class");
    }

    #[tokio::test]
    async fn test_search_by_question_substring_boost() {
        let temp = TempDir::new().unwrap();
        setup_test_cards(temp.path()).await;
        let config = make_test_config(temp.path());

        // "consonant" is a substring of "What makes two tones sound consonant?"
        let params = SearchByQuestionParams {
            question: "consonant".to_string(),
            limit: 10,
        };
        let response = search_by_question(&config, params).await.unwrap();

        // Should find the acoustic consonance card via substring boost
        let acoustic_match = response
            .matches
            .iter()
            .find(|m| m.concept_id == "acoustic-consonance");
        assert!(
            acoustic_match.is_some(),
            "Expected to find acoustic-consonance via substring match"
        );
    }

    #[tokio::test]
    async fn test_search_by_question_respects_limit() {
        let temp = TempDir::new().unwrap();
        setup_test_cards(temp.path()).await;
        let config = make_test_config(temp.path());

        let params = SearchByQuestionParams {
            question: "what".to_string(),
            limit: 2,
        };
        let response = search_by_question(&config, params).await.unwrap();

        assert!(response.matches.len() <= 2);
        // total may be larger than the returned matches
        assert!(response.total >= response.matches.len());
    }

    #[tokio::test]
    async fn test_search_by_question_no_matches() {
        let temp = TempDir::new().unwrap();
        setup_test_cards(temp.path()).await;
        let config = make_test_config(temp.path());

        let params = SearchByQuestionParams {
            question: "xyzzy plugh nothing relevant".to_string(),
            limit: 10,
        };
        let response = search_by_question(&config, params).await.unwrap();

        assert!(response.matches.is_empty());
        assert_eq!(response.total, 0);
    }

    #[tokio::test]
    async fn test_search_by_question_empty_directory() {
        let temp = TempDir::new().unwrap();
        let config = make_test_config(temp.path());

        let params = SearchByQuestionParams {
            question: "anything".to_string(),
            limit: 10,
        };
        let response = search_by_question(&config, params).await.unwrap();

        assert!(response.matches.is_empty());
        assert_eq!(response.total, 0);
    }

    #[tokio::test]
    async fn test_search_by_question_sorted_by_similarity() {
        let temp = TempDir::new().unwrap();
        setup_test_cards(temp.path()).await;
        let config = make_test_config(temp.path());

        let params = SearchByQuestionParams {
            question: "what".to_string(),
            limit: 20,
        };
        let response = search_by_question(&config, params).await.unwrap();

        // Verify descending order
        for window in response.matches.windows(2) {
            assert!(
                window[0].similarity >= window[1].similarity,
                "Results should be sorted by descending similarity: {} >= {}",
                window[0].similarity,
                window[1].similarity,
            );
        }
    }

    #[tokio::test]
    async fn test_search_by_question_skips_cards_without_questions() {
        let temp = TempDir::new().unwrap();
        setup_test_cards(temp.path()).await;
        let config = make_test_config(temp.path());

        let params = SearchByQuestionParams {
            question: "No Questions Card".to_string(),
            limit: 10,
        };
        let response = search_by_question(&config, params).await.unwrap();

        // The "no-questions" card should never appear since it has no answers_questions
        for m in &response.matches {
            assert_ne!(m.concept_id, "no-questions");
        }
    }

    #[tokio::test]
    async fn test_search_by_question_response_fields() {
        let temp = TempDir::new().unwrap();
        setup_test_cards(temp.path()).await;
        let config = make_test_config(temp.path());

        let params = SearchByQuestionParams {
            question: "What is the circle of fifths?".to_string(),
            limit: 10,
        };
        let response = search_by_question(&config, params).await.unwrap();

        assert_eq!(response.query, "What is the circle of fifths?");

        let top = &response.matches[0];
        assert_eq!(top.concept_id, "circle-of-fifths");
        assert_eq!(top.concept_title, "Circle of Fifths");
        assert_eq!(top.category, "harmony");
        assert_eq!(top.tier, Some("intermediate".to_string()));
        assert!(top.similarity > 0.0);
        assert!(top.similarity <= 1.0);
    }

    #[tokio::test]
    async fn test_search_by_question_uses_slug_as_concept_id() {
        let temp = TempDir::new().unwrap();

        let card = r#"---
title: "Test Card"
slug: "my-custom-slug"
category: "test"
answers_questions:
  - "What is this test about?"
---
# Test"#;

        fs::write(temp.path().join("different-filename.md"), card)
            .await
            .unwrap();

        let config = make_test_config(temp.path());
        let params = SearchByQuestionParams {
            question: "What is this test about?".to_string(),
            limit: 10,
        };
        let response = search_by_question(&config, params).await.unwrap();

        assert!(!response.matches.is_empty());
        // Should use slug, not filename stem
        assert_eq!(response.matches[0].concept_id, "my-custom-slug");
    }

    #[tokio::test]
    async fn test_search_by_question_falls_back_to_stem_without_slug() {
        let temp = TempDir::new().unwrap();

        let card = r#"---
title: "No Slug Card"
category: "test"
answers_questions:
  - "What is this card about?"
---
# No Slug"#;

        fs::write(temp.path().join("no-slug-card.md"), card)
            .await
            .unwrap();

        let config = make_test_config(temp.path());
        let params = SearchByQuestionParams {
            question: "What is this card about?".to_string(),
            limit: 10,
        };
        let response = search_by_question(&config, params).await.unwrap();

        assert!(!response.matches.is_empty());
        assert_eq!(response.matches[0].concept_id, "no-slug-card");
    }

    #[tokio::test]
    async fn test_search_by_question_default_limit() {
        let params: SearchByQuestionParams =
            serde_json::from_str(r#"{"question":"test"}"#).unwrap();
        assert_eq!(params.limit, 10);
    }

    #[tokio::test]
    async fn test_search_by_question_custom_limit() {
        let params: SearchByQuestionParams =
            serde_json::from_str(r#"{"question":"test","limit":5}"#).unwrap();
        assert_eq!(params.limit, 5);
    }

    #[tokio::test]
    async fn test_search_by_question_similarity_bounded() {
        let temp = TempDir::new().unwrap();
        setup_test_cards(temp.path()).await;
        let config = make_test_config(temp.path());

        let params = SearchByQuestionParams {
            question: "consonant".to_string(),
            limit: 50,
        };
        let response = search_by_question(&config, params).await.unwrap();

        for m in &response.matches {
            assert!(
                m.similarity > SIMILARITY_THRESHOLD,
                "All matches should exceed threshold: {} > {}",
                m.similarity,
                SIMILARITY_THRESHOLD,
            );
            assert!(
                m.similarity <= 1.0,
                "Similarity should not exceed 1.0: {}",
                m.similarity,
            );
        }
    }

    #[tokio::test]
    async fn test_search_by_question_handles_malformed_frontmatter() {
        let temp = TempDir::new().unwrap();

        let bad_card = "# No frontmatter at all\n\nJust content.";
        fs::write(temp.path().join("bad.md"), bad_card)
            .await
            .unwrap();

        let config = make_test_config(temp.path());
        let params = SearchByQuestionParams {
            question: "anything".to_string(),
            limit: 10,
        };
        let response = search_by_question(&config, params).await.unwrap();

        // Should not error, just return empty
        assert!(response.matches.is_empty());
    }

    #[tokio::test]
    async fn test_search_by_question_total_reflects_pre_limit_count() {
        let temp = TempDir::new().unwrap();
        setup_test_cards(temp.path()).await;
        let config = make_test_config(temp.path());

        // Use a broad query to get many matches, but a small limit
        let params = SearchByQuestionParams {
            question: "what".to_string(),
            limit: 1,
        };
        let response = search_by_question(&config, params).await.unwrap();

        assert_eq!(response.matches.len(), 1);
        // total should reflect ALL matches before limiting
        assert!(response.total >= 1);
    }

    #[test]
    fn test_question_match_serialization_skips_none_tier() {
        let m = QuestionMatch {
            concept_id: "test".to_string(),
            concept_title: "Test".to_string(),
            matched_question: "What?".to_string(),
            category: "cat".to_string(),
            tier: None,
            similarity: 0.8,
        };
        let json = serde_json::to_string(&m).unwrap();
        assert!(!json.contains("tier"));
    }

    #[test]
    fn test_question_match_serialization_includes_tier() {
        let m = QuestionMatch {
            concept_id: "test".to_string(),
            concept_title: "Test".to_string(),
            matched_question: "What?".to_string(),
            category: "cat".to_string(),
            tier: Some("foundational".to_string()),
            similarity: 0.8,
        };
        let json = serde_json::to_string(&m).unwrap();
        assert!(json.contains("foundational"));
    }
}
