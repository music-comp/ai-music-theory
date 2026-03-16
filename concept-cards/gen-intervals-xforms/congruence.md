---
concept: Congruence
slug: congruence

category: mathematical-foundations
subcategory: algebraic-structures
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.10.1-1.10.4"

extraction_confidence: high

aliases: []

prerequisites:
  - equivalence-relation
  - semigroup
extends:
  - equivalence-relation
related:
  - quotient-group
  - homomorphism
  - natural-map
contrasts_with: []

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

A congruence is an equivalence relation on a semigroup that respects the algebraic structure: if x1 is equivalent to y1 and x2 is equivalent to y2, then x1x2 is equivalent to y1y2.

# Core Definition

"An equivalence relation on a semigroup is a congruence if it has this property: Given x1 equivalent to y1 and x2 equivalent to y2, then x1x2 is equivalent to y1y2" (Lewin, Definition 1.10.1, p. 39). This compatibility ensures that the quotient X/CONG inherits a well-defined semigroup structure (Theorems 1.10.2-1.10.3). For congruence classes C1 and C2, there is a unique class C3 such that x1x2 is in C3 whenever x1 is in C1 and x2 is in C2.

# Prerequisites

- **Equivalence Relation** — a congruence is a special type of equivalence relation
- **Semigroup** — congruences are defined on semigroups

# Key Properties

1. Reflexive, symmetric, transitive (equivalence relation properties)
2. Compatible with the operation: x1 ~ y1 and x2 ~ y2 implies x1x2 ~ y1y2
3. The quotient X/CONG is a semigroup under the induced operation
4. The natural map C: X -> X/CONG is a homomorphism
5. Any quotient semigroup of a group is itself a group (Theorem 1.12.3)

# Construction / Recognition

## To Construct:
1. Define an equivalence relation on a semigroup
2. Verify compatibility: if x1 ~ y1 and x2 ~ y2, then x1x2 ~ y1y2

## To Recognize:
1. Verify it is an equivalence relation
2. Verify the compatibility condition with the semigroup operation

# Context & Application

Congruence modulo 12 on integers gives the pitch-class interval group Z12. Congruence modulo powers of 2 on frequency ratios gives pitch-class intervals in just intonation. Congruences allow working with "reduced" interval systems where certain distinctions (like octaves) are collapsed.

# Examples

**Example 1** (1.10.4.1, p. 39): On integers under addition, define (x, y) congruent if y - x is a multiple of 12. The quotient is "integers mod 12" with 12 classes C(0) through C(11). Addition: C(5) + C(8) = C(1) since 5 + 8 = 13 = 1 mod 12.

**Example 2** (1.10.4.2, p. 39): On rational numbers 2^a * 3^b * 5^c under multiplication, define (x, y) congruent if y = x * 2^n for some integer n. The quotient models pitch-class intervals in just intonation (intervals modulo the octave).

# Relationships

## Builds Upon
- **Equivalence Relation** — a congruence is an equivalence relation with extra structure
- **Semigroup** — congruences are defined on semigroups

## Enables
- **Quotient Group** — the quotient of a group by a congruence is a group
- **Homomorphism** — the natural map to the quotient is a homomorphism

# Common Errors

- **Error**: Defining a quotient without checking the congruence property.
  **Correction**: Not every equivalence relation on a semigroup is a congruence. The compatibility condition must be verified.

# Common Confusions

- **Confusion**: Equating "congruence" with "congruence mod n."
  **Clarification**: "Congruence mod n" is a specific example. The general concept applies to any semigroup with any compatible equivalence relation.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definitions 1.10.1-1.10.3, Examples 1.10.4.1-1.10.4.2, pp. 39-40.

# Verification Notes

- Definition source: direct from Definition 1.10.1
- Confidence rationale: explicit definition with detailed examples and proofs
- Re-extracted from v2 card; preserved: integers mod 12 example, just intonation example, verification of congruence property
