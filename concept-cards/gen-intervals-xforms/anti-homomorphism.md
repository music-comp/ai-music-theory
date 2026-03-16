---
concept: Anti-Homomorphism
slug: anti-homomorphism

category: mathematical-foundations
subcategory: algebraic-structures
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.11.4"

extraction_confidence: high

aliases: []

prerequisites:
  - homomorphism
  - semigroup
extends: []
related:
  - isomorphism
  - left-orthography
contrasts_with:
  - homomorphism

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

An anti-homomorphism is a function between semigroups that reverses the order of products: f(x1)f(x2) = f(x2x1), or equivalently f(x1x2) = f(x2)f(x1).

# Core Definition

"An anti-homomorphism of one semigroup into another is a function f satisfying f(x1)f(x2) = f(x2x1)" (Lewin, Section 1.11.4, p. 42). Every anti-homomorphism of (X, BIN) is a homomorphism of (X, ANTIBIN), where ANTIBIN(x1, x2) = BIN(x2, x1). An anti-isomorphism is an anti-homomorphism that is 1-to-1 and onto.

# Prerequisites

- **Homomorphism** — anti-homomorphism is the order-reversing analog
- **Semigroup** — anti-homomorphisms map between semigroups

# Key Properties

1. f(x1)f(x2) = f(x2x1) (reverses product order)
2. Equivalently: f(x1x2) = f(x2)f(x1)
3. Every anti-homomorphism of (X, BIN) is a homomorphism of (X, ANTIBIN)
4. To a commutative group, anti-homomorphisms and homomorphisms coincide
5. Switching between left and right orthography converts anti-homomorphisms to homomorphisms

# Construction / Recognition

## To Construct:
1. Define a function between semigroups
2. Verify f(x1x2) = f(x2)f(x1) for all pairs

## To Recognize:
1. Check if the function reverses the order of products
2. If f(xy) = f(y)f(x), it is an anti-homomorphism

# Context & Application

Anti-homomorphisms arise when dealing with both transposition and interval-preserving operations. The map from intervals i to transposition operations Ti is an anti-isomorphism: PiPj = Pij (homomorphism for interval-preserving operations) but TiTj = Tji (anti-isomorphism for transpositions). This distinction between P and T operations is fundamental and arises from the choice of left orthography.

# Examples

**Example 1** (Section 1.11.4, p. 42): Consider a group with elements i, j, k, ... and two families of operations: P-operations and T-operations.
- P-operations combine: PiPj = Pij (the map i -> Pi is an isomorphism)
- T-operations combine: TiTj = Tji (the map i -> Ti is an anti-isomorphism)

**Example 2** (p. 42): Using right orthography for T-operations would make i -> Ti an isomorphism, but then i -> Pi would become an anti-isomorphism.

# Relationships

## Builds Upon
- **Homomorphism** — anti-homomorphism reverses the product order

## Related
- **Left Orthography** — the choice of orthography determines which maps are homomorphisms vs anti-homomorphisms

## Contrasts With
- **Homomorphism** — preserves order: f(xy) = f(x)f(y)

# Common Errors

- **Error**: Writing f(xy) = f(x)f(y) when the map is an anti-homomorphism.
  **Correction**: For an anti-homomorphism, f(xy) = f(y)f(x). The order reverses.

# Common Confusions

- **Confusion**: Thinking anti-homomorphisms only matter for non-commutative groups.
  **Clarification**: For commutative groups, anti-homomorphisms and homomorphisms coincide. The distinction only matters when the group is non-commutative.

# Source Reference

Chapter 1: Mathematical Preliminaries, Section 1.11.4, p. 42.

# Verification Notes

- Definition source: direct from Section 1.11.4
- Confidence rationale: explicit definition with motivating discussion of P and T operations
- Re-extracted from v2 card; preserved: P vs T operations example, orthography connection
