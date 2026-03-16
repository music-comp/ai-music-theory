---
concept: Modular Integers
slug: modular-integers

category: modular-arithmetic
subcategory: chromatic-scales
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Modular Equivalence on the Integers"

extraction_confidence: high

aliases:
  - Z_m
  - integers mod m

prerequisites:
  - modular-equivalence-on-the-integers
extends:
  - modular-equivalence-on-the-integers
related:
  - modular-arithmetic
  - monoid
  - group
  - cyclic-group-and-generator
  - modular-chromatic-intervals
contrasts_with: []

answers_questions:
  - "What are modular integers?"
  - "How does the group Z_12 represent chromatic pitch classes?"
  - "Why is addition well-defined on equivalence classes?"
---

# Quick Definition

The elements of Z_m, the set of equivalence classes of integers modulo m, which form a group under addition with [k] + [l] = [k + l].

# Core Definition

For m in Z+, the modular integers Z_m = {[0], [1], [2], ..., [m-1]} form a group under addition defined by [k] + [l] = [k + l]. This is well-defined: if [k'] = [k] and [l'] = [l], then k' = k + pm and l' = l + qm, so k' + l' = k + l + (p+q)m, giving [k' + l'] = [k + l]. The additive identity is [0] and the additive inverse of [k] is [-k] = [m - k] (Wright, pp. 86-87).

# Prerequisites

- **Modular equivalence on the integers** — Z_m is the set of equivalence classes under this relation

# Key Properties

1. Z_m has exactly m elements
2. Addition is well-defined (independent of representative choice)
3. (Z_m, +) is a commutative group
4. (Z_m, +) is cyclic with generator [1] of order m
5. The identity element is [0]
6. The inverse of [k] is [-k] = [m - k]
7. We write (M, *) to indicate a monoid's operation when context requires clarity

# Construction / Recognition

## To Perform Addition in Z_m
1. Choose representatives: [k] and [l]
2. Add: k + l
3. Reduce modulo m: compute (k + l) mod m = r with 0 <= r < m
4. The result is [r]

# Context & Application

Z_12 is the group of modular chromatic intervals in the standard 12-chromatic scale. Each element represents a note class or interval class. Modular integer arithmetic captures interval composition under octave equivalence. For non-standard scales, Z_n plays the same role with n replacing 12.

# Examples

**Example 1** (p. 87): In Z_12: [7] + [5] = [12] = [0] (fifth + fourth = unison mod octave).

**Example 2** (p. 87): In Z_12: [7] + [7] = [14] = [2] (two fifths = whole step mod octave).

**Example 3** (p. 87): In Z_9: [6] + [13] = [19] = [1] (since 19 = 2 * 9 + 1).

**Example 4** (p. 87): The additive inverse of [5] in Z_12 is [7] (since 5 + 7 = 12 = 0 mod 12).

# Relationships

## Builds Upon
- **Modular equivalence on the integers** — Z_m is the quotient set

## Enables
- **Modular arithmetic** — Operations in Z_m
- **Cyclic group and generator** — Z_m is the prototypical cyclic group
- **Modular chromatic intervals** — Z_12 models chromatic intervals

## Related
- **Monoid** — (Z_m, +) is an example of a commutative monoid (and group)
- **Group** — (Z_m, +) is an example of a group

# Common Errors

- **Error**: Adding representatives without reducing modulo m
  **Correction**: After adding k + l, reduce the result modulo m to get the canonical representative in {0, 1, ..., m-1}

# Common Confusions

- **Confusion**: Thinking Z_m is an infinite set of integers
  **Clarification**: Z_m is a finite set with exactly m elements; each element is an equivalence class containing infinitely many integers

- **Confusion**: Believing [5] in Z_12 is just the number 5
  **Clarification**: [5] is the equivalence class {..., -19, -7, 5, 17, 29, ...}

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 86-87. See the well-definedness proof for addition.

# Verification Notes

- Definition source: Direct from Wright, pp. 86-87, with well-definedness proof
- Confidence rationale: High — explicit definition with proof
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: well-definedness proof, Z_9 example, equivalence class contents illustration
