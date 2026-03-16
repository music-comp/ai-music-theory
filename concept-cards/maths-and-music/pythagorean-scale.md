---
# === CORE IDENTIFICATION ===
concept: Pythagorean Scale
slug: pythagorean-scale

# === CLASSIFICATION ===
category: tuning-systems
subcategory: pythagorean
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Tuning The Scale To Obtain Rational Intervals"
chapter_number: 12
pdf_page: 152
section: "The Pythagorean Scale"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - Pythagorean tuning
  - 3-limit scale
  - Pythagorean diatonic scale

# === TYPED RELATIONSHIPS ===
prerequisites:
  - just-fifth
  - p-limit-tuning
  - greater-whole-tone
extends: []
related:
  - pythagorean-hemitone
  - pythagorean-major-third
  - pythagorean-chromatic-scale
  - comma-of-pythagoras
contrasts_with:
  - just-intonation-scale
  - mean-tone-scale

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the Pythagorean scale?"
  - "How do I build a Pythagorean scale?"
  - "What distinguishes the Pythagorean scale from the just intonation scale?"
---

# Quick Definition

The Pythagorean scale is a 3-limit tuning system that tunes all diatonic intervals using only the primes 2 and 3, achieved by making all fifths between diatonic notes just fifths (ratio 3:2).

# Core Definition

Wright defines: "This scale, deriving its name from Pythagoras' high regard for the just fifth (ratio 3:2), tunes the scale so that all intervals between scale tones are rational intervals involving only the primes 2 and 3. This means it has 3-limit tuning: all intervals between scale tones have ratios that can be expressed as 2^a * 3^b" (p. 152). The scale arises from tuning each interval in the upward sequence of fifths: 4-hat -> 1-hat -> 5-hat -> 2-hat -> 6-hat -> 3-hat -> 7-hat to be 3:2.

# Prerequisites

- **Just fifth** -- The entire scale is built from iterated just fifths
- **p-limit tuning** -- The Pythagorean scale is classified as 3-limit
- **Greater whole tone** -- The whole-step interval of the Pythagorean scale

# Key Properties

1. 3-limit tuning: all ratios have form 2^a * 3^b
2. All five whole steps are the greater whole tone (9/8 ~ 203.91 cents)
3. Both half steps are the Pythagorean hemitone (256/243 ~ 90.22 cents)
4. Only one size of whole tone (unlike just intonation)
5. Major third is 81/64 ~ 407.82 cents (poor -- ~22 cents sharp of just)
6. Adjacent intervals: 9:8, 9:8, 256:243, 9:8, 9:8, 9:8, 256:243

# Construction / Recognition

## To Build a Pythagorean Scale

1. Start at 4-hat (the subdominant) on the circle of fifths
2. Tune each consecutive fifth 4->1->5->2->6->3->7 as a just fifth (3:2)
3. Bring each note into the same octave by dividing by appropriate powers of 2
4. Example: scale tone 2 = (3/2)^2 / 2 = 9/8
5. Example: scale tone 3 = (3/2)^4 / 4 = 81/64

## Resulting Scale Ratios

| 1-hat | 2-hat | 3-hat | 4-hat | 5-hat | 6-hat | 7-hat | 8-hat |
|---|---|---|---|---|---|---|---|
| 1/1 | 9/8 | 81/64 | 4/3 | 3/2 | 27/16 | 243/128 | 2/1 |

# Context & Application

Named after Pythagoras (c. 540-510 BC), who considered the 3:2 fifth a symbol of universal perfection. The scale gives excellent fifths (just, at 3:2) almost everywhere, but its thirds are poor: the major third at 81/64 is sharp of the just major third by the comma of Didymus (~22 cents), producing audible dissonance in major triads. This was adequate for medieval music based on parallel fifths but became unacceptable as thirds gained importance in the 14th-15th centuries.

# Examples

**Example 1** (p. 153): Scale tone 2 = two just fifths minus one octave: (3/2)^2 / 2 = 9/8.

**Example 2** (p. 153): Scale tone 3 = four just fifths minus two octaves: (3/2)^4 / 4 = 81/64, measured at ~407.82 cents, about 8 cents sharp of the tempered major third.

**Example 3** (p. 154): Adjacent intervals: 9:8, 9:8, 256:243, 9:8, 9:8, 9:8, 256:243.

**Example 4** (p. 154): Pythagorean hemitone: 256/243 = 2^8/3^5 ~ 90.22 cents.

# Relationships

## Builds Upon
- **Just fifth** -- Every diatonic fifth is tuned as 3:2
- **p-limit tuning** -- The Pythagorean scale is the paradigmatic 3-limit system

## Enables
- **Pythagorean chromatic scale** -- Extended by continuing fifths around the circle
- **Pythagorean major third** -- The interval 81/64 that arises from four just fifths

## Related
- **Comma of Pythagoras** -- The discrepancy preventing the circle from closing
- **Greater whole tone** -- The whole-step interval, named "Pythagorean whole tone" for this reason

## Contrasts With
- **Just intonation scale** -- 5-limit tuning that sacrifices uniform whole tones for just thirds
- **Mean-tone scale** -- Shrinks fifths to achieve just major thirds

# Common Errors

- **Error**: Expecting the Pythagorean scale to have just major thirds
  **Correction**: 3-limit tuning cannot produce 5/4; the Pythagorean major third is 81/64, which is ~22 cents sharp

# Common Confusions

- **Confusion**: Thinking the Pythagorean scale has two different whole-tone sizes like just intonation
  **Clarification**: The Pythagorean scale has only one whole-tone size (9/8); having two sizes is specific to 5-limit tuning

- **Confusion**: Thinking the comma of Pythagoras means the Pythagorean scale is flawed
  **Clarification**: The comma is a mathematical necessity that every tuning system must accommodate; the Pythagorean scale places it as one "bad" fifth

# Source Reference

Chapter 12: "Tuning The Scale To Obtain Rational Intervals," pp. 152-155.

# Verification Notes

- Definition source: Direct from p. 152
- Confidence rationale: Explicit definition with complete ratio table and construction
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: complete ratio table, adjacent interval pattern, historical context, Didymus comma relationship
