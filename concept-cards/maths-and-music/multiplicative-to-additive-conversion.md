---
# === CORE IDENTIFICATION ===
concept: Multiplicative-to-Additive Conversion
slug: multiplicative-to-additive-conversion

# === CLASSIFICATION ===
category: logarithms-and-measurement
subcategory: conversion
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Logarithms and Musical Intervals"
chapter_number: 5
pdf_page: 66
section: "Converting Intervals from Multiplicative to Additive Measurement"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - ratio to cents conversion
  - interval conversion formulas

# === TYPED RELATIONSHIPS ===
prerequisites:
  - logarithm-properties
  - change-of-base-formula
  - natural-logarithm
extends:
  - logarithmic-pitch-scale
related:
  - n-chromatic-scale
  - interval-as-frequency-ratio
  - group-of-intervals
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do you convert a frequency ratio to cents, semitones, or octaves?"
  - "How do logarithms convert multiplicative intervals to additive measurements?"
  - "What base should be used for each unit of measurement?"
---

# Quick Definition

The logarithm converts multiplicative interval measurements (frequency ratios) to additive measurements (octaves, semitones, cents), with the conversion formulas r -> 1200 * log_2(r) for cents, r -> 12 * log_2(r) for semitones, and r -> log_2(r) for octaves.

# Core Definition

The complete conversion framework (Wright, pp. 70-72):

Ratio to additive:
- Octaves: x = log_2(r) — formula (5.3)
- Semitones: x = 12 * log_2(r) — formula (5.2)
- Cents: x = 1200 * log_2(r) — formula (5.1)
- General n-chromatic units: x = n * log_2(r)

Additive to ratio:
- From octaves: r = 2^x
- From semitones: r = 2^(x/12)
- From cents: r = 2^(x/1200)
- From n-chromatic units: r = 2^(x/n)

The derivation: if the octave is to correspond to distance 1 on the logarithmic axis, then log_b(2) = 1, giving b = 2. Since r = 2^(x/1200) in cents, applying log_2 gives x = 1200 * log_2(r).

# Prerequisites

- **Logarithm properties** — L2 ensures the conversion preserves interval relationships
- **Change of base formula** — Needed for practical computation via ln or log_10
- **Natural logarithm** — Provides the computational tool via ln(r)/ln(2)

# Key Properties

1. The conversion is exact (lossless); rounding only occurs when reducing to integer semitones or cents
2. Negative results indicate a descending interval (r < 1 means f2 < f1)
3. The exponential and logarithm are exact inverses: converting back and forth recovers the original
4. The conversion x = n * log_2(r) can be written as x = log_b(r) where b = 2^(1/n)

# Construction / Recognition

## To Convert a Ratio r to Cents
1. Compute ln(r) and ln(2) using a calculator
2. Evaluate x = 1200 * (ln(r) / ln(2))
3. The result x is the interval in cents (negative if r < 1)
4. To find the best chromatic approximation, round to the nearest multiple of 100

# Context & Application

This conversion framework unifies the two ways musicians think about intervals. When a musician says "a fifth plus a fourth equals an octave," they work additively (7 + 5 = 12 semitones). When a physicist says "the frequency ratio 3/2 times 4/3 equals 2," they work multiplicatively. The logarithm shows these are the same statement: log_2(3/2) + log_2(4/3) = log_2(2) = 1 octave.

# Examples

**Example 1** (p. 72): Converting the ratio 3/2 to cents:
x = 1200 * (ln(3/2) / ln(2)) = 1200 * ((ln 3 - ln 2) / ln 2) ~ 701.955 cents.
The fifth (700 cents) is the best chromatic approximation, with an error of ~2 cents.

**Example 2** (p. 71): The semitone conversion can be expressed as x = log_{2^(1/12)}(r), since 2^(1/12) is the ratio of one semitone.

**Example 3** (p. 71): If r < 1, then ln r < 0, hence x < 0 (a descending interval).

# Relationships

## Builds Upon
- **Logarithmic pitch scale** — The conversion formulas formalize the logarithmic pitch representation
- **Logarithm properties** — L2 ensures the conversion preserves interval structure

## Enables
- **N-chromatic scale** — The general formula x = n * log_2(r) converts ratios to n-chromatic units
- **Approximating standard keyboard intervals** — Converting ratios to n-chromatic units enables comparison

## Related
- **Group of intervals** — The conversion is the isomorphism between (R, +) and (R+, *)
- **Interval as frequency ratio** — The multiplicative side of the conversion

# Common Errors

- **Error**: Forgetting to divide by ln(2) when using ln for the conversion
  **Correction**: The formula is 1200 * ln(r) / ln(2), not 1200 * ln(r)

- **Error**: Computing 1200 * log_2(r) when semitones are desired
  **Correction**: For semitones use 12 * log_2(r); for cents use 1200 * log_2(r)

# Common Confusions

- **Confusion**: Thinking the conversion introduces approximation
  **Clarification**: The conversion is exact; rounding only occurs when reducing to integer semitones or cents

- **Confusion**: Believing base 2 is used "because of binary"
  **Clarification**: Base 2 is used because the octave is a 2:1 ratio; log_2(2) = 1 makes the octave the unit

# Source Reference

Chapter 5: "Logarithms and Musical Intervals," pp. 70-72. See formulas (5.1), (5.2), (5.3) and the worked example converting 3/2 to cents.

# Verification Notes

- Definition source: Direct from Wright, pp. 70-72, with explicitly numbered formulas
- Confidence rationale: High — explicit formulas with derivations and worked examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: complete conversion table (both directions), musician vs. physicist unification example, base 2 justification
