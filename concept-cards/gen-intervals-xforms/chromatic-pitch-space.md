---
# === CORE IDENTIFICATION ===
concept: Chromatic Pitch Space
slug: chromatic-pitch-space

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: musical-spaces
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "2.1.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Example 2.1.2"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
  - function
extends: []
related:
  - diatonic-pitch-space
  - pitch-class-space
  - generalized-interval-system
  - time-point-space
contrasts_with:
  - diatonic-pitch-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I construct a GIS from a musical space?"
---

# Quick Definition

Chromatic pitch space is a GIS where equally-tempered pitches are the elements and intervals measure the number of semitones between pitches.

# Core Definition

"The musical space is a gamut of chromatic pitches under twelve-tone equal temperament. Given pitches s and t, int(s, t) is the number of semitones one must move in an upwards-oriented sense to get from s to t, not counting s itself" (Lewin, Example 2.1.2, p. 47). The GIS has S = chromatic pitches extended indefinitely, IVLS = integers under addition, and int as semitone count (Section 2.4).

# Prerequisites

- **Group** — IVLS = (Z, +) is a group
- **Function** — int: S x S -> IVLS is a function

# Key Properties

1. S = chromatic pitches under equal temperament, extended indefinitely
2. IVLS = (Z, +), the integers under addition
3. int(s, t) = number of semitones up from s to t (not counting s)
4. Negative intervals represent downward motion
5. Structurally identical to time-point space (both use IVLS = Z)

# Construction / Recognition

## To Construct:
1. Fix the chromatic scale under equal temperament
2. Extend indefinitely in both directions
3. Define int(s, t) = semitone count upward from s to t

## To Recognize:
1. Space consists of equally-tempered pitches
2. Intervals are integers counting semitones

# Context & Application

This is the most common GIS for pitch analysis in twelve-tone and post-tonal music. Every pitch interval can be expressed as an integer number of semitones. This system underlies set theory, twelve-tone analysis, and much of modern music theory.

# Examples

**Example 1** (p. 47): int(C4, D4) = 2 (whole tone), int(C4, G4) = 7 (perfect fifth), int(C4, C5) = 12 (octave), int(C4, F3) = -7 (fifth down), int(C4, F2) = -19 (octave + fifth down).

# Relationships

## Builds Upon
- **Group** — uses (Z, +) as the interval group

## Enables
- **Pitch-Class Space** — wrapping chromatic space mod 12

## Related
- **Time-Point Space** — structurally identical GIS (both use Z under addition)

## Contrasts With
- **Diatonic Pitch Space** — counts scale steps, not semitones

# Common Errors

- **Error**: Counting the starting pitch in the interval.
  **Correction**: int(C4, D4) = 2, not 3. The starting pitch is not counted.

# Common Confusions

- **Confusion**: Assuming the space is limited to audible frequencies.
  **Clarification**: The space extends theoretically to include all frequencies, including those beyond hearing range, to satisfy Condition (B).

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.2, Section 2.4, pp. 47, 53.

# Verification Notes

- Definition source: direct from Example 2.1.2 and Section 2.4
- Confidence rationale: explicit example with full GIS specification
- Re-extracted from v2 card; preserved: all five interval examples, Condition (B) note
