---
concept: Isomorphism
slug: isomorphism

category: mathematical-foundations
subcategory: algebraic-structures
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.11.2-1.11.3"

extraction_confidence: high

aliases: []

prerequisites:
  - homomorphism
  - one-to-one-function
  - onto-function
extends:
  - homomorphism
related:
  - anti-homomorphism
  - quotient-group
contrasts_with:
  - anti-homomorphism

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

An isomorphism is a 1-to-1 onto homomorphism between semigroups -- a bijective structure-preserving map that establishes two algebraic structures as "essentially the same."

# Core Definition

"A homomorphism is an isomorphism (into) if it is 1-to-1. If f is an isomorphism of (X, BIN) onto (X', BIN'), we say the two semigroups are isomorphic (via f). In that case the inverse map f^(-1) is an isomorphism of (X', BIN') onto (X, BIN)" (Lewin, Definition 1.11.2, p. 41). Furthermore, any homomorphic image of a semigroup is isomorphic to some quotient semigroup: "it suffices to study the possible congruence relations on (X, BIN), in order to know all possible homomorphisms" (Section 1.11.3).

# Prerequisites

- **Homomorphism** — an isomorphism is a special homomorphism
- **One-to-One Function** — must be injective
- **Onto Function** — must be surjective

# Key Properties

1. Bijective: both 1-to-1 and onto
2. Structure-preserving: f(x1x2) = f(x1)f(x2)
3. The inverse f^(-1) is also an isomorphism
4. Isomorphic structures have identical algebraic properties
5. Every homomorphic image is isomorphic to a quotient (Section 1.11.3)

# Construction / Recognition

## To Construct:
1. Define a function between two semigroups
2. Verify it is 1-to-1, onto, and a homomorphism

## To Recognize:
1. Check bijectivity (1-to-1 and onto)
2. Check structure preservation: f(xy) = f(x)f(y)

# Context & Application

Isomorphic groups have the same abstract structure. The transposition group {T0, ..., T11} is isomorphic to (Z12, +): Tn maps to n, and TmTn = Tm+n corresponds to m + n. This means any theorem about Z12 applies to transpositions. Recognizing isomorphisms enables transferring results between musical domains.

# Examples

**Example 1** (p. 41): The transposition group {T0, ..., T11} under composition is isomorphic to (Z12, +). The map Tn -> n is the isomorphism.

**Example 2** (Section 1.11.3, p. 41): Any homomorphic image (X', BIN') of (X, BIN) is isomorphic to the quotient semigroup (X, BIN)/CONG, where CONG is the congruence induced by the homomorphism.

**Example 3** (Section 1.11.4, p. 42): The map i -> Pi (interval-preserving operations) is an isomorphism, while i -> Ti (transpositions) is an anti-isomorphism.

# Relationships

## Builds Upon
- **Homomorphism** — an isomorphism is a bijective homomorphism

## Enables
- **Quotient Group** — every homomorphic image is isomorphic to a quotient

## Contrasts With
- **Anti-Homomorphism** — an anti-isomorphism reverses the operation order

# Common Errors

- **Error**: Assuming a homomorphism is automatically an isomorphism.
  **Correction**: An isomorphism requires being both 1-to-1 AND onto in addition to being a homomorphism.

# Common Confusions

- **Confusion**: Thinking isomorphic structures are "the same."
  **Clarification**: Isomorphic structures are algebraically identical but may have different elements. (Z12, +) and {T0,...,T11} have different elements but the same algebraic behavior.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.11.2, Section 1.11.3, pp. 41-42.

# Verification Notes

- Definition source: direct from Definition 1.11.2
- Confidence rationale: explicit definition with deep structural result (1.11.3)
- Re-extracted from v2 card; preserved: transposition/Z12 example, quotient isomorphism theorem, P vs T distinction
