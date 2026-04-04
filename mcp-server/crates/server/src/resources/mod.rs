/// Default conventions content used as fallback when the file is not on disk.
pub fn default_conventions() -> String {
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

/// Default scope content used as fallback when the file is not on disk.
pub fn default_scope() -> String {
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

/// Default sources content used as fallback when the file is not on disk.
pub fn default_sources() -> String {
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

/// Default index content used as fallback when the file is not on disk.
pub fn default_index() -> String {
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
}
