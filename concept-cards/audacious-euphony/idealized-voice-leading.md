---
concept: Idealized Voice Leading
slug: idealized-voice-leading

category: voice-leading
subcategory: null
tier: foundational

source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Mapping the Triadic Universe"
chapter_number: 1
pdf_page: 19
section: "Three Ways to Calculate Triadic Distance"

extraction_confidence: high

aliases:
  - "optimal voice leading"
  - "minimal voice leading"

prerequisites: []
extends: []
related:
  - voice-leading-work
  - voice-leading-approach
  - single-semitonal-displacement
contrasts_with: []

answers_questions:
  - "What is idealized voice leading?"
  - "How does idealized voice leading differ from actual compositional voice leading?"
---

# Quick Definition
The optimal one-to-one pairing of tones between two chords that minimizes total voice-leading work, abstracting from actual registral realization and instrumental constraints.

# Core Definition
Idealized voice leading refers to the ordered dyads between two chords when their tones are paired one-to-one to minimize total voice-leading work. The concept operates in pitch-class space rather than pitch space, treating voice leading as an abstract relationship independent of compositional texture. The concept, attributed to Godfrey Winham by Proctor (1978), is so familiar that "it takes a special effort to acknowledge it" (synthesized from Ch. 1, p. 26). Cohn notes that theorists assume idealized voice leading "every day of their working lives."

# Prerequisites
This is a foundational concept with no prerequisites within this source.

# Key Properties
1. Operates in pitch-class space (octave equivalence assumed)
2. Pairs tones one-to-one between two chords
3. Minimizes total semitonal displacement
4. Independent of actual registral spacing or voicing
5. Provides a standard basis for comparing triadic connections
6. Not a prescription for composition but an analytical abstraction

# Construction / Recognition
For chords X = {x1, x2, x3} and Y = {y1, y2, y3}:
1. Consider all possible one-to-one pairings of tones
2. For each pairing, calculate total semitonal displacement
3. The pairing with minimum total displacement is the idealized voice leading
4. Distances measured mod 12, taking the smaller of the two possible directions

Example: C major {C, E, G} to F major {F, A, C}
- Optimal: (C, C), (E, F), (G, A) = 0 + 1 + 2 = 3 units
- Not: (C, F), (E, A), (G, C) = 5 + 5 + 5 = 15 units

# Context & Application
Idealized voice leading underlies all distance calculations in the book. Common-tone counting, voice-leading work, and the geometric representations (Tonnetz, Cube Dance) all assume idealized pairing. The concept enables systematic comparison across different compositional realizations.

# Examples
**Example 1** (Ch. 1): Common-tone retention between triads is counted using idealized voice leading.

**Example 2** (Ch. 2): "Under idealized voice leading one voice moves up (G to G#) and one moves down (C to B), while the third voice, E, holds its place" when transposing C major to E major (p. 37).

# Relationships
## Builds Upon
This is a foundational analytical concept.

## Enables
- **voice-leading-work** — Calculated under idealized voice leading
- **voice-leading-approach** — Built on the assumption of idealized pairing
- **balanced-voice-leading** — Defined in terms of idealized voice-leading behavior

## Related
- **single-semitonal-displacement** — The minimum possible idealized voice-leading distance

## Contrasts With
No direct contrasts within this source.

# Common Errors
- **Error**: Expecting actual compositions to always realize idealized voice leading
  **Correction**: Composers may voice chords differently for musical reasons; idealized voice leading is an analytical abstraction

# Common Confusions
- **Confusion**: Thinking "idealized" means "better" or "preferred"
  **Clarification**: It means abstracted for comparison purposes, not prescriptive for composition

# Source Reference
Chapter 1: Mapping the Triadic Universe, pp. 25-27. Glossary, p. 229. The concept is attributed to Godfrey Winham via Proctor (1978).

# Verification Notes
- Re-extracted from v2 card; preserved: the Winham/Proctor attribution, the F major worked example, the point about theoretical ubiquity
- Confidence: HIGH — the concept is explicitly described and is foundational throughout the book
