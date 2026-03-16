---
concept: Generalized Division Algorithm
slug: generalized-division-algorithm

category: algebra-in-music
subcategory: groups
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Generalized Division Algorithm"

extraction_confidence: high

aliases:
  - extended division algorithm

prerequisites:
  - well-ordering-principle-variations
extends:
  - division-algorithm
related:
  - modular-equivalence-on-the-real-numbers
  - wrapping-real-line-around-circle
contrasts_with:
  - division-algorithm

answers_questions:
  - "How does the Generalized Division Algorithm extend the standard one?"
  - "Why does every equivalence class modulo m have a unique representative in [0, m)?"
---

# Quick Definition

An extension of the Division Algorithm that allows the dividend to be any real number (not just an integer), guaranteeing a unique decomposition x = qm + r with q in Z and 0 <= r < m.

# Core Definition

Given m in Z+ and x in R, there exist unique q in Z and r in R with 0 <= r < m such that x = qm + r. The proof uses WOP.4: the set S = {l in Z | l*m <= x} has upper bound x/m, so it has a largest element q. Then qm <= x < (q+1)m, and setting r = x - qm gives 0 <= r < m. The divisor m can in fact be any positive real number, not just a positive integer (Wright, pp. 83-84).

# Prerequisites

- **Well-ordering principle variations** — WOP.4 is used in the proof

# Key Properties

1. The dividend x can be any real number, not just an integer
2. The quotient q is always an integer
3. The remainder r is a real number with 0 <= r < m
4. Both q and r are uniquely determined by x and m
5. When x is an integer and m is a positive integer, this reduces to the standard Division Algorithm

# Construction / Recognition

## To Compute q and r
1. Given x in R and m in Z+
2. Compute x/m
3. Let q = floor(x/m) (the largest integer <= x/m)
4. Set r = x - qm
5. Verify 0 <= r < m

# Context & Application

The Generalized Division Algorithm guarantees that every equivalence class in R/~ has exactly one representative in [0, m). This is essential for the wrapping function (mapping R onto the circle of circumference m) and for parameterizing modular equivalence classes. Musically, it ensures any interval has a unique octave-equivalent representative.

# Examples

**Example 1**: m = 8, x = 13.5: 13.5 = 1 * 8 + 5.5, so q = 1, r = 5.5.

**Example 2**: m = 12, x = -7.3: -7.3 = (-1) * 12 + 4.7, so q = -1, r = 4.7.

**Example 3**: m = 12, x = 25: 25 = 2 * 12 + 1, so q = 2, r = 1.

# Relationships

## Builds Upon
- **Well-ordering principle variations** — WOP.4 provides the existence of the largest element in the proof

## Enables
- **Modular equivalence on the real numbers** — Each class has a unique representative in [0, m)
- **Wrapping real line around circle** — The remainder r identifies the position on the circle

## Contrasts With
- **Division algorithm** — The standard version requires x in Z; this generalization allows x in R

# Common Errors

- **Error**: Assuming the remainder r must be an integer
  **Correction**: When x is real, r can be any real number in [0, m); it is an integer only when x is an integer

# Common Confusions

- **Confusion**: Thinking uniqueness is obvious without proof
  **Clarification**: Both existence and uniqueness require proof; the uniqueness proof shows q' = q from the maximality condition

- **Confusion**: Believing the algorithm only works for positive x
  **Clarification**: It works for any real x, including negative values; the quotient q will be negative when x is sufficiently negative

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 83-84 (Generalized Division Algorithm section). See the complete proof using WOP.4.

# Verification Notes

- Definition source: Direct from Wright, pp. 83-84, with complete proof
- Confidence rationale: High — theorem stated and proved
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: footnote about m being any positive real, all three numerical examples
