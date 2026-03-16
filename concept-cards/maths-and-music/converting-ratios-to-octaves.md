---
concept: Converting Ratios to Octaves
slug: converting-ratios-to-octaves

category: pitch-and-intervals
subcategory: measurement
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: null

extraction_confidence: medium

aliases:
  - ratio to octave conversion

prerequisites:
  - interval-as-frequency-ratio
extends: []
related:
  - converting-ratios-to-semitones
  - converting-ratios-to-cents
contrasts_with: []

answers_questions:
  - "How do I measure an interval in octaves?"
  - "Why is base-2 the natural logarithm base for music?"
---

# Quick Definition

An interval ratio r is converted to octaves by the formula x = log_2(r). This is the most fundamental additive measurement, from which semitones (multiply by 12) and cents (multiply by 1200) are derived.

# Core Definition

Given an interval ratio r in R+, its measurement in octaves is:

x = log_2(r)

This is the base case from which all other conversions derive. To make the octave correspond to exactly 1 unit, we need log_b(2) = 1, which requires b = 2. Therefore base 2 is the natural base for musical logarithms. The formulas for semitones (12 * log_2(r)) and cents (1200 * log_2(r)) are simply rescalings of this fundamental measurement (Wright, Ch. 4-5).

# Prerequisites

- **Interval as Frequency Ratio** -- Must understand intervals as ratios to apply the logarithmic conversion

# Key Properties

1. x = log_2(r) converts ratio to octaves
2. Base 2 is the unique base where the octave maps to 1
3. Semitones = 12 * log_2(r) = 12 * x (rescaling by 12)
4. Cents = 1200 * log_2(r) = 1200 * x (rescaling by 1200)
5. Plotting pitches by log_2(frequency) produces equally spaced octaves

# Construction / Recognition

## To Convert a Ratio to Octaves

1. Given ratio r
2. Compute x = log_2(r) = ln(r) / ln(2)
3. The result is the number of octaves (can be fractional)

# Context & Application

Measuring in octaves is the coarsest standard measurement. For most practical purposes, the finer measurements (semitones or cents) are more useful, but the octave measurement is conceptually the simplest and serves as the foundation. Plotting pitches by log_2(frequency) produces equally spaced octaves, correcting the non-equal spacing seen on a linear frequency axis. The concept is implicit in Ch. 4's discussion of the n-chromatic unit formula r = 2^(x/n) and developed explicitly with logarithms in Ch. 5 (Wright, Ch. 4-5).

# Examples

**Example 1**: Ratio 2: log_2(2) = 1 octave.

**Example 2**: Ratio 4: log_2(4) = 2 octaves.

**Example 3**: Ratio 3/2: log_2(3/2) ~ 0.585 octaves (a fifth is slightly more than half an octave).

**Example 4**: Ratio 1: log_2(1) = 0 octaves (unison).

**Example 5**: Ratio 1/2: log_2(1/2) = -1 octave (down one octave).

# Relationships

## Builds Upon

- **Interval as Frequency Ratio** -- The logarithm converts the multiplicative ratio to an additive measurement

## Related

- **Converting Ratios to Semitones** -- Semitones = 12 * octaves
- **Converting Ratios to Cents** -- Cents = 1200 * octaves

# Common Errors

- **Error**: Using log_10 or ln instead of log_2
  **Correction**: log_2 is required so that the octave (ratio 2) maps to exactly 1; use the change of base formula for computation

# Common Confusions

- **Confusion**: Thinking octave measurement is commonly used in practice
  **Clarification**: Semitones and cents are the practical units; octave measurement is the conceptual foundation
- **Confusion**: Interpreting a fractional octave value as "part of an octave"
  **Clarification**: It means the interval is not a whole number of octaves; e.g., 0.585 octaves is a fifth, not "58.5% of an octave" in any musical sense

# Source Reference

Chapter 4: "Ratios and Musical Intervals" (implicit in the n-chromatic unit formula). Chapter 5: "Logarithms and Musical Intervals," pp. 70-71 (explicit formula).

# Verification Notes

- Definition source: Synthesized from Ch. 4 (n-chromatic formula with n=1) and Ch. 5 (explicit logarithmic formula)
- Confidence rationale: Medium -- the concept is implicit in Ch. 4 and explicit in Ch. 5; synthesized across chapters
- Uncertainties: The explicit formula appears in Ch. 5, not Ch. 4
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card (which had Ch. 5 provenance); preserved: base-2 uniqueness argument, logarithmic pitch plotting, relationship between octave/semitone/cent measurements
