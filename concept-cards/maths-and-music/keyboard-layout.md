---
# === CORE IDENTIFICATION ===
concept: Keyboard Layout
slug: keyboard-layout

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: notation
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Notes"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "piano keyboard"
  - "keyboard notes"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-and-frequency
  - note-notation-and-the-staff
extends: []
related:
  - musical-intervals
  - diatonic-and-chromatic-scales
  - accidentals
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How are notes arranged on a keyboard instrument?"
  - "What is the abstract infinite keyboard?"
  - "What are keyboard notes?"
---

# Quick Definition

The arrangement of white and black keys on a keyboard instrument, forming a repeating pattern that represents the chromatic scale, with an abstract infinite extension used for mathematical purposes.

# Core Definition

The keyboard consists of "white notes" and "black notes" arranged in a specific pattern that repeats every octave (12 semitones). Abstractly, Wright envisions a keyboard extending infinitely in both directions, giving an infinite set of notes. The notes appearing on this extended keyboard are called *keyboard notes*. This set does not represent all pitches, as there are pitches between adjacent notes (Wright, p. 18).

# Prerequisites

- **Pitch and Frequency** — Keyboard notes correspond to specific frequencies
- **Note Notation and the Staff** — Letter names and subscripts identify keyboard notes

# Key Properties

1. The pattern repeats every octave (12 keys: 7 white + 5 black)
2. Adjacent keys (white or black) are separated by one semitone
3. Black keys are grouped in alternating clusters of 2 and 3
4. Half steps between white keys occur at E-F and B-C (no black key between them)
5. The actual piano has 88 keys ($A_0$ to $C_8$); the abstract keyboard extends infinitely

# Construction / Recognition

## To identify the keyboard pattern:

1. Locate the groups of 2 and 3 black keys
2. The white key immediately left of the 2-black-key group is C
3. White keys from C to B span one octave of the diatonic scale
4. All adjacent keys (including black) are one semitone apart

# Context & Application

The repeating pattern within each octave consists of 7 white keys (C, D, E, F, G, A, B — the diatonic notes) and 5 black keys (the sharps/flats). The physical layout reflects the asymmetric interval pattern of the diatonic scale. The abstract infinite keyboard provides a concrete model for $\mathbb{Z}$ when notes are numbered by semitone distance from a reference note.

# Examples

- The white keys from C to B span one octave of the diatonic scale (p. 18)
- Black keys provide the remaining 5 chromatic notes within each octave (p. 18)
- The actual piano keyboard ranges from $A_0$ to $C_8$ (88 keys)
- Adjacent keys are always one semitone apart in equal temperament

# Relationships

## Builds Upon
- **Pitch and Frequency** — Each key corresponds to a frequency in $\mathbb{R}^+$
- **Note Notation and the Staff** — Letter names label the keys

## Enables
- **Musical Intervals** — Intervals are counted between keyboard notes
- **Diatonic and Chromatic Scales** — White keys form the diatonic scale in C

## Related
- **Accidentals** — Black keys correspond to sharped/flatted notes

# Common Errors

- **Error**: Assuming all adjacent white keys are a whole step apart
  **Correction**: E to F and B to C are half steps (no black key between them)

# Common Confusions

- **Confusion**: Thinking the keyboard represents all possible pitches
  **Clarification**: The keyboard is a discrete sampling of the pitch continuum; pitches between adjacent keys exist but are not represented
- **Confusion**: Conflating "keyboard note" with "any pitch"
  **Clarification**: "Keyboard note" refers specifically to notes available on the abstract keyboard, as opposed to arbitrary pitches in $\mathbb{R}^+$

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Notes" section, p. 18 (PDF).

# Verification Notes

- Definition source: Direct from source, p. 18
- Confidence rationale: High — explicit description with abstract extension defined
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: piano range ($A_0$ to $C_8$), logarithmic spacing insight, discrete vs. continuous distinction
