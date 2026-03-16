---
concept: Modular Equivalence on the Integers
slug: modular-equivalence-on-the-integers

category: modular-arithmetic
subcategory: chromatic-scales
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Modular Equivalence on the Integers"

extraction_confidence: high

aliases:
  - congruence modulo m
  - integer modular equivalence

prerequisites:
  - modular-equivalence-on-the-real-numbers
extends:
  - modular-equivalence-on-the-real-numbers
related:
  - modular-integers
  - modular-arithmetic
  - octave-equivalence-formalized
  - modular-chromatic-intervals
contrasts_with:
  - modular-equivalence-on-the-real-numbers

answers_questions:
  - "What is modular equivalence on the integers?"
  - "How does modular equivalence partition Z into equivalence classes?"
---

# Quick Definition

The restriction of modular equivalence to the integers, where two integers are equivalent modulo m when their difference is divisible by m, forming the foundation of chromatic interval arithmetic.

# Core Definition

For a fixed positive integer m, integers k and l satisfy k = l (mod m) if and only if m | (k - l) in Z. The equivalence class of k is denoted [k], and [k] = [l] iff m divides k - l. The set of equivalence classes Z_m = {[0], [1], ..., [m-1]} has exactly m elements. Elements of Z_m are called modular integers (Wright, pp. 86-87).

# Prerequisites

- **Modular equivalence on the real numbers** — Integer modular equivalence is the restriction of ~ from R to Z

# Key Properties

1. k = l (mod m) iff m | (k - l)
2. Z_m has exactly m elements: [0], [1], ..., [m-1]
3. Z_m is a subset of R/~
4. If x ~ y and x in Z, then y in Z (the relation restricts consistently)
5. The notation [k] does not reference m; the modulus must be clear from context
6. Z_m is visualized as m equally spaced points on the circle (an "m-hour clock")

# Construction / Recognition

## To Determine if k = l (mod m)
1. Compute k - l
2. Check if m divides k - l (i.e., (k - l)/m is an integer)
3. If yes, [k] = [l] in Z_m

# Context & Application

When m = 12, the equivalence k = l (mod 12) captures octave equivalence for chromatic intervals: two intervals in semitones are octave-equivalent if they differ by a multiple of 12. The 12 elements of Z_12 correspond to the 12 note classes.

# Examples

**Example 1** (p. 86): 5 = 19 (mod 7), so [5] = [19] in Z_7, since 19 - 5 = 14 = 2 * 7.

**Example 2** (implied): In Z_12: [14] = [2], since 14 - 2 = 12. So 14 semitones is octave-equivalent to 2 semitones (a whole step).

**Example 3** (p. 86): The "clock" visualization: Z_8 has 8 positions around a circle, with [0] at the top.

# Relationships

## Builds Upon
- **Modular equivalence on the real numbers** — Integer equivalence is the restriction to Z

## Enables
- **Modular integers** — Z_m is the set of equivalence classes
- **Modular arithmetic** — Operations on Z_m

## Related
- **Octave equivalence formalized** — The case m = 12
- **Modular chromatic intervals** — Z_12 represents chromatic interval classes

## Contrasts With
- **Modular equivalence on the real numbers** — The real version has uncountably many classes parameterized by the circle; the integer version has exactly m classes

# Common Errors

- **Error**: Writing [k] = [l] when k - l is not a multiple of m
  **Correction**: [k] = [l] requires m | (k - l); for example, [5] != [8] in Z_12 since 8 - 5 = 3, and 12 does not divide 3

# Common Confusions

- **Confusion**: Thinking [k] = [l] means k = l
  **Clarification**: [k] = [l] means k and l differ by a multiple of m, not that they are equal as integers

- **Confusion**: Assuming the notation [k] specifies the modulus
  **Clarification**: The bracket notation does not indicate m; the modulus must always be established from context

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 86-87 (Modular Equivalence on the Integers section).

# Verification Notes

- Definition source: Direct from Wright, pp. 86-87
- Confidence rationale: High — explicit definition with examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Z_7 example, clock visualization, notation ambiguity warning
