---
# === CORE IDENTIFICATION ===
concept: Interval as Frequency Ratio
slug: interval-as-frequency-ratio

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: ratios
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "The Ratio Associated to an Interval"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - interval ratio
  - frequency ratio

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - equivalence-relation-on-ordered-pairs
  - multiplicative-composition-of-intervals
  - interval-orientation
  - unison-and-opposite-intervals
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an interval in mathematical terms (as frequency ratio)?"
  - "Why is an interval a ratio rather than a difference of frequencies?"
  - "What ratio corresponds to an octave?"
---

# Quick Definition

A musical interval is properly measured not by the difference between two frequencies, but by their ratio. The octave corresponds to a frequency ratio of 2:1, regardless of the absolute frequencies involved.

# Core Definition

Given two pitches with frequencies f_1 and f_2, the interval from f_1 to f_2 is determined by the ratio r = f_2/f_1. Two pairs of pitches (f_2, f_1) and (f_2', f_1') represent the same interval if and only if f_2/f_1 = f_2'/f_1'. Each r in R+ gives a unique interval. Both the real number r and the corresponding equivalence class f_2 : f_1 are referred to as the "interval" or "interval ratio" determined by the frequencies f_1 and f_2 (Wright, pp. 58-59).

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. An interval is a ratio, not a difference, of frequencies
2. Each positive real number r gives a unique interval
3. The octave has ratio 2 (a factor of 2 in frequency)
4. Upward intervals have r > 1; downward intervals have 0 < r < 1
5. Unison has ratio r = 1
6. The function phi: (R+ : R+) -> R+ defined by phi((a:b)) = a/b is a bijection

# Construction / Recognition

## To Determine an Interval Ratio

1. Identify the two frequencies f_1 (starting pitch) and f_2 (ending pitch)
2. Compute the ratio r = f_2 / f_1
3. If r > 1: the interval is upward
4. If r < 1: the interval is downward
5. If r = 1: unison (no change)

# Context & Application

The insight that intervals correspond to ratios rather than differences is fundamental. The interval from A3 (220 Hz) to A4 (440 Hz) is one octave, and the interval from A4 (440 Hz) to A5 (880 Hz) is also one octave. The frequency differences are 220 Hz and 440 Hz respectively -- different -- but the ratios are both 2:1. This ratio-based understanding was known to the ancient Greeks and forms the foundation for all mathematical treatment of intervals (Wright, pp. 58-59).

# Examples

**Example 1** (p. 58): A4 = 440 Hz, A5 = 880 Hz, A3 = 220 Hz. The intervals A3->A4 and A4->A5 are both one octave (ratio 2), despite having different frequency differences (220 Hz vs. 440 Hz).

**Example 2** (p. 59): Various ratios create recognizable intervals: 3/2 (close to a fifth), sqrt(2) (tritone in equal temperament), and even transcendental numbers like pi and e define unique intervals.

# Relationships

## Enables

- **Multiplicative Composition of Intervals** -- Intervals compose by multiplication because they are ratios
- **Interval Orientation** -- Direction is determined by whether the ratio is greater or less than 1
- **Unison and Opposite Intervals** -- Unison is ratio 1; opposite is the reciprocal
- **Semitone Ratio** -- The semitone ratio is derived from the octave ratio

## Related

- **Equivalence Relation on Ordered Pairs** -- The formal mathematical foundation for ratios

# Common Errors

- **Error**: Computing the interval as a frequency difference (f_2 - f_1) rather than a ratio (f_2 / f_1)
  **Correction**: Equal intervals correspond to equal ratios, not equal differences

# Common Confusions

- **Confusion**: Thinking a ratio of 1 means "one unit of interval"
  **Clarification**: A ratio of 1 is unison (no interval at all); it is the identity element
- **Confusion**: Believing that equal frequency differences produce equal intervals
  **Clarification**: A difference of 220 Hz from 220 to 440 Hz is an octave, but 220 Hz from 440 to 660 Hz is only a fifth -- same difference, different intervals

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 58-59.

# Verification Notes

- Definition source: Direct from pp. 58-59
- Confidence rationale: High -- the central concept of the chapter, explicitly defined
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: bijection phi, examples with transcendental numbers, ancient Greeks reference
