//! Source reference resolver with alias and fuzzy matching support.
//!
//! This module resolves source references from concept cards to
//! configured source IDs, using exact title matches, aliases,
//! and fuzzy matching for suggestions.

use std::collections::{HashMap, HashSet};

use crate::config::SourcesConfig;
use crate::sources::types::{ResolutionMethod, SourceSuggestion};

/// Resolver for source references with alias support.
pub struct SourceResolver {
    /// Maps exact titles (extracted from filenames) to source IDs
    title_to_id: HashMap<String, String>,
    /// Maps configured aliases to source IDs
    alias_to_id: HashMap<String, String>,
    /// All known source IDs
    known_ids: HashSet<String>,
    /// Maps source IDs to their extracted titles (for fuzzy matching)
    id_to_title: HashMap<String, String>,
}

impl SourceResolver {
    /// Create a new resolver from configuration.
    pub fn from_config(config: &SourcesConfig) -> Self {
        let mut resolver = Self {
            title_to_id: HashMap::new(),
            alias_to_id: HashMap::new(),
            known_ids: HashSet::new(),
            id_to_title: HashMap::new(),
        };

        // Process each category
        for (category, source_cat) in [
            ("oxford", &config.oxford),
            ("general", &config.general),
            ("papers", &config.papers),
        ] {
            for (file_id, filename) in &source_cat.files {
                let source_id = format!("{}-{}", category, file_id);
                let title = extract_title_from_filename(filename);

                resolver.known_ids.insert(source_id.clone());
                resolver
                    .title_to_id
                    .insert(title.clone(), source_id.clone());
                resolver.id_to_title.insert(source_id.clone(), title);

                // Add aliases
                if let Some(aliases) = source_cat.aliases.get(file_id) {
                    for alias in aliases {
                        resolver
                            .alias_to_id
                            .insert(alias.clone(), source_id.clone());
                    }
                }
            }
        }

        resolver
    }

    /// Resolve a source reference to its config ID.
    ///
    /// Returns the resolved ID (if found) and the method used.
    pub fn resolve(&self, reference: &str) -> (Option<String>, ResolutionMethod) {
        // 1. Try direct ID match
        if self.known_ids.contains(reference) {
            return (Some(reference.to_string()), ResolutionMethod::DirectId);
        }

        // 2. Try exact title match
        if let Some(id) = self.title_to_id.get(reference) {
            return (Some(id.clone()), ResolutionMethod::ExactTitle);
        }

        // 3. Try alias match
        if let Some(id) = self.alias_to_id.get(reference) {
            return (
                Some(id.clone()),
                ResolutionMethod::Alias(reference.to_string()),
            );
        }

        // 4. No match found
        (None, ResolutionMethod::Unresolved)
    }

    /// Get fuzzy match suggestions for an unresolved reference.
    pub fn suggest_matches(&self, reference: &str, threshold: f32) -> Vec<SourceSuggestion> {
        let mut suggestions = Vec::new();

        for (id, title) in &self.id_to_title {
            let similarity = strsim::normalized_levenshtein(reference, title) as f32;
            if similarity >= threshold {
                suggestions.push(SourceSuggestion::new(id, title, similarity));
            }
        }

        // Sort by similarity descending
        suggestions.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

        // Return top 3
        suggestions.truncate(3);
        suggestions
    }

    /// Get all known source IDs.
    pub fn known_ids(&self) -> &HashSet<String> {
        &self.known_ids
    }

    /// Get all titles mapped to IDs.
    pub fn titles(&self) -> impl Iterator<Item = (&String, &String)> {
        self.title_to_id.iter()
    }
}

/// Extract title from a source filename.
///
/// Expected format: `[YEAR] Author - Title.ext`
/// Returns the title portion after ` - ` and before the file extension.
pub fn extract_title_from_filename(filename: &str) -> String {
    // Extract title (after ` - ` and before .ext)
    if let Some(dash_pos) = filename.find(" - ") {
        if let Some(ext_pos) = filename.rfind('.') {
            if ext_pos > dash_pos + 3 {
                return filename[dash_pos + 3..ext_pos].trim().to_string();
            }
        }
        // No extension found, take everything after ` - `
        return filename[dash_pos + 3..].trim().to_string();
    }

    // Fallback: use filename without extension
    if let Some(ext_pos) = filename.rfind('.') {
        filename[..ext_pos].to_string()
    } else {
        filename.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_title_from_filename_standard() {
        let title = extract_title_from_filename("[2011] Tymoczko - A Geometry of Music.pdf");
        assert_eq!(title, "A Geometry of Music");
    }

    #[test]
    fn test_extract_title_from_filename_with_colon() {
        let title = extract_title_from_filename(
            "[1961] Persichetti - Twentieth-Century Harmony: Creative Aspects and Practice.pdf",
        );
        assert_eq!(
            title,
            "Twentieth-Century Harmony: Creative Aspects and Practice"
        );
    }

    #[test]
    fn test_extract_title_from_filename_no_extension() {
        let title = extract_title_from_filename("[2022] Gotham - Open Music Theory");
        assert_eq!(title, "Open Music Theory");
    }

    #[test]
    fn test_extract_title_from_filename_no_year() {
        let title = extract_title_from_filename("Author - Title.pdf");
        assert_eq!(title, "Title");
    }

    #[test]
    fn test_extract_title_from_filename_fallback() {
        let title = extract_title_from_filename("just-a-filename.pdf");
        assert_eq!(title, "just-a-filename");
    }

    #[test]
    fn test_resolver_resolve_direct_id() {
        let config = create_test_config();
        let resolver = SourceResolver::from_config(&config);

        let (id, method) = resolver.resolve("general-test-source");
        assert_eq!(id, Some("general-test-source".to_string()));
        assert_eq!(method, ResolutionMethod::DirectId);
    }

    #[test]
    fn test_resolver_resolve_exact_title() {
        let config = create_test_config();
        let resolver = SourceResolver::from_config(&config);

        let (id, method) = resolver.resolve("Test Title");
        assert_eq!(id, Some("general-test-source".to_string()));
        assert_eq!(method, ResolutionMethod::ExactTitle);
    }

    #[test]
    fn test_resolver_resolve_alias() {
        let config = create_test_config();
        let resolver = SourceResolver::from_config(&config);

        let (id, method) = resolver.resolve("Alternate Name");
        assert_eq!(id, Some("general-test-source".to_string()));
        assert!(matches!(method, ResolutionMethod::Alias(_)));
    }

    #[test]
    fn test_resolver_resolve_unresolved() {
        let config = create_test_config();
        let resolver = SourceResolver::from_config(&config);

        let (id, method) = resolver.resolve("Unknown Source");
        assert_eq!(id, None);
        assert_eq!(method, ResolutionMethod::Unresolved);
    }

    #[test]
    fn test_resolver_suggest_matches() {
        let config = create_test_config();
        let resolver = SourceResolver::from_config(&config);

        let suggestions = resolver.suggest_matches("Test Titl", 0.7); // Typo
        assert!(!suggestions.is_empty());
        assert!(suggestions[0].similarity > 0.7);
    }

    fn create_test_config() -> SourcesConfig {
        use std::collections::HashMap;

        let mut files = HashMap::new();
        files.insert(
            "test-source".to_string(),
            "[2020] Author - Test Title.pdf".to_string(),
        );

        let mut aliases = HashMap::new();
        aliases.insert(
            "test-source".to_string(),
            vec!["Alternate Name".to_string()],
        );

        SourcesConfig {
            oxford: crate::config::SourceCategory::default(),
            general: crate::config::SourceCategory {
                path: "/test".to_string(),
                files,
                aliases,
            },
            papers: crate::config::SourceCategory::default(),
        }
    }
}
