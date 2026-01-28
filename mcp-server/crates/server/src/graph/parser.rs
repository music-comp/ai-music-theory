//! Parser for extracting concept relationships from markdown.
//!
//! This module provides functionality to parse the "Related Concepts" section
//! from concept card markdown files and extract relationship information.

/// Parsed relationships from a "Related Concepts" section.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RelatedConcepts {
    /// Concepts that are prerequisites for this concept
    pub prerequisite: Vec<String>,
    /// Concepts that this concept leads to
    pub leads_to: Vec<String>,
    /// Related concepts for further exploration
    pub see_also: Vec<String>,
}

impl RelatedConcepts {
    /// Check if all relationship lists are empty.
    pub fn is_empty(&self) -> bool {
        self.prerequisite.is_empty() && self.leads_to.is_empty() && self.see_also.is_empty()
    }

    /// Get total count of all relationships.
    pub fn total_count(&self) -> usize {
        self.prerequisite.len() + self.leads_to.len() + self.see_also.len()
    }
}

/// Parse the "Related Concepts" section from markdown content.
///
/// This function looks for a markdown heading "## Related Concepts" followed by
/// bullet points in the format:
/// ```markdown
/// ## Related Concepts
///
/// - **Prerequisite**: concept-id-1, concept-id-2
/// - **Leads to**: concept-id-3
/// - **See also**: concept-id-4, concept-id-5
/// ```
///
/// # Arguments
///
/// * `markdown` - The markdown content to parse
///
/// # Returns
///
/// Returns `Some(RelatedConcepts)` if the section is found, `None` otherwise.
pub fn parse_related_concepts(markdown: &str) -> Option<RelatedConcepts> {
    // Find the "Related Concepts" heading
    let section_start = find_related_concepts_section(markdown)?;

    // Extract content from heading to next heading or end
    let section_content = extract_section_content(markdown, section_start)?;

    // Parse the three relationship types
    let mut related = RelatedConcepts::default();

    for line in section_content.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        // Match bullet points with bold keywords
        if let Some(ids) = parse_relationship_line(line, "Prerequisite") {
            related.prerequisite = ids;
        } else if let Some(ids) = parse_relationship_line(line, "Leads to") {
            related.leads_to = ids;
        } else if let Some(ids) = parse_relationship_line(line, "See also") {
            related.see_also = ids;
        }
    }

    // Only return Some if we found at least one relationship
    if related.is_empty() {
        None
    } else {
        Some(related)
    }
}

/// Find the start position of the "Related Concepts" section.
fn find_related_concepts_section(markdown: &str) -> Option<usize> {
    // Look for various heading formats
    for heading in &[
        "## Related Concepts",
        "##Related Concepts",
        "## related concepts",
    ] {
        if let Some(pos) = markdown.find(heading) {
            return Some(pos);
        }
    }
    None
}

/// Extract content from a heading to the next heading or end of document.
fn extract_section_content(markdown: &str, start: usize) -> Option<&str> {
    // Find the end of the heading line
    let content_start = markdown[start..].find('\n')? + start + 1;

    // Find the next heading (line starting with #) or end of document
    let rest = &markdown[content_start..];
    let content_end = rest
        .lines()
        .position(|line| line.trim_start().starts_with('#'))
        .map(|line_idx| {
            // Calculate byte position of that line
            rest.lines()
                .take(line_idx)
                .map(|l| l.len() + 1) // +1 for newline
                .sum::<usize>()
        })
        .unwrap_or(rest.len());

    Some(&rest[..content_end])
}

/// Parse a single relationship line and extract concept IDs.
///
/// Expected format: `- **Keyword**: id1, id2, id3`
fn parse_relationship_line(line: &str, keyword: &str) -> Option<Vec<String>> {
    // Remove bullet point and trim
    let line = line.trim_start_matches('-').trim();

    // Check for bold keyword followed by colon
    // Formats: **Keyword**: or ** Keyword **: or **Keyword:**
    let patterns = [
        format!("**{}**:", keyword),
        format!("** {} **:", keyword),
        format!("**{}:**", keyword),
    ];

    for pattern in &patterns {
        if line.starts_with(pattern) {
            let ids_str = line[pattern.len()..].trim();
            return Some(parse_concept_ids(ids_str));
        }
    }

    None
}

/// Parse comma-separated concept IDs and normalize them.
fn parse_concept_ids(ids_str: &str) -> Vec<String> {
    ids_str
        .split(',')
        .map(|id| normalize_concept_id(id.trim()))
        .filter(|id| !id.is_empty())
        .collect()
}

/// Normalize a concept ID: trim whitespace, convert to lowercase, replace spaces with hyphens.
fn normalize_concept_id(id: &str) -> String {
    id.trim()
        .to_lowercase()
        .split_whitespace() // Split on any whitespace, collapsing multiple spaces
        .collect::<Vec<&str>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_section() {
        let markdown = r#"# Suspension

Some content here.

## Related Concepts

- **Prerequisite**: dissonance, voice-leading, non-chord-tone
- **Leads to**: retardation, anticipation
- **See also**: appoggiatura, passing-tone

## Common Confusions

More content.
"#;

        let related = parse_related_concepts(markdown).unwrap();
        assert_eq!(
            related.prerequisite,
            vec!["dissonance", "voice-leading", "non-chord-tone"]
        );
        assert_eq!(related.leads_to, vec!["retardation", "anticipation"]);
        assert_eq!(related.see_also, vec!["appoggiatura", "passing-tone"]);
        assert_eq!(related.total_count(), 7);
    }

    #[test]
    fn test_parse_missing_section() {
        let markdown = r#"# Suspension

Some content here.

## Common Confusions

More content.
"#;

        let related = parse_related_concepts(markdown);
        assert_eq!(related, None);
    }

    #[test]
    fn test_parse_empty_section() {
        let markdown = r#"# Suspension

## Related Concepts

## Common Confusions
"#;

        let related = parse_related_concepts(markdown);
        assert_eq!(related, None);
    }

    #[test]
    fn test_parse_partial_section() {
        let markdown = r#"## Related Concepts

- **Prerequisite**: dissonance
- **Leads to**:

## Next Section
"#;

        let related = parse_related_concepts(markdown).unwrap();
        assert_eq!(related.prerequisite, vec!["dissonance"]);
        assert_eq!(related.leads_to, Vec::<String>::new());
        assert_eq!(related.see_also, Vec::<String>::new());
    }

    #[test]
    fn test_normalize_concept_id() {
        assert_eq!(normalize_concept_id("Dissonance"), "dissonance");
        assert_eq!(normalize_concept_id(" voice-leading "), "voice-leading");
        assert_eq!(normalize_concept_id("Non Chord Tone"), "non-chord-tone");
        assert_eq!(normalize_concept_id("  Mixed   Case  "), "mixed-case");
    }

    #[test]
    fn test_parse_concept_ids() {
        let ids = parse_concept_ids("dissonance, Voice Leading, non-chord-tone");
        assert_eq!(ids, vec!["dissonance", "voice-leading", "non-chord-tone"]);
    }

    #[test]
    fn test_parse_concept_ids_with_extra_spaces() {
        let ids = parse_concept_ids("  dissonance  ,  voice-leading  ,  non-chord-tone  ");
        assert_eq!(ids, vec!["dissonance", "voice-leading", "non-chord-tone"]);
    }

    #[test]
    fn test_parse_concept_ids_single() {
        let ids = parse_concept_ids("dissonance");
        assert_eq!(ids, vec!["dissonance"]);
    }

    #[test]
    fn test_parse_concept_ids_empty() {
        let ids = parse_concept_ids("");
        assert_eq!(ids, Vec::<String>::new());
    }

    #[test]
    fn test_parse_relationship_line_prerequisite() {
        let line = "- **Prerequisite**: dissonance, voice-leading";
        let ids = parse_relationship_line(line, "Prerequisite").unwrap();
        assert_eq!(ids, vec!["dissonance", "voice-leading"]);
    }

    #[test]
    fn test_parse_relationship_line_leads_to() {
        let line = "- **Leads to**: retardation, anticipation";
        let ids = parse_relationship_line(line, "Leads to").unwrap();
        assert_eq!(ids, vec!["retardation", "anticipation"]);
    }

    #[test]
    fn test_parse_relationship_line_see_also() {
        let line = "- **See also**: appoggiatura, passing-tone";
        let ids = parse_relationship_line(line, "See also").unwrap();
        assert_eq!(ids, vec!["appoggiatura", "passing-tone"]);
    }

    #[test]
    fn test_parse_relationship_line_no_match() {
        let line = "- Something else: value";
        let ids = parse_relationship_line(line, "Prerequisite");
        assert_eq!(ids, None);
    }

    #[test]
    fn test_related_concepts_is_empty() {
        let empty = RelatedConcepts::default();
        assert!(empty.is_empty());

        let non_empty = RelatedConcepts {
            prerequisite: vec!["test".to_string()],
            leads_to: vec![],
            see_also: vec![],
        };
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn test_related_concepts_total_count() {
        let related = RelatedConcepts {
            prerequisite: vec!["a".to_string(), "b".to_string()],
            leads_to: vec!["c".to_string()],
            see_also: vec!["d".to_string(), "e".to_string(), "f".to_string()],
        };
        assert_eq!(related.total_count(), 6);
    }

    #[test]
    fn test_parse_case_insensitive_heading() {
        let markdown = r#"## related concepts

- **Prerequisite**: test
"#;
        let related = parse_related_concepts(markdown).unwrap();
        assert_eq!(related.prerequisite, vec!["test"]);
    }

    #[test]
    fn test_parse_without_spaces_in_heading() {
        let markdown = r#"##Related Concepts

- **Prerequisite**: test
"#;
        let related = parse_related_concepts(markdown).unwrap();
        assert_eq!(related.prerequisite, vec!["test"]);
    }

    #[test]
    fn test_parse_at_end_of_document() {
        let markdown = r#"# Title

## Related Concepts

- **Prerequisite**: test
- **Leads to**: other"#;

        let related = parse_related_concepts(markdown).unwrap();
        assert_eq!(related.prerequisite, vec!["test"]);
        assert_eq!(related.leads_to, vec!["other"]);
    }

    #[test]
    fn test_parse_with_bullet_variations() {
        // Test with extra whitespace
        let markdown = r#"## Related Concepts

  -  **Prerequisite**:  test
-   **Leads to**: other
"#;
        let related = parse_related_concepts(markdown).unwrap();
        assert_eq!(related.prerequisite, vec!["test"]);
        assert_eq!(related.leads_to, vec!["other"]);
    }
}
