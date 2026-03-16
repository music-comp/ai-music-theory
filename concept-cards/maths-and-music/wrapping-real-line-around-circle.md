---
concept: Wrapping Real Line Around Circle
slug: wrapping-real-line-around-circle

category: modular-arithmetic
subcategory: chromatic-scales
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Modular Equivalence on the Real Numbers"

extraction_confidence: high

aliases:
  - wrapping function
  - circle parameterization

prerequisites:
  - modular-equivalence-on-the-real-numbers
extends:
  - modular-equivalence-on-the-real-numbers
related:
  - modular-clock
  - homomorphism
  - generalized-division-algorithm
  - octave-equivalence-formalized
contrasts_with: []

answers_questions:
  - "How does the wrapping function model octave equivalence geometrically?"
  - "What is the relationship between the number line and the circle of circumference m?"
---

# Quick Definition

The geometric visualization of modular equivalence as a function that wraps the infinite number line around a circle of circumference m, preserving distance as arc length.

# Core Definition

The wrapping function w: R -> R/~ is defined by w(x) = x-bar (the equivalence class of x modulo m). It maps each real number to its position on a circle of circumference m, with x = 0 placed at the top. The function preserves additive structure: w(x + y) = w(x) + w(y), making it a surjective group homomorphism from (R, +) to (R/~, +). It is onto but not one-to-one (Wright, pp. 85-86).

# Prerequisites

- **Modular equivalence on the real numbers** — The wrapping function maps x to its equivalence class x-bar

# Key Properties

1. w is a surjective group homomorphism
2. w preserves distance locally (as arc length)
3. w identifies points differing by multiples of m
4. For each point on the circle, there is exactly one representative in [0, m)
5. The restriction to Z maps integers to m equally spaced points (the "m-hour clock")
6. R/~ is parameterized by the circle just as R is parameterized by the line

# Construction / Recognition

## To Visualize the Wrapping
1. Draw a circle of circumference m with 0 at the top
2. Map the interval [0, m) onto the circle, preserving arc length
3. Points beyond m wrap around: m maps back to 0, m+1 maps to the same point as 1, etc.
4. Points below 0 wrap counterclockwise: -1 maps to the same point as m-1

# Context & Application

When m = 12 (semitones), wrapping the real line around a circle of circumference 12 identifies all octave-equivalent intervals. The 12 clock positions represent the 12 note classes. The continuous wrapping captures all intervals (including microtonal), while the discrete restriction to Z captures only chromatic intervals.

# Examples

**Example 1** (p. 85): For m = 8: the numbers 0, 8, 16, -8 all wrap to the top of the circle.

**Example 2** (p. 85): For m = 8: 3 and 11 wrap to the same point (since 11 - 3 = 8).

**Example 3** (p. 85): The origin x = 0 is placed at the top of the circle.

**Example 4** (p. 86): Z maps to m equally spaced points, giving the "m-hour clock" representation of Z_m.

# Relationships

## Builds Upon
- **Modular equivalence on the real numbers** — The wrapping function maps to the quotient R/~

## Enables
- **Modular clock** — The discrete version (restriction to Z) gives the clock visualization
- **Group of modular intervals** — The image of the wrapping is the group of modular intervals

## Related
- **Homomorphism** — w is a group homomorphism (example 2 in the homomorphism section)
- **Generalized division algorithm** — Guarantees the unique representative for each point on the circle

# Common Errors

- **Error**: Assuming the circle has radius m
  **Correction**: The circle has circumference m, not radius m or diameter m

# Common Confusions

- **Confusion**: Thinking the wrapping function is an isomorphism
  **Clarification**: w is a homomorphism but NOT an isomorphism; many distinct real numbers map to the same point on the circle

- **Confusion**: Believing wrapping distorts distances
  **Clarification**: Wrapping preserves distance locally as arc length; it only identifies points that differ by multiples of m

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 85-86 (Modular Equivalence on the Real Numbers section). See the diagram of the wrapping function for m = 8.

# Verification Notes

- Definition source: Direct from Wright, pp. 85-86
- Confidence rationale: High — explicit definition with diagram
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: m=8 examples, circumference clarification, continuous vs. discrete distinction
