---
# === CORE IDENTIFICATION ===
concept: In-the-Cracks Intervals
slug: in-the-cracks-intervals

# === CLASSIFICATION ===
category: pitch-and-intervals
subcategory: approximation
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Integers as Intervals"
chapter_number: 9
pdf_page: 110
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "quarter-tone intervals"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - keyboard-approximation-of-integer-ratios
  - error-calculation-in-cents
extends: []
related:
  - prime-interval-personality
  - overtone-series
contrasts_with:
  - powers-of-two-as-exact-keyboard-intervals

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are in-the-cracks intervals?"
  - "Which integer ratios are most poorly approximated by the keyboard?"
---

# Quick Definition

Integer ratios whose cent values fall nearly halfway between two adjacent semitones, making them poorly approximated by any note on the equally tempered keyboard. The primes 11 and 13 are the most prominent examples, with errors approaching 50 cents (a quarter tone).

# Core Definition

An "in the cracks" interval is one whose cent value has a fractional semitone part close to 0.5, meaning the interval lies approximately a quarter tone from the nearest keyboard notes. Wright describes the interval of 11 as "truly 'in the cracks', lying about a quarter step from the closest tempered scale intervals" (Ch. 9, p. 114).

# Prerequisites

- **Keyboard Approximation of Integer Ratios** -- Understanding the approximation process
- **Error Calculation in Cents** -- The method for computing the error

# Key Properties

1. The theoretical worst case is 50 cents (exactly between two semitones)
2. Prime 11 has ~49 cents error -- very close to the worst case
3. Prime 13 has ~41 cents error -- also very poor
4. These intervals are foreign to the 12-tone equal-tempered system
5. They appear naturally as overtones (11th and 13th harmonics)

# Construction / Recognition

## To identify an in-the-cracks interval:
1. Compute $c = 1200 \log_2 n$
2. Compute the fractional part: $f = |c/100 - \text{round}(c/100)|$
3. If $f$ is close to 0.5, the interval is "in the cracks"
4. The closer $f$ is to 0.5, the worse the approximation

# Context & Application

In-the-cracks intervals cannot be meaningfully represented on a standard keyboard. They exist naturally in the overtone series (as the 11th and 13th harmonics) and can be produced on continuous-pitch instruments (voice, strings, trombone). Some non-Western musical traditions and microtonal systems incorporate these intervals.

# Examples

**Example 1** (p. 114): 11: $1200 \log_2 11 \approx 4151.32$ cents. Lies between 41 semitones (4100 cents, three octaves + fourth) and 42 semitones (4200 cents, three octaves + tritone), about 49 cents from the latter. "Truly in the cracks."

**Example 2** (p. 115): 13: $1200 \log_2 13 \approx 4440.53$ cents. Between 44 semitones (4400 cents) and 45 semitones (4500 cents), about 41 cents flat of the nearest note.

**Example 3**: By contrast, 3 has only ~2 cents error, and even 7 has "only" ~31 cents error.

# Relationships

## Builds Upon
- **Keyboard Approximation of Integer Ratios** -- These are the extreme error cases

## Related
- **Prime Interval Personality** -- Primes 11, 13 have distinctly foreign personalities
- **Overtone Series** -- The 11th and 13th harmonics produce these intervals

## Contrasts With
- **Powers of Two as Exact Keyboard Intervals** -- The opposite extreme: zero error

# Common Errors

- **Error**: Assuming in-the-cracks intervals are musically useless
  **Correction**: They appear naturally in the overtone series and can be produced on continuous-pitch instruments

# Common Confusions

- **Confusion**: Thinking "in the cracks" means the interval is wrong or unnatural
  **Clarification**: It means only that the 12-tone equal-tempered keyboard cannot represent it; the interval itself is perfectly natural as part of the overtone series

# Source Reference

Chapter 9: "The Integers as Intervals," pp. 114-115.

# Verification Notes

- Definition source: Direct quote ("in the cracks") from p. 114
- Confidence rationale: Explicit description with calculations
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: both examples, "quarter step" quote
