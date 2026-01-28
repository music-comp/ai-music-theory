//! Tantivy schema for full-text search of concept cards.
//!
//! This module defines the Tantivy schema mapping from SearchDocument,
//! with appropriate field types and options for efficient full-text search.

use tantivy::schema::{Field, Schema, TextFieldIndexing, TextOptions, FAST, STORED, STRING};
use tantivy::tokenizer::TokenizerManager;

/// Tantivy schema for concept card search.
///
/// Maps SearchDocument fields to Tantivy fields with appropriate indexing options:
/// - Identity fields (id, path): STORED for retrieval
/// - Full-text fields (title, description, content): TEXT with positions for phrase queries
/// - Facet fields (category, source, tags): STRING | FAST for filtering
/// - Metadata fields (chapter, part, etc.): STORED for display
///
/// Will be used by Tantivy search backend (Phase 2+).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SearchSchema {
    pub schema: Schema,

    // Identity fields
    pub id: Field,
    pub path: Field,

    // Full-text fields (indexed with positions for phrase queries)
    pub title: Field,
    pub description: Field,
    pub content: Field,

    // Facet fields (for filtering and faceted search)
    pub category: Field,
    pub source: Field,
    pub tags: Field,

    // Structured metadata (stored but not full-text indexed)
    pub chapter: Field,
    pub part: Field,
    pub author: Field,
    pub date: Field,

    // Content type discrimination (v0.3.0)
    pub content_type: Field, // STRING | FAST | STORED - for filtering by document type
    pub section: Field,      // STORED - fine-grained location (page numbers, sections)
}

impl SearchSchema {
    /// Build the Tantivy schema with all required fields.
    /// Will be used by Tantivy search backend (Phase 2+).
    #[allow(dead_code)]
    ///
    /// # Field Configuration
    ///
    /// **Full-text fields** (title, description, content):
    /// - Tokenized with en_stem (English stemming: "harmonic" matches "harmony")
    /// - Indexed with positions (enables phrase queries like "parallel fifths")
    /// - Stored for snippet generation
    ///
    /// **Facet fields** (category, source, tags):
    /// - STRING type for exact matching
    /// - FAST for efficient filtering
    /// - STORED for retrieval
    ///
    /// **Metadata fields** (id, path, chapter, part, author, date):
    /// - STORED only (not searchable, used for display)
    ///
    /// # Returns
    ///
    /// Returns a `SearchSchema` with all field handles.
    pub fn build() -> Self {
        let mut schema_builder = Schema::builder();

        // Identity fields (stored only)
        let id = schema_builder.add_text_field("id", STORED);
        let path = schema_builder.add_text_field("path", STORED);

        // Full-text fields with positions (for phrase queries)
        let text_indexing = TextFieldIndexing::default()
            .set_tokenizer("en_stem")
            .set_index_option(tantivy::schema::IndexRecordOption::WithFreqsAndPositions);

        let text_options = TextOptions::default()
            .set_indexing_options(text_indexing)
            .set_stored();

        let title = schema_builder.add_text_field("title", text_options.clone());
        let description = schema_builder.add_text_field("description", text_options.clone());
        let content = schema_builder.add_text_field("content", text_options);

        // Facet fields (for filtering)
        let category = schema_builder.add_text_field("category", STRING | FAST | STORED);
        let source = schema_builder.add_text_field("source", STRING | FAST | STORED);
        let tags = schema_builder.add_text_field("tags", STRING | FAST | STORED);

        // Structured metadata (stored only)
        let chapter = schema_builder.add_text_field("chapter", STORED);
        let part = schema_builder.add_u64_field("part", STORED);
        let author = schema_builder.add_text_field("author", STORED);
        let date = schema_builder.add_text_field("date", STORED);

        // Content type fields (v0.3.0)
        let content_type = schema_builder.add_text_field("content_type", STRING | FAST | STORED);
        let section = schema_builder.add_text_field("section", STORED);

        let schema = schema_builder.build();

        SearchSchema {
            schema,
            id,
            path,
            title,
            description,
            content,
            category,
            source,
            tags,
            chapter,
            part,
            author,
            date,
            content_type,
            section,
        }
    }

    /// Register tokenizers with the Tantivy tokenizer manager.
    ///
    /// This method registers the "en_stem" tokenizer used by full-text fields.
    /// Call this before creating an index.
    /// Will be used by Tantivy search backend (Phase 2+).
    ///
    /// # Arguments
    ///
    /// * `tokenizer_manager` - Tantivy's tokenizer manager
    #[allow(dead_code)]
    pub fn register_tokenizers(tokenizer_manager: &TokenizerManager) {
        use tantivy::tokenizer::{LowerCaser, SimpleTokenizer, Stemmer, TextAnalyzer};

        // Register en_stem tokenizer: lowercase + English stemming
        let tokenizer = TextAnalyzer::builder(SimpleTokenizer::default())
            .filter(LowerCaser)
            .filter(Stemmer::default())
            .build();

        tokenizer_manager.register("en_stem", tokenizer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_build_succeeds() {
        let schema = SearchSchema::build();
        assert_eq!(schema.schema.num_fields(), 14); // v0.3.0: added content_type and section
    }

    #[test]
    fn test_schema_has_identity_fields() {
        let schema = SearchSchema::build();
        let field_entry = schema.schema.get_field_entry(schema.id);
        assert_eq!(field_entry.name(), "id");
        let field_entry = schema.schema.get_field_entry(schema.path);
        assert_eq!(field_entry.name(), "path");
    }

    #[test]
    fn test_schema_has_fulltext_fields() {
        let schema = SearchSchema::build();

        let title_entry = schema.schema.get_field_entry(schema.title);
        assert_eq!(title_entry.name(), "title");

        let desc_entry = schema.schema.get_field_entry(schema.description);
        assert_eq!(desc_entry.name(), "description");

        let content_entry = schema.schema.get_field_entry(schema.content);
        assert_eq!(content_entry.name(), "content");
    }

    #[test]
    fn test_schema_has_facet_fields() {
        let schema = SearchSchema::build();

        let cat_entry = schema.schema.get_field_entry(schema.category);
        assert_eq!(cat_entry.name(), "category");

        let source_entry = schema.schema.get_field_entry(schema.source);
        assert_eq!(source_entry.name(), "source");

        let tags_entry = schema.schema.get_field_entry(schema.tags);
        assert_eq!(tags_entry.name(), "tags");
    }

    #[test]
    fn test_schema_has_metadata_fields() {
        let schema = SearchSchema::build();

        let chapter_entry = schema.schema.get_field_entry(schema.chapter);
        assert_eq!(chapter_entry.name(), "chapter");

        let part_entry = schema.schema.get_field_entry(schema.part);
        assert_eq!(part_entry.name(), "part");

        let author_entry = schema.schema.get_field_entry(schema.author);
        assert_eq!(author_entry.name(), "author");

        let date_entry = schema.schema.get_field_entry(schema.date);
        assert_eq!(date_entry.name(), "date");
    }

    #[test]
    fn test_schema_fulltext_fields_are_stored() {
        let schema = SearchSchema::build();

        let title_entry = schema.schema.get_field_entry(schema.title);
        assert!(title_entry.is_stored());

        let desc_entry = schema.schema.get_field_entry(schema.description);
        assert!(desc_entry.is_stored());

        let content_entry = schema.schema.get_field_entry(schema.content);
        assert!(content_entry.is_stored());
    }

    #[test]
    fn test_schema_fulltext_fields_are_indexed() {
        let schema = SearchSchema::build();

        let title_entry = schema.schema.get_field_entry(schema.title);
        assert!(title_entry.is_indexed());

        let desc_entry = schema.schema.get_field_entry(schema.description);
        assert!(desc_entry.is_indexed());

        let content_entry = schema.schema.get_field_entry(schema.content);
        assert!(content_entry.is_indexed());
    }

    #[test]
    fn test_schema_facet_fields_are_stored() {
        let schema = SearchSchema::build();

        let cat_entry = schema.schema.get_field_entry(schema.category);
        assert!(cat_entry.is_stored());

        let source_entry = schema.schema.get_field_entry(schema.source);
        assert!(source_entry.is_stored());

        let tags_entry = schema.schema.get_field_entry(schema.tags);
        assert!(tags_entry.is_stored());
    }

    #[test]
    fn test_schema_metadata_fields_are_stored() {
        let schema = SearchSchema::build();

        let id_entry = schema.schema.get_field_entry(schema.id);
        assert!(id_entry.is_stored());

        let path_entry = schema.schema.get_field_entry(schema.path);
        assert!(path_entry.is_stored());

        let chapter_entry = schema.schema.get_field_entry(schema.chapter);
        assert!(chapter_entry.is_stored());

        let author_entry = schema.schema.get_field_entry(schema.author);
        assert!(author_entry.is_stored());

        let date_entry = schema.schema.get_field_entry(schema.date);
        assert!(date_entry.is_stored());
    }

    #[test]
    fn test_register_tokenizers_doesnt_panic() {
        let _schema = SearchSchema::build();
        let tokenizer_manager = TokenizerManager::default();
        SearchSchema::register_tokenizers(&tokenizer_manager);

        // Verify tokenizer is registered
        assert!(tokenizer_manager.get("en_stem").is_some());
    }

    #[test]
    fn test_schema_field_count() {
        let schema = SearchSchema::build();
        // 2 identity + 3 full-text + 3 facets + 4 metadata + 2 content type = 14 fields
        assert_eq!(schema.schema.num_fields(), 14);
    }

    #[test]
    fn test_schema_has_content_type_field() {
        let schema = SearchSchema::build();
        let field_entry = schema.schema.get_field_entry(schema.content_type);
        assert_eq!(field_entry.name(), "content_type");
        assert!(field_entry.is_stored());
    }

    #[test]
    fn test_schema_has_section_field() {
        let schema = SearchSchema::build();
        let field_entry = schema.schema.get_field_entry(schema.section);
        assert_eq!(field_entry.name(), "section");
        assert!(field_entry.is_stored());
    }
}
