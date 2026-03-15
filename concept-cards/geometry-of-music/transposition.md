---
# === CORE IDENTIFICATION ===
concept: Transposition
slug: transposition

# === CLASSIFICATION ===
category: geometric-theory
subcategory: transformation
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "translation"
  - "Tx"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-space
  - distance-in-music
extends: []
related:
  - inversion
  - pitch-class-space
  - optic-symmetries
  - transpositional-set-class
contrasts_with:
  - inversion

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What does transposition mean in both pitch and pitch-class spaces?"
  - "What are the distance-preserving transformations of musical space?"
---

# Quick Definition
Transposition moves every pitch in the same direction by the same amount, corresponding geometrically to translation in pitch space or rotation in pitch-class space.

# Core Definition
Transposition is one of only two distance-preserving transformations of musical space (the other being inversion). In pitch space, the transposition of pitch p by x semitones is Tx(p) = p + x, shifting every point the same distance in the same direction. In pitch-class space, transposition corresponds to rotation of the circle. Transpositions have a "size" — the distance by which notes are moved — and can be distinguished by this size. Transposition preserves all intervallic relationships between pitches, which is why transposed melodies sound "the same but higher/lower."

# Prerequisites
- **pitch-space** — Transposition is defined as addition in pitch space
- **distance-in-music** — Transposition is characterized as distance-preserving

# Key Properties
1. Tx(p) = p + x in pitch space; p + x (mod 12) in pitch-class space
2. Distance-preserving: the interval between any two notes is unchanged
3. Geometrically: translation (pitch space) or rotation (pitch-class space)
4. Has a well-defined "size" (the amount x)
5. Listeners perceive transposition as preserving melodic identity
6. Sensitivity to transposition may be innate and shared with nonhuman animals

# Construction / Recognition
## To Construct/Create:
1. Choose a transposition amount x (in semitones)
2. Add x to every pitch: p becomes p + x
3. In pitch-class space, reduce modulo 12
## To Identify/Recognize:
1. Check whether all intervals between notes are preserved
2. Check whether all notes have moved the same direction by the same amount

# Context & Application
Transposition is fundamental to both musical practice and theory. It is the T in OPTIC symmetries and is the basis for defining chord types (transpositional set classes). Two chords belong to the same type when one is a transposition of the other. Transposition also plays a central role in voice-leading analysis, where uniformly transposed voice leadings represent "the same musical pattern at a different transpositional level."

# Examples
**Example 1** (p. 51, Fig 2.3.1): Ascending transposition by two semitones moves every point in pitch space to the right by 2.

**Example 2** (p. 47): A whistler who sings a tune on Tuesday and reproduces it a bit higher on Wednesday has transposed the tune, preserving frequency ratios (and hence pitch-space distances).

# Relationships
## Builds Upon
- **pitch-space** — The space in which transposition is defined
- **distance-in-music** — Transposition preserves distances
## Enables
- **transpositional-set-class** — Defined using transposition equivalence
- **optic-symmetries** — T is one of the five OPTIC symmetries
## Related
- **pitch-class-space** — In this space, transposition is rotation
## Contrasts With
- **inversion** — The other distance-preserving transformation; reverses direction while preserving distances

# Common Errors
- **Error**: Confusing transposition (moves all notes by the same amount) with octave shift (moves one note by an octave)
  **Correction**: Transposition moves all notes uniformly; octave shift moves a single note by 12 semitones

# Common Confusions
- **Confusion**: Thinking transposition in pitch-class space and transposition in pitch space are the same
  **Clarification**: In pitch-class space, T12 = T0 (the identity); in pitch space, T12 moves everything up an octave

# Source Reference
Chapter 2: Harmony and Voice Leading, Section 2.3, pages 51-53.

# Verification Notes
- Definition source: Direct from Section 2.3 with mathematical formula
- Confidence rationale: High — standard music-theoretical concept precisely defined
- Cross-reference status: Verified; used throughout the book
