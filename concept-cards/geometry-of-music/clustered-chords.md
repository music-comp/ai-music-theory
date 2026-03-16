---
concept: Clustered Chords
slug: clustered-chords

category: geometric-theory
subcategory: chord-structure
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 98
section: "3.6, 3.9"

extraction_confidence: high

aliases:
  - "highly uneven chords"
  - "chords near the boundary"
  - "chromatic clusters"

prerequisites:
  - two-note-chord-space
extends: []
related:
  - nearly-even-chords
  - boundary-behavior
  - harmonic-consistency-and-efficient-voice-leading
contrasts_with:
  - nearly-even-chords

answers_questions:
  - "What are clustered chords and where do they lie in chord space?"
  - "How can very uneven chords also participate in efficient voice leading?"
---

# Quick Definition
Chords whose notes are packed closely together (like chromatic clusters), lying near the mirror boundary of chord space. They can be connected to nearby transpositions by efficient voice leading that "bounces" off the boundary.

# Core Definition
Clustered chords are the opposite extreme from nearly even chords: where nearly even chords divide the octave as equally as possible, clustered chords pack their notes as closely together as possible. In chord space, clustered chords lie near the mirror boundary, which represents chords containing multiple copies of the same note (multisets). The mirror boundary enables a distinctive type of efficient voice leading: the voice leading (D, Eb) -> (Eb, D), for instance, connects a minor second to itself by semitonal contrary motion, represented by a short vertical arrow that bounces off the mirror. From this, nearby transpositions can be reached by adding a small parallel component. Clustered chords thus form a second class (alongside nearly even chords) that permits harmonic consistency with efficient voice leading.

# Prerequisites
- Two-note chord space geometry
- Mirror boundary behavior

# Key Properties
1. Notes packed closely together in pitch space
2. Lie near the mirror boundary of chord space
3. Reach nearby transpositions by bouncing off the mirror
4. The "bounce" corresponds to voices passing through a near-unison
5. Available transpositions are small (one or two semitones)
6. Less commonly exploited in Western music than nearly even chords

# Construction / Recognition
## To Construct:
1. Choose notes that are chromatically adjacent (e.g., C, C#, D)
2. The resulting chord will be near the boundary of chord space
## To Recognize:
1. All notes within a small chromatic span
2. The chord lies near the edge/boundary of chord space

# Context & Application
While nearly even chords are the primary building blocks of Western tonality, clustered chords play an important role in certain twentieth-century contexts, particularly in music that exploits chromatic voice leading among close-position chords. The concept is less prominent than near-evenness in the book but is essential for a complete understanding of the geometry: it explains why the boundary regions of chord space are not musically dead zones.

# Examples
**Example 1** (p. 98-99): The minor second {D, Eb} connects to itself by contrary motion (D, Eb) -> (Eb, D), bouncing off the mirror boundary.
**Example 2** (p. 99): By adding a small transposition, the minor second can also reach its one-semitone transposition efficiently.

# Relationships
## Builds Upon
- **two-note-chord-space** — The space where clustering is geometrically visible
- **boundary-behavior** — The mirror bounce that enables efficient voice leading
## Related
- **harmonic-consistency-and-efficient-voice-leading** — Clustered chords offer one solution
## Contrasts With
- **nearly-even-chords** — The opposite extreme, exploiting the twist rather than the mirror

# Common Errors
- **Error**: Assuming only nearly even chords can participate in efficient voice leading
  **Correction**: Clustered chords also participate, but via a different mechanism (mirror bounce) and with different available transpositions

# Common Confusions
- **Confusion**: Thinking "clustered" means "dissonant" in this context
  **Clarification**: "Clustered" refers to geometric position in chord space (near the boundary), which correlates with but is not identical to acoustic dissonance

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.6, pages 98-99.

# Verification Notes
- Definition source: From Section 3.6 discussion of the third possibility for efficient voice leading
- Confidence rationale: High — explicitly described as one of three voice-leading strategies
- Cross-reference status: Verified against higher-dimensional generalization in Section 3.9
