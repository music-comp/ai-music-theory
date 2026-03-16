---
concept: Octave Equivalence
slug: octave-equivalence

category: pitch-and-intervals
subcategory: frequency
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Octave Equivalence"

extraction_confidence: high

aliases:
  - "modulo octave"
  - "octave identification"

prerequisites:
  - musical-intervals
  - equivalence-relations
extends:
  - equivalence-relations
related:
  - note-classes
  - equivalence-classes
  - division-algorithm
contrasts_with:
  - enharmonic-equivalence

answers_questions:
  - "What is octave equivalence?"
  - "How is octave equivalence formalized as an equivalence relation?"
  - "How many note classes exist under octave equivalence?"
---

# Quick Definition

The principle that notes separated by one or more octaves are considered equivalent, reducing the infinite set of keyboard notes to just twelve note classes.

# Core Definition

Octave equivalence is an equivalence relation on the set of chromatic scale notes: two notes are related if the interval between them is $n$ octaves (i.e., $12n$ semitones) for some integer $n \in \mathbb{Z}$. This relation satisfies reflexivity ($n = 0$), symmetry (if the interval is $12n$, the reverse is $12(-n)$), and transitivity (if intervals are $12n$ and $12m$, their sum is $12(n+m)$). The term "modulo octave" refers to this equivalence relation (Wright, pp. 20-21).

# Prerequisites

- **Musical Intervals** — Octave equivalence is defined using intervals measured in semitones
- **Equivalence Relations** — Octave equivalence is verified as an equivalence relation

# Key Properties

1. Under octave equivalence, there are exactly 12 distinct note classes
2. A note written without subscript (e.g., "A") denotes an equivalence class
3. Intervals are also subject to octave equivalence: a whole step and a ninth are equivalent modulo octave
4. Each equivalence class of intervals has a unique representative between 0 and 11 semitones
5. This connects to the Division Algorithm: $n = 12q + r$ with $0 \leq r < 12$

# Construction / Recognition

## To reduce a note or interval modulo octave:

1. Express the interval in semitones: call it $n$
2. Apply the Division Algorithm: $n = 12q + r$ with $0 \leq r < 12$
3. The remainder $r$ identifies the interval class (or note class, if counting from a reference)
4. The quotient $q$ tells how many octaves were traversed

# Context & Application

Music notation and terminology routinely assume octave equivalence: key signatures, chord names, and scale patterns all operate modulo octave. Under octave equivalence, the standard scale has 7 notes (not 8, since the final note is redundant). Only 7 scale degree numbers ($\hat{1}$ through $\hat{7}$) are needed, though $\hat{9}$ is sometimes used when octave identification is suspended.

# Examples

- $B^\flat_2$ and $B^\flat_5$ are equivalent modulo octave (p. 20)
- The note class $B^\flat$ is the equivalence class $\{B^\flat_n \mid n \in \mathbb{Z}\}$ (p. 20)
- A whole step (2 semitones) and a ninth (14 semitones) are equivalent modulo octave, since $14 = 1 \cdot 12 + 2$ (p. 20)
- Modulo octave, there are exactly 12 note classes (p. 20)

# Relationships

## Builds Upon
- **Musical Intervals** — Octave equivalence identifies intervals differing by 12 semitones
- **Equivalence Relations** — Octave equivalence is a specific equivalence relation

## Enables
- **Note Classes** — The 12 equivalence classes under octave equivalence
- **Diatonic and Chromatic Scales** — Scales are described modulo octave
- **Key Signatures and the Circle of Fifths** — Key signatures operate modulo octave

## Related
- **Division Algorithm** — Reduction modulo 12 uses the Division Algorithm

## Contrasts With
- **Enharmonic Equivalence** — Different relation: same pitch, different name (vs. same name, different octave)

# Common Errors

- **Error**: Including the octave note when counting scale notes under octave equivalence
  **Correction**: The standard scale under octave equivalence has 7 notes (C D E F G A B), not 8

# Common Confusions

- **Confusion**: Thinking octave equivalence is a physical fact
  **Clarification**: It is a mathematical choice — $A_2$ (110 Hz) and $A_5$ (880 Hz) are perceptually similar but physically distinct
- **Confusion**: Conflating octave equivalence of notes with octave equivalence of intervals
  **Clarification**: Both apply: notes modulo octave gives 12 note classes; intervals modulo octave gives interval classes with representatives in 0-11 semitones

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Octave Equivalence" section, pp. 20-21 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 20-21
- Confidence rationale: High — explicit definition with three properties verified
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Division Algorithm connection, interval equivalence examples, 7 vs 8 scale notes
