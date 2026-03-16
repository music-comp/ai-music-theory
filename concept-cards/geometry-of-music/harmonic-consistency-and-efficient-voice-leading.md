---
concept: Harmonic Consistency and Efficient Voice Leading
slug: harmonic-consistency-and-efficient-voice-leading

category: voice-leading
subcategory: constraints
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 97
section: "3.6"

extraction_confidence: high

aliases:
  - "combining constraints"
  - "harmonic and contrapuntal constraints"

prerequisites:
  - two-note-chord-space
  - voice-leading-size
extends:
  - five-components-of-tonality
related:
  - nearly-even-chords
  - clustered-chords
  - even-vs-uneven-dyads
  - tritone-as-midpoint
contrasts_with: []

answers_questions:
  - "How can composers combine harmonic consistency with efficient voice leading?"
  - "Why do nearly even chords support efficient voice leading?"
  - "What geometric fact explains the relationship between chord evenness and voice-leading efficiency?"
---

# Quick Definition
The central question of tonal composition: under what circumstances can transpositionally related chords (harmonic consistency) be connected by stepwise voice leading (efficient voice leading)? The answer depends on whether chords are nearly even, nearly clustered, or neither.

# Core Definition
Section 3.6 poses the question abstractly: given a requirement to use transpositionally related two-note chords (harmonic consistency) connected by stepwise voice leading (voice motions of at most two semitones), what options are available? Inspection of two-note chord space reveals three solutions: (1) Pure parallel motion — always possible but musically unsatisfying because voices are not independent. (2) Nearly even chords near the center of the space — these can reach their transpositions by crossing the center of the strip, exploiting the Mobius twist; tritone-related perfect fifths, for instance, can be linked by semitonal voice leading. (3) Clustered chords near the boundary — these reach nearby transpositions by bouncing off the mirror boundary. For chords that are neither particularly even nor particularly uneven, no efficient non-parallel voice leading is available. This principle — that symmetry (near-evenness or near-clustering) is required for efficient voice leading — generalizes to all dimensions.

# Prerequisites
- Two-note chord space geometry
- Voice-leading size as path length
- Concepts of transposition and harmonic consistency

# Key Properties
1. Three types of chords permit efficient voice leading: parallel (all chords), nearly even, and clustered
2. Nearly even chords exploit the twist of the Mobius strip
3. Clustered chords exploit the mirror boundaries
4. Chords of intermediate evenness cannot combine harmonic consistency with efficient voice leading
5. This result generalizes to higher dimensions

# Construction / Recognition
## To Determine:
1. Locate the chord type on the vertical axis of chord space
2. If near the center (even): efficient contrary voice leading to tritone-related transpositions
3. If near the boundary (clustered): efficient contrary voice leading to nearby transpositions via mirror bounce
4. If in between: only parallel motion provides efficient connection

# Context & Application
This is one of the book's central theoretical results. It explains why Western tonal music gravitates toward certain chord types (major and minor triads, seventh chords) — they are nearly even and therefore support the simultaneous satisfaction of harmonic consistency and efficient voice leading. The geometry provides a rigorous foundation for what might otherwise seem like arbitrary conventions.

# Examples
**Example 1** (p. 98): Tritone-related perfect fifths can be linked by semitonal contrary voice leading — a vertical arrow in chord space (Figure 3.6.1).
**Example 2** (p. 98): Major thirds can be linked only to their tritone transpositions by stepwise voice leading, since they are farther from the center.
**Example 3** (p. 98-99): Minor seconds (clustered chords) can be linked to themselves by stepwise contrary motion, bouncing off the nearby mirror boundary.

# Relationships
## Builds Upon
- **two-note-chord-space** — The geometric setting
- **voice-leading-size** — The measure of efficiency
## Enables
- **nearly-even-chords** — The chord type that best combines the constraints
- **even-vs-uneven-dyads** — The spectrum of possibilities
## Related
- **five-components-of-tonality** — Harmonic consistency and efficient voice leading are two of the five components
- **clustered-chords** — The other extreme that permits efficient voice leading

# Common Errors
- **Error**: Thinking efficient voice leading is possible for any chord type
  **Correction**: Only chords near the extremes of evenness (very even or very clustered) permit efficient non-parallel voice leading

# Common Confusions
- **Confusion**: Equating harmonic consistency with "using the same chord type"
  **Clarification**: Harmonic consistency means using transpositionally related chords; the specific chord type matters because of its position in chord space

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.6, pages 97-99.

# Verification Notes
- Definition source: Directly from Section 3.6
- Confidence rationale: High — the section explicitly poses and answers the central question
- Cross-reference status: Verified against the generalized result in Section 3.10
