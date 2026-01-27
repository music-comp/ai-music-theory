//! Tantivy indexer for concept cards.
//!
//! This module provides the Indexer for adding SearchDocuments to a Tantivy index.

use std::path::Path;
use tantivy::{Index, IndexWriter, TantivyDocument};

use crate::error::{Error, Result};
use crate::search::{SearchDocument, SearchSchema};

/// Tantivy indexer for concept cards.
///
/// Provides methods to add SearchDocuments to a Tantivy index,
/// with conversion from SearchDocument to Tantivy Document.
/// Will be used when search backends are implemented (Phase 3+).
#[allow(dead_code)]
pub struct Indexer {
    schema: SearchSchema,
    index: Index,
    writer: IndexWriter,
}

impl Indexer {
    /// Create or open a Tantivy index at the specified path.
    ///
    /// Creates the directory if it doesn't exist, and either opens
    /// an existing index or creates a new one.
    ///
    /// # Arguments
    ///
    /// * `index_path` - Path to the Tantivy index directory
    ///
    /// # Returns
    ///
    /// Returns an `Indexer` ready to add documents.
    ///
    /// # Errors
    ///
    /// Returns `Err` if:
    /// - Directory cannot be created
    /// - Index cannot be opened or created
    /// - Index writer cannot be created
    pub fn new(index_path: &Path) -> Result<Self> {
        // Create directory if needed
        if !index_path.exists() {
            std::fs::create_dir_all(index_path).map_err(|e| Error::io_with_path(e, index_path))?;
        }

        // Build schema
        let schema = SearchSchema::build();

        // Try to open existing index, or create new one
        let index = if index_path.join("meta.json").exists() {
            Index::open_in_dir(index_path).map_err(|e| {
                Error::search_error(format!(
                    "Failed to open existing index at {}: {}",
                    index_path.display(),
                    e
                ))
            })?
        } else {
            Index::create_in_dir(index_path, schema.schema.clone()).map_err(|e| {
                Error::search_error(format!(
                    "Failed to create new index at {}: {}",
                    index_path.display(),
                    e
                ))
            })?
        };

        // Register tokenizers
        SearchSchema::register_tokenizers(index.tokenizers());

        // Create writer with 50MB buffer
        let writer = index
            .writer(50_000_000)
            .map_err(|e| Error::search_error(format!("Failed to create index writer: {}", e)))?;

        Ok(Indexer {
            schema,
            index,
            writer,
        })
    }

    /// Add a SearchDocument to the index.
    ///
    /// Converts the SearchDocument to a Tantivy Document and adds it to the index.
    /// Changes are not persisted until `commit()` is called.
    ///
    /// # Arguments
    ///
    /// * `doc` - The SearchDocument to add
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the document was added successfully.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the document cannot be added to the index.
    pub fn add_document(&mut self, doc: &SearchDocument) -> Result<()> {
        let tantivy_doc = self.convert_to_tantivy_doc(doc)?;
        self.writer
            .add_document(tantivy_doc)
            .map_err(|e| Error::search_error(format!("Failed to add document to index: {}", e)))?;
        Ok(())
    }

    /// Commit pending changes to the index.
    ///
    /// Flushes all pending documents to disk. This must be called
    /// to persist any documents added via `add_document()`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if changes were committed successfully.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the commit fails.
    pub fn commit(&mut self) -> Result<()> {
        self.writer
            .commit()
            .map_err(|e| Error::search_error(format!("Failed to commit index: {}", e)))?;
        Ok(())
    }

    /// Clear the index by deleting all documents.
    ///
    /// This prepares the index for a full rebuild. Changes are committed immediately.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the index was cleared successfully.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the clear or commit fails.
    pub fn clear(&mut self) -> Result<()> {
        self.writer
            .delete_all_documents()
            .map_err(|e| Error::search_error(format!("Failed to delete all documents: {}", e)))?;
        self.commit()?;
        Ok(())
    }

    /// Get a reference to the Index for creating searchers.
    ///
    /// This is used by search backends to create Tantivy Searchers.
    /// Will be used when search backends are implemented (Phase 3+).
    #[allow(dead_code)]
    pub fn index(&self) -> &Index {
        &self.index
    }

    /// Convert a SearchDocument to a Tantivy Document.
    ///
    /// Maps all fields from SearchDocument to the appropriate Tantivy field types.
    ///
    /// # Arguments
    ///
    /// * `doc` - The SearchDocument to convert
    ///
    /// # Returns
    ///
    /// Returns a Tantivy `Document` ready to be indexed.
    ///
    /// # Errors
    ///
    /// Returns `Err` if field conversion fails (should not happen in practice).
    fn convert_to_tantivy_doc(&self, doc: &SearchDocument) -> Result<TantivyDocument> {
        let mut tantivy_doc = TantivyDocument::default();

        // Identity fields
        tantivy_doc.add_text(self.schema.id, &doc.id);
        tantivy_doc.add_text(self.schema.path, &doc.path);

        // Full-text fields (always present)
        tantivy_doc.add_text(self.schema.title, &doc.title);
        tantivy_doc.add_text(self.schema.description, &doc.description);
        tantivy_doc.add_text(self.schema.content, &doc.content);

        // Facet fields
        tantivy_doc.add_text(self.schema.category, &doc.category);

        if let Some(source) = &doc.source {
            tantivy_doc.add_text(self.schema.source, source);
        }

        for tag in &doc.tags {
            tantivy_doc.add_text(self.schema.tags, tag);
        }

        // Metadata fields (optional)
        if let Some(chapter) = &doc.chapter {
            tantivy_doc.add_text(self.schema.chapter, chapter);
        }

        if let Some(part) = doc.part {
            tantivy_doc.add_u64(self.schema.part, part as u64);
        }

        if let Some(author) = &doc.author {
            tantivy_doc.add_text(self.schema.author, author);
        }

        if let Some(date) = &doc.date {
            tantivy_doc.add_text(self.schema.date, date);
        }

        Ok(tantivy_doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn sample_search_doc() -> SearchDocument {
        SearchDocument {
            id: "test-harmony".to_string(),
            path: "/test/harmony.md".to_string(),
            category: "harmony".to_string(),
            source: Some("Open Music Theory".to_string()),
            tags: vec!["triads".to_string(), "chords".to_string()],
            title: "Triads and Seventh Chords".to_string(),
            description: "An introduction to triads and seventh chords".to_string(),
            content: "Triads are three-note chords...".to_string(),
            chapter: Some("Chapter 3".to_string()),
            part: Some(1),
            author: Some("Test Author".to_string()),
            date: Some("2024-01-01".to_string()),
        }
    }

    #[test]
    fn test_indexer_new_creates_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        assert!(!index_path.exists());

        let result = Indexer::new(&index_path);
        assert!(result.is_ok());
        assert!(index_path.exists());
    }

    #[test]
    fn test_indexer_new_opens_existing_index() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        // Create initial index
        let indexer1 = Indexer::new(&index_path).expect("Failed to create index");
        drop(indexer1);

        // Open existing index
        let result = Indexer::new(&index_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_indexer_add_document_success() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        let mut indexer = Indexer::new(&index_path).expect("Failed to create indexer");
        let doc = sample_search_doc();

        let result = indexer.add_document(&doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_indexer_commit_success() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        let mut indexer = Indexer::new(&index_path).expect("Failed to create indexer");
        let doc = sample_search_doc();

        indexer.add_document(&doc).expect("Failed to add document");
        let result = indexer.commit();
        assert!(result.is_ok());
    }

    #[test]
    fn test_indexer_clear_success() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        let mut indexer = Indexer::new(&index_path).expect("Failed to create indexer");
        let doc = sample_search_doc();

        indexer.add_document(&doc).expect("Failed to add document");
        indexer.commit().expect("Failed to commit");

        let result = indexer.clear();
        assert!(result.is_ok());
    }

    #[test]
    fn test_indexer_index_reference() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        let indexer = Indexer::new(&index_path).expect("Failed to create indexer");
        let index = indexer.index();

        // Verify we can access index methods
        assert!(index.schema().num_fields() > 0);
    }

    #[test]
    fn test_convert_to_tantivy_doc_all_fields() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        let indexer = Indexer::new(&index_path).expect("Failed to create indexer");
        let search_doc = sample_search_doc();

        let result = indexer.convert_to_tantivy_doc(&search_doc);
        assert!(result.is_ok());

        let tantivy_doc = result.unwrap();
        let field_values = tantivy_doc.field_values();

        // Should have 12 field values (all fields populated in sample)
        assert_eq!(field_values.len(), 13); // +2 for two tags
    }

    #[test]
    fn test_convert_to_tantivy_doc_minimal_fields() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        let indexer = Indexer::new(&index_path).expect("Failed to create indexer");
        let search_doc = SearchDocument {
            id: "test".to_string(),
            path: "/test.md".to_string(),
            category: "harmony".to_string(),
            source: None,
            tags: vec![],
            title: "Test".to_string(),
            description: "Test description".to_string(),
            content: "Test content".to_string(),
            chapter: None,
            part: None,
            author: None,
            date: None,
        };

        let result = indexer.convert_to_tantivy_doc(&search_doc);
        assert!(result.is_ok());

        let tantivy_doc = result.unwrap();
        let field_values = tantivy_doc.field_values();

        // Should have 6 field values: id, path, category, title, description, content
        assert_eq!(field_values.len(), 6);
    }

    #[test]
    fn test_convert_to_tantivy_doc_with_tags() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        let indexer = Indexer::new(&index_path).expect("Failed to create indexer");
        let search_doc = SearchDocument {
            id: "test".to_string(),
            path: "/test.md".to_string(),
            category: "harmony".to_string(),
            source: None,
            tags: vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            title: "Test".to_string(),
            description: "Test description".to_string(),
            content: "Test content".to_string(),
            chapter: None,
            part: None,
            author: None,
            date: None,
        };

        let result = indexer.convert_to_tantivy_doc(&search_doc);
        assert!(result.is_ok());

        let tantivy_doc = result.unwrap();
        let field_values = tantivy_doc.field_values();

        // Should have 6 base fields + 3 tags = 9
        assert_eq!(field_values.len(), 9);
    }

    #[test]
    fn test_indexer_add_multiple_documents() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");

        let mut indexer = Indexer::new(&index_path).expect("Failed to create indexer");

        let doc1 = SearchDocument {
            id: "doc1".to_string(),
            path: "/doc1.md".to_string(),
            category: "harmony".to_string(),
            source: None,
            tags: vec![],
            title: "Document 1".to_string(),
            description: "First document".to_string(),
            content: "Content 1".to_string(),
            chapter: None,
            part: None,
            author: None,
            date: None,
        };

        let doc2 = SearchDocument {
            id: "doc2".to_string(),
            path: "/doc2.md".to_string(),
            category: "rhythm".to_string(),
            source: None,
            tags: vec![],
            title: "Document 2".to_string(),
            description: "Second document".to_string(),
            content: "Content 2".to_string(),
            chapter: None,
            part: None,
            author: None,
            date: None,
        };

        assert!(indexer.add_document(&doc1).is_ok());
        assert!(indexer.add_document(&doc2).is_ok());
        assert!(indexer.commit().is_ok());
    }
}
