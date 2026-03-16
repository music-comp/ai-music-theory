---
concept: Identity Element
slug: identity-element

category: mathematical-foundations
subcategory: algebraic-structures
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.5.1-1.5.2"

extraction_confidence: high

aliases:
  - identity
  - neutral element

prerequisites:
  - semigroup
extends: []
related:
  - identity-transformation
  - inverse-element
  - group
contrasts_with: []

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

An identity element e in a semigroup satisfies ex = xe = x for all elements x -- it leaves every element unchanged when combined with it.

# Core Definition

"A left identity for a semigroup is an element l such that for every x, lx = x. A right identity is defined dually: For every x, xr = x. An identity is an element e which is both a left identity and a right identity" (Lewin, Definition 1.5.1, p. 36). If a semigroup has both a left identity l and a right identity r, then l = r, so there can be at most one identity element (Theorem 1.5.2).

# Prerequisites

- **Semigroup** — identity is defined within a semigroup

# Key Properties

1. ex = xe = x for all x (identity property)
2. Uniqueness: if both a left identity and right identity exist, they are equal (Theorem 1.5.2)
3. Not all semigroups have an identity
4. A semigroup can have infinitely many left identities without having a right identity
5. Notation: e or 1 in multiplicative context, 0 in additive context

# Construction / Recognition

## To Construct:
1. In a given semigroup, find an element e such that ex = xe = x for all x

## To Recognize:
1. Check that the candidate e satisfies ex = x for all x (left identity)
2. Check that it satisfies xe = x for all x (right identity)

# Context & Application

The identity element represents "no change" in any transformation or interval system. In pitch-class transposition, 0 is the identity (transposing by 0). In multiplicative interval groups, 1 is the identity. In any GIS, int(s, s) = e, meaning the interval from any element to itself is the identity.

# Examples

**Example 1** (p. 36): In integers mod 12 under addition: e = 0, since 0 + n = n + 0 = n for all n.

**Example 2** (p. 36): In positive rationals under multiplication: e = 1, since 1 * r = r * 1 = r for all r.

**Example 3** (p. 36): Proof of uniqueness: lr = r (since l is a left identity) and lr = l (since r is a right identity), so l = r.

**Example 4** (p. 36): A semigroup with infinitely many left identities: take any family X and define BIN(x, y) = y. Then every element is a left identity, but no right identity exists.

# Relationships

## Builds Upon
- **Semigroup** — identity is defined within a semigroup

## Enables
- **Inverse Element** — inverses are defined relative to the identity
- **Group** — a group is a semigroup with identity where every element has an inverse

## Related
- **Identity Transformation** — the concrete version for transformation semigroups

# Common Errors

- **Error**: Assuming every semigroup has an identity.
  **Correction**: Some semigroups lack identity elements. Only "monoids" (semigroups with identity) are guaranteed to have one.

# Common Confusions

- **Confusion**: Confusing "left identity" with "identity."
  **Clarification**: A left identity satisfies lx = x for all x but need not satisfy xl = x. Only an element that is both left and right identity is "the" identity.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definitions 1.5.1, Theorem 1.5.2, p. 36.

# Verification Notes

- Definition source: direct from Definition 1.5.1 and Theorem 1.5.2
- Confidence rationale: explicit definition with uniqueness proof
- Re-extracted from v2 card; preserved: proof of uniqueness, pathological left-identity semigroup example
