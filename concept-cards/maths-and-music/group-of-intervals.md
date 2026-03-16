---
# === CORE IDENTIFICATION ===
concept: Group of Intervals
slug: group-of-intervals

# === CLASSIFICATION ===
category: algebra-in-music
subcategory: groups
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "The Group of Intervals"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - interval group

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
  - isomorphism
extends:
  - group
related:
  - group-of-modular-intervals
  - interval-group-isomorphisms
  - homomorphism
contrasts_with:
  - group-of-modular-intervals

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do musical intervals form a group?"
  - "What is the relationship between additive and multiplicative interval measurements as groups?"
---

# Quick Definition

The set of all musical intervals forms a group under composition, identified with (R, +) in additive measurement or (R+, *) in multiplicative measurement, with these two representations being isomorphic.

# Core Definition

The set of musical intervals forms a group under interval composition. The identity element is the unison interval and the inverse of an interval is its opposite. Using additive measurement (semitones, cents), the group is (R, +). Using multiplicative measurement (frequency ratios), the group is (R+, *). The isomorphisms f(r) = b^r and g(x) = log_b(x) convert between these representations (Wright, p. 89).

# Prerequisites

- **Group** — The interval set satisfies the group axioms
- **Isomorphism** — The two representations are isomorphic

# Key Properties

1. Identity element: unison (0 additively, 1 multiplicatively)
2. Inverse: opposite interval (-r additively, 1/x multiplicatively)
3. The group is commutative (abelian)
4. (R, +) and (R+, *) are isomorphic via exponential and logarithm
5. The group is NOT cyclic (unlike Z_12)

# Construction / Recognition

## To Verify the Group Structure
1. Identity: the unison interval composed with any interval yields that interval
2. Inverses: every interval has an opposite (going up = going down by the same amount)
3. Associativity: grouping of interval compositions does not matter
4. Closure: composing two intervals yields an interval

# Context & Application

This isomorphism is exactly the conversion between cents/semitones and frequency ratios. Adding cents corresponds to multiplying frequency ratios. The group structure captures everyday musical intuition: intervals can be combined, reversed, and the order of combination doesn't matter.

# Examples

**Example 1** (p. 89): Unison: additive identity 0, multiplicative identity 1.

**Example 2** (p. 89): Opposite of a fifth up (700 cents): fifth down (-700 cents), ratio 1/2^(7/12).

**Example 3** (p. 89): Fifth + fourth = octave: 700 + 500 = 1200 cents, or (3/2) * (4/3) = 2 (in just intonation).

# Relationships

## Builds Upon
- **Group** — Musical intervals satisfy the group axioms
- **Isomorphism** — Exponential/logarithm provide the isomorphism between representations

## Enables
- **Group of modular intervals** — Quotienting by octave equivalence
- **Interval group isomorphisms** — The detailed study of the exp/log isomorphism

## Related
- **Homomorphism** — exp and log are group homomorphisms (and isomorphisms)

## Contrasts With
- **Group of modular intervals** — The full interval group is (R, +); the modular version is (R/~, +)

# Common Errors

- **Error**: Treating the additive and multiplicative groups as different groups
  **Correction**: They are two representations of the same abstract group, connected by isomorphism

# Common Confusions

- **Confusion**: Thinking the group of intervals is cyclic like Z_12
  **Clarification**: (R, +) is NOT cyclic; it contains all possible intervals, not just chromatic ones

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," p. 89 (The Group of Intervals section).

# Verification Notes

- Definition source: Direct from Wright, p. 89
- Confidence rationale: High — explicit identification with (R, +) and (R+, *)
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: dual representation, non-cyclic nature, fifth+fourth example
