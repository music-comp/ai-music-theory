---
# === CORE IDENTIFICATION ===
concept: Pitch-Class Space (Circular)
slug: pitch-class-space

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
section: "2.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "circular pitch-class space"
  - "chroma circle"
  - "pitch-class circle"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-space
extends:
  - pitch-space
related:
  - octave-equivalence
  - pitch-class-interval
  - voice-leading-in-pitch-class-space
contrasts_with:
  - pitch-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is pitch space vs. pitch-class space?"
  - "What is pitch-class space?"
---

# Quick Definition
Pitch-class space is a circular one-dimensional space where octave-related pitches are identified as the same point, representing the quality or "chroma" shared by all pitches of the same name.

# Core Definition
Pitch-class space is formed from pitch space by "abstracting away" octave information: all pitches sharing the same chroma (e.g., all C's in every octave) collapse to a single point. Geometrically, this produces a circle with circumference 12 (semitones). Points are labeled with numbers in the range 0 <= x < 12. The space is continuous, admitting fractional values (e.g., 2.5 for D quarter-tone sharp). Distance between pitch classes is defined as the shortest distance between any two pitches belonging to those classes. Pitch-class space is mathematically equivalent to R/12Z (the real numbers modulo 12).

# Prerequisites
- **pitch-space** — Pitch-class space is derived from pitch space by octave identification

# Key Properties
1. Circular geometry with circumference 12 semitones
2. Points labeled 0-12 (with 0 = 12 = C)
3. Continuous — admits fractional/microtonal values
4. Distance = shortest distance between any two representative pitches
5. No absolute "above" or "below" — E is both 4 semitones above C and 8 semitones below C
6. Arithmetic is modular (mod 12): 6 + 3 + 1.5 + 1.5 = 0

# Construction / Recognition
## To Construct/Create:
1. Begin with linear pitch space
2. Identify all points that differ by a multiple of 12 (octave equivalence)
3. The resulting quotient space is a circle of circumference 12
## To Identify/Recognize:
1. A circular representation where C4, C5, C3 all map to the same point (0)
2. Distance measured as shortest path around the circle
3. Arithmetic performed modulo 12

# Context & Application
Pitch-class space is the fundamental abstraction underlying most music-theoretical discussion of harmony. Chords are collections of points on this circle. Chord types correspond to particular patterns of arc lengths on the circle. Pitch-class space provides a powerful language for making generalizations: the statement "E is four semitones above C" summarizes infinitely many pitch-space facts (E4 is 4 above C4, E5 is 4 above C5, etc.). Tymoczko uses paths in pitch-class space (not just distances) to model intervals, capturing directional information.

# Examples
**Example 1** (p. 48, Fig 2.2.1): The pitch-class circle with C at 0, C# at 1, D at 2, etc. The number 0.17 refers to the pitch class 17 cents above C; 2.5 refers to D quarter-tone sharp.

**Example 2** (p. 49): "Pitch class E is four semitones away from C" means that for every pitch with chroma C, the nearest pitch with chroma E is exactly four semitones away.

# Relationships
## Builds Upon
- **pitch-space** — Derived by identifying octave-related pitches
## Enables
- **pitch-class-interval** — Intervals measured as paths on the circle
- **chord** — Chords as unordered sets of points on the circle
- **transposition** — Rotation of the circle
- **inversion** — Reflection of the circle
## Related
- **octave-equivalence** — The principle that creates pitch-class space
## Contrasts With
- **pitch-space** — Linear, preserves octave information; pitch-class space is circular, discards it

# Common Errors
- **Error**: Thinking pitch-class space is embedded in a two-dimensional plane
  **Correction**: It is a one-dimensional space unto itself; "circularity" means a straight path eventually returns to its starting point

# Common Confusions
- **Confusion**: Confusing pitch-class distance with pitch-class intervals (as paths)
  **Clarification**: Distance is a single number (shortest distance); a path has both direction and magnitude, and can wrap around the circle

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.2, pages 48-51.

# Verification Notes
- Definition source: Direct from Section 2.2 with geometric and mathematical characterization
- Confidence rationale: High — precisely defined with clear geometric model
- Cross-reference status: Verified; central to the entire book
