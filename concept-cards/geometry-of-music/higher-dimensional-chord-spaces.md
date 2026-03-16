---
concept: Higher-Dimensional Chord Spaces
slug: higher-dimensional-chord-spaces

category: geometric-theory
subcategory: chord-spaces
tier: intermediate-advanced

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 110
section: "3.9"

extraction_confidence: high

aliases:
  - "n-dimensional chord space"
  - "four-note chord space"
  - "generalized chord spaces"

prerequisites:
  - two-note-chord-space
  - three-note-chord-space
extends:
  - three-note-chord-space
related:
  - voice-leading-lattices
  - nearly-even-chords
  - near-symmetry
contrasts_with: []

answers_questions:
  - "What is the general structure of chord space for n-note chords?"
  - "How do the principles of the Mobius strip generalize to higher dimensions?"
---

# Quick Definition
The n-note chord space is an n-dimensional space formed by identifying equivalent points in ordered pitch space. Its boundary consists of chords with duplicate notes, its center contains perfectly even chords, and voice leadings are generalized line segments in n dimensions.

# Core Definition
For n-note chords, the chord space is n-dimensional, formed by taking one "tile" of the n-dimensional wallpaper pattern of ordered pitch space and identifying equivalent points (those representing the same unordered set of pitch classes). The general structure follows the same principles as the Mobius strip and triangular prism: perfectly even chords lie at the center; the boundary contains chords with duplicate notes; the space wraps around with a (360/n)-degree twist. The boundary acts as a mirror, and motion through the center connects chords to specific transpositions determined by 12/n. Four-note chord space is four-dimensional with diminished seventh chords at the center; it wraps with a 90-degree twist. Five-note and larger chord spaces follow the same pattern but become impossible to visualize directly.

# Prerequisites
- Two-note and three-note chord spaces
- The principle of near symmetry

# Key Properties
1. n dimensions for n-note chords
2. (360/n)-degree twist at the identification boundary
3. Perfectly even chords (dividing octave into n equal parts) at the center
4. Boundary contains chords with duplicate notes
5. Pure contrary voice leading connects chords to (12/n)-semitone transpositions
6. Adding small parallel components extends connections to nearby transpositions
7. The perfect fourth appears in every row of Figure 3.10.8, uniquely useful across all chord sizes

# Construction / Recognition
## General Principle:
1. Use n dimensions (one per voice)
2. Identify the repeating tile of the n-dimensional wallpaper
3. Fold with a (360/n)-degree twist
4. Boundary = duplicate notes; center = maximum evenness

# Context & Application
Higher-dimensional chord spaces are mostly too complex to visualize, but their essential properties — the relationship between evenness and position, the twist, the mirror boundaries — transfer from the simpler cases. For practical purposes, the discrete voice-leading lattices of Section 3.11 capture the musically relevant features of these high-dimensional spaces in manageable, low-dimensional graphs.

# Examples
**Example 1** (p. 110): Four-note chord space is four-dimensional with diminished seventh chords at the center, connected to dominant and half-diminished sevenths by single-semitone voice leading.
**Example 2** (p. 119-120): Figure 3.10.8 shows that for five-note chords, pure contrary motion connects to 2.4 and 4.8-semitone transpositions — neither of which is in 12-TET, explaining why five-note chord voice leading always requires some parallel component.

# Relationships
## Builds Upon
- **two-note-chord-space** — The foundational 2D case
- **three-note-chord-space** — The 3D extension
## Enables
- **voice-leading-lattices** — Practical low-dimensional representations of high-dimensional spaces
## Related
- **near-symmetry** — Determines the available transpositions in each dimension
- **nearly-even-chords** — Always near the center regardless of dimension

# Common Errors
- **Error**: Trying to fully visualize spaces beyond 3 dimensions
  **Correction**: Tymoczko acknowledges this is "an exercise in diminishing returns" — the lattices of Section 3.11 are the practical alternative

# Common Confusions
- **Confusion**: Thinking each dimension adds qualitatively new phenomena
  **Clarification**: The same principles (twist, mirror, centrality of even chords) apply in all dimensions; the differences are quantitative (twist angle, transposition intervals)

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.9, pages 110-113.

# Verification Notes
- Definition source: From Section 3.9 general discussion
- Confidence rationale: High — the generalization is explicitly stated
- Cross-reference status: Verified against the specific 2D and 3D cases
