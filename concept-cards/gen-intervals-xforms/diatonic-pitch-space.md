---
# === CORE IDENTIFICATION ===
concept: Diatonic Pitch Space
slug: diatonic-pitch-space

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
section: "2.1.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Example 2.1.1"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
  - function
extends: []
related:
  - chromatic-pitch-space
  - generalized-interval-system
  - diatonic-pitch-class-space
contrasts_with:
  - chromatic-pitch-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I construct a GIS from a musical space?"
---

# Quick Definition

Diatonic pitch space is a GIS where pitches are arranged in diatonic scalar order, and intervals measure the number of scale steps between pitches.

# Core Definition

"The musical space is a diatonic gamut of pitches arranged in scalar order. Given pitches s and t, int(s, t) is the number of scale steps one must move in an upwards-oriented sense to get from s to t" (Lewin, Example 2.1.1, p. 47). The GIS has S = the diatonic gamut extended indefinitely up and down, IVLS = the integers under addition, and int(s, t) = number of steps up from s to t (Section 2.4).

# Prerequisites

- **Group** — IVLS = (Z, +) is a group
- **Function** — int: S x S -> IVLS is a function

# Key Properties

1. S = diatonic pitches, extended indefinitely up and down
2. IVLS = (Z, +), the integers under addition
3. int(s, t) = number of scale steps up from s to t
4. Negative intervals represent downward motion: -n steps up = n steps down
5. Traditional interval names are replaced by additive numbering: 2 + 2 = 4, not "3rd + 3rd = 5th"

# Construction / Recognition

## To Construct:
1. Fix a diatonic scale (e.g., C major)
2. Extend the gamut indefinitely in both directions
3. Define int(s, t) = number of scale steps upward from s to t

## To Recognize:
1. Space consists of diatonic pitches in scalar order
2. Intervals are integers counting scale steps
3. Addition of intervals is ordinary integer addition

# Context & Application

This GIS models step-wise melodic motion in diatonic music. Lewin emphasizes that the non-traditional numbering (2 + 2 = 4 instead of "3rd + 3rd = 5th") is necessary for the algebra of Condition (A) to work. The space must include theoretically infinite extension for Condition (B).

# Examples

**Example 1** (p. 47): int(C4, C4) = 0, int(C4, D4) = 1, int(C4, E4) = 2, int(C4, C5) = 7, int(C4, A3) = -2.

**Example 2** (p. 47): Interval composition: int(C4, E4) = 2, int(E4, G4) = 2, int(C4, G4) = 4, and 2 + 2 = 4. This "obviates a defect in the traditional measurements which tell us, for example, that a '3rd' and another '3rd' compose to form a '5th.'"

# Relationships

## Builds Upon
- **Group** — uses (Z, +) as the interval group

## Enables
- **Diatonic Pitch-Class Space** — wrapping diatonic space around a clock

## Contrasts With
- **Chromatic Pitch Space** — counts semitones, not scale steps

# Common Errors

- **Error**: Using traditional interval names (3rd + 3rd = 5th).
  **Correction**: In this GIS, 2 + 2 = 4. Intervals count steps, not inclusive note-spans.

# Common Confusions

- **Confusion**: Thinking the space includes only pitches in the audible range.
  **Clarification**: The space must extend indefinitely for Condition (B) to be satisfied, including "supersonic and subsonic pitches."

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.1, Section 2.4, pp. 47, 53.

# Verification Notes

- Definition source: direct from Example 2.1.1 and Section 2.4
- Confidence rationale: explicit example with full GIS specification
- Re-extracted from v2 card; preserved: interval arithmetic correction (2+2=4 vs 3rd+3rd=5th), space extension note
