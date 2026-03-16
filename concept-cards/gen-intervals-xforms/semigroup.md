---
# === CORE IDENTIFICATION ===
concept: Semigroup
slug: semigroup

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
section: "1.4.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - binary-composition
  - associativity
extends: []
related:
  - semigroup-of-transformations
  - identity-element
  - inverse-element
contrasts_with:
  - group

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a semigroup?"
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

A semigroup is an ordered pair (X, BIN) comprising a set X and an associative binary composition BIN on X -- elements can be combined, and the combination is associative.

# Core Definition

"A semigroup is an ordered pair (X, BIN) comprising a family X and an associative binary composition BIN on X" (Lewin, Definition 1.4.3, p. 35). In multiplicative notation, the associative law reads x(yz) = (xy)z. Lewin warns that to define a particular semigroup, one must specify both the family X and the composition BIN, though it is customary to refer to "the semigroup X" when BIN is understood.

# Prerequisites

- **Binary Composition** — the semigroup is defined by a binary composition
- **Associativity** — the binary composition must be associative

# Key Properties

1. Consists of a set X and an associative binary composition BIN
2. Closure is built into the definition of binary composition (BIN maps X x X into X)
3. No identity element is required (unlike monoids)
4. No inverses are required (unlike groups)
5. The abstract definitions of "semigroup" (1.4.3) and "group" (1.7) are consistent with the concrete transformation-based terms (1.3.2, 1.3.4)

# Construction / Recognition

## To Construct:
1. Specify a set X
2. Define a binary composition BIN on X
3. Verify that BIN is associative

## To Recognize:
1. Check that BIN is defined for all pairs (x, y) in X and produces a result in X
2. Check that BIN is associative: x(yz) = (xy)z for all x, y, z

# Context & Application

Semigroups model collections of transformations that can be composed but may lack inverses or an identity. They are more general than groups, accommodating a wider range of algebraic situations. In Lewin's framework, the abstract semigroup concept generalizes the concrete semigroup of transformations. The quotient semigroup construction and homomorphism theory apply to semigroups in general.

# Examples

**Example 1** (p. 35): (Integers, addition) is a semigroup. In fact, it is a group, since it also has an identity (0) and inverses (-n for each n).

**Example 2** (p. 35): Using multiplicative notation, the associative law for a semigroup reads x(yz) = (xy)z. One must not carry intuitions about numerical multiplication into specific semigroups.

**Non-example** (p. 35): (Natural numbers, exponentiation) is NOT a semigroup because exponentiation is not associative.

# Relationships

## Builds Upon
- **Binary Composition** — the operation of the semigroup
- **Associativity** — required for semigroup structure

## Enables
- **Identity Element** — may or may not exist in a semigroup
- **Inverse Element** — may or may not exist in a semigroup
- **Group** — a semigroup with identity in which every element has an inverse
- **Congruence** — congruences are defined on semigroups
- **Homomorphism** — structure-preserving maps between semigroups

## Contrasts With
- **Group** — a group has identity and inverses; a semigroup need not

# Common Errors

- **Error**: Assuming all semigroups have an identity element.
  **Correction**: Identity is an extra condition. Semigroups with identity are sometimes called "monoids."

# Common Confusions

- **Confusion**: Thinking a semigroup is "half a group."
  **Clarification**: A semigroup requires only closure and associativity. A group additionally requires identity and inverses. The name "semigroup" is historical.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.4.3, p. 35.

# Verification Notes

- Definition source: direct from Definition 1.4.3
- Confidence rationale: explicit definition in source
- Re-extracted from v2 card; preserved: warning about multiplicative notation, consistency note with concrete definitions
