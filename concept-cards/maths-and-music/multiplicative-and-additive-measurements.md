---
# === CORE IDENTIFICATION ===
concept: Multiplicative and Additive Measurements
slug: multiplicative-and-additive-measurements

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: measurement
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "Multiplicative and Additive Measurements"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - multiplicative vs additive interval measurement

# === TYPED RELATIONSHIPS ===
prerequisites:
  - multiplicative-composition-of-intervals
extends: []
related:
  - interval-as-frequency-ratio
  - semitone-ratio
  - cents
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the difference between multiplicative and additive interval measurements?"
  - "Why do musicians 'add' intervals while mathematicians 'multiply' them?"
---

# Quick Definition

Interval measurement by frequency ratio is called multiplicative (juxtaposing intervals = multiplying ratios), while measurement in semitones, cents, or octaves is called additive (juxtaposing intervals = adding measurements). The logarithm converts between the two.

# Core Definition

A multiplicative measurement assigns to each interval a positive real number r in R+, where juxtaposing intervals corresponds to multiplying their measurements. An additive measurement assigns a real number x in R, where juxtaposing intervals corresponds to adding their measurements. The relationship is: r = 2^(x/n) for appropriate n, and x = n * log_2(r). "The measurement of intervals by ratio is called multiplicative." Conventional measurements such as semitones, steps, or octaves "are called additive because when we juxtapose two intervals we think of adding or subtracting" (Wright, pp. 59-60).

# Prerequisites

- **Multiplicative Composition of Intervals** -- Must understand why ratios multiply when intervals are juxtaposed

# Key Properties

1. Multiplicative: juxtaposition = multiplication of ratios
2. Additive: juxtaposition = addition of semitones/cents/octaves
3. The groups (R+, *) and (R, +) are isomorphic via the logarithm
4. The logarithm transforms multiplication to addition, division to subtraction
5. The choice of n determines the additive unit: n=1 (octaves), n=12 (semitones), n=1200 (cents)
6. The multiplicative framework is more fundamental; the additive is derived from it

# Construction / Recognition

## To Convert Between Frameworks

1. Multiplicative to additive: x = n * log_2(r), where n depends on the unit
2. Additive to multiplicative: r = 2^(x/n)
3. n = 12 for semitones; n = 1200 for cents; n = 1 for octaves

# Context & Application

Musicians naturally think additively: "2 semitones plus 3 semitones equals 5 semitones," "a fifth is a major third plus a minor third." The ratio framework is more fundamental mathematically, but the additive framework is more intuitive musically. The conversion between the two is the central theme connecting Chapters 4 and 5 (Wright, pp. 59-60).

# Examples

**Example 1** (p. 60): Additive: 4 semitones + 3 semitones = 7 semitones (major third + minor third = fifth).

**Example 2** (p. 60): Multiplicative: 2^(4/12) * 2^(3/12) = 2^(7/12) (the same calculation with ratios).

**Example 3**: "A semitone is a major sixth minus a minor sixth" -- additive language for the multiplicative operation r_major6 / r_minor6.

# Relationships

## Builds Upon

- **Multiplicative Composition of Intervals** -- The multiplicative property motivates the distinction

## Enables

- **Converting Ratios to Semitones** -- Converting between frameworks
- **Converting Ratios to Cents** -- Converting between frameworks

## Related

- **Interval as Frequency Ratio** -- The multiplicative measurement
- **Semitone Ratio** -- The bridge between multiplicative and additive (r = 2^(x/12))
- **Cents** -- An additive unit

# Common Errors

- **Error**: Adding ratios when combining intervals
  **Correction**: Ratios are multiplied; semitone counts are added; these are two descriptions of the same operation

# Common Confusions

- **Confusion**: Thinking multiplicative and additive are alternative systems
  **Clarification**: They describe the same reality in different mathematical languages; the logarithm is the dictionary between them
- **Confusion**: Believing the additive framework is more fundamental because it is more familiar
  **Clarification**: The multiplicative (ratio) framework is more fundamental; additive measurement is derived from it via logarithms

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 59-60.

# Verification Notes

- Definition source: Direct from pp. 59-60
- Confidence rationale: High -- explicitly named and defined
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: isomorphism between (R+,*) and (R,+), examples showing parallel additive/multiplicative calculations
