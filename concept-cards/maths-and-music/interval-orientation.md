---
concept: Interval Orientation
slug: interval-orientation

category: pitch-and-intervals
subcategory: ratios
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "Orientation of Intervals"

extraction_confidence: high

aliases:
  - interval direction

prerequisites:
  - interval-as-frequency-ratio
extends: []
related:
  - unison-and-opposite-intervals
  - multiplicative-composition-of-intervals
contrasts_with: []

answers_questions:
  - "How do you determine if an interval is upward or downward?"
  - "What ratio values correspond to upward vs. downward intervals?"
---

# Quick Definition

Intervals have an upward or downward orientation. An interval from f_1 to f_2 is upward when f_2 > f_1 (ratio > 1) and downward when f_2 < f_1 (ratio < 1). Unison (ratio = 1) is the boundary.

# Core Definition

The interval given by pitches (f_2, f_1), read as "the interval from f_1 to f_2," is:

- Upward if f_2 > f_1, equivalently r = f_2/f_1 > 1
- Downward if f_2 < f_1, equivalently r = f_2/f_1 < 1
- Unison if f_2 = f_1, equivalently r = 1

Thus the set of downward intervals = {x in R | 0 < x < 1} = (0, 1), and the set of upward intervals = {x in R | 1 < x} = (1, infinity) (Wright, p. 59).

# Prerequisites

- **Interval as Frequency Ratio** -- Orientation is a property of the ratio

# Key Properties

1. Upward intervals: ratio r > 1
2. Downward intervals: 0 < r < 1
3. Unison: r = 1 (the boundary between upward and downward)
4. The two sets (0,1) and (1,infinity) are related by the inversion map r -> r^(-1)
5. Default convention: if orientation is not stated, upward is assumed

# Construction / Recognition

## To Determine Orientation

1. Compute the interval ratio r = f_2/f_1
2. If r > 1: upward
3. If r < 1: downward
4. If r = 1: unison

# Context & Application

The logarithm transforms this partition into negative reals (downward) and positive reals (upward), with 0 (unison) as the boundary -- matching the more intuitive additive picture. Musicians say "up a fifth" for ratio approximately 3/2 (> 1) and "down a fifth" for ratio approximately 2/3 (< 1). By convention, "the interval of a fourth" means "the upward interval of a fourth" (Wright, p. 59).

# Examples

**Example 1** (p. 60): Up a major third: ratio 2^(1/3) ~ 1.26 (greater than 1).

**Example 2** (p. 60): Down a minor third: ratio 2^(-1/4) ~ 0.84 (less than 1).

**Example 3** (p. 59): Unison: ratio 1 (no change in pitch).

# Relationships

## Builds Upon

- **Interval as Frequency Ratio** -- Orientation is determined by whether the ratio exceeds, equals, or is less than 1

## Related

- **Unison and Opposite Intervals** -- Unison is the boundary; opposite reverses orientation
- **Multiplicative Composition of Intervals** -- Composing upward and downward intervals uses multiplication

# Common Errors

- **Error**: Assuming downward intervals have negative ratios
  **Correction**: All interval ratios are positive; downward intervals have ratios between 0 and 1

# Common Confusions

- **Confusion**: Interpreting the ratio 0.5 as "half an interval"
  **Clarification**: Ratio 0.5 means "down one octave" (since 1/2 = 2^(-1)); it is a full octave, not "half" of anything
- **Confusion**: Thinking orientation is determined by the absolute size of the ratio
  **Clarification**: Orientation depends only on whether r > 1 or r < 1, not on how far from 1

# Source Reference

Chapter 4: "Ratios and Musical Intervals," p. 59.

# Verification Notes

- Definition source: Direct from p. 59
- Confidence rationale: High -- explicitly defined with set-theoretic notation
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: logarithm transformation context, set notation for upward/downward partition
