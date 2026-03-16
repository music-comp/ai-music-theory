---
# === CORE IDENTIFICATION ===
concept: Group
slug: group

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
section: "1.7"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - semigroup
  - identity-element
  - inverse-element
extends:
  - semigroup
related:
  - group-of-operations
  - commutativity
  - generalized-interval-system
contrasts_with:
  - semigroup

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a mathematical group?"
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

A group is a semigroup with identity in which every element has an inverse -- the fundamental algebraic structure underlying interval systems and transformation theory.

# Core Definition

"A group is a semigroup with identity in which every element has an inverse" (Lewin, Definition 1.7, p. 36). The abstract definitions of "semigroup" and "group" (1.4.3; 1.7) are consistent with the earlier use of those terms in connection with families of transformations (1.3.2; 1.3.4). A group thus satisfies: (1) closure under an associative binary composition, (2) existence of an identity element, and (3) existence of an inverse for every element.

# Prerequisites

- **Semigroup** — a group is a semigroup with additional properties
- **Identity Element** — a group must have an identity
- **Inverse Element** — every element in a group must have an inverse

# Key Properties

1. Closure: for all a, b in G, ab is in G
2. Associativity: (ab)c = a(bc) for all a, b, c
3. Identity: there exists e such that ea = ae = a for all a
4. Inverses: for each a, there exists a^(-1) such that a^(-1)a = aa^(-1) = e
5. A group may be commutative (abelian) or non-commutative

# Construction / Recognition

## To Construct:
1. Start with a set and a binary composition
2. Verify associativity (semigroup)
3. Verify existence of an identity element
4. Verify that every element has an inverse

## To Recognize:
1. Check all four group axioms: closure, associativity, identity, inverses
2. Or equivalently: check that the set with its operation is a semigroup with identity in which every element is invertible

# Context & Application

Groups are the algebraic foundation of GIS theory. The group IVLS in any GIS provides the intervals. Key musical groups include: the integers mod 12 under addition (for pitch-class intervals), the T/I group of 24 transposition and inversion operations, and the group of frequency ratios under multiplication (for just intonation intervals). Lewin's entire framework rests on group structure.

# Examples

**Example 1** (p. 36): The group of transposition and inversion operations on the twelve pitch classes is non-commutative. IT2 = J (inversion about B), but T2I = K (inversion about C#). Thus T2 and I do not commute.

**Example 2**: (Z, +): integers under addition. Identity = 0, inverse of n is -n.

**Example 3**: (Z12, +): integers mod 12 under addition. Identity = 0, inverse of n is 12 - n.

**Example 4**: Positive rationals of form 2^a * 3^b * 5^c under multiplication. Identity = 1, inverse of x is 1/x.

# Relationships

## Builds Upon
- **Semigroup** — a group extends a semigroup with identity and inverses

## Enables
- **Generalized Interval System** — the IVLS component of a GIS must be a group
- **Homomorphism** — structure-preserving maps between groups
- **Quotient Group** — groups can be quotiented by congruences

## Related
- **Group of Operations** — the concrete transformation-based version
- **Commutativity** — a group may or may not be commutative

## Contrasts With
- **Semigroup** — a semigroup need not have identity or inverses

# Common Errors

- **Error**: Forgetting that a group requires ALL FOUR properties.
  **Correction**: Closure, associativity, identity, AND inverses must all hold.

- **Error**: Assuming all groups are commutative.
  **Correction**: The T/I group is non-commutative: IT2 differs from T2I.

# Common Confusions

- **Confusion**: Equating "semigroup" with "group."
  **Clarification**: Every group is a semigroup, but not every semigroup is a group. A semigroup may lack an identity or inverses.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.7, p. 36.

# Verification Notes

- Definition source: direct from Definition 1.7
- Confidence rationale: explicit, concise definition with consistency note to concrete versions
- Re-extracted from v2 card; preserved: T/I non-commutativity example, four-axiom formulation
