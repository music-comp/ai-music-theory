---
# === CORE IDENTIFICATION ===
concept: Logarithmic Pitch Scale
slug: logarithmic-pitch-scale

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
section: "Logarithmic Scale for Pitch"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - logarithmic frequency axis
  - log pitch axis

# === TYPED RELATIONSHIPS ===
prerequisites:
  - logarithm-properties
  - logarithmic-functions-as-inverses
extends:
  - logarithm-properties
related:
  - multiplicative-to-additive-conversion
  - pitch-and-frequency
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "Why do equal musical intervals appear as equal distances on a logarithmic pitch scale?"
  - "What base should be used for a logarithmic pitch axis where octaves are unit distances?"
---

# Quick Definition

Plotting pitches by the logarithm of their frequency produces a scale where equal musical intervals appear as equal distances, matching the way musicians perceive and notate pitch.

# Core Definition

On a logarithmic pitch axis, each pitch with frequency f is plotted at position log_b(f). Property (L2) guarantees that if x/y = x'/y' (same interval ratio), then log_b(x) - log_b(y) = log_b(x') - log_b(y') (same distance on the axis). The base b determines the unit of measurement: choosing b = 2 makes the octave correspond to a distance of 1 on the logarithmic axis (Wright, pp. 68-70).

# Prerequisites

- **Logarithm properties** — L2 is the key property guaranteeing equal intervals produce equal distances
- **Logarithmic functions as inverses** — The logarithm function must be understood to construct the scale

# Key Properties

1. Equal intervals (same frequency ratio) produce equal distances on the logarithmic axis
2. The base b determines the unit distance: b = 2 gives octaves as unit, b = 2^(1/12) gives semitones
3. The scale transforms the exponential frequency-to-pitch relationship into a linear one
4. To make the octave correspond to distance n on the axis, choose b = 2^(1/n)

# Construction / Recognition

## To Plot Pitches on a Logarithmic Axis
1. Choose a base b (b = 2 for octaves as unit distance)
2. For each pitch with frequency f, compute log_b(f)
3. Plot pitches at their computed positions
4. Verify that equal intervals (same ratio) produce equal spacings

# Context & Application

The logarithmic pitch scale matches musical intuition and notation. On a musical staff, the vertical distance between any two notes one octave apart appears the same. Standard tuning meters, frequency analyzers, MIDI pitch numbers, and the piano keyboard itself all use logarithmic pitch representation. The logarithmic scale formalizes what musical notation does visually.

# Examples

**Example 1** (p. 68): Linear axis: A2=110 Hz, A3=220 Hz, A4=440 Hz, A5=880 Hz have unequal spacings (110, 220, 440 Hz).

**Example 2** (p. 68): Log_10 axis: the same notes appear at positions ~2.041, ~2.342, ~2.643, ~2.944, which are equally spaced (difference ~0.301 = log_10(2)).

**Example 3** (p. 70): Choosing b = 2: the octave interval log_2(2) = 1, so octaves appear as unit distances.

# Relationships

## Builds Upon
- **Logarithm properties** — L2 is the mathematical guarantee of equal spacing for equal intervals

## Enables
- **Multiplicative-to-additive conversion** — The logarithmic scale is the geometric realization of converting ratios to additive measurements

## Related
- **Pitch and frequency** — The logarithmic scale relates perceived pitch (additive) to physical frequency (multiplicative)

# Common Errors

- **Error**: Plotting pitches on a linear frequency axis and expecting equal intervals to appear equally spaced
  **Correction**: Equal intervals are equally spaced only on a logarithmic axis, not a linear frequency axis

# Common Confusions

- **Confusion**: Believing the logarithmic pitch scale distorts intervals
  **Clarification**: It represents intervals more faithfully than a linear frequency scale; it matches musical perception

- **Confusion**: Thinking the choice of base affects whether equal intervals are equally spaced
  **Clarification**: Any valid base gives equal spacing for equal intervals; the base only affects the unit distance

# Source Reference

Chapter 5: "Logarithms and Musical Intervals," pp. 68-70. See the comparison of linear vs. logarithmic plotting of A2-A5.

# Verification Notes

- Definition source: Direct from Wright, pp. 68-70
- Confidence rationale: High — explicit discussion with numerical examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: linear vs. logarithmic comparison with numerical values, piano keyboard analogy
