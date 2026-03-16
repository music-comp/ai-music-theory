---
concept: "Inversion (Twelve-Tone)"
slug: inversion-twelve-tone
category: twelve-tone
subcategory: operations
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Twelve-Tone Music"
chapter_number: 6
pdf_page: 314
section: "6.2.5 Inversion"
extraction_confidence: high
aliases:
  - "I-form"
  - "inverted ordering"
  - "serial inversion"
prerequisites:
  - twelve-tone-series
  - prime-ordering
  - inversion
extends:
  - inversion
related:
  - retrograde
  - retrograde-inversion
  - index-number-twelve-tone
  - invariants
contrasts_with:
  - prime-ordering
answers_questions:
  - "What is the inversion of a twelve-tone series?"
  - "How does inversion affect the interval succession?"
---

# Quick Definition
The inversion (I) of a series inverts each pitch class (0 to 0, 1 to 11, 2 to 10, etc.), producing an interval succession where each interval is replaced by its complement mod 12 while maintaining the same order.

# Core Definition
"The *inversion* of the series involves inverting each pitch class in the series: pitch-class 0 inverts to 0, 1 inverts to 11, 2 inverts to 10, 3 inverts to 9, and so on" (Straus, p. 314). The interval succession is "the same as that of the prime ordering, but each interval is replaced by its complement mod 12." The intervals are "the same as in the retrograde, but in reverse order" (p. 315). I_n is the inverted ordering beginning on pitch-class n. Note that "we are now using I_n as the name for a series, whereas previously we used it only as an operation" (p. 314).

# Prerequisites
- **Prime ordering** -- I is defined relative to P
- **Inversion (general)** -- the underlying pitch-class operation

# Key Properties
1. I_n begins on pitch-class n
2. Each pitch class x in P_0 becomes (n - x) mod 12 in I_n
3. Intervals: same order as P, each replaced by its complement mod 12
4. Series related by inversion (P and I, R and RI) have "complementary intervals in the same order" (p. 317)
5. I_n serves dual notation: both as operation and as series name

# Construction / Recognition
- Invert each pitch class of P_0 to get I_0
- Transpose I_0 to start on pitch-class n to get I_n
- Verify: intervals are complements of P's intervals in the same order

# Context & Application
Inversion is fundamental to twelve-tone practice. It creates the basis for combinatoriality (pairing P and I forms to produce aggregates), invariance (segmental subsets preserved under inversion), and inversional symmetry. Schoenberg, Webern, and Berg all exploited inversional relationships extensively.

# Examples
**Example 1** (p. 314, Ex. 6-5): Schoenberg, String Quartet No. 4 -- I7 begins on pitch-class 7. Its interval succession shows each P interval replaced by its complement: where P has 11, I has 1; where P has 8, I has 4.

**Example 2** (p. 321): Webern, "Wie bin ich froh!" -- uses P7 and I7 (among the four forms P7, R7, I7, RI7).

# Relationships
## Builds Upon
- **Inversion** -- extends the general pitch-class operation to ordered series

## Enables
- **Retrograde-inversion** -- RI is the retrograde of I
- **Hexachordal combinatoriality** -- typically pairs P and I forms
- **Invariants** -- inversionally related series share subsets

## Related
- **Index number (twelve-tone)** -- the sum that relates P_n and I_m

# Common Errors
- Confusing the operation I_n (mapping pitch classes) with the series name I_n (a specific ordering)
- Expecting pitch inversion to produce contour inversion (it may or may not)

# Common Confusions
- **I as operation vs. I as series name**: Context clarifies; as an operation, I_n maps x to n - x; as a series name, I_n is the inverted ordering starting on pc n
- **Complementary intervals**: Inversion does not reverse the interval order (that is retrograde); it replaces each interval with its complement

# Source Reference
Chapter 6, Section 6.2.5, pp. 314--315

# Verification Notes
Preserved from old card: dual notation issue, interval complementation. Added: v3 template, direct quotations, relationship to retrograde intervals.
