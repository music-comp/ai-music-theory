---
concept: N-Chromatic Units
slug: n-chromatic-units

category: pitch-and-intervals
subcategory: measurement
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "Arbitrary Chromatic Units"

extraction_confidence: high

aliases:
  - arbitrary chromatic units
  - n-tone equal temperament

prerequisites:
  - semitone-ratio
extends:
  - semitone-ratio
related:
  - cents
  - microtuning
contrasts_with: []

answers_questions:
  - "What are n-chromatic units?"
  - "How do you generalize equal temperament to n divisions of the octave?"
---

# Quick Definition

An n-chromatic unit is the interval obtained by dividing the octave into n equal parts. The interval of x n-chromatic units has ratio 2^(x/n).

# Core Definition

For a positive integer n, dividing the octave into n equal subintervals produces units called n-chromatic units. Each n-chromatic unit has ratio 2^(1/n). More generally:

The interval of x n-chromatic units has ratio 2^(x/n).    (Formula 4.4)

This generalizes both the semitone formula (n = 12, giving 2^(x/12)) and the cent formula (n = 1200, giving 2^(x/1200)) (Wright, p. 62).

# Prerequisites

- **Semitone Ratio** -- N-chromatic units generalize the semitone; the semitone is the case n = 12

# Key Properties

1. Each n-chromatic unit has ratio 2^(1/n)
2. x units have ratio 2^(x/n) -- Formula 4.4
3. n = 12: standard semitones
4. n = 1200: cents
5. Different values of n yield different equal-tempered tuning systems
6. The formula works for any real x, not just integers

# Construction / Recognition

## To Work with N-Chromatic Units

1. Choose the number of divisions n
2. Each unit has ratio 2^(1/n)
3. To convert x units to a ratio: r = 2^(x/n)
4. To convert a ratio r to units: x = n * log_2(r)

# Context & Application

Different values of n yield different equal-tempered tuning systems. The 12-chromatic scale is standard in Western music, but other divisions have been explored for microtonal music: n = 19, 24, 31, 53, etc. The concept provides a unified framework for analyzing any equal temperament (Wright, p. 62).

# Examples

**Example 1** (p. 62): n = 12: standard semitones, each with ratio 2^(1/12).

**Example 2** (p. 62): n = 1200: cents, each with ratio 2^(1/1200).

**Example 3**: n = 19: 19-tone equal temperament, each unit with ratio 2^(1/19).

# Relationships

## Builds Upon

- **Semitone Ratio** -- N-chromatic units generalize the semitone

## Related

- **Cents** -- Cents are the special case n = 1200
- **Microtuning** -- Alternative values of n enable microtonal tuning systems

# Common Errors

- **Error**: Assuming n must be 12
  **Correction**: n can be any positive integer; 12 is the Western standard but other values are musically valid

# Common Confusions

- **Confusion**: Thinking different values of n produce objectively "better" or "worse" tunings
  **Clarification**: Different values of n have different approximation properties for just intervals; none is universally superior
- **Confusion**: Believing cents are a fundamentally different concept from n-chromatic units
  **Clarification**: Cents are simply n-chromatic units with n = 1200

# Source Reference

Chapter 4: "Ratios and Musical Intervals," p. 62. Formula 4.4.

# Verification Notes

- Definition source: Direct from p. 62
- Confidence rationale: High -- explicitly defined with formula
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: examples of alternative n values, formula 4.4 reference
