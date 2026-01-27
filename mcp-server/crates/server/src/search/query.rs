//! Query building for Tantivy search.
//!
//! This module provides QueryBuilder for constructing weighted multi-field queries
//! that match the relevance scoring used in simple search.

use tantivy::query::{
    BooleanQuery, BoostQuery, FuzzyTermQuery, Occur, PhraseQuery, Query, TermQuery,
};
use tantivy::schema::IndexRecordOption;
use tantivy::tokenizer::{LowerCaser, SimpleTokenizer, Stemmer, TextAnalyzer};
use tantivy::Term;

use crate::config::SearchConfig;
use crate::error::{Error, Result};
use crate::search::SearchSchema;

/// Parse query into phrases and remaining terms.
///
/// Extracts quoted phrases from the query string and returns them separately
/// from the non-quoted terms.
///
/// # Arguments
///
/// * `query` - The query string to parse
///
/// # Returns
///
/// Returns a tuple of (phrases, remaining_terms) where:
/// - `phrases` is a vector of quoted strings (without quotes)
/// - `remaining_terms` is the query with quoted phrases removed
fn parse_phrases(query: &str) -> (Vec<String>, String) {
    #[cfg(feature = "fts")]
    {
        use regex::Regex;
        let phrase_regex = Regex::new(r#""([^"]+)""#).unwrap();
        let mut phrases = Vec::new();

        for capture in phrase_regex.captures_iter(query) {
            if let Some(phrase) = capture.get(1) {
                phrases.push(phrase.as_str().to_string());
            }
        }

        // Remove phrases from query to get remaining terms
        let remaining = phrase_regex.replace_all(query, "").to_string();

        (phrases, remaining)
    }
    #[cfg(not(feature = "fts"))]
    {
        (vec![], query.to_string())
    }
}

/// Query builder for Tantivy search.
///
/// Builds weighted multi-field queries with configurable fuzzy matching.
/// Implements the same relevance weighting as simple search:
/// - Title: 3.0x boost
/// - Description: 2.0x boost
/// - Content: 1.0x boost (baseline)
///
/// Will be used when search backends are implemented (Phase 4+).
#[allow(dead_code)]
pub struct QueryBuilder<'a> {
    schema: &'a SearchSchema,
    config: &'a SearchConfig,
    fuzzy_enabled: bool,
    fuzzy_distance: u8,
}

impl<'a> QueryBuilder<'a> {
    /// Create a new QueryBuilder.
    ///
    /// # Arguments
    ///
    /// * `schema` - Reference to the SearchSchema
    /// * `config` - Search configuration with fuzzy settings
    #[allow(dead_code)]
    pub fn new(schema: &'a SearchSchema, config: &'a SearchConfig) -> Self {
        QueryBuilder {
            schema,
            config,
            fuzzy_enabled: config.fuzzy_search,
            fuzzy_distance: config.fuzzy_distance,
        }
    }

    /// Build a weighted multi-field query.
    ///
    /// Creates a BooleanQuery with Should (OR) clauses for each field,
    /// with appropriate boost weights to match simple search relevance.
    ///
    /// # Arguments
    ///
    /// * `query_str` - The search query string
    ///
    /// # Returns
    ///
    /// Returns a boxed Query ready for Tantivy search.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the query string is empty or invalid.
    #[allow(dead_code)]
    pub fn build_query(&self, query_str: &str) -> Result<Box<dyn Query>> {
        let query_str = query_str.trim();
        if query_str.is_empty() {
            return Err(Error::search_error(
                "Query string cannot be empty".to_string(),
            ));
        }

        // Parse phrases and terms
        let (phrases, remaining_query) = parse_phrases(query_str);

        // Apply stopword filtering to remaining terms if enabled
        let filtered_query = if self.config.enable_stopwords {
            let filter = crate::search::StopwordFilter::new(self.config);
            filter.filter(&remaining_query)
        } else {
            remaining_query
        };

        // Split filtered query into terms
        let terms: Vec<&str> = filtered_query.split_whitespace().collect();

        // Check if we have any searchable content
        if phrases.is_empty() && terms.is_empty() {
            return Err(Error::search_error(
                "Query contains no valid terms".to_string(),
            ));
        }

        // Create Should (OR) clauses for each field with appropriate boosts
        let mut clauses: Vec<(Occur, Box<dyn Query>)> = Vec::new();

        // Add phrase queries if present
        if !phrases.is_empty() {
            for phrase in &phrases {
                // Title field (3.0x boost)
                if let Ok(phrase_query) = self.create_phrase_query(self.schema.title, phrase) {
                    clauses.push((Occur::Should, Box::new(BoostQuery::new(phrase_query, 3.0))));
                }

                // Description field (2.0x boost)
                if let Ok(phrase_query) = self.create_phrase_query(self.schema.description, phrase)
                {
                    clauses.push((Occur::Should, Box::new(BoostQuery::new(phrase_query, 2.0))));
                }

                // Content field (1.0x boost - baseline)
                if let Ok(phrase_query) = self.create_phrase_query(self.schema.content, phrase) {
                    clauses.push((Occur::Should, phrase_query));
                }
            }
        }

        // Add term queries if present
        if !terms.is_empty() {
            // Title field (3.0x boost)
            if let Ok(query) = self.create_field_query(self.schema.title, &terms, 3.0) {
                clauses.push((Occur::Should, query));
            }

            // Description field (2.0x boost)
            if let Ok(query) = self.create_field_query(self.schema.description, &terms, 2.0) {
                clauses.push((Occur::Should, query));
            }

            // Content field (1.0x boost - baseline)
            if let Ok(query) = self.create_field_query(self.schema.content, &terms, 1.0) {
                clauses.push((Occur::Should, query));
            }
        }

        if clauses.is_empty() {
            return Err(Error::search_error(
                "Failed to create any field queries".to_string(),
            ));
        }

        Ok(Box::new(BooleanQuery::new(clauses)))
    }

    /// Determine the Occur mode based on query mode and term count.
    ///
    /// # Arguments
    ///
    /// * `term_count` - Number of terms in the query
    ///
    /// # Returns
    ///
    /// Returns the appropriate Occur mode for the query.
    fn determine_occur_mode(&self, term_count: usize) -> Occur {
        use crate::config::QueryMode;

        match &self.config.query_mode {
            QueryMode::Or => Occur::Should,
            QueryMode::And => Occur::Must,
            QueryMode::MinimumMatch(_) => Occur::Should, // Use with min_should_match
            QueryMode::Smart => {
                if term_count <= 2 {
                    Occur::Must // AND for 1-2 words
                } else {
                    Occur::Should // OR for 3+ with minimum match
                }
            }
        }
    }

    /// Create a phrase query for exact phrase matching.
    ///
    /// Tokenizes the phrase and creates a PhraseQuery that matches the exact
    /// sequence of tokens.
    ///
    /// # Arguments
    ///
    /// * `field` - The field to search
    /// * `phrase` - The phrase string to match
    ///
    /// # Returns
    ///
    /// Returns a boxed PhraseQuery for exact matching.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the phrase contains no valid tokens.
    fn create_phrase_query(
        &self,
        field: tantivy::schema::Field,
        phrase: &str,
    ) -> Result<Box<dyn Query>> {
        // Create tokenizer (same as index: lowercase + stem)
        let mut tokenizer = TextAnalyzer::builder(SimpleTokenizer::default())
            .filter(LowerCaser)
            .filter(Stemmer::default())
            .build();

        // Tokenize the phrase
        let mut token_stream = tokenizer.token_stream(phrase);
        let mut terms = Vec::new();

        while let Some(token) = token_stream.next() {
            let term = Term::from_field_text(field, &token.text);
            terms.push(term);
        }

        if terms.is_empty() {
            return Err(Error::search_error(
                "Phrase contains no valid terms".to_string(),
            ));
        }

        Ok(Box::new(PhraseQuery::new(terms)))
    }

    /// Create a field-specific query with boosting.
    ///
    /// For a single term, creates a TermQuery or FuzzyTermQuery.
    /// For multiple terms, creates a BooleanQuery with Should clauses.
    /// Wraps the result in a BoostQuery with the specified weight.
    ///
    /// # Arguments
    ///
    /// * `field` - The field to search
    /// * `terms` - The query terms
    /// * `boost` - The boost weight for this field
    ///
    /// # Returns
    ///
    /// Returns a boxed Query for this field with boost applied.
    ///
    /// # Errors
    ///
    /// Returns `Err` if term parsing fails.
    fn create_field_query(
        &self,
        field: tantivy::schema::Field,
        terms: &[&str],
        boost: f32,
    ) -> Result<Box<dyn Query>> {
        if terms.is_empty() {
            return Err(Error::search_error("No terms provided".to_string()));
        }

        // Create tokenizer (same as index: lowercase + stem)
        let mut tokenizer = TextAnalyzer::builder(SimpleTokenizer::default())
            .filter(LowerCaser)
            .filter(Stemmer::default())
            .build();

        // Tokenize all terms
        let mut tokenized_terms: Vec<String> = Vec::new();
        for term_str in terms {
            let mut token_stream = tokenizer.token_stream(term_str);
            while let Some(token) = token_stream.next() {
                tokenized_terms.push(token.text.to_string());
            }
        }

        let query: Box<dyn Query> = if tokenized_terms.len() == 1 {
            // Single term: use TermQuery or FuzzyTermQuery
            let term = Term::from_field_text(field, &tokenized_terms[0]);

            if self.fuzzy_enabled {
                Box::new(FuzzyTermQuery::new(term, self.fuzzy_distance, true))
            } else {
                Box::new(TermQuery::new(
                    term,
                    IndexRecordOption::WithFreqsAndPositions,
                ))
            }
        } else {
            // Multiple terms: use BooleanQuery with configurable occur mode
            let occur_mode = self.determine_occur_mode(tokenized_terms.len());
            let mut term_clauses: Vec<(Occur, Box<dyn Query>)> = Vec::new();

            for tokenized_term in &tokenized_terms {
                let term = Term::from_field_text(field, tokenized_term);

                let term_query: Box<dyn Query> = if self.fuzzy_enabled {
                    Box::new(FuzzyTermQuery::new(term, self.fuzzy_distance, true))
                } else {
                    Box::new(TermQuery::new(
                        term,
                        IndexRecordOption::WithFreqsAndPositions,
                    ))
                };

                term_clauses.push((occur_mode, term_query));
            }

            // For OR mode with 3+ terms, apply minimum match threshold
            // Note: Tantivy 0.22 doesn't have set_min_should_match
            // Future enhancement: when upgrading Tantivy, implement minimum match logic
            // For now, the occur mode change (AND vs OR) provides the main benefit
            let bool_query = BooleanQuery::new(term_clauses);

            Box::new(bool_query)
        };

        // Apply boost
        Ok(Box::new(BoostQuery::new(query, boost)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SearchConfig;

    fn test_config() -> SearchConfig {
        use crate::config::QueryMode;
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
            stopword_allowlist: vec![],
        }
    }

    fn test_config_with_fuzzy() -> SearchConfig {
        use crate::config::QueryMode;
        SearchConfig {
            backend: "tantivy".to_string(),
            index_path: ".tantivy-index".to_string(),
            rebuild_on_startup: false,
            snippet_size: 200,
            fuzzy_search: true,
            fuzzy_distance: 2,
            query_mode: QueryMode::Smart,
            minimum_match_percent: 0.6,
            enable_stopwords: true,
            custom_stopwords: vec![],
            stopword_allowlist: vec![],
        }
    }

    #[test]
    fn test_query_builder_new() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        assert!(!builder.fuzzy_enabled);
        assert_eq!(builder.fuzzy_distance, 2);
    }

    #[test]
    fn test_query_builder_with_fuzzy() {
        let schema = SearchSchema::build();
        let config = test_config_with_fuzzy();
        let builder = QueryBuilder::new(&schema, &config);

        assert!(builder.fuzzy_enabled);
        assert_eq!(builder.fuzzy_distance, 2);
    }

    #[test]
    fn test_build_query_single_term() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        let result = builder.build_query("harmony");
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_query_multiple_terms() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        let result = builder.build_query("parallel fifths");
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_query_empty_string() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        let result = builder.build_query("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_build_query_whitespace_only() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        let result = builder.build_query("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_query_with_fuzzy() {
        let schema = SearchSchema::build();
        let config = test_config_with_fuzzy();
        let builder = QueryBuilder::new(&schema, &config);

        // Should create fuzzy queries for typos
        let result = builder.build_query("haromny");
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_field_query_single_term() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        let terms = vec!["harmony"];
        let result = builder.create_field_query(schema.title, &terms, 3.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_field_query_multiple_terms() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        let terms = vec!["parallel", "fifths"];
        let result = builder.create_field_query(schema.content, &terms, 1.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_field_query_empty_terms() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        let terms: Vec<&str> = vec![];
        let result = builder.create_field_query(schema.title, &terms, 3.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_field_query_with_boost() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        let terms = vec!["test"];

        // All boosts should succeed
        assert!(builder
            .create_field_query(schema.title, &terms, 3.0)
            .is_ok());
        assert!(builder
            .create_field_query(schema.description, &terms, 2.0)
            .is_ok());
        assert!(builder
            .create_field_query(schema.content, &terms, 1.0)
            .is_ok());
    }

    #[test]
    fn test_query_builder_fuzzy_distance() {
        use crate::config::QueryMode;
        let schema = SearchSchema::build();
        let config = SearchConfig {
            backend: "tantivy".to_string(),
            index_path: ".tantivy-index".to_string(),
            rebuild_on_startup: false,
            snippet_size: 200,
            fuzzy_search: true,
            fuzzy_distance: 1,
            query_mode: QueryMode::Smart,
            minimum_match_percent: 0.6,
            enable_stopwords: true,
            custom_stopwords: vec![],
            stopword_allowlist: vec![],
        };
        let builder = QueryBuilder::new(&schema, &config);

        assert!(builder.fuzzy_enabled);
        assert_eq!(builder.fuzzy_distance, 1);
    }

    #[test]
    fn test_determine_occur_mode_smart_single_term() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        // Single term should use Must (AND) in Smart mode
        let occur = builder.determine_occur_mode(1);
        assert!(matches!(occur, Occur::Must));
    }

    #[test]
    fn test_determine_occur_mode_smart_two_terms() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        // Two terms should use Must (AND) in Smart mode
        let occur = builder.determine_occur_mode(2);
        assert!(matches!(occur, Occur::Must));
    }

    #[test]
    fn test_determine_occur_mode_smart_three_terms() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        // Three+ terms should use Should (OR) in Smart mode
        let occur = builder.determine_occur_mode(3);
        assert!(matches!(occur, Occur::Should));
    }

    #[test]
    fn test_determine_occur_mode_explicit_and() {
        use crate::config::QueryMode;
        let schema = SearchSchema::build();
        let mut config = test_config();
        config.query_mode = QueryMode::And;
        let builder = QueryBuilder::new(&schema, &config);

        // All term counts should use Must (AND) in And mode
        assert!(matches!(builder.determine_occur_mode(1), Occur::Must));
        assert!(matches!(builder.determine_occur_mode(2), Occur::Must));
        assert!(matches!(builder.determine_occur_mode(3), Occur::Must));
    }

    #[test]
    fn test_determine_occur_mode_explicit_or() {
        use crate::config::QueryMode;
        let schema = SearchSchema::build();
        let mut config = test_config();
        config.query_mode = QueryMode::Or;
        let builder = QueryBuilder::new(&schema, &config);

        // All term counts should use Should (OR) in Or mode
        assert!(matches!(builder.determine_occur_mode(1), Occur::Should));
        assert!(matches!(builder.determine_occur_mode(2), Occur::Should));
        assert!(matches!(builder.determine_occur_mode(3), Occur::Should));
    }

    #[test]
    fn test_determine_occur_mode_minimum_match() {
        use crate::config::QueryMode;
        let schema = SearchSchema::build();
        let mut config = test_config();
        config.query_mode = QueryMode::MinimumMatch(0.7);
        let builder = QueryBuilder::new(&schema, &config);

        // MinimumMatch mode always uses Should (OR with min_should_match)
        assert!(matches!(builder.determine_occur_mode(1), Occur::Should));
        assert!(matches!(builder.determine_occur_mode(2), Occur::Should));
        assert!(matches!(builder.determine_occur_mode(3), Occur::Should));
    }

    #[test]
    fn test_build_query_smart_mode_two_words() {
        let schema = SearchSchema::build();
        let config = test_config(); // Smart mode by default
        let builder = QueryBuilder::new(&schema, &config);

        // Two words should use AND logic (Must)
        let result = builder.build_query("authentic cadence");
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_query_smart_mode_three_words() {
        let schema = SearchSchema::build();
        let config = test_config(); // Smart mode by default
        let builder = QueryBuilder::new(&schema, &config);

        // Three words should use OR logic with minimum match
        let result = builder.build_query("fugue subject answer");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_phrases_single() {
        let (phrases, remaining) = parse_phrases(r#""imperfect consonance""#);
        assert_eq!(phrases, vec!["imperfect consonance"]);
        assert_eq!(remaining.trim(), "");
    }

    #[test]
    fn test_parse_phrases_mixed() {
        let (phrases, remaining) = parse_phrases(r#""perfect fifth" and octave"#);
        assert_eq!(phrases, vec!["perfect fifth"]);
        assert!(remaining.contains("and octave"));
    }

    #[test]
    fn test_parse_phrases_multiple() {
        let (phrases, remaining) = parse_phrases(r#""leading tone" to "tonic""#);
        assert_eq!(phrases, vec!["leading tone", "tonic"]);
        assert!(remaining.contains("to"));
    }

    #[test]
    fn test_parse_phrases_no_quotes() {
        let (phrases, remaining) = parse_phrases("cadence harmony");
        assert_eq!(phrases, Vec::<String>::new());
        assert_eq!(remaining, "cadence harmony");
    }

    #[test]
    fn test_parse_phrases_empty() {
        let (phrases, remaining) = parse_phrases("");
        assert_eq!(phrases, Vec::<String>::new());
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_create_phrase_query() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        let result = builder.create_phrase_query(schema.title, "perfect authentic cadence");
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_phrase_query_empty() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        let result = builder.create_phrase_query(schema.title, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_query_with_phrase() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        // Query with quoted phrase
        let result = builder.build_query(r#""imperfect consonance""#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_query_with_phrase_and_terms() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        // Mixed: phrase + regular terms
        let result = builder.build_query(r#""leading tone" resolution"#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_build_query_with_multiple_phrases() {
        let schema = SearchSchema::build();
        let config = test_config();
        let builder = QueryBuilder::new(&schema, &config);

        // Multiple phrases
        let result = builder.build_query(r#""perfect cadence" "dominant seventh""#);
        assert!(result.is_ok());
    }
}
