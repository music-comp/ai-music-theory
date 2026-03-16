---
concept: Comma of Pythagoras
slug: comma-of-pythagoras

category: rational-intervals
subcategory: commas
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
section: "The Comma of Pythagoras"

extraction_confidence: high

aliases:
  - Pythagorean comma
  - ditonic comma

prerequisites:
  - just-fifth
extends: []
related:
  - pythagorean-scale
  - comma-placement-on-circle-of-fifths
  - irrationality-of-equally-tempered-intervals
contrasts_with:
  - comma-of-didymus

answers_questions:
  - "What is the comma of Pythagoras?"
  - "Why can't the circle of fifths be closed with just fifths?"
---

# Quick Definition

The comma of Pythagoras is the small interval 3^12/2^19 (= 531441/524288), approximately 23.46 cents, representing the discrepancy between twelve just fifths and seven octaves.

# Core Definition

The comma of Pythagoras is the rational interval corresponding to the ratio (3/2)^12 / 2^7 = 3^12 / 2^19 = 531441/524288 in Q+. Wright demonstrates: "(3/2)^12 = 3^12/2^12 = 531441/4096 ~ 129.75" while "2^7 = 128." The interval between them is "3^12/2^19 = 531441/524288 ~ 1.01364, which is measured in cents by 1200 * log2(3^12/2^19) ~ 23.46. It is called the comma of Pythagoras" (pp. 144-145).

# Prerequisites

- **Just fifth** -- The comma arises from iterating twelve just fifths

# Key Properties

1. Ratio: 3^12 / 2^19 = 531441/524288
2. Cents: ~23.46
3. Approximately one quarter of a semitone
4. Prime factorization: 2^(-19) * 3^12 (3-limit interval)
5. Measures the overshoot of twelve just fifths beyond seven octaves
6. A consequence of unique factorization: no power of 3 equals any power of 2
7. Equal temperament distributes this comma equally: each tempered fifth is flat by ~1.96 cents (= 23.46/12)

# Construction / Recognition

1. Compute twelve just fifths: (3/2)^12 = 531441/4096 ~ 129.75
2. Compute seven octaves: 2^7 = 128
3. Take the ratio: 531441/4096 / 128 = 531441/524288 ~ 1.01364
4. Convert to cents: 1200 * log2(531441/524288) ~ 23.46

# Context & Application

Pythagoras (c. 540-510 BC) discovered this discrepancy and "found it greatly disturbing" (p. 145). Any tuning system based on just fifths must accommodate this comma somewhere. In the Pythagorean scale, the comma is placed as a "small fifth" between two adjacent positions on the circle of fifths (often between scale degree 7 and flat-5, or between flat-5 and flat-2). The tempered fifth distributes the comma equally, each fifth being flat by 1/12 of the comma. This is the fundamental reason why perfect tuning in all keys is impossible.

# Examples

**Example 1** (p. 144): (3/2)^12 = 531441/4096 ~ 129.75 versus 2^7 = 128.

**Example 2** (p. 145): 531441/524288 ~ 1.01364, or ~23.46 cents.

**Example 3** (p. 145): The tempered fifth distributes this evenly: 12 * 700 = 8400 = 7 * 1200 exactly, while 12 * 701.96 = 8423.46, overshooting by ~23.46 cents.

**Example 4** (p. 145): On the 7-octave circle of just fifths, the twelve intervals wrap around and end up clockwise of the starting position by exactly the comma.

# Relationships

## Builds Upon
- **Just fifth** -- The comma is the accumulation of twelve just fifths' deviation from seven octaves

## Enables
- **Pythagorean scale** -- Must accommodate the comma somewhere on the circle
- **Comma placement on circle of fifths** -- The practical question of where to put the discrepancy

## Related
- **Irrationality of equally tempered intervals** -- Equal temperament's solution: distribute the comma equally

## Contrasts With
- **Comma of Didymus** -- A different comma (~21.51 cents) measuring the 3-limit vs. 5-limit gap; involves only primes 2, 3, and 5

# Common Errors

- **Error**: Confusing the comma of Pythagoras (~23.46 cents) with the comma of Didymus (~21.51 cents)
  **Correction**: Though similar in size, they measure different tuning discrepancies and have different prime factorizations

# Common Confusions

- **Confusion**: Thinking the comma means Pythagorean tuning is "wrong"
  **Clarification**: The comma is a mathematical necessity -- no power of 3 equals any power of 2, so just fifths are inherently incompatible with octave closure

- **Confusion**: Assuming equal temperament "eliminates" the comma
  **Clarification**: Equal temperament distributes the comma equally, making each fifth ~1.96 cents flat; the total discrepancy is preserved but shared

# Source Reference

Chapter 11: "The Rational Numbers As Musical Intervals," pp. 144-145.

# Verification Notes

- Definition source: Direct from pp. 144-145
- Confidence rationale: Explicitly named and defined with full calculation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all calculations, circle diagram description, equal temperament distribution
