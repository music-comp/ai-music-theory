---
concept: Approximating Standard Keyboard Intervals
slug: approximating-standard-keyboard-intervals

category: modular-arithmetic
subcategory: chromatic-scales
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
section: "Approximating Standard Keyboard Intervals"

extraction_confidence: high

aliases:
  - interval approximation in n-chromatic scales

prerequisites:
  - n-chromatic-scale
  - multiplicative-to-additive-conversion
extends:
  - n-chromatic-scale
related:
  - non-standard-chromatic-scales
  - twelve-chromatic-scale
contrasts_with: []

answers_questions:
  - "How closely can a non-standard n-chromatic scale approximate standard keyboard intervals?"
  - "How do you convert semitones to n-chromatic units?"
---

# Quick Definition

The process of finding the closest interval in a non-standard n-chromatic scale to a given standard 12-chromatic keyboard interval.

# Core Definition

To express l semitones in n-chromatic units: l semitones = (n/12) * l n-chromatic units. The best approximation is the nearest integer. More generally, to convert any frequency ratio r to n-chromatic units: x = n * log_2(r) = n * ln(r)/ln(2) (Wright, pp. 75-76).

# Prerequisites

- **N-chromatic scale** — The target scale in which approximation is measured
- **Multiplicative-to-additive conversion** — The formula for converting ratios to n-chromatic units

# Key Properties

1. Conversion factor from semitones to n-chromatic units is n/12
2. The n-chromatic unit in cents is 1200/n
3. k n-chromatic units equals 1200k/n cents
4. The approximation error is the difference between the exact value and its nearest integer, convertible to cents
5. The tritone is exactly n/2 units in any even-n scale (exact, not approximate)

# Construction / Recognition

## To Approximate l Semitones in the n-Chromatic Scale
1. Compute l * (n/12) n-chromatic units
2. Round to the nearest integer k
3. The approximation error in n-chromatic units is |l * (n/12) - k|
4. Convert to cents: error_cents = |l * (n/12) - k| * (1200/n)

## To Convert Ratio r to n-Chromatic Units
1. Compute x = n * ln(r)/ln(2)
2. Round to nearest integer for the best approximation

# Context & Application

When experimenting with non-standard chromatic scales, musicians need to know how closely familiar intervals can be reproduced. Some scales approximate certain standard intervals well while others may be poorly approximated, affecting the recognizability of familiar chords and melodies.

# Examples

**Example 1** (p. 76): In the 14-chromatic scale, a fourth (5 semitones) = (7/6) * 5 = 35/6 ~ 5.833 14-chromatic units. Best approximated by 6 units = 6 * (1200/14) ~ 514.29 cents (14.29 cents sharp of the 500-cent fourth).

**Example 2** (p. 76): The tritone is exactly n/2 units in any even-n scale (being exactly half an octave).

**Example 3** (p. 76): The ratio 0.75 in the 14-scale: 14 * ln(0.75)/ln(2) ~ -5.81 units (5.81 units downward).

# Relationships

## Builds Upon
- **N-chromatic scale** — Approximation is defined within an n-chromatic context
- **Multiplicative-to-additive conversion** — Provides the conversion formula

## Enables
Understanding interval quality in non-standard tuning systems.

## Related
- **Non-standard chromatic scales** — Approximation quality characterizes these scales
- **Twelve-chromatic scale** — The reference standard being approximated

# Common Errors

- **Error**: Using n/12 * l as the exact interval rather than an approximation target
  **Correction**: The exact value n/12 * l is generally not an integer; the nearest integer is the best approximation, with non-zero error

# Common Confusions

- **Confusion**: Assuming good approximation of one interval implies good approximation of all
  **Clarification**: Approximation quality varies by interval; a scale that well-approximates fifths may poorly approximate thirds

# Source Reference

Chapter 6: "Chromatic Scales," pp. 75-76 (approximating standard keyboard intervals section).

# Verification Notes

- Definition source: Direct from Wright, pp. 75-76
- Confidence rationale: High — explicit formulas and worked examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: n=14 fourth approximation, tritone exactness, ratio 0.75 example
