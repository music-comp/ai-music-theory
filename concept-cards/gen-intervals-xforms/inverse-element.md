---
# === CORE IDENTIFICATION ===
concept: Inverse Element
slug: inverse-element

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
section: "1.6.1-1.6.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - inverse

# === TYPED RELATIONSHIPS ===
prerequisites:
  - semigroup
  - identity-element
extends: []
related:
  - inverse-function
  - group
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

An inverse of element x in a semigroup with identity e is an element x' such that x'x = xx' = e -- it "undoes" the effect of x.

# Core Definition

"Given a semigroup with identity e; given an element x, a left inverse for x is an element l satisfying lx = e. A right inverse for x is an element r satisfying xr = e. An inverse for x is an x' which is both a left inverse and a right inverse" (Lewin, Definition 1.6.1, p. 36). If x has both a left inverse l and a right inverse r, then l = r, so x can have at most one inverse (Theorem 1.6.2). In multiplicative notation, the inverse of x is written x^(-1).

# Prerequisites

- **Semigroup** — inverses are defined within a semigroup
- **Identity Element** — inverses are defined relative to the identity

# Key Properties

1. x^(-1)x = xx^(-1) = e (identity)
2. Uniqueness: if a left inverse and right inverse both exist, they are equal
3. Proof: l = le = l(xr) = (lx)r = er = r (Theorem 1.6.2)
4. In a group, every element has a unique inverse
5. In multiplicative notation: x^(-1); in additive notation: -x

# Construction / Recognition

## To Construct:
1. In a semigroup with identity e, find x' such that x'x = e and xx' = e

## To Recognize:
1. Check x'x = e (left inverse condition)
2. Check xx' = e (right inverse condition)
3. Both must hold for x' to be the inverse of x

# Context & Application

Inverses allow "reversal" of musical transformations. The inverse of transposition by n is transposition by -n. In GIS theory, int(t, s) = int(s, t)^(-1): the interval from t to s is the inverse of the interval from s to t. Inverses are essential for group structure and for analyzing symmetrical musical relationships.

# Examples

**Example 1** (p. 36): In integers mod 12: the inverse of 5 is 7, since 5 + 7 = 12 = 0 mod 12.

**Example 2** (p. 36): In interval ratios under multiplication: the inverse of 3/2 is 2/3, since (3/2)(2/3) = 1.

**Example 3** (p. 36): Proof of uniqueness: l = le = l(xr) = (lx)r = er = r. The key step uses associativity.

# Relationships

## Builds Upon
- **Identity Element** — inverses are defined relative to the identity

## Enables
- **Group** — a group requires every element to have an inverse

## Related
- **Inverse Function** — the concrete version for functions

# Common Errors

- **Error**: Assuming a left inverse alone guarantees a right inverse.
  **Correction**: In semigroups, a left inverse alone does not guarantee a right inverse. Both conditions are needed.

# Common Confusions

- **Confusion**: Thinking the inverse of a composition preserves order.
  **Clarification**: The inverse of xy is y^(-1)x^(-1) (order reverses), not x^(-1)y^(-1).

# Source Reference

Chapter 1: Mathematical Preliminaries, Definitions 1.6.1-1.6.3, Theorem 1.6.2, p. 36.

# Verification Notes

- Definition source: direct from Definitions 1.6.1-1.6.3 and Theorem 1.6.2
- Confidence rationale: explicit definitions with uniqueness proof
- Re-extracted from v2 card; preserved: uniqueness proof, mod 12 and ratio examples
