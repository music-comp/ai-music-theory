---
# === CORE IDENTIFICATION ===
concept: Note Durational Values
slug: note-durational-values

# === CLASSIFICATION ===
category: rhythm-and-form
subcategory: duration
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
section: "Duration of Notes"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "note values"
  - "durational notes"
  - "note durations"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - beats-and-tempo
extends: []
related:
  - dotted-note-duration-formula
  - tuplets
  - meter-and-time-signatures
  - ties-and-slurs
  - equivalence-classes
  - rests
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How is the system of note durations organized?"
  - "What determines whether a notehead is filled or unfilled?"
  - "What is a durational note in Wright's terminology?"
---

# Quick Definition

The system of note durations based on powers of 2, where each successive subdivision halves the duration: whole note, half note, quarter note, eighth note, and so on.

# Core Definition

Note durations are based on the *whole note*, whose duration in beats is dictated by the time signature. Notes with duration in proportion $1/2^n$ ($n$ a non-negative integer) to the whole note are named accordingly: the $\frac{1}{2^n}$-th note. Wright introduces the non-standard term *durational note* for a note distinguished by its duration independent of pitch — formally, an equivalence class of all notes having the same duration (Wright, pp. 30-32).

# Prerequisites

- **Beats and Tempo** — Notes are measured in beats

# Key Properties

1. The duration system is built on powers of 2: $1, \frac{1}{2}, \frac{1}{4}, \frac{1}{8}, \frac{1}{16}, \frac{1}{32}, \frac{1}{64}, \ldots$
2. The system is "highly oriented around the prime number 2 and its powers" (p. 34)
3. Whole and half notes have unfilled noteheads; for $n \geq 2$, noteheads are filled
4. All notes except the whole note ($n = 0$) have stems
5. For $n \geq 3$, the stem has $n - 2$ flags (eighth note has 1 flag, sixteenth has 2, etc.)
6. Adjacent flagged notes may use beams instead of individual flags

# Construction / Recognition

## To identify a note's duration:

1. Check the notehead: unfilled = whole or half note; filled = quarter note or shorter
2. Check the stem: no stem = whole note; stem present = half note or shorter
3. Count flags/beams: 0 = half or quarter; 1 = eighth; 2 = sixteenth; etc.
4. The formula is: $n - 2$ flags for the $\frac{1}{2^n}$-th note (where $n \geq 3$)

# Context & Application

The notation system encodes duration through notehead filling, stem presence, and flag count. "Durational note" as equivalence class parallels octave equivalence: grouping by duration rather than by pitch. To divide into non-power-of-2 parts requires tuplets. Pitches sounded simultaneously may share a common stem.

# Examples

- Whole note ($n = 0$): duration 1 (relative to whole note), unfilled head, no stem (p. 31)
- Half note ($n = 1$): duration $\frac{1}{2}$, unfilled head, stem (p. 31)
- Quarter note ($n = 2$): duration $\frac{1}{4}$, filled head, stem, no flags (p. 31)
- Eighth note ($n = 3$): 1 flag, duration $\frac{1}{8}$ (p. 31)
- Sixty-fourth note ($n = 6$): 4 flags, duration $\frac{1}{64}$ (p. 31)
- If whole note = 4 beats, the sixty-fourth note = $\frac{1}{16}$ of a beat (p. 31)

# Relationships

## Builds Upon
- **Beats and Tempo** — Durations are measured in beats

## Enables
- **Dotted Note Duration Formula** — Dots extend the power-of-2 system
- **Tuplets** — Required for non-power-of-2 divisions
- **Rests** — Rests follow the same durational hierarchy

## Related
- **Equivalence Classes** — "Durational note" is an equivalence class concept
- **Ties and Slurs** — Ties combine durations additively

# Common Errors

- **Error**: Computing the flag count as $n$ instead of $n - 2$
  **Correction**: The $\frac{1}{2^n}$-th note has $n - 2$ flags, starting at $n = 3$ (eighth note)

# Common Confusions

- **Confusion**: Thinking "durational note" is standard terminology
  **Clarification**: It is Wright's non-standard term for the equivalence class of notes having the same duration (e.g., "half note" regardless of pitch)
- **Confusion**: Conflating durational equivalence with octave equivalence
  **Clarification**: These are different equivalence relations — one classifies by duration, the other by pitch class

# Source Reference

Chapter 2: "Horizontal Structure", "Duration of Notes" and "Noteheads, Stems, Flags, and Beams" sections, pp. 30-32 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 30-32
- Confidence rationale: High — systematic presentation with formula for flag count
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: flag count formula ($n-2$), "durational note" as equivalence class, prime-number-2 observation
