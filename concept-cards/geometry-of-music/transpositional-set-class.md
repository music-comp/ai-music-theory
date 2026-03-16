---
concept: Transpositional Set Class (Chord Type)
slug: transpositional-set-class

category: harmony
subcategory: classification
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.4"

extraction_confidence: high

aliases:
  - "chord type"
  - "OPTC class"
  - "Tn-type"

prerequisites:
  - chord
  - transposition-symmetry
extends:
  - chord
related:
  - optic-symmetries
  - set-class
  - harmonic-consistency
contrasts_with:
  - set-class
  - chord

answers_questions:
  - "What is a transpositional set class?"
  - "What is a chord vs. a chord type?"
  - "What is harmonic consistency?"
---

# Quick Definition
A transpositional set class (or chord type) groups together all chords related by transposition, so that all major triads, for instance, belong to the same chord type.

# Core Definition
A transpositional set class is an equivalence class formed by four OPTIC symmetry operations: O, P, T, and C (OPTC). Two chords belong to the same chord type if one can be rotated into the other on the pitch-class circle. Such chords share the same sequence of arc-length distances between adjacent notes. For example, all major triads divide the pitch-class circle into arcs of 4, 3, and 5 semitones (reading clockwise from the root). Traditional musical terms like "major chord," "minor seventh chord," and "diminished triad" are names for chord types.

# Prerequisites
- **chord** — Chord types group chords that are transpositionally related
- **transposition-symmetry** — The additional operation that defines chord types

# Key Properties
1. OPTC equivalence class
2. All members share the same interval pattern (arc lengths on the circle)
3. Geometrically: chords that can be rotated into each other on the pitch-class circle
4. Corresponds to traditional terms like "major chord," "minor seventh"
5. Does not consider inversion — major and minor triads are different chord types

# Construction / Recognition
## To Construct/Create:
1. Take a chord (set of pitch classes)
2. Generate all transpositions
3. The resulting collection is the chord type
## To Identify/Recognize:
1. Compute the distances between adjacent notes on the pitch-class circle
2. Two chords with the same distance sequence (in the same order) belong to the same type

# Context & Application
Chord types are essential for understanding harmonic consistency. When we say a passage uses "all major triads" or "all dominant sevenths," we are describing consistency at the level of chord type. Harmonic consistency means successive chords tend to belong to the same (or related) chord types.

# Examples
**Example 1** (p. 55-56, Fig 2.4.3): C major and D major are both "major chords" — they divide the circle into arcs of 4, 3, 5 semitones. One is a rotation of the other.

# Relationships
## Builds Upon
- **chord** — Chord types are equivalence classes of chords
- **transposition-symmetry** — T relates the chords within a type
## Enables
- **harmonic-consistency** — Defined in terms of using the same chord types
## Related
- **optic-symmetries** — OPTC in the framework
## Contrasts With
- **chord** — A chord is a specific set of pitch classes; a chord type is a class of transpositions
- **set-class** — Set class adds inversional equivalence, grouping major and minor together

# Common Errors
- **Error**: Equating chord type with set class
  **Correction**: Chord type (OPTC) does not include inversion; set class (OPTIC) does

# Common Confusions
- **Confusion**: Thinking major and minor triads are the same chord type
  **Clarification**: They are different chord types (OPTC) but the same set class (OPTIC)

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.4, pages 55-56.

# Verification Notes
- Definition source: Direct from Section 2.4 and Figures 2.4.3, 2.4.7
- Confidence rationale: High — precisely defined in the OPTIC framework
- Cross-reference status: Verified; used throughout the book
