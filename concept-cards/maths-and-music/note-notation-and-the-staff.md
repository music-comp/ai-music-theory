---
# === CORE IDENTIFICATION ===
concept: Note Notation and the Staff
slug: note-notation-and-the-staff

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
  - "staff notation"
  - "note naming convention"
  - "scientific pitch notation"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-and-frequency
extends: []
related:
  - keyboard-layout
  - accidentals
  - note-classes
  - musical-intervals
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How are specific pitches notated on a musical staff?"
  - "What is the subscript convention for naming keyboard notes?"
  - "What is middle C in subscript notation?"
---

# Quick Definition

The system for representing specific pitches using notes placed on a five-line staff with treble and bass clefs, employing letters A through G and integer subscripts to identify exact pitches.

# Core Definition

Specific pitches are notated as notes on a staff using treble and bass clefs. Notes are labeled with letters A through G. A subscript notation uniquely identifies each keyboard note: $C_0$ is the C four octaves below middle C; for any integer $n$, $C_n$ is the C lying $n$ octaves above $C_0$ (below when $n$ is negative). Hence middle C is $C_4$. Other notes receive the subscript of the highest C that is lower than or equal to that note, after stripping away any sharp or flat alteration (Wright, pp. 18-19).

# Prerequisites

- **Pitch and Frequency** — Notes represent specific pitches on the staff

# Key Properties

1. Notes are labeled A through G, repeating in each octave
2. The subscript changes at C, not at A: $B_3$ is immediately below $C_4$
3. Middle C is $C_4$; the lowest C on the piano is $C_1$; $C_0$ is below the piano range
4. To determine the subscript of an accidental note: strip the accidental, find the highest C at or below, and reattach
5. The treble clef (G clef) and bass clef (F clef) are the standard clefs

# Construction / Recognition

## To name a keyboard note with subscript:

1. Identify the letter name (A through G)
2. Strip any accidental (sharp or flat)
3. Find the highest C that is lower than or equal to the natural note
4. Assign that C's subscript number to the original note
5. Reattach the accidental: e.g., $F^\sharp$ below $C_4$ becomes $F^\sharp_3$

# Context & Application

The subscript system establishes a bijection between keyboard notes and a discrete, ordered subset of $\mathbb{R}^+$ (via frequency). This provides a systematic enumeration of all notes on the infinite abstract keyboard. The system interacts with accidentals and enharmonic equivalence at octave boundaries.

# Examples

- Middle C is $C_4$; the C below middle C is $C_3$ (p. 18)
- The lowest C on the piano keyboard is $C_1$ (p. 18)
- $F^\sharp$ below $C_4$ is $F^\sharp_3$; $F^\sharp$ above $C_4$ is $F^\sharp_4$ (p. 19)
- $B^\sharp_3$ and $C^\flat_4$ both coincide with $C_4$ (p. 19)
- The lowest $B^\flat$ on the piano is $B^\flat_0$ (p. 19)

# Relationships

## Builds Upon
- **Pitch and Frequency** — Notes represent specific frequencies

## Enables
- **Musical Intervals** — Intervals are measured between named notes
- **Octave Equivalence** — Notes with same letter but different subscript are octave-equivalent
- **Accidentals** — Sharps and flats alter named notes

## Related
- **Keyboard Layout** — The physical arrangement that notes represent
- **Note Classes** — Removing subscripts gives note classes

# Common Errors

- **Error**: Assigning subscripts based on alphabetical order (changing at A)
  **Correction**: The subscript changes at C, not A: $B_3$ is immediately below $C_4$

# Common Confusions

- **Confusion**: Thinking $B^\sharp_3 = B_3$ raised by a semitone
  **Clarification**: $B^\sharp_3$ has the same pitch as $C_4$ — the subscript is determined before applying the accidental
- **Confusion**: Expecting the accidental to affect the subscript assignment
  **Clarification**: Accidentals are stripped before determining the subscript, then reattached: find the highest C below the natural note

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Notes" section, pp. 18-19 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 18-19
- Confidence rationale: High — explicit definition with worked examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: subscript assignment procedure, enharmonic overlap examples ($B^\sharp_3 = C_4$)
