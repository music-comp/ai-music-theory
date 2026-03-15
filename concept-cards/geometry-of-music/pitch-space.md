---
# === CORE IDENTIFICATION ===
concept: Pitch Space (Linear)
slug: pitch-space

# === CLASSIFICATION ===
category: geometric-theory
subcategory: space
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "linear pitch space"
  - "pitch line"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - pitch-class-space
  - transposition
  - voice-leading-in-pitch-space
  - distance-in-music
contrasts_with:
  - pitch-class-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is pitch space vs. pitch-class space?"
  - "How is musical distance measured?"
---

# Quick Definition
Pitch space is a continuous one-dimensional line in which each point represents a specific pitch, with distance measured in semitones using a logarithmic mapping from frequency.

# Core Definition
Linear pitch space is a continuous, infinite line where each point corresponds to a specific pitch. It is constructed by mapping fundamental frequency f onto a number p using the equation p = c1 + c2 * log2(f/440), where c1 = 69 and c2 = 12. This makes the semitone the unit of distance, assigns middle C the number 60, and converts the multiplicative relationships of frequency ratios into additive relationships of pitch distances. The space is continuous, meaning fractional values (like 60.17 for a pitch 17 cents above middle C) are valid.

# Prerequisites
This is a foundational concept with no prerequisites within this source.

# Key Properties
1. Continuous one-dimensional line extending infinitely in both directions
2. Distance measured in semitones via subtraction: |p - q|
3. Middle C = 60; each successive semitone adds 1
4. The logarithmic mapping converts frequency ratios to pitch differences
5. Octaves are not special in this space; the distance 12 is like any other
6. Contains fractional values — not limited to equal-tempered pitches

# Construction / Recognition
## To Construct/Create:
1. Take the logarithm (base 2) of frequency ratios
2. Scale by 12 (so the octave = 12 semitones)
3. Set middle C = 60, A440 = 69
## To Identify/Recognize:
1. A representation where each pitch has a unique numerical value
2. Octave-related pitches (e.g., C4 and C5) have different values (60 and 72)
3. Distance is measured by subtraction

# Context & Application
Pitch space is the more concrete of the two basic musical spaces, preserving octave information. It is the space in which actual music is realized — specific notes on specific instruments at specific registers. Voice leadings in pitch space describe the actual motion of individual voices. Pitch space is contrasted with the more abstract pitch-class space, which collapses octave information.

# Examples
**Example 1** (p. 47-48): Middle C = 60, C#4 = 61, D4 = 62, etc. The pitch 17 cents above middle C is 60.17. "C#4" and "61" are different names for the same point in pitch space.

**Example 2** (p. 47, Fig 2.1.2): Linear pitch space represented as a continuous line with familiar equal-tempered pitches marked as points.

# Relationships
## Builds Upon
- No prerequisites within this source
## Enables
- **transposition** — Defined as addition in pitch space
- **inversion** — Defined as subtraction from a constant in pitch space
- **voice-leading-in-pitch-space** — Voice leadings are paths in this space
## Related
- **distance-in-music** — Pitch space provides the first notion of musical distance
## Contrasts With
- **pitch-class-space** — The circular space that results from collapsing octave information

# Common Errors
- **Error**: Thinking pitch space contains only equal-tempered (integer) values
  **Correction**: Pitch space is continuous; any real number is a valid pitch

# Common Confusions
- **Confusion**: Confusing pitch space with pitch-class space
  **Clarification**: In pitch space, C4 (60) and C5 (72) are different points 12 semitones apart; in pitch-class space they are the same point

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.1, pages 47-48.

# Verification Notes
- Definition source: Direct from Section 2.1 with specific equation and constants
- Confidence rationale: High — precisely defined with mathematical formula
- Cross-reference status: Verified; used throughout the book
