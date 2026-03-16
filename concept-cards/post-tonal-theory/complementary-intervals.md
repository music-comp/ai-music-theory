---
concept: Complementary Intervals
slug: complementary-intervals

category: intervals
subcategory: interval class
tier: foundational

source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 26
section: "1.8 Ordered Pitch-Class Intervals"

extraction_confidence: high

aliases:
  - complements mod 12
  - complementation mod 12
  - inversionally related intervals

prerequisites:
  - ordered-pitch-class-interval
  - mod-12-arithmetic
extends: []
related:
  - interval-class
  - unordered-pitch-class-interval
contrasts_with: []

answers_questions:
  - "What are complementary intervals?"
  - "What happens when you reverse the order of two pitch classes?"
---

# Quick Definition
Complementary intervals are pairs of ordered pitch-class intervals that sum to 12 (= 0 mod 12), produced when the order of two pitch classes is reversed.

# Core Definition
When you reverse the order of two pitch classes, the resulting ordered pitch-class intervals are each other's complement mod 12, meaning they add up to 12 (or equivalently, 0 mod 12). These complementary pairs are what traditional tonal theory calls "inversions" (e.g., minor thirds and major sixths, perfect fourths and perfect fifths). Each interval class (except ic 0 and ic 6, which are self-complementary) comprises a pair of complementary ordered pitch-class intervals.

# Prerequisites
- **Ordered Pitch-Class Interval** — complementary intervals are defined in terms of opci
- **Mod 12 Arithmetic** — complementation operates mod 12

# Key Properties
1. Complementary pairs sum to 12 (= 0 mod 12)
2. Reversing the order of two pitch classes produces the complement
3. Formula: if opci(x,y) = n, then opci(y,x) = 12 - n
4. Both members of a complementary pair belong to the same interval class
5. ic 6 is self-complementary (6 + 6 = 12)

# Construction / Recognition
## To Construct:
1. Calculate the opci from x to y: n = (y - x) mod 12
2. The complement is 12 - n (or equivalently, (x - y) mod 12)

## To Recognize:
1. Two opci values that sum to 12
2. Two intervals that are "inversions" of each other in traditional terminology

# Context & Application
Understanding complementation is essential for working with interval classes (which group complementary intervals together) and for recognizing inversional relationships. It also plays a central role in the complement relation for pitch-class sets (hexachordal complementation) and in understanding the symmetry of the mod 12 system.

# Examples
**Example 1-13** (p. 27): The pairs of complements mod 12:

| Pair |
|------|
| 0 and 12 (= 0) |
| 1 and 11 (or -1) |
| 2 and 10 (or -2) |
| 3 and 9 (or -3) |
| 4 and 8 (or -4) |
| 5 and 7 (or -5) |
| 6 and 6 |

**From Example 1-12** (p. 26): The opci from C# to Eb is 2; the opci from Eb to C# is 10. These are complements because 2 + 10 = 12.

# Relationships
## Builds Upon
- **Ordered Pitch-Class Interval** — complementation relates pairs of opcis
- **Mod 12 Arithmetic** — the modular framework

## Enables
- **Interval Class** — defined by grouping complementary intervals together
- **Complement Relation** — extends to pitch-class sets

## Related
- **Unordered Pitch-Class Interval** — always takes the smaller of a complementary pair

## Contrasts With
- (no direct contrast within this chapter)

# Common Errors
- **Error**: Thinking complements are the same interval
  **Correction**: Complementary intervals belong to the same interval class but represent opposite directions around the clockface. opci 3 and opci 9 are different ordered intervals, though both are ic 3.

# Common Confusions
- **Confusion**: Confusing interval complementation with set complementation
  **Clarification**: Interval complements are pairs of opcis summing to 12. Set complements are pitch-class sets whose union is the aggregate (all 12 pitch classes). Both use "complement" but in different contexts.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.8, pp. 26-27.

# Verification Notes
- Definition source: direct (Straus Section 1.8 and Example 1-13)
- Confidence rationale: explicit list of complementary pairs with worked examples
- Re-extraction notes: Re-extracted from v2 card; preserved: complement pair table, C#-Eb worked example, distinction from set complementation
