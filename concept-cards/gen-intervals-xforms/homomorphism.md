---
# === CORE IDENTIFICATION ===
concept: Homomorphism
slug: homomorphism

# === CLASSIFICATION ===
category: mathematical-foundations
subcategory: algebraic-structures
tier: foundational

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.11.1, 1.12.1-1.12.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - semigroup
extends: []
related:
  - isomorphism
  - anti-homomorphism
  - congruence
  - natural-map
  - quotient-group
contrasts_with:
  - anti-homomorphism

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

A homomorphism is a function between semigroups that preserves the algebraic structure: the image of a product equals the product of the images, f(x1x2) = f(x1)f(x2).

# Core Definition

"A function f from a semigroup (X, BIN) into a semigroup (X', BIN') is a homomorphism if it satisfies the law: BIN'(f(x1), f(x2)) = f(BIN(x1, x2)) for all x1 and all x2 in X. One can express this law colloquially by saying, 'The combination of the images is the image of the combination'" (Lewin, Definition 1.11.1, p. 41). For groups: if e is the identity in X, then f(e) is the identity in X'; if x has inverse x^(-1), then f(x^(-1)) = f(x)^(-1) (Theorem 1.12.1). A homomorphic image of a group is a group (Theorem 1.12.2).

# Prerequisites

- **Semigroup** — homomorphisms map between semigroups

# Key Properties

1. f(x1)f(x2) = f(x1x2) for all x1, x2 (structure preservation)
2. For groups: f(e) = e' (identity maps to identity)
3. For groups: f(x^(-1)) = f(x)^(-1) (inverses map to inverses)
4. Homomorphic image of a group is a group
5. The natural map to a quotient is always a homomorphism: C(x1)C(x2) = C(x1x2)

# Construction / Recognition

## To Construct:
1. Define a function f between two semigroups
2. Verify f(x1x2) = f(x1)f(x2) for all pairs

## To Recognize:
1. Check the fundamental law: does the function preserve the operation?
2. For groups, check identity and inverse preservation as a shortcut

# Context & Application

Homomorphisms formalize structure-preserving relationships between musical systems. The reduction from chromatic pitch intervals to pitch-class intervals (mod 12) is a homomorphism. Any homomorphic image is isomorphic to some quotient semigroup (Section 1.11.3), so studying quotients suffices to understand all homomorphic images.

# Examples

**Example 1** (p. 41): The natural map C from integers to integers mod 12: C(x1) + C(x2) = C(x1 + x2). For instance, C(5) + C(8) = C(13) = C(1).

**Example 2** (Theorem 1.12.1, p. 42): If f is a homomorphism and e is the identity in X, then f(e) is the identity in X'. If x has inverse x^(-1), then f(x^(-1)) is the inverse of f(x).

**Example 3** (Section 1.11.3, p. 41): Any homomorphic image of a semigroup (X, BIN) is isomorphic to some quotient semigroup of (X, BIN).

# Relationships

## Builds Upon
- **Semigroup** — the domain and codomain of a homomorphism

## Enables
- **Isomorphism** — a 1-to-1 onto homomorphism
- **Quotient Group** — the natural map is a homomorphism

## Related
- **Congruence** — every homomorphism induces a congruence via the kernel

## Contrasts With
- **Anti-Homomorphism** — reverses the operation order: f(x1x2) = f(x2)f(x1)

# Common Errors

- **Error**: Assuming a homomorphism must be 1-to-1.
  **Correction**: A homomorphism need not be 1-to-1 (that would make it an isomorphism).

# Common Confusions

- **Confusion**: Confusing homomorphisms with anti-homomorphisms.
  **Clarification**: A homomorphism satisfies f(xy) = f(x)f(y); an anti-homomorphism satisfies f(xy) = f(y)f(x). The difference matters for non-commutative groups.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.11.1, Theorems 1.12.1-1.12.2, pp. 41-42.

# Verification Notes

- Definition source: direct from Definition 1.11.1
- Confidence rationale: explicit definition with multiple theorems
- Re-extracted from v2 card; preserved: natural map example, identity/inverse preservation theorems, quotient isomorphism result
