---
# === CORE IDENTIFICATION ===
concept: Quotient Group
slug: quotient-group

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
section: "1.10.3, 1.12.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - quotient semigroup

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
  - congruence
extends:
  - quotient-set
related:
  - homomorphism
  - natural-map
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

A quotient group is formed by taking a group and "dividing out" by a congruence -- the congruence classes become elements of a new group with an induced operation.

# Core Definition

"Let CONG be a congruence on the semigroup (X, BIN). Then the quotient family X/CONG becomes a semigroup itself under the binary composition BIN/CONG defined as follows. Given congruence classes C1 and C2, the composition (BIN/CONG)(C1, C2) is the congruence class C3 of Theorem 1.10.2, that is the unique congruence class which contains BIN(x1, x2) whenever x1 belongs to C1 and x2 belongs to C2" (Lewin, Theorem 1.10.3, p. 40). "Any quotient semigroup of a group is a group" (Theorem 1.12.3, p. 42).

# Prerequisites

- **Group** — the original structure being quotiented
- **Congruence** — the equivalence relation must be a congruence

# Key Properties

1. Elements of the quotient are congruence classes C(x)
2. Operation: C(x1)C(x2) = C(x1x2) is well-defined by the congruence property
3. Identity: C(e), where e is the identity of the original group
4. Inverses: C(x)^(-1) = C(x^(-1))
5. The natural map C is a homomorphism onto the quotient

# Construction / Recognition

## To Construct:
1. Start with a group (X, BIN) and a congruence CONG
2. Form the congruence classes
3. Define the operation on classes: C(x1)C(x2) = C(x1x2)
4. The result is a group

## To Recognize:
1. Elements are equivalence classes of a group
2. The class operation is induced by the group operation
3. The natural map is a homomorphism

# Context & Application

The integers mod 12 form the quotient group Z/12Z, modeling pitch-class intervals. When we work with pitch classes instead of pitches, we are implicitly working in a quotient structure. Duration-classes mod M also form quotient groups. The quotient construction is one of two main ways to derive new groups from old (the other being direct products).

# Examples

**Example 1** (1.10.4.1, p. 39): Integers under addition modulo 12. Classes C(0) through C(11). Operation: C(5) + C(8) = C(13) = C(1). Identity: C(0). Inverse of C(5) is C(7) since C(5) + C(7) = C(12) = C(0).

**Example 2** (1.10.4.2, p. 39): Frequency ratios modulo powers of 2 give pitch-class intervals in just intonation. Each congruence class consists of one interval give or take any number of octaves.

# Relationships

## Builds Upon
- **Congruence** — the quotient requires a congruence
- **Group** — the original structure

## Enables
- **Generalized Interval System** — many GIS interval groups arise as quotient groups

## Related
- **Homomorphism** — the natural map to the quotient is a homomorphism
- **Natural Map** — projects elements to their classes

# Common Errors

- **Error**: Attempting to form a quotient without verifying the congruence property.
  **Correction**: The equivalence relation must be a congruence for the quotient operation to be well-defined.

# Common Confusions

- **Confusion**: Thinking the quotient of a group might not be a group.
  **Clarification**: Theorem 1.12.3 guarantees that any quotient semigroup of a group is itself a group.

# Source Reference

Chapter 1: Mathematical Preliminaries, Theorems 1.10.2-1.10.3, 1.12.3, pp. 39-42.

# Verification Notes

- Definition source: direct from Theorems 1.10.2-1.10.3 and 1.12.3
- Confidence rationale: explicit theorems with proofs
- Re-extracted from v2 card; preserved: Z12 example with arithmetic, just intonation example
