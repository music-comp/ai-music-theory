---
concept: Transpositional Near-Symmetry
slug: transpositional-near-symmetry

category: geometric-theory
subcategory: symmetry
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.9.1"

extraction_confidence: high

aliases:
  - "near-T-symmetry"
  - "near-transpositional invariance"

prerequisites:
  - transposition
  - near-symmetry
extends:
  - near-symmetry
related:
  - near-evenness
  - efficient-voice-leading
contrasts_with:
  - inversional-near-symmetry
  - permutational-near-symmetry

answers_questions:
  - "What is transpositional near-symmetry?"
  - "When can a chord be connected to its transposition by efficient voice leading?"
---

# Quick Definition
A chord is nearly transpositionally symmetrical when it is close to a chord that is unchanged by some transposition, enabling efficient voice leading between the chord and that transposition of itself.

# Core Definition
A chord A is near-T-symmetrical when there is a small voice leading from A to a chord S that is exactly transpositionally symmetrical (i.e., Tx(S) = S for some x). Transpositionally symmetrical chords either divide the pitch-class circle evenly (like the augmented triad or diminished seventh) or can be decomposed into equal-size subsets that do (like two tritones {B, C, F, F#}). The argument proceeds: since A is close to S, and S = Tx(S), then Tx(A) is also close to Tx(S) = S. Both A and Tx(A) are close to S, hence close to each other, hence connectable by efficient voice leading.

# Prerequisites
- **transposition** — The transformation under consideration
- **near-symmetry** — The general principle being applied

# Key Properties
1. Chord is close to one that divides the circle evenly or has evenly-dividing subsets
2. Enables efficient voice leading to the chord's x-semitone transposition
3. Two types: near-evenly-dividing (augmented triad) and near-evenly-decomposable (two tritones)
4. A larger chord can exploit the symmetry of a smaller chord (at the cost of extra voices)
5. The symmetrical chord need not exist in the tuning system

# Construction / Recognition
## To Construct/Create:
1. Start with a transpositionally symmetrical chord (augmented triad, diminished seventh, etc.)
2. Slightly perturb one or more notes
3. The result is near-T-symmetrical
## To Identify/Recognize:
1. Check if the chord is close to one dividing the circle evenly
2. Check if it decomposes into subsets close to symmetrical subsets

# Context & Application
Transpositional near-symmetry explains why major triads (near augmented triads) can be connected by efficient voice leading to their major-third transpositions, and why the pentatonic scale (near the perfectly even five-note chord) can be connected to its transpositions. It also explains jazz tritone substitutions: chords near {B, C, F, F#} can be efficiently linked to their tritone transpositions.

# Examples
**Example 1** (p. 72-73, Fig 2.9.3): C major is close to the augmented triad. Following the argument: (C, E, G) -> (C, E, G#) is small, transpose by major third to get (E, G#, B) -> (E, G#, C), retrograde and glue to get (C, E, G) -> (B, E, G#).

**Example 2** (p. 73-74, Fig 2.9.4): {B, C, E, F#} is near {B, C, F, F#} (tritone-symmetrical). Efficient voice leading to tritone transposition: (B, C, E, F#) -> (Bb, C, F, Gb).

# Relationships
## Builds Upon
- **near-symmetry** — Transpositional near-symmetry is one of three types
- **transposition** — The relevant transformation
## Enables
- Understanding of which chord pairs permit efficient voice leading
## Related
- **near-evenness** — The most common form of near-T-symmetry
- **efficient-voice-leading** — What near-T-symmetry enables
## Contrasts With
- **inversional-near-symmetry** — A different type of near-symmetry
- **permutational-near-symmetry** — Another type

# Common Errors
- **Error**: Thinking only evenly-dividing chords are relevant
  **Correction**: Chords decomposable into evenly-dividing subsets (like two tritones) also count

# Common Confusions
- **Confusion**: Thinking the symmetrical chord must be playable on a standard keyboard
  **Clarification**: The perfectly even five-note chord {0, 2.4, 4.8, 7.2, 9.6} is not in 12-TET but still governs the voice-leading properties of pentatonic scales

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.9.1, pages 71-74.

# Verification Notes
- Definition source: Direct from Section 2.9.1
- Confidence rationale: High — detailed argument with multiple examples
- Cross-reference status: Verified; elaborated in Chapter 3
