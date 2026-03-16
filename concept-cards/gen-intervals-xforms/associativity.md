---
concept: Associativity
slug: associativity

category: mathematical-foundations
subcategory: algebraic-structures
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.3.5, 1.4.2"

extraction_confidence: high

aliases:
  - associative law
  - associative property

prerequisites:
  - binary-composition
extends: []
related:
  - semigroup
  - composition-of-functions
contrasts_with:
  - commutativity

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

Associativity is the property that grouping does not matter when combining three elements: x(yz) = (xy)z for all elements.

# Core Definition

"A binary composition on X is associative if BIN(x, BIN(y, z)) = BIN(BIN(x, y), z) for all x, y, and z" (Lewin, Definition 1.4.2, p. 35). For transformations, the Associative Law states that "the result of applying f to the (gh)-transform of s is the same as the result of applying (fg) to the h-transform of the given s" (Section 1.3.5, p. 35). Composition of functions is always associative.

# Prerequisites

- **Binary Composition** — associativity is a property of a binary composition

# Key Properties

1. x(yz) = (xy)z for all x, y, z in X
2. Allows unambiguous notation xyz without parentheses
3. Required for semigroup and group structure
4. Composition of functions is always associative
5. Not all binary compositions are associative (e.g., exponentiation)

# Construction / Recognition

## To Construct:
1. Not directly constructed; it is a property that a binary composition either has or does not have

## To Recognize:
1. Check whether x(yz) = (xy)z for all x, y, z
2. Finding any triple where the equation fails proves non-associativity

# Context & Application

Associativity ensures that chains of musical transformations can be computed in any grouping order. When analyzing T3T5T2, we can compute (T3T5)T2 = T8T2 = T10, or T3(T5T2) = T3T7 = T10, obtaining the same result. This property is essential for transformation networks where paths can be evaluated step by step in any grouping.

# Examples

**Example 1** (p. 35): For transformations on S, f(gh) = (fg)h. Given any s, applying f to the (gh)-transform of s gives the same result as applying (fg) to the h-transform of s.

**Example 2** (p. 35): Exponentiation on natural numbers is NOT associative: 3^(2^3) = 3^8 = 6561, but (3^2)^3 = 9^3 = 729. Since 6561 differs from 729, the associative law fails.

# Relationships

## Builds Upon
- **Binary Composition** — associativity is a property of binary compositions

## Enables
- **Semigroup** — a semigroup requires an associative binary composition

## Related
- **Composition of Functions** — always associative

## Contrasts With
- **Commutativity** — associativity concerns grouping (parentheses); commutativity concerns order (xy vs yx)

# Common Errors

- **Error**: Confusing associativity with commutativity.
  **Correction**: Associativity is x(yz) = (xy)z (grouping); commutativity is xy = yx (order). A non-commutative group is still associative.

# Common Confusions

- **Confusion**: Assuming non-associative operations can form semigroups.
  **Clarification**: A semigroup requires associativity by definition. Exponentiation cannot form a semigroup.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.4.2, Section 1.3.5, pp. 34-35.

# Verification Notes

- Definition source: direct from Definition 1.4.2 and Section 1.3.5
- Confidence rationale: explicit definition with counterexample in source
- Re-extracted from v2 card; preserved: exponentiation counterexample, transposition grouping example
