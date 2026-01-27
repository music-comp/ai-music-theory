//! Stopword filtering for search queries.
//!
//! Filters common English stopwords from queries while preserving domain-specific
//! terms like Roman numerals (I, V, ii) and solfège syllables (do, re, mi).
//!
//! Uses the `stop-words` crate for comprehensive, industry-standard stopword lists.

use std::collections::HashSet;

use stop_words::{get, LANGUAGE};

use crate::config::SearchConfig;

/// Stopword filter for preprocessing search queries.
///
/// Removes common English stopwords while preserving domain-specific terms
/// that are important for music theory searches.
pub struct StopwordFilter {
    /// Set of lowercase stopwords to filter
    stopwords: HashSet<String>,
    /// Set of terms to preserve (case-sensitive)
    allowlist: HashSet<String>,
}

impl StopwordFilter {
    /// Create a new StopwordFilter from configuration.
    ///
    /// Uses the `stop-words` crate for comprehensive English stopword lists.
    ///
    /// # Arguments
    ///
    /// * `config` - SearchConfig with stopword settings
    ///
    /// # Returns
    ///
    /// Returns a StopwordFilter ready to filter queries.
    pub fn new(config: &SearchConfig) -> Self {
        // Get English stopwords from stop-words crate (500+ words)
        let base_stopwords = get(LANGUAGE::English);

        // Build stopwords set (lowercase)
        let mut stopwords: HashSet<String> = base_stopwords
            .into_iter()
            .map(|s| s.to_lowercase())
            .collect();

        // Add custom stopwords from config
        for word in &config.custom_stopwords {
            stopwords.insert(word.to_lowercase());
        }

        // Build allowlist set (case-sensitive for Roman numerals)
        let allowlist: HashSet<String> = config
            .stopword_allowlist
            .iter()
            .map(|s| s.to_string())
            .collect();

        Self {
            stopwords,
            allowlist,
        }
    }

    /// Filter stopwords from a query string.
    ///
    /// Removes stopwords while preserving allowlisted terms. If all words are
    /// stopwords, returns the original query to avoid empty queries.
    ///
    /// # Arguments
    ///
    /// * `query` - The query string to filter
    ///
    /// # Returns
    ///
    /// Returns the filtered query string with stopwords removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use music_theory_mcp::search::StopwordFilter;
    /// use music_theory_mcp::config::{Config, SearchConfig, QueryMode};
    ///
    /// let config = SearchConfig {
    ///     backend: "tantivy".to_string(),
    ///     index_path: ".tantivy-index".to_string(),
    ///     rebuild_on_startup: false,
    ///     snippet_size: 200,
    ///     fuzzy_search: false,
    ///     fuzzy_distance: 2,
    ///     query_mode: QueryMode::Smart,
    ///     minimum_match_percent: 0.6,
    ///     enable_stopwords: true,
    ///     custom_stopwords: vec![],
    ///     stopword_allowlist: vec!["I".to_string(), "V".to_string()],
    /// };
    ///
    /// let filter = StopwordFilter::new(&config);
    ///
    /// // Filters common words
    /// assert_eq!(filter.filter("what is a cadence"), "cadence");
    ///
    /// // Preserves Roman numerals
    /// assert_eq!(filter.filter("V I resolution"), "V I resolution");
    /// ```
    pub fn filter(&self, query: &str) -> String {
        let filtered_words: Vec<&str> = query
            .split_whitespace()
            .filter(|word| {
                let word_lower = word.to_lowercase();
                // Keep if: not a stopword OR is allowlisted
                !self.stopwords.contains(&word_lower) || self.allowlist.contains(*word)
            })
            .collect();

        // If all words were stopwords, return original query to avoid empty query
        if filtered_words.is_empty() {
            query.to_string()
        } else {
            filtered_words.join(" ")
        }
    }

    /// Check if a word is a stopword (not allowlisted).
    ///
    /// # Arguments
    ///
    /// * `word` - The word to check
    ///
    /// # Returns
    ///
    /// Returns `true` if the word is a stopword and not allowlisted.
    #[allow(dead_code)]
    pub fn is_stopword(&self, word: &str) -> bool {
        let word_lower = word.to_lowercase();
        self.stopwords.contains(&word_lower) && !self.allowlist.contains(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::QueryMode;

    fn test_config() -> SearchConfig {
        SearchConfig {
            backend: "tantivy".to_string(),
            index_path: ".tantivy-index".to_string(),
            rebuild_on_startup: false,
            snippet_size: 200,
            fuzzy_search: false,
            fuzzy_distance: 2,
            query_mode: QueryMode::Smart,
            minimum_match_percent: 0.6,
            enable_stopwords: true,
            custom_stopwords: vec![],
            stopword_allowlist: vec![
                "I".to_string(),
                "V".to_string(),
                "ii".to_string(),
                "IV".to_string(),
                "vi".to_string(),
                "vii".to_string(),
                "do".to_string(),
                "re".to_string(),
                "mi".to_string(),
            ],
        }
    }

    #[test]
    fn test_stopword_filter_new() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        assert!(filter.stopwords.len() > 0);
        assert!(filter.allowlist.len() > 0);
    }

    #[test]
    fn test_filter_removes_common_words() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        assert_eq!(filter.filter("what is a cadence"), "cadence");
        assert_eq!(filter.filter("how to write fugue"), "write fugue");
        assert_eq!(filter.filter("the theory of harmony"), "theory harmony");
    }

    #[test]
    fn test_filter_preserves_allowlist() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        // Roman numerals preserved
        assert_eq!(filter.filter("V I resolution"), "V I resolution");
        assert_eq!(
            filter.filter("the ii V I progression"),
            "ii V I progression"
        );

        // Solfège preserved
        assert_eq!(filter.filter("do re mi"), "do re mi");
        assert_eq!(filter.filter("sing do re mi fa"), "sing do re mi fa");
    }

    #[test]
    fn test_filter_all_stopwords() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        // Should preserve original query if all words are stopwords
        assert_eq!(filter.filter("what is this"), "what is this");
        assert_eq!(filter.filter("the a an"), "the a an");
    }

    #[test]
    fn test_filter_empty_query() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        assert_eq!(filter.filter(""), "");
    }

    #[test]
    fn test_filter_whitespace_only() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        assert_eq!(filter.filter("   "), "   ");
    }

    #[test]
    fn test_filter_mixed_case() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        // Stopwords are case-insensitive
        assert_eq!(filter.filter("What Is A Cadence"), "Cadence");

        // Allowlist is case-sensitive (Roman numerals)
        // Note: stop-words crate filters both 'v' and 'i' as stopwords
        assert_eq!(filter.filter("v i resolution"), "resolution"); // lowercase filtered
        assert_eq!(filter.filter("V I resolution"), "V I resolution"); // uppercase preserved
    }

    #[test]
    fn test_filter_no_stopwords() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        assert_eq!(
            filter.filter("cadence suspension tritone"),
            "cadence suspension tritone"
        );
        assert_eq!(
            filter.filter("fugue subject answer"),
            "fugue subject answer"
        );
    }

    #[test]
    fn test_is_stopword() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        // Common stopwords
        assert!(filter.is_stopword("the"));
        assert!(filter.is_stopword("is"));
        assert!(filter.is_stopword("what"));

        // Allowlisted terms (not stopwords)
        assert!(!filter.is_stopword("I"));
        assert!(!filter.is_stopword("V"));

        // Regular words (not stopwords)
        assert!(!filter.is_stopword("cadence"));
        assert!(!filter.is_stopword("harmony"));
    }

    #[test]
    fn test_is_stopword_case_insensitive() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        assert!(filter.is_stopword("The"));
        assert!(filter.is_stopword("THE"));
        assert!(filter.is_stopword("the"));
    }

    #[test]
    fn test_custom_stopwords() {
        let mut config = test_config();
        config.custom_stopwords = vec!["music".to_string(), "theory".to_string()];

        let filter = StopwordFilter::new(&config);

        assert_eq!(filter.filter("music theory cadence"), "cadence");
        // When all words are stopwords, preserve original query
        assert_eq!(
            filter.filter("what is music theory"),
            "what is music theory"
        );
    }

    #[test]
    fn test_stopwords_from_crate() {
        // Verify stop-words crate provides comprehensive stopwords
        use stop_words::{get, LANGUAGE};

        let stopwords = get(LANGUAGE::English);
        let stopword_set: HashSet<&str> = stopwords.iter().map(|s| s.as_str()).collect();

        // Articles
        assert!(stopword_set.contains("a"));
        assert!(stopword_set.contains("an"));
        assert!(stopword_set.contains("the"));

        // Common verbs
        assert!(stopword_set.contains("is"));
        assert!(stopword_set.contains("are"));
        assert!(stopword_set.contains("have"));

        // Question words
        assert!(stopword_set.contains("what"));
        assert!(stopword_set.contains("how"));
        assert!(stopword_set.contains("why"));

        // Prepositions
        assert!(stopword_set.contains("in"));
        assert!(stopword_set.contains("on"));
        assert!(stopword_set.contains("for"));

        // Verify we have a comprehensive list (500+ words)
        assert!(
            stopwords.len() > 500,
            "Expected 500+ stopwords, got {}",
            stopwords.len()
        );
    }

    #[test]
    fn test_filter_preserves_word_order() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        assert_eq!(
            filter.filter("what is the authentic cadence in music"),
            "authentic cadence music"
        );
    }

    #[test]
    fn test_filter_handles_multiple_spaces() {
        let config = test_config();
        let filter = StopwordFilter::new(&config);

        // Multiple spaces should be normalized to single space
        assert_eq!(filter.filter("what  is   a    cadence"), "cadence");
    }
}
