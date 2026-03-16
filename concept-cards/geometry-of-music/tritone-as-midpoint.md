---
concept: Tritone as Midpoint
slug: tritone-as-midpoint

category: geometric-theory
subcategory: chord-structure
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 90
section: "3.3"

extraction_confidence: high

aliases:
  - "tritone at center of chord space"
  - "central horizontal line"

prerequisites:
  - two-note-chord-space
extends: []
related:
  - nearly-even-chords
  - even-vs-uneven-dyads
  - harmonic-consistency-and-efficient-voice-leading
contrasts_with: []

answers_questions:
  - "Why does the tritone occupy the center of two-note chord space?"
  - "What is special about the tritone's position in chord space geometry?"
---

# Quick Definition
The tritone, which divides the octave into two exactly equal halves, lies on the central horizontal line of two-note chord space, exactly midway between the mirror boundaries of unisons. This is the most even possible two-note chord.

# Core Definition
In two-note chord space, intervals are arranged by how evenly they divide the octave: unisons lie on the mirror boundary, minor seconds slightly inside, and so on, with more even intervals progressively closer to the center. The tritone, dividing the pitch-class circle into two exactly equal halves, lies precisely at the midpoint of the space. The abstract circle of tritones is therefore half the length of the circles containing other intervals (since there are half as many tritones as instances of any other interval class in twelve-tone equal temperament). The tritone's central position is musically significant: tritone-related chords can be connected by pure contrary voice leading (vertical motion in chord space), which is why tritone substitution in jazz and tritone relationships in chromatic harmony are so effective.

# Prerequisites
- Two-note chord space layout

# Key Properties
1. The tritone lies at the exact center of two-note chord space
2. It divides the octave into two equal halves (6+6 semitones)
3. The circle of tritones is half as long as circles of other intervals
4. Every dyad lies on the same vertical line as its tritone transposition
5. Tritone-related chords can be connected by pure contrary motion

# Construction / Recognition
## To Identify:
1. Find the central horizontal line of the Mobius strip
2. All tritones lie on this line
3. Note: there are only 6 equal-tempered tritones, versus 12 of every other interval

# Context & Application
The tritone's central position explains its dual role in tonal music: it is both the most "unstable" interval (furthest from unison) and the most efficient point for contrary voice leading between dyads. In higher dimensions, the analogous central position is occupied by augmented triads (3-note) and diminished seventh chords (4-note), explaining their corresponding roles in harmonic practice.

# Examples
**Example 1** (p. 90): The central horizontal line of Figure 3.3.1 contains all tritones.
**Example 2** (p. 91): {C, F#}, {G, B}, and {Ab, Bb} lie on the same vertical line, summing to 6 in pitch-class arithmetic — they can all be linked by pure contrary motion.

# Relationships
## Builds Upon
- **two-note-chord-space** — The space whose center the tritone occupies
## Enables
- **nearly-even-chords** — Near-evenness is defined relative to the tritone's perfect evenness
## Related
- **even-vs-uneven-dyads** — The tritone is the perfectly even end of the spectrum
- **harmonic-consistency-and-efficient-voice-leading** — Tritone-related chords are efficiently connected

# Common Errors
- **Error**: Thinking the tritone's instability is purely an acoustic phenomenon
  **Correction**: The tritone's special status is also geometric — it sits at the exact center of chord space

# Common Confusions
- **Confusion**: Why half as many tritones?
  **Clarification**: Because the tritone is its own inversion (C-F# = F#-C), each tritone is counted only once. The circle of tritones is literally half the size.

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.3, pages 90-91.

# Verification Notes
- Definition source: From Section 3.3, confirmed by the explicit statement about the midpoint
- Confidence rationale: High — clearly stated geometric fact
- Cross-reference status: Verified against the footnote on half-length circles (p. 90)
