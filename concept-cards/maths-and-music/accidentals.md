---
# === CORE IDENTIFICATION ===
concept: Accidentals
slug: accidentals

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
section: "Accidentals"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "sharps and flats"
  - "chromatic alterations"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - note-notation-and-the-staff
  - keyboard-layout
extends: []
related:
  - enharmonic-equivalence
  - note-classes
  - key-signatures-and-the-circle-of-fifths
  - diatonic-and-chromatic-scales
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are accidentals in music notation?"
  - "How do sharps, flats, naturals, and double accidentals alter pitch?"
  - "What are the rules for how long an accidental applies?"
---

# Quick Definition

Symbols (sharp, flat, natural, double sharp, double flat) placed before notes to raise or lower their pitch by one or two semitones, creating chromatic alterations.

# Core Definition

An accidental is a symbol that alters the pitch of a note (Wright, p. 21):
- Sharp ($\sharp$): raises pitch by one semitone
- Flat ($\flat$): lowers pitch by one semitone
- Natural ($\natural$): cancels a previous sharp or flat
- Double sharp ($\times$): raises pitch by two semitones
- Double flat ($\flat\flat$): lowers pitch by two semitones

The altered note class is denoted with the accidental as a superscript: $D^\sharp$, $A^\flat$.

# Prerequisites

- **Note Notation and the Staff** — Accidentals modify named notes
- **Keyboard Layout** — Accidentals correspond to adjacent keys on the keyboard

# Key Properties

1. Sharp raises by one semitone; flat lowers by one semitone
2. Natural cancels a sharp or flat
3. Double sharp/flat alter by two semitones
4. An accidental applies within the measure to all notes of the same note class, unless cancelled
5. Tied notes carry the accidental across bar lines, but other notes of the same class in the new measure are unaffected
6. Cautionary accidentals (in parentheses) are redundant reminders for clarity

# Construction / Recognition

## To apply accidental rules within a measure:

1. When an accidental appears, it alters that note and all subsequent notes of the same note class within the measure
2. A new accidental on the same note class cancels the previous one
3. At the bar line, all accidentals reset (except for tied notes)
4. If a tied note crosses a bar line, its accidental applies only to that tied note, not to other notes of the same class

# Context & Application

Accidentals define a function on note classes: the sharp operation shifts each note up by one semitone, and flat is the inverse. In modular arithmetic terms, $\sharp$ adds 1 (mod 12) and $\flat$ subtracts 1 (mod 12). When two different note names produce the same pitch (e.g., $F^\sharp$ and $G^\flat$), they are enharmonically equivalent.

# Examples

- $F^\sharp$ is the same pitch as $G^\flat$ (enharmonic equivalence) (p. 21)
- $C^\flat_5$ is the same note as $B_4$ (p. 19)
- $B^\sharp_3$ coincides with $C_4$ (p. 19)
- In the key of G, $\flat\hat{6}$ denotes $E^\flat$ (p. 28)
- $\sharp\hat{3} = \hat{4}$ in many keys — raising a diatonic note by a semitone can land on another diatonic note (p. 28)

# Relationships

## Builds Upon
- **Note Notation and the Staff** — Accidentals modify staff notation
- **Keyboard Layout** — Each accidental maps to an adjacent key

## Enables
- **Enharmonic Equivalence** — Defined by accidentals producing the same pitch
- **Key Signatures and the Circle of Fifths** — Key signatures are collections of accidentals

## Related
- **Note Classes** — Accidentals define relationships between note classes
- **Diatonic and Chromatic Scales** — Chromatic notes require accidentals in a given key

# Common Errors

- **Error**: Forgetting that an accidental applies to all notes of the same note class for the rest of the measure
  **Correction**: An accidental persists within its measure for all subsequent notes of the same class

# Common Confusions

- **Confusion**: Thinking a tied note's accidental applies to all notes of the same class in the new measure
  **Clarification**: A tied accidental applies only to the tied note itself, not to other notes of the same class in the new measure (p. 37)
- **Confusion**: Expecting $\sharp\hat{3}$ to always be a chromatic note
  **Clarification**: $\sharp\hat{3} = \hat{4}$ in major keys because the interval from $\hat{3}$ to $\hat{4}$ is a half step

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Accidentals" section, p. 21 (PDF); Chapter 2: "Rules about accidentals" section, p. 37 (PDF).

# Verification Notes

- Definition source: Direct from source, p. 21 and p. 37
- Confidence rationale: High — explicit definitions with all five accidental types listed
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: modular arithmetic interpretation, tied note bar-line rule from Ch 2, cautionary accidentals
