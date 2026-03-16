---
# === CORE IDENTIFICATION ===
concept: Diatonic and Chromatic Scales
slug: diatonic-and-chromatic-scales

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: scales
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Scales"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "major scale"
  - "standard scale"
  - "chromatic scale"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - musical-intervals
  - note-classes
  - octave-equivalence
extends: []
related:
  - tetrachords
  - key-signatures-and-the-circle-of-fifths
  - accidentals
  - cyclic-permutations
  - ecclesiastical-modes
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the diatonic scale and what is its interval pattern?"
  - "What is the chromatic scale?"
  - "What makes two scales equivalent?"
---

# Quick Definition

The diatonic scale is the seven-note standard scale (e.g., C D E F G A B) with a specific pattern of whole and half steps; the chromatic scale contains all twelve notes within an octave.

# Core Definition

The standard (diatonic) scale based on C is the sequence C D E F G A B, with the interval pattern $1, 1, \frac{1}{2}, 1, 1, 1, \frac{1}{2}$ (in whole and half steps). The chromatic scale contains all 12 notes modulo octave. Two sequences of pitches are *equivalent* if the sequence of respective intervals is the same. A *standard scale* is any sequence of eight consecutive notes equivalent to the C scale. A scale is defined as a subsequence of the chromatic scale notes. In a given key, *diatonic notes* are those within the diatonic scale (Wright, pp. 21-22, 25).

# Prerequisites

- **Musical Intervals** — Scales are defined by their interval patterns
- **Note Classes** — Scales operate modulo octave
- **Octave Equivalence** — The last scale note is redundant under octave equivalence

# Key Properties

1. The diatonic scale has 7 notes (modulo octave); the chromatic scale has 12
2. The interval sequence $1, 1, \frac{1}{2}, 1, 1, 1, \frac{1}{2}$ characterizes the standard (major/Ionian) scale
3. This interval sequence has no non-trivial cyclic permutation equal to itself
4. The scale contains two equivalent tetrachords: C-D-E-F and G-A-B-C, each with pattern $1, 1, \frac{1}{2}$
5. Diatonic notes are key-dependent: $F^\sharp$ is diatonic in G major but chromatic in C major
6. The interval pattern is built into notation, making C the "default" key

# Construction / Recognition

## To construct a standard scale in any key:

1. Start with the tonic note
2. Apply the interval sequence: whole, whole, half, whole, whole, whole, half
3. Adjust accidentals as needed to maintain the pattern
4. Verify the result spans exactly one octave

# Context & Application

The diatonic scale corresponds to the white keys on the keyboard (in C). The pattern of intervals is built into musical notation, making C the "default" key — nothing in the notation itself indicates that E to F is a half step while F to G is a whole step. The seven modes (cyclic permutations) and the system of key signatures both derive from this scale structure.

# Examples

- C major: $C \xrightarrow{1} D \xrightarrow{1} E \xrightarrow{1/2} F \xrightarrow{1} G \xrightarrow{1} A \xrightarrow{1} B \xrightarrow{1/2} C$ (p. 21)
- $E^\flat$ major: $E^\flat, F, G, A^\flat, B^\flat, C, D, E^\flat$ is a standard scale (p. 21)
- Tetrachord equivalence: C-D-E-F and G-A-B-C share interval pattern $1, 1, \frac{1}{2}$ (p. 21)
- The chromatic scale has 12 notes modulo octave; the diatonic has 7 (p. 25)

# Relationships

## Builds Upon
- **Musical Intervals** — Scales are sequences of intervals
- **Note Classes** — Scales operate on note classes

## Enables
- **Key Signatures and the Circle of Fifths** — Key signatures tailor notes to produce the standard scale
- **Cyclic Permutations** — Modes are cyclic permutations of the diatonic scale
- **Ecclesiastical Modes** — The seven modes arise from cyclic permutations of the standard scale

## Related
- **Tetrachords** — The standard scale decomposes into two equivalent tetrachords
- **Accidentals** — Chromatic notes require accidentals within a given key

# Common Errors

- **Error**: Assuming all adjacent notes in the diatonic scale are a whole step apart
  **Correction**: Half steps occur between scale degrees 3-4 and 7-8 (E-F and B-C in C major)

# Common Confusions

- **Confusion**: Thinking "standard scale" means any scale
  **Clarification**: Wright uses "standard scale" specifically for scales with the major/Ionian interval pattern $1, 1, \frac{1}{2}, 1, 1, 1, \frac{1}{2}$
- **Confusion**: Assuming "chromatic" notes are fixed
  **Clarification**: Which notes are "chromatic" depends on the key: $F^\sharp$ is diatonic in G major but chromatic in C major

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Scales" and "Diatonic and Chromatic notes" sections, pp. 21-22, 25 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 21-22, 25
- Confidence rationale: High — explicit definitions with interval patterns and examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: tetrachord equivalence, notation-as-default-key insight, key-dependent chromatic notes
