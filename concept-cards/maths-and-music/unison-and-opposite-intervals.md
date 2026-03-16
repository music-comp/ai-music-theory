---
# === CORE IDENTIFICATION ===
concept: Unison and Opposite Intervals
slug: unison-and-opposite-intervals

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
section: "Orientation of Intervals"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - unison interval
  - inverse interval

# === TYPED RELATIONSHIPS ===
prerequisites:
  - interval-as-frequency-ratio
extends: []
related:
  - interval-orientation
  - multiplicative-composition-of-intervals
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the unison interval?"
  - "What is the opposite of an interval?"
  - "What ratio represents unison?"
---

# Quick Definition

The unison interval is the "zero interval" where f_1 = f_2, corresponding to ratio 1. Every interval f_2 : f_1 has a unique opposite interval f_1 : f_2, which spans the same distance in the opposite direction and has ratio r^(-1).

# Core Definition

The unison interval is given by the ratio f : f for any f in R+, corresponding via phi to the number 1. For any interval with ratio r = f_2/f_1, its opposite interval has ratio r^(-1) = f_1/f_2. If the original interval is upward (r > 1), the opposite is downward (r^(-1) < 1), and vice-versa. The unison interval is its own opposite (1^(-1) = 1). By convention, if orientation is not stated, an upward interval is meant (Wright, p. 59).

# Prerequisites

- **Interval as Frequency Ratio** -- Unison and opposite intervals are defined in terms of the ratio framework

# Key Properties

1. Unison interval: r = 1 (identity element in the group (R+, *))
2. Opposite interval of r: r^(-1) (multiplicative inverse)
3. Upward intervals have r > 1; their opposites have r^(-1) < 1 (downward)
4. The unison interval is its own opposite
5. Default convention: unspecified orientation means upward
6. The orientation partition divides R+ into three sets: {r < 1}, {1}, {r > 1}

# Construction / Recognition

## To Find the Opposite Interval

1. Given an interval with ratio r
2. Compute r^(-1) = 1/r
3. This is the opposite interval (same "distance," reversed direction)

# Context & Application

In the multiplicative group (R+, *), the unison is the identity and the opposite is the inverse. This makes intervals a group under multiplication. The convention that unspecified orientation means "upward" is important: "the interval of a fourth" means "the upward interval of a fourth" (Wright, p. 59).

# Examples

**Example 1** (p. 59): Unison: ratio 1 (same pitch, f : f for any f).

**Example 2** (p. 59): Up an octave: ratio 2; down an octave (opposite): ratio 1/2.

**Example 3**: Up a fifth (ratio ~3/2); down a fifth (opposite): ratio ~2/3.

# Relationships

## Builds Upon

- **Interval as Frequency Ratio** -- Unison and opposite are defined within the ratio framework

## Related

- **Interval Orientation** -- Orientation determines whether an interval is upward or downward
- **Multiplicative Composition of Intervals** -- Unison is the identity element; opposite is the inverse

# Common Errors

- **Error**: Computing the opposite interval by negating the ratio (-r) instead of inverting it (1/r)
  **Correction**: The opposite interval has ratio r^(-1) = 1/r, not -r; all ratios are positive

# Common Confusions

- **Confusion**: Thinking unison means "ratio 0" or "no ratio"
  **Clarification**: Unison is ratio 1, not 0; there is no ratio 0 in this system
- **Confusion**: Thinking "opposite" means complementary interval (e.g., fourth as opposite of fifth)
  **Clarification**: "Opposite" means the same interval in the reversed direction, not the interval that completes an octave

# Source Reference

Chapter 4: "Ratios and Musical Intervals," p. 59.

# Verification Notes

- Definition source: Direct from p. 59
- Confidence rationale: High -- explicitly defined
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: group structure observation (identity and inverse), convention about default upward orientation
