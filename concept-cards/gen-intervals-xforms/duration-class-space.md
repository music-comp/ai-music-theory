---
concept: Duration-Class Space
slug: duration-class-space

category: generalized-interval-systems
subcategory: musical-spaces
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "2.2.4"

extraction_confidence: high

aliases:
  - "Example 2.2.4"
  - modular duration space

prerequisites:
  - group
  - congruence
  - quotient-group
  - duration-proportion-space
extends:
  - duration-proportion-space
related:
  - pitch-class-space
  - generalized-interval-system
contrasts_with:
  - duration-proportion-space

answers_questions:
  - "How do I construct a GIS from a musical space?"
---

# Quick Definition

Duration-class space is a GIS that reduces duration proportions by a modulus M, so durations differing by powers of M are considered equivalent -- the rhythmic analog of pitch-class space.

# Core Definition

"We reduce the system of 2.2.3 by a durational modulus M greater than 1. Two durations are conceived as equivalent if one is some integral power of M times the other. This leads us to a modular musical space whose elements are duration-classes (equivalence classes of durations)" (Lewin, Example 2.2.4, p. 53). The intervals become ratio-classes (congruence classes of ratios under the same modular reduction). The reduction is mathematically identical to the reduction from pitches/ratios to pitch-classes/intervals-modulo-the-octave.

# Prerequisites

- **Group** — the quotient group of ratio-classes
- **Congruence** — the equivalence is a congruence on the ratio group
- **Quotient Group** — IVLS is a quotient group
- **Duration Proportion Space** — this is its modular reduction

# Key Properties

1. Equivalence: durations s ~ t if t = s * M^n for some integer n
2. S = duration-classes (equivalence classes under M-reduction)
3. IVLS = ratio-classes (quotient group of ratios mod powers of M)
4. With M = 2, this parallels octave equivalence for pitches
5. Both intervals and the space are reduced modularly

# Construction / Recognition

## To Construct:
1. Start with duration proportion space (2.2.3)
2. Choose a modulus M > 1
3. Declare durations equivalent if they differ by powers of M
4. Form equivalence classes of durations and of ratios

## To Recognize:
1. Elements are classes of durations (not individual durations)
2. Intervals are classes of ratios
3. The modulus M determines the equivalence

# Context & Application

With M = 2, durations differing by factors of 2 are equivalent -- a quarter note and a half note belong to the same class. This is "durational octave equivalence." Stockhausen argued the plausibility of this system (M = 2) and its connection to traditional pitch systems. The formal construction uses the quotient GIS technique developed in Chapter 3.

# Examples

**Example 1** (p. 53): With M = 2: class r = {..., 5/32, 5/16, 5/8, 5/4, 5/2, 5, 10, 20, ...}, class s = {..., 1/12, 1/6, 1/3, 2/3, 4/3, 8/3, ...}, class t = {..., 7/20, 7/10, 7/5, 14/5, ...}.

**Example 2** (p. 53): int(r, s) = ratio-class containing 16/15 = {(2^n)(16/15) : n in Z}. int(s, t) = ratio-class containing 21/20.

**Example 3** (p. 53): Irrational example: class u = {..., sqrt(2)/4, sqrt(2)/2, sqrt(2), 2*sqrt(2), ...}. int(s, u) = ratio-class containing 3*sqrt(2)/8.

# Relationships

## Builds Upon
- **Duration Proportion Space** — this is its modular reduction
- **Quotient Group** — IVLS is a quotient group of ratios

## Related
- **Pitch-Class Space** — the tonal analog (reducing pitches mod octave)

## Contrasts With
- **Duration Proportion Space** — unreduced vs. modularly reduced

# Common Errors

- **Error**: Confusing individual durations with duration-classes.
  **Correction**: A duration-class contains infinitely many durations differing by powers of M.

# Common Confusions

- **Confusion**: Assuming M must equal 2.
  **Clarification**: Any modulus M > 1 is valid. M = 2 is standard but not required.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.4, Section 2.4, pp. 53-54.

# Verification Notes

- Definition source: direct from Example 2.2.4
- Confidence rationale: explicit example with detailed classes and intervals
- Re-extracted from v2 card; preserved: Stockhausen reference, irrational example, specific duration classes
