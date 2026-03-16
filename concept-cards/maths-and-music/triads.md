---
# === CORE IDENTIFICATION ===
concept: Triads
slug: triads

# === CLASSIFICATION ===
category: chord-theory
subcategory: triads
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Harmony and Related Numerology"
chapter_number: 3
pdf_page: 44
section: "Triads"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - triadic harmony

# === TYPED RELATIONSHIPS ===
prerequisites:
  - chords-as-note-class-collections
extends: []
related:
  - major-triad
  - minor-triad
  - diminished-triad
  - augmented-triad
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a triad?"
  - "What are the four standard triad types?"
  - "What does 'triadic' mean in music?"
---

# Quick Definition

Triads are chords containing exactly three distinct note classes (modulo octave). They have been a fundamental element of Western harmony since the seventeenth century.

# Core Definition

A triad is a chord consisting of exactly three note classes modulo octave. The four standard triad types in Western music are defined by their interval sequences in Z_12: major (4, 3, 5), minor (3, 4, 5), diminished (3, 3, 6), and augmented (4, 4, 4). The term "triadic" is applied to music that primarily features triads (Wright, pp. 46-47).

# Prerequisites

- **Chords as Note Class Collections** -- Triads are a specific case of chords defined by note class collections

# Key Properties

1. Exactly three distinct note classes (modulo octave)
2. Four standard types: major, minor, diminished, augmented
3. Each type defined by a cyclic sequence of three intervals summing to 12
4. Two types (major, minor) have asymmetric sequences with unique roots
5. One type (diminished) has repeated intervals but still a unique root
6. One type (augmented) has full cyclic symmetry and no discernible root
7. A voicing may use more than three notes through octave doubling

# Construction / Recognition

## To Identify a Triad

1. Reduce all sounding notes to note classes
2. Remove duplicates from octave doublings
3. Verify exactly three distinct note classes remain
4. Compute the interval sequence between successive note classes
5. Match to one of the four standard types: (4,3,5), (3,4,5), (3,3,6), or (4,4,4)

# Context & Application

Triads are the simplest chords and the foundation of triadic harmony, which dominated Western music from approximately 1600 to 1900. The major and minor triads are the most common. The major triad is also referred to as "major chord" and the minor triad as "minor chord" -- the terms "major triad" and "minor triad" are used when precision is needed to distinguish from chords that contain these triads (such as seventh chords) (Wright, pp. 46-47).

# Examples

**Example 1** (pp. 45-46): Major triad (4, 3, 5): C-E-G.

**Example 2** (pp. 46-47): Minor triad (3, 4, 5): A-C-E.

**Example 3** (p. 47): Diminished triad (3, 3, 6): B-D-F.

**Example 4** (p. 47): Augmented triad (4, 4, 4): C-E-G#.

# Relationships

## Builds Upon

- **Chords as Note Class Collections** -- Triads are three-note-class chords

## Enables

- **Major Triad** -- A specific triad type
- **Minor Triad** -- A specific triad type
- **Diminished Triad** -- A specific triad type
- **Augmented Triad** -- A specific triad type

## Related

- **Seventh Chord** -- Four-note chords that extend triads

# Common Errors

- **Error**: Assuming a triad must be voiced with exactly three pitches
  **Correction**: A triad has three note *classes*; a voicing may double notes at the octave, using more than three pitches

# Common Confusions

- **Confusion**: Thinking "triad" means any group of three notes
  **Clarification**: "Triad" specifically means three distinct note classes forming a recognized chord type
- **Confusion**: Assuming the augmented triad is the only triad without a discernible root because it is "unusual"
  **Clarification**: The augmented triad lacks a root specifically because of its mathematical symmetry (4, 4, 4), not because of any musical unusualness

# Source Reference

Chapter 3: "Harmony and Related Numerology," pp. 46-47.

# Verification Notes

- Definition source: Direct from pp. 46-47
- Confidence rationale: High -- explicitly defined
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: note about "triadic" music, distinction between triad note classes and voicing notes
