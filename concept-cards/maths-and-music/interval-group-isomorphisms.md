---
concept: Interval Group Isomorphisms
slug: interval-group-isomorphisms

category: algebra-in-music
subcategory: morphisms
tier: advanced

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "The Group of Intervals"

extraction_confidence: high

aliases:
  - exponential-logarithm isomorphism
  - additive-multiplicative interval isomorphism

prerequisites:
  - isomorphism
  - group-of-intervals
extends:
  - isomorphism
related:
  - logarithm-properties
  - exponents-and-exponential-functions
  - multiplicative-to-additive-conversion
contrasts_with: []

answers_questions:
  - "How do the exponential and logarithm provide isomorphisms between interval representations?"
  - "What is the deep mathematical connection between additive and multiplicative interval measurement?"
---

# Quick Definition

The isomorphism between (R, +) and (R+, *) established by the exponential and logarithm functions, which is precisely the mathematical statement that additive and multiplicative interval measurements are equivalent.

# Core Definition

For any base b in R+, the function f: R -> R+ defined by f(r) = b^r and its inverse g: R+ -> R defined by g(x) = log_b(x) are group isomorphisms between (R, +) and (R+, *). They satisfy f(r + s) = f(r) * f(s) (the homomorphism condition for f) and g(xy) = g(x) + g(y) (the homomorphism condition for g). Since both are bijective, they are isomorphisms (Wright, p. 90, example 3).

# Prerequisites

- **Isomorphism** — The concept of bijective homomorphism
- **Group of intervals** — The two group representations being connected

# Key Properties

1. f(r + s) = b^(r+s) = b^r * b^s = f(r) * f(s) (exponential is a homomorphism)
2. g(xy) = log_b(xy) = log_b(x) + log_b(y) = g(x) + g(y) (logarithm is a homomorphism)
3. Both are bijective, hence isomorphisms
4. f and g are inverses: f(g(x)) = x and g(f(r)) = r
5. The base b determines the unit: b = 2 for octaves, b = 2^(1/12) for semitones

# Construction / Recognition

## To Convert Between Representations
1. Additive to multiplicative: apply f(r) = b^r
2. Multiplicative to additive: apply g(x) = log_b(x)
3. Choose base b according to desired unit of additive measurement

# Context & Application

The isomorphism shows that (R, +) and (R+, *) are the same abstract group with different concrete representations. The musical significance is that adding cents corresponds to multiplying frequency ratios: 700 + 500 = 1200 cents maps to 2^(7/12) * 2^(5/12) = 2 (ratio of octave). This is not a coincidence but a structural equivalence.

# Examples

**Example 1** (p. 90): With b = 2: f(1) = 2 (1 octave -> ratio 2), g(3/2) = log_2(3/2) ~ 0.585 octaves.

**Example 2** (p. 90): f maps 0 to 1: f(0) = b^0 = 1 (identity to identity).

**Example 3** (p. 90): Adding 700 + 500 = 1200 cents corresponds to 2^(7/12) * 2^(5/12) = 2^1 = 2.

# Relationships

## Builds Upon
- **Isomorphism** — These are specific isomorphisms between interval groups
- **Group of intervals** — The two representations being connected

## Enables
Understanding that additive and multiplicative interval measurements are not merely convenient but algebraically equivalent.

## Related
- **Logarithm properties** — L1 is the homomorphism condition for g
- **Exponents and exponential functions** — The law b^(r+s) = b^r * b^s is the homomorphism condition for f
- **Multiplicative-to-additive conversion** — The practical application of these isomorphisms

# Common Errors

- **Error**: Thinking the choice of base b changes the abstract group structure
  **Correction**: The base determines the unit of additive measurement but does not change the algebraic structure; all bases give the same isomorphism type

# Common Confusions

- **Confusion**: Thinking the conversion between cents and ratios is "just a formula"
  **Clarification**: It is a deep structural equivalence (isomorphism), not merely a computational convenience

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 89-90 (The Group of Intervals section, Example 3).

# Verification Notes

- Definition source: Direct from Wright, p. 90, example 3
- Confidence rationale: High — explicitly stated as isomorphism with verification
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: base-determines-unit clarification, deep structural equivalence point
