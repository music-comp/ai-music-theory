use serde::{Deserialize, Serialize};
use std::fs;

use crate::config::Config;
use crate::error::Result;

// Resource constants
const MIME_TYPE_MARKDOWN: &str = "text/markdown";

const URI_CONVENTIONS: &str = "skill://conventions";
const NAME_CONVENTIONS: &str = "Music Theory Conventions";
const DESC_CONVENTIONS: &str = "Notation conventions and terminology used in this skill";

const URI_SCOPE: &str = "skill://scope";
const NAME_SCOPE: &str = "Skill Scope";
const DESC_SCOPE: &str = "Topics covered and learning objectives of this skill";

const URI_SOURCES: &str = "skill://sources";
const NAME_SOURCES: &str = "Source Materials";
const DESC_SOURCES: &str = "Bibliography and source attribution";

const URI_INDEX: &str = "skill://index";
const NAME_INDEX: &str = "Skill Index";
const DESC_INDEX: &str = "Complete index of concepts, topics, and materials";

/// Resource information for MCP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub mime_type: String,
}

/// List all available resources.
pub fn list_resources() -> Vec<ResourceInfo> {
    vec![
        ResourceInfo {
            uri: URI_CONVENTIONS.to_string(),
            name: NAME_CONVENTIONS.to_string(),
            description: DESC_CONVENTIONS.to_string(),
            mime_type: MIME_TYPE_MARKDOWN.to_string(),
        },
        ResourceInfo {
            uri: URI_SCOPE.to_string(),
            name: NAME_SCOPE.to_string(),
            description: DESC_SCOPE.to_string(),
            mime_type: MIME_TYPE_MARKDOWN.to_string(),
        },
        ResourceInfo {
            uri: URI_SOURCES.to_string(),
            name: NAME_SOURCES.to_string(),
            description: DESC_SOURCES.to_string(),
            mime_type: MIME_TYPE_MARKDOWN.to_string(),
        },
        ResourceInfo {
            uri: URI_INDEX.to_string(),
            name: NAME_INDEX.to_string(),
            description: DESC_INDEX.to_string(),
            mime_type: MIME_TYPE_MARKDOWN.to_string(),
        },
    ]
}

/// Get resource content by URI.
pub fn get_resource(config: &Config, uri: &str) -> Result<String> {
    let skill_docs_path = config.paths.skill_docs_path()?;

    let content = match uri {
        URI_CONVENTIONS => {
            let path = skill_docs_path.join("CONVENTIONS.md");
            fs::read_to_string(path).unwrap_or_else(|_| default_conventions())
        }
        URI_SCOPE => {
            let path = skill_docs_path.join("SCOPE.md");
            fs::read_to_string(path).unwrap_or_else(|_| default_scope())
        }
        URI_SOURCES => {
            let path = skill_docs_path.join("SOURCES.md");
            fs::read_to_string(path).unwrap_or_else(|_| default_sources())
        }
        URI_INDEX => {
            let path = skill_docs_path.join("INDEX.md");
            fs::read_to_string(path).unwrap_or_else(|_| default_index())
        }
        _ => {
            return Err(crate::error::Error::not_found(std::path::PathBuf::from(
                uri,
            )))
        }
    };

    Ok(content)
}

fn default_conventions() -> String {
    r#"# Music Theory Conventions

## Notation

This skill uses the following notation conventions:

- **Pitches**: Scientific pitch notation (C4, D#5, Bb3)
- **Intervals**: Numeric intervals (P5, M3, d7) or semitone counts
- **Chords**: Roman numerals (I, ii, V7) and chord symbols (Cmaj7, Dm)
- **Set Classes**: Prime form notation [0,1,4,8]

## Terminology

- **Diatonic**: Within a major or minor key
- **Chromatic**: Including all 12 pitch classes
- **Enharmonic**: Different notation, same pitch (C# = Db)

## References

Sources are cited using author-date format (Lewin 2007, Tymoczko 2011).
"#
    .to_string()
}

fn default_scope() -> String {
    r#"# Music Theory Skill Scope

## Topics Covered

This skill provides comprehensive coverage of:

### Fundamentals
- Pitch, intervals, scales
- Rhythm and meter
- Clefs and notation

### Harmony
- Triads and seventh chords
- Chord progressions
- Voice leading

### Advanced Topics
- Neo-Riemannian transformations
- Set theory and twelve-tone techniques
- Transformational theory

## Learning Objectives

After studying this skill, you should be able to:

1. Analyze harmonic progressions in tonal music
2. Apply transformational theory to analyze chromatic music
3. Understand mathematical structures in music theory
4. Use set theory for atonal analysis

## Prerequisites

Basic music literacy (reading notation, understanding scales) is recommended.
"#
    .to_string()
}

fn default_sources() -> String {
    r#"# Source Materials

## Primary Sources

### Transformational Theory
- Lewin, David. *Generalized Musical Intervals and Transformations* (2007)

### Geometry of Music
- Tymoczko, Dmitri. *A Geometry of Music* (2011)
- Tymoczko, Dmitri. *Tonality: An Owner's Manual* (2023)

### Neo-Riemannian Theory
- Cohn, Richard. *Audacious Euphony* (2012)
- Gollin, Edward. *The Oxford Handbook of Neo-Riemannian Music Theories* (2012)

### Post-Tonal Theory
- Straus, Joseph. *Introduction to Post-Tonal Theory* (2016)

### Online Resources
- Gotham, Mark. *Open Music Theory* (2022)
- Hutchinson, Bryn. *Music Theory for the 21st-Century Classroom* (2023)

## Attribution

All source materials are used for educational purposes and are properly attributed.
"#
    .to_string()
}

fn default_index() -> String {
    r#"# Skill Index

## Concepts by Category

### Fundamentals
- Intervals
- Scales
- Key Signatures
- Rhythm

### Harmony
- Triads
- Seventh Chords
- Chord Progressions
- Voice Leading
- Cadences

### Transformational Theory
- GIS (Generalized Interval Systems)
- Transformational Networks
- Klumpenhouwer Networks

### Neo-Riemannian Theory
- PLR Transformations
- Tonnetz
- Triadic Transformations

### Set Theory
- Pitch Class Sets
- Prime Form
- Set Class Relations
- Interval Class Vectors

## Sources Index

See the `list_sources` tool for a complete list of available source materials.

## Guides Index

See the `list_guides` tool for topic-specific guides.
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_resources() {
        let resources = list_resources();
        assert_eq!(resources.len(), 4);

        // Check conventions resource
        assert_eq!(resources[0].uri, "skill://conventions");
        assert_eq!(resources[0].name, "Music Theory Conventions");
        assert_eq!(resources[0].mime_type, "text/markdown");

        // Check scope resource
        assert_eq!(resources[1].uri, "skill://scope");
        assert_eq!(resources[1].name, "Skill Scope");

        // Check sources resource
        assert_eq!(resources[2].uri, "skill://sources");
        assert_eq!(resources[2].name, "Source Materials");

        // Check index resource
        assert_eq!(resources[3].uri, "skill://index");
        assert_eq!(resources[3].name, "Skill Index");
    }

    #[test]
    fn test_get_resource_conventions() {
        let config = Config::load().expect("Config should load");
        let content = get_resource(&config, URI_CONVENTIONS).expect("Should get conventions");
        assert!(content.contains("Music Theory Conventions") || content.contains("Notation"));
    }

    #[test]
    fn test_get_resource_scope() {
        let config = Config::load().expect("Config should load");
        let content = get_resource(&config, URI_SCOPE).expect("Should get scope");
        assert!(!content.is_empty(), "Scope resource should have content");
        assert!(
            content.starts_with('#'),
            "Scope should be valid markdown with heading"
        );
    }

    #[test]
    fn test_get_resource_sources() {
        let config = Config::load().expect("Config should load");
        let content = get_resource(&config, URI_SOURCES).expect("Should get sources");
        assert!(!content.is_empty(), "Sources resource should have content");
        assert!(
            content.starts_with('#'),
            "Sources should be valid markdown with heading"
        );
    }

    #[test]
    fn test_get_resource_index() {
        let config = Config::load().expect("Config should load");
        let content = get_resource(&config, URI_INDEX).expect("Should get index");
        assert!(
            content.contains("Skill Index")
                || content.contains("Index")
                || content.contains("Concepts")
        );
    }

    #[test]
    fn test_get_resource_unknown_uri() {
        let config = Config::load().expect("Config should load");
        let result = get_resource(&config, "skill://unknown");
        assert!(result.is_err());
        assert!(result.unwrap_err().is_not_found());
    }

    #[test]
    fn test_default_conventions() {
        let content = default_conventions();
        assert!(content.contains("Music Theory Conventions"));
        assert!(content.contains("Notation"));
        assert!(content.contains("Scientific pitch notation"));
        assert!(content.contains("Terminology"));
    }

    #[test]
    fn test_default_scope() {
        let content = default_scope();
        assert!(content.contains("Music Theory Skill Scope"));
        assert!(content.contains("Topics Covered"));
        assert!(content.contains("Learning Objectives"));
        assert!(content.contains("Fundamentals"));
        assert!(content.contains("Harmony"));
    }

    #[test]
    fn test_default_sources() {
        let content = default_sources();
        assert!(content.contains("Source Materials"));
        assert!(content.contains("Primary Sources"));
        assert!(content.contains("Lewin"));
        assert!(content.contains("Tymoczko"));
    }

    #[test]
    fn test_default_index() {
        let content = default_index();
        assert!(content.contains("Skill Index"));
        assert!(content.contains("Concepts by Category"));
        assert!(content.contains("Fundamentals"));
        assert!(content.contains("Harmony"));
        assert!(content.contains("Neo-Riemannian Theory"));
    }

    #[test]
    fn test_resource_info_serialization() {
        let resource = ResourceInfo {
            uri: "skill://test".to_string(),
            name: "Test Resource".to_string(),
            description: "A test resource".to_string(),
            mime_type: "text/markdown".to_string(),
        };

        let json = serde_json::to_string(&resource).expect("Should serialize");
        assert!(json.contains("skill://test"));
        assert!(json.contains("Test Resource"));
        assert!(json.contains("text/markdown"));
    }

    #[test]
    fn test_resource_info_deserialization() {
        let json = r#"{
            "uri": "skill://test",
            "name": "Test",
            "description": "Desc",
            "mime_type": "text/plain"
        }"#;

        let resource: ResourceInfo = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(resource.uri, "skill://test");
        assert_eq!(resource.name, "Test");
        assert_eq!(resource.description, "Desc");
        assert_eq!(resource.mime_type, "text/plain");
    }
}
