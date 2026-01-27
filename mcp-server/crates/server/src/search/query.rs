//! Query building for Tantivy search.
//!
//! This module provides QueryBuilder for constructing weighted multi-field queries
//! that match the relevance scoring used in simple search.

use tantivy::query::{BooleanQuery, BoostQuery, FuzzyTermQuery, Occur, Query, TermQuery};
use tantivy::schema::IndexRecordOption;
use tantivy::tokenizer::{LowerCaser, SimpleTokenizer, Stemmer, TextAnalyzer};
use tantivy::Term;

use crate::config::SearchConfig;
use crate::error::{Error, Result};
use crate::search::SearchSchema;

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
    pub fn new(schema: &'a SearchSchema, config: &SearchConfig) -> Self {
        QueryBuilder {
            schema,
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
            return Err(Error::search_error("Query string cannot be empty".to_string()));
        }

        // For now, treat the entire query as a single term
        // Future enhancement: parse into multiple terms and use phrase queries
        let terms: Vec<&str> = query_str.split_whitespace().collect();

        if terms.is_empty() {
            return Err(Error::search_error("Query contains no valid terms".to_string()));
        }

        // Create Should (OR) clauses for each field with appropriate boosts
        let mut clauses: Vec<(Occur, Box<dyn Query>)> = Vec::new();

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

        if clauses.is_empty() {
            return Err(Error::search_error(
                "Failed to create any field queries".to_string(),
            ));
        }

        Ok(Box::new(BooleanQuery::new(clauses)))
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
                Box::new(TermQuery::new(term, IndexRecordOption::WithFreqsAndPositions))
            }
        } else {
            // Multiple terms: use BooleanQuery with Should (OR)
            let mut term_clauses: Vec<(Occur, Box<dyn Query>)> = Vec::new();

            for tokenized_term in tokenized_terms {
                let term = Term::from_field_text(field, &tokenized_term);

                let term_query: Box<dyn Query> = if self.fuzzy_enabled {
                    Box::new(FuzzyTermQuery::new(term, self.fuzzy_distance, true))
                } else {
                    Box::new(TermQuery::new(term, IndexRecordOption::WithFreqsAndPositions))
                };

                term_clauses.push((Occur::Should, term_query));
            }

            Box::new(BooleanQuery::new(term_clauses))
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
        SearchConfig {
            backend: "tantivy".to_string(),
            index_path: ".tantivy-index".to_string(),
            rebuild_on_startup: false,
            snippet_size: 200,
            fuzzy_search: false,
            fuzzy_distance: 2,
        }
    }

    fn test_config_with_fuzzy() -> SearchConfig {
        SearchConfig {
            backend: "tantivy".to_string(),
            index_path: ".tantivy-index".to_string(),
            rebuild_on_startup: false,
            snippet_size: 200,
            fuzzy_search: true,
            fuzzy_distance: 2,
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
        assert!(builder.create_field_query(schema.title, &terms, 3.0).is_ok());
        assert!(builder.create_field_query(schema.description, &terms, 2.0).is_ok());
        assert!(builder.create_field_query(schema.content, &terms, 1.0).is_ok());
    }

    #[test]
    fn test_query_builder_fuzzy_distance() {
        let schema = SearchSchema::build();
        let config = SearchConfig {
            backend: "tantivy".to_string(),
            index_path: ".tantivy-index".to_string(),
            rebuild_on_startup: false,
            snippet_size: 200,
            fuzzy_search: true,
            fuzzy_distance: 1,
        };
        let builder = QueryBuilder::new(&schema, &config);

        assert!(builder.fuzzy_enabled);
        assert_eq!(builder.fuzzy_distance, 1);
    }
}
