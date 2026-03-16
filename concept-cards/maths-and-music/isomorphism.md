---
concept: Isomorphism
slug: isomorphism

category: algebra-in-music
subcategory: morphisms
tier: advanced

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Homomorphism"

extraction_confidence: high

aliases:
  - group isomorphism

prerequisites:
  - homomorphism
extends:
  - homomorphism
related:
  - interval-group-isomorphisms
  - group-of-intervals
  - cyclic-group-and-generator
contrasts_with:
  - homomorphism

answers_questions:
  - "What is an isomorphism?"
  - "What does it mean for two groups to be isomorphic?"
---

# Quick Definition

A bijective homomorphism between two groups, establishing that they have identical algebraic structure despite possibly different elements and operations.

# Core Definition

A homomorphism phi: G -> G' is an isomorphism if it is bijective (one-to-one and onto). In this case, the inverse function phi^(-1): G' -> G is also an isomorphism. Two groups G and G' are called isomorphic if there exists an isomorphism between them (Wright, p. 90).

# Prerequisites

- **Homomorphism** — An isomorphism is a bijective homomorphism

# Key Properties

1. phi is one-to-one (injective) and onto (surjective)
2. phi^(-1) is also an isomorphism
3. Isomorphic groups are algebraically indistinguishable
4. Any cyclic group of order m is isomorphic to Z_m
5. Any infinite cyclic group is isomorphic to Z

# Construction / Recognition

## To Verify an Isomorphism
1. Verify phi is a homomorphism: phi(x * y) = phi(x) o phi(y)
2. Verify phi is injective: phi(x) = phi(y) implies x = y
3. Verify phi is surjective: every element of G' is in the image of phi

# Context & Application

The isomorphism (R, +) ~ (R+, *) via f(r) = b^r is the mathematical statement that additive interval measurement (cents, semitones) and multiplicative measurement (frequency ratios) are equivalent. The exponential and logarithm are precisely the conversions between these representations.

# Examples

**Example 1** (p. 90): f(r) = 2^r is an isomorphism from (R, +) to (R+, *).

**Example 2** (p. 90): g(x) = log_2(x) is its inverse isomorphism from (R+, *) to (R, +).

**Example 3** (p. 90): phi: {1, -1} -> Z_2 with phi(1) = [0], phi(-1) = [1] is an isomorphism.

**Example 4** (p. 90): The wrapping function w: R -> R/~ is a homomorphism but NOT an isomorphism (not one-to-one).

# Relationships

## Builds Upon
- **Homomorphism** — An isomorphism adds bijectivity to the homomorphism condition

## Enables
- **Interval group isomorphisms** — The specific isomorphisms relevant to music theory

## Related
- **Group of intervals** — The two interval group representations are isomorphic
- **Cyclic group and generator** — Cyclic groups of the same order are isomorphic

## Contrasts With
- **Homomorphism** — A homomorphism that is not bijective is not an isomorphism

# Common Errors

- **Error**: Assuming any homomorphism between groups of the same size is an isomorphism
  **Correction**: Bijectivity must be verified; a homomorphism can fail to be injective even between groups of the same cardinality (for infinite groups)

# Common Confusions

- **Confusion**: Thinking isomorphic groups must have the same elements
  **Clarification**: Isomorphic groups have identical algebraic properties but may have completely different elements (e.g., real numbers vs. positive reals)

- **Confusion**: Believing the existence of an isomorphism is a property of a specific function
  **Clarification**: Two groups are isomorphic if ANY isomorphism exists between them; the choice of specific isomorphism is not unique

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," p. 90 (Homomorphism section). See the definition and examples.

# Verification Notes

- Definition source: Direct from Wright, p. 90
- Confidence rationale: High — explicit definition
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: inverse isomorphism property, wrapping function counterexample, cyclic group isomorphism theorem
