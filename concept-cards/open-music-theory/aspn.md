---
# === CORE IDENTIFICATION ===
concept: "American Standard Pitch Notation (ASPN)"
slug: aspn

# === CLASSIFICATION ===
category: fundamentals
subcategory: pitch
tier: foundational

# === PROVENANCE ===
source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "American Standard Pitch Notation (ASPN)"
chapter_number: 6
pdf_page: null
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "scientific pitch notation"
  - "SPN"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch
  - octave-equivalence
extends: []
related:
  - middle-c
  - pitch-class
  - enharmonic-equivalence
contrasts_with:
  - pitch-class

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is ASPN?"
  - "How do you label specific pitches in ASPN?"
  - "Where do octave designations change?"
---

# Quick Definition

A labeling system that identifies specific pitches by combining a note name with a subscript octave number (e.g., C4 = middle C).

# Core Definition

**American Standard Pitch Notation** (ASPN) designates specific musical pitches by combining a note name (letter plus optional accidental) with a subscript octave designation (integer). Each octave begins on C and ends on B: C4, D4, E4, F4, G4, A4, B4 are all in octave 4. Octave designations increment at each C: B3 is followed by C4. Middle C = C4 (~261.63 Hz). The piano keyboard primarily spans octaves 1 through 7. ASPN labels are instrument-independent: C4 is C4 whether played on flute, cello, or voice.

# Prerequisites

- **Pitch**: the concept being labeled
- **Octave equivalence**: the repeating cycle that ASPN organizes

# Key Properties

- Format: [Letter][Accidental][Octave Number] (e.g., C4, G#5, Bb3)
- Each octave: C to B (octave changes at C, not A)
- Middle C = C4 (critical reference point to memorize)
- Piano spans octaves 1-7 (with partial 0 and 8)
- Accidentals do not change octave number: B#3 and C4 are enharmonically equivalent but have different ASPN labels
- ASPN specifies a pitch (with octave); pitch class does not

# Construction / Recognition

To determine an ASPN label, identify the letter name and accidental, then determine which octave the note is in by counting from the nearest C.

# Context & Application

ASPN provides unambiguous pitch communication. Without it, "play a C" is ambiguous (which octave?). With ASPN, "play C4" is precise. ASPN is essential in academic analysis, instrument specifications, and anywhere exact pitch identification matters. Standard tuning reference: A4 = 440 Hz.

# Examples

- C4 = middle C (~261.63 Hz)
- A4 = 440 Hz (standard tuning reference)
- All notes from C4 through B4 are in octave 4
- B3 to C4 crosses an octave boundary
- B#3 and C4: enharmonically equivalent but different ASPN octave numbers

# Relationships

- **Leads to**: pitch-class, transposition
- **See also**: middle-c, enharmonic-equivalence, piano-keyboard

# Common Errors

- Placing the octave boundary at A instead of C
- Labeling middle C as C3 or C5 instead of C4

# Common Confusions

- Octaves begin on C, not A (B3 is followed by C4, not B3 by A4)
- Accidentals do not change octave number: B#3 is NOT the same ASPN label as C4
- ASPN specifies pitch (with octave); pitch class abstracts away octave

# Source Reference

Open Music Theory, Part I, Chapter 6: "American Standard Pitch Notation (ASPN)"

# Verification Notes

Re-extracted from v2 card; preserved: format specification, B#3/C4 enharmonic example, A4=440Hz reference, octave boundary rule.
