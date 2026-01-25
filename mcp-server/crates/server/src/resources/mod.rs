use serde::{Deserialize, Serialize};
use std::fs;

use crate::config::Config;
use crate::error::Result;

/// Resource information for MCP.
// Allow unused - will be used when resource features are implemented
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub mime_type: String,
}

/// List all available resources.
// Allow unused - will be used when resource features are implemented
#[allow(dead_code)]
pub fn list_resources() -> Vec<ResourceInfo> {
    vec![
        ResourceInfo {
            uri: "skill://conventions".to_string(),
            name: "Music Theory Conventions".to_string(),
            description: "Notation conventions and terminology used in this skill".to_string(),
            mime_type: "text/markdown".to_string(),
        },
        ResourceInfo {
            uri: "skill://scope".to_string(),
            name: "Skill Scope".to_string(),
            description: "Topics covered and learning objectives of this skill".to_string(),
            mime_type: "text/markdown".to_string(),
        },
        ResourceInfo {
            uri: "skill://sources".to_string(),
            name: "Source Materials".to_string(),
            description: "Bibliography and source attribution".to_string(),
            mime_type: "text/markdown".to_string(),
        },
        ResourceInfo {
            uri: "skill://index".to_string(),
            name: "Skill Index".to_string(),
            description: "Complete index of concepts, topics, and materials".to_string(),
            mime_type: "text/markdown".to_string(),
        },
    ]
}

/// Get resource content by URI.
// Allow unused - will be used when resource features are implemented
#[allow(dead_code)]
pub fn get_resource(config: &Config, uri: &str) -> Result<String> {
    let skill_docs_path = config.paths.skill_docs_path()?;

    let content = match uri {
        "skill://conventions" => {
            let path = skill_docs_path.join("CONVENTIONS.md");
            fs::read_to_string(path).unwrap_or_else(|_| default_conventions())
        }
        "skill://scope" => {
            let path = skill_docs_path.join("SCOPE.md");
            fs::read_to_string(path).unwrap_or_else(|_| default_scope())
        }
        "skill://sources" => {
            let path = skill_docs_path.join("SOURCES.md");
            fs::read_to_string(path).unwrap_or_else(|_| default_sources())
        }
        "skill://index" => {
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

// Allow unused - will be used when resource features are implemented
#[allow(dead_code)]
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

// Allow unused - will be used when resource features are implemented
#[allow(dead_code)]
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

// Allow unused - will be used when resource features are implemented
#[allow(dead_code)]
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

// Allow unused - will be used when resource features are implemented
#[allow(dead_code)]
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
