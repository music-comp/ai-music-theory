---
# === CORE IDENTIFICATION ===
concept: Duration-Class Modular Space
slug: duration-class-modular-space

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
section: "2.2.6"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Example 2.2.6"
  - additive duration-class space

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
  - additive-duration-space
  - equivalence-class
extends: []
related:
  - beat-class-space
  - generalized-interval-system
contrasts_with:
  - additive-duration-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I construct a GIS from a musical space?"
---

# Quick Definition

Duration-class modular space rescues the failed additive duration space (Example 2.2.5) by wrapping durations around an M-hour clock, making "negative" duration-classes meaningful as modular differences.

# Core Definition

"Example 2.2.6 can be regarded as one means of salvaging example 2.2.5 in this connection, by providing a convention that attaches meaning to the concept of a negative duration-class. E.g. we can think of duration-class '-5' as that class containing all durations lasting just 5 units less than some multiple of the modulus duration" (Lewin, Section 2.4, p. 56). S = M duration-classes, IVLS = ZM (additive group of integers mod M), int(s, t) = clockwise distance from s to t on an M-hour clock. Condition (B), which failed for 2.2.5, obtains here.

# Prerequisites

- **Group** — IVLS = (ZM, +) is a group
- **Additive Duration Space** — this rescues the failed Example 2.2.5
- **Equivalence Class** — duration-classes are equivalence classes

# Key Properties

1. S = M duration-classes (durations mod M)
2. IVLS = ZM = integers under addition mod M
3. int(s, t) = clockwise distance on M-hour clock
4. "-5" becomes "M - 5" in modular arithmetic, giving it meaning
5. Condition (B) holds here (unlike Example 2.2.5)

# Construction / Recognition

## To Construct:
1. Restrict to durations that are positive integral multiples of a basic unit
2. Wrap around an M-hour clock
3. Define int(s, t) = (t - s) mod M

## To Recognize:
1. Finite set of M duration-classes
2. Intervals are integers mod M
3. The additive interval int(s, t) = t is int(s, t) units longer than s, give or take M-unit "measures"

# Context & Application

This GIS rescues the additive approach to duration by using modular arithmetic. Duration-class "-5" becomes "M - 5," meaning "5 units less than a multiple of the modulus." If M = 16 (whole-note modulus), then duration-class 12 represents a dotted half (12 sixteenths), and the interval from 8 to 4 is 12 mod 16.

# Examples

**Example 1** (p. 55): M = 16 (time unit = sixteenth note). s = 8 (half note), t = 4 (quarter note). int(s, t) = 4 - 8 = -4 = 12 mod 16. Interpretation: a quarter note tied to an extra whole note is a dotted half longer than a half note.

# Relationships

## Builds Upon
- **Additive Duration Space** — this salvages the failed Example 2.2.5

## Related
- **Beat-Class Space** — structurally identical (both use ZM)

## Contrasts With
- **Additive Duration Space** — modular vs. non-modular; GIS vs. non-GIS

# Common Errors

- **Error**: Thinking this is the same as Example 2.2.5.
  **Correction**: Example 2.2.5 fails to be a GIS; this modular version succeeds by giving meaning to "negative" durations via modular arithmetic.

# Common Confusions

- **Confusion**: Conflating "duration-class" (mod M, additive) with "duration-class" from Example 2.2.4 (mod M, multiplicative).
  **Clarification**: Example 2.2.4 uses multiplicative modular reduction; Example 2.2.6 uses additive modular reduction. They are different constructions.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.6, Section 2.4, pp. 55-56.

# Verification Notes

- Definition source: direct from Example 2.2.6 and Section 2.4 discussion
- Confidence rationale: explicit example with detailed arithmetic
- New card (no prior version existed)
