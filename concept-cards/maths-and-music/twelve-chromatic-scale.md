---
concept: Twelve-Chromatic Scale
slug: twelve-chromatic-scale

category: modular-arithmetic
subcategory: twelve-tone
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
section: null

extraction_confidence: high

aliases:
  - 12-tone equal temperament
  - 12-EDO
  - standard chromatic scale

prerequisites:
  - n-chromatic-scale
extends:
  - n-chromatic-scale
related:
  - modular-chromatic-intervals
  - generating-interval
  - twelve-tone-technique
  - z-twelve-as-chromatic-interval-group
contrasts_with:
  - non-standard-chromatic-scales

answers_questions:
  - "What is the twelve-chromatic scale?"
  - "Why does the standard chromatic scale divide the octave into 12 parts?"
---

# Quick Definition

The standard Western chromatic scale, formed by dividing the octave into 12 equal semitone intervals, each measuring 100 cents with ratio 2^(1/12) ~ 1.05946.

# Core Definition

The 12-chromatic scale is the n-chromatic scale with n = 12. Its smallest interval, the semitone, has ratio 2^(1/12) = 12th_root(2) ~ 1.05946 and measures exactly 100 cents. The set of modular chromatic intervals is identified with Z_12 (Wright, p. 74).

# Prerequisites

- **N-chromatic scale** — The 12-chromatic scale is the specific case n = 12

# Key Properties

1. The chromatic unit (semitone) is exactly 100 cents
2. The semitone ratio is 2^(1/12) ~ 1.05946
3. Any keyboard interval is an integer multiple of 100 cents
4. Modular arithmetic in Z_12 captures interval composition under octave equivalence
5. The number of possible orderings of all 12 note classes is 12! = 479,001,600
6. The generating intervals are [1], [5], [7], [11] (those coprime to 12)

# Construction / Recognition

## To Identify Standard Keyboard Intervals
1. Count the number of semitones (100-cent steps) in the interval
2. The semitone count maps directly to an element of Z_12
3. Interval names: 1=semitone, 2=whole step, 3=minor third, 4=major third, 5=fourth, 6=tritone, 7=fifth, 8=minor sixth, 9=major sixth, 10=minor seventh, 11=major seventh, 0=unison/octave

# Context & Application

The subdivision of the octave into 12 equal intervals became standard in Western music only within the last 200 years. It is not universal across musical traditions. Its adoption was influenced by the desire to approximate just intervals (like the 3:2 fifth and 5:4 major third) while enabling free modulation between keys.

# Examples

**Example 1** (p. 74): The semitone (100 cents) is the 12-chromatic unit.

**Example 2** (implied, p. 82): A fourth + a fifth = [5] + [7] = [12] = [0] (unison modulo octave).

**Example 3** (implied): Two fifths = [7] + [7] = [14] = [2] (a whole step modulo octave).

**Example 4** (pp. 75-76): The generating intervals of Z_12 are [1], [5], [7], [11], since gcd(m, 12) = 1 for m = 1, 5, 7, 11.

# Relationships

## Builds Upon
- **N-chromatic scale** — The 12-chromatic scale is the most important special case

## Enables
- **Twelve-tone technique** — Based entirely on the 12-chromatic scale's structure
- **Z_12 as chromatic interval group** — The algebraic formalization of the 12-chromatic scale

## Related
- **Modular chromatic intervals** — The intervals of the 12-chromatic scale form Z_12
- **Generating interval** — The four generators of Z_12 are musically significant intervals

## Contrasts With
- **Non-standard chromatic scales** — Scales with n != 12 produce different interval vocabularies

# Common Errors

- **Error**: Assuming equal temperament and just intonation produce the same intervals
  **Correction**: Equal-tempered fifths are slightly flat (~2 cents) and thirds are slightly sharp (~14 cents) compared to just intervals

# Common Confusions

- **Confusion**: Believing the choice of 12 is purely arbitrary
  **Clarification**: The choice of 12 relates to how well powers of 2^(1/12) approximate simple frequency ratios like 3/2 and 5/4

# Source Reference

Chapter 6: "Chromatic Scales," p. 74. The 12-chromatic scale is the reference case throughout the chapter.

# Verification Notes

- Definition source: Direct from Wright, p. 74
- Confidence rationale: High — explicitly defined as the n = 12 case
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: historical context about 200-year adoption, 12! count, generator list
