---
# === CORE IDENTIFICATION ===
concept: Sets and Number Systems
slug: sets-and-number-systems

# === CLASSIFICATION ===
category: mathematical-foundations
subcategory: number-systems
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Sets and Numbers"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "number sets"
  - "standard number systems"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - interval-notation
  - pitch-and-frequency
  - division-algorithm
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What number sets are used in mathematical music theory?"
  - "How are the standard number systems denoted?"
  - "What is the relationship between Z, Q, and R?"
---

# Quick Definition

The standard number sets used throughout mathematical music theory: integers ($\mathbb{Z}$), rationals ($\mathbb{Q}$), and reals ($\mathbb{R}$), along with their positive subsets.

# Core Definition

Wright introduces the standard number sets with their conventional notation (p. 14): $\mathbb{R}$ for real numbers, $\mathbb{Q}$ for rational numbers, and $\mathbb{Z}$ for integers. The positive subsets are defined as $\mathbb{R}^+ = \{x \in \mathbb{R} \mid x > 0\}$, $\mathbb{Q}^+ = \{x \in \mathbb{Q} \mid x > 0\}$, and $\mathbb{Z}^+ = \{x \in \mathbb{Z} \mid x > 0\}$. The set $\mathbb{Z}^+$ is also called the set of natural numbers, denoted $\mathbb{N}$.

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. The sets have the containment relationship $\mathbb{Z} \subset \mathbb{Q} \subset \mathbb{R}$ and similarly $\mathbb{Z}^+ \subset \mathbb{Q}^+ \subset \mathbb{R}^+$
2. All three sets are ordered with the standard inequality symbols $<, \leq, >, \geq$
3. Ordering properties include: if $a < b$ and $c > 0$, then $ac < bc$; if $a < b$ and $c < 0$, then $ac > bc$
4. Basic set-theoretic notions (element, subset, union, intersection, function, one-to-one, onto) are assumed

# Construction / Recognition

## To identify which number set applies in a musical context:

1. If the quantity is continuous (e.g., frequency, pitch), use $\mathbb{R}$ or $\mathbb{R}^+$
2. If the quantity involves counting discrete units (e.g., semitones, octave numbers), use $\mathbb{Z}$ or $\mathbb{Z}^+$
3. If the quantity involves exact ratios (e.g., frequency ratios in just intonation, note durations), use $\mathbb{Q}$ or $\mathbb{Q}^+$

# Context & Application

These number sets provide the mathematical foundation for the entire text. The set $\mathbb{R}^+$ is identified with the set of all pitches via frequency in hertz. Integers $\mathbb{Z}$ arise in note subscript numbering, semitone counting, and modular arithmetic for octave equivalence. Rationals $\mathbb{Q}$ appear in frequency ratios for just intonation and in durational values of notes (powers of $1/2$).

# Examples

- The pitch A above middle C corresponds to $440 \in \mathbb{R}^+$ (frequency 440 Hz) (p. 17)
- The 12 semitones of the chromatic scale are counted using $\mathbb{Z}^+$ (p. 19)
- Frequency ratios like 3/2 (perfect fifth in just intonation) belong to $\mathbb{Q}^+$

# Relationships

## Builds Upon

This is a foundational concept.

## Enables
- **Pitch and Frequency** — Pitches are identified with $\mathbb{R}^+$
- **Division Algorithm** — Operates on $\mathbb{Z}$ and $\mathbb{Z}^+$
- **Interval Notation** — Defines subsets of $\mathbb{R}$

## Related
- **Interval Notation** — Uses $\mathbb{R}$ for defining interval subsets
- **Pitch and Frequency** — Maps pitches to $\mathbb{R}^+$

# Common Errors

- **Error**: Using $\mathbb{Z}$ when $\mathbb{R}$ is needed for continuous pitch
  **Correction**: Keyboard notes are discrete ($\mathbb{Z}$-indexed), but the full set of pitches is $\mathbb{R}^+$

# Common Confusions

- **Confusion**: Believing $\mathbb{Z}^+$ and $\mathbb{N}$ always mean the same thing
  **Clarification**: Wright defines $\mathbb{N} = \mathbb{Z}^+$, which excludes 0; some authors include 0 in $\mathbb{N}$
- **Confusion**: Thinking the set of pitches is discrete
  **Clarification**: The set of pitches is $\mathbb{R}^+$, not $\mathbb{Z}^+$, even though we typically work with discrete keyboard notes

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Sets and Numbers" section, p. 14 (PDF).

# Verification Notes

- Definition source: Direct from source text, p. 14
- Confidence rationale: High — explicit definitions with standard notation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: musical context examples (pitch as $\mathbb{R}^+$, semitone counting, frequency ratios), confusion about $\mathbb{Z}^+$ vs $\mathbb{N}$
