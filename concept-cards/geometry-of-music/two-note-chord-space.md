---
# === CORE IDENTIFICATION ===
concept: Two-Note Chord Space
slug: two-note-chord-space

# === CLASSIFICATION ===
category: geometric-theory
subcategory: chord-spaces
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 89
section: "3.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "two-note Mobius strip"
  - "dyad space"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - ordered-pitch-space
  - wallpaper-periodicity
  - parable-of-the-ant
extends: []
related:
  - mobius-strip-topology
  - boundary-behavior
  - chord-progressions-vs-voice-leadings
  - three-note-chord-space
contrasts_with:
  - circular-pitch-class-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the geometric space of all two-note chords?"
  - "Why is two-note chord space a Mobius strip?"
  - "How are intervals arranged in this space?"
---

# Quick Definition
The geometric space of all unordered pairs of pitch classes, which has the topology of a Mobius strip. Points represent two-note chords; unisons lie on the boundary and tritones lie at the center.

# Core Definition
Two-note chord space is formed by "folding" ordered pitch space to identify all points representing the same unordered pair of pitch classes — that is, ignoring both the octave and the order of the two notes. The result is a single quadrant of the periodic wallpaper, with octave designations removed. This space is a Mobius strip: the left edge is glued to the right with a half-twist, and the top and bottom edges are mirror boundaries. Horizontal position corresponds to transposition level (chords on the same horizontal line are transpositionally related), while vertical position indicates how evenly the chord divides the octave. Unisons occupy the boundary edges, tritones lie on the central horizontal line, and other intervals are arranged by size between these extremes, with more even divisions closer to the center.

# Prerequisites
- Ordered pitch space and its wallpaper periodicity
- The Parable of the Ant (folding wallpaper into a single tile)
- Concept of pitch class (ignoring octave)

# Key Properties
1. The space is a Mobius strip — it has only one edge and one side
2. Horizontal motion represents parallel motion (transposition) in both voices
3. Vertical motion represents contrary motion (voices moving in opposite directions by equal amounts)
4. Oblique motion (one voice stationary) lies along 45-degree diagonals
5. Unisons are on the boundary; tritones are at the center
6. The boundary is an abstract circle very similar to pitch-class space (Figure 3.3.2)
7. Chords on the same vertical line have pitch classes that sum to the same value
8. The space is continuous, containing every conceivable dyad including microtonal ones

# Construction / Recognition
## To Construct:
1. Begin with ordered pitch space (Section 3.1)
2. Identify all points representing the same unordered pair of pitch classes
3. Select a single tile (quadrant) from the wallpaper pattern
4. Remove octave designations
5. The result is a Mobius strip with the left and right edges glued with a twist
## To Recognize:
1. A two-dimensional space where each point represents an unordered pair of pitch classes
2. Mirror boundaries at top and bottom; twisted identification at left and right

# Context & Application
Two-note chord space is the simplest and most fully visualizable chord space. It serves as the primary pedagogical model for understanding the geometry of higher-dimensional chord spaces. All the essential features — mirror boundaries, twisted identification, the relationship between evenness and centrality, the correspondence between voice leadings and line segments — appear here in their simplest form. Tymoczko uses it extensively for analytical examples, including passages from Josquin, Brahms, and medieval counterpoint.

# Examples
**Example 1** (p. 89): Figure 3.3.1 shows the complete two-note chord space as a Mobius strip, with labeled dyads at equal-tempered positions.
**Example 2** (p. 90): Figure 3.3.2 demonstrates that the boundary is an abstract circle, analogous to pitch-class space but representing unisons rather than single pitch classes.
**Example 3** (p. 91): The vertical line at the center contains {C, F#}, {G, B}, and {Ab, Bb}, all of which sum to 6 in pitch-class arithmetic.

# Relationships
## Builds Upon
- **ordered-pitch-space** — The unfolded, infinite version of this space
- **wallpaper-periodicity** — The periodic structure that gets folded
- **parable-of-the-ant** — The conceptual framework for the folding process
## Enables
- **chord-progressions-vs-voice-leadings** — Progressions and voice leadings are represented differently in this space
- **generalized-line-segments** — Voice leadings as paths in the Mobius strip
- **harmonic-consistency-and-efficient-voice-leading** — The geometry reveals when these can be combined
## Related
- **mobius-strip-topology** — The topology of this space
- **three-note-chord-space** — The three-note analogue
## Contrasts With
- **circular-pitch-class-space** — The simpler one-dimensional representation

# Common Errors
- **Error**: Thinking the left and right edges of the strip are truly separate
  **Correction**: They are identified (glued together) with a twist; what appear as two edges are really one

# Common Confusions
- **Confusion**: Why a Mobius strip and not a torus?
  **Clarification**: A torus would result from ignoring octave but NOT order (footnote 3, p. 89). The Mobius strip results from ignoring both octave and order.
- **Confusion**: Thinking points near the edges represent "smaller" chords
  **Clarification**: Points near the edges represent more uneven chords (closer to unisons), while points near the center represent more even chords (closer to tritones)

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.3, pages 88-91.

# Verification Notes
- Definition source: Directly from Section 3.3, synthesized with Sections 3.1-3.2
- Confidence rationale: High — the central construction of the chapter, extensively illustrated
- Cross-reference status: Verified against Sections 3.4-3.7 which all build on this space
