---
concept: Index Number (Sums) for Inversion
slug: index-number-sums
category: operations
subcategory: inversion properties
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 119
section: "3.3 Common Tones Under Inversion (In)"
extraction_confidence: high
aliases:
  - "index number for common tones"
  - "pairwise sums"
prerequisites:
  - inversion
  - index-number
  - mod-12-arithmetic
extends:
  - index-number
related:
  - common-tones-under-inversion
  - addition-table
contrasts_with:
  - interval-class
answers_questions:
  - "How do index numbers relate to common tones under inversion?"
  - "Why do we use sums for inversion instead of differences?"
---

# Quick Definition
The use of pairwise sums (mod 12) of pitch classes within a set to determine common tones under inversion; the sum of any pair of notes equals the index number n such that In maps those notes onto each other.

# Core Definition
When determining common tones under transposition, one considers intervals (differences) between notes. For common tones under inversion, one considers index numbers (sums). The sum of any pair of notes x + y (mod 12) = n identifies In as the inversion that maps x onto y and y onto x. If x and y are both in the set, they become common tones at In (mapped onto each other). If x + x = n (mod 12), then x maps onto itself and is a single common tone at In (Straus, pp. 119-120).

# Prerequisites
- **Inversion (In)** -- the operation producing common tones
- **Index number** -- the general concept of sums under inversion
- **Mod-12 arithmetic** -- sums are computed modulo 12

# Key Properties
1. Sum of two different notes = index number mapping them onto each other
2. Each pair-sum generates 2 common tones at that In
3. Each self-sum (2x mod 12) generates 1 common tone at that In
4. The method uses sums, contrasting with the difference-based method for transposition
5. All possible sums can be systematically computed via an addition table

# Construction / Recognition
For set {a, b, c, d}:
1. Compute all distinct pairwise sums: a+b, a+c, a+d, b+c, b+d, c+d (mod 12)
2. Each sum n: both members of the pair are common tones at In
3. Compute all self-sums: 2a, 2b, 2c, 2d (mod 12)
4. Each self-sum n: that note alone is a common tone at In

# Context & Application
The sum-based approach is fundamental to understanding inversion in post-tonal theory. While transposition preserves intervals (differences), inversion preserves sums. This distinction is the key conceptual difference between analyzing common tones under T versus under I.

# Examples
**Example 1** (p. 120, Ex. 3-10): [1, 3, 6, 9]:
- 1 + 3 = 4: both common tones at I4
- 1 + 6 = 7: both common tones at I7
- 3 + 9 = 0: both common tones at I0
- 1 + 1 = 2: 1 is a common tone at I2
- 6 + 6 = 0: 6 is a common tone at I0 (so I0 has 3 total: notes 3, 6, 9)

# Relationships
## Builds Upon
- **Index number** -- the general concept applied here for common-tone calculation

## Enables
- **Addition table** -- the systematic computation of all sums
- **Common tones under inversion** -- the primary application

## Contrasts With
- **Interval class** -- differences for transposition vs. sums for inversion

# Common Errors
- Confusing sums (for inversion) with differences (for transposition)
- Forgetting to compute self-sums (a + a)

# Common Confusions
- Two different notes summing to n generate 2 common tones; a note summing with itself generates 1
- The sum method has no direct analog of the "interval-class vector shortcut" available for transposition

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.3, pp. 119-121

# Verification Notes
New card extracted from source. The concept of using sums vs. differences was distributed across common-tones-under-inversion and addition-table cards; now given its own focused card.
