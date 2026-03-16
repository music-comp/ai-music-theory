---
# === CORE IDENTIFICATION ===
concept: Commutative Group
slug: commutative-group

# === CLASSIFICATION ===
category: algebra-in-music
subcategory: groups
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Commutativity"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - abelian group

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
extends:
  - group
related:
  - monoid
  - modular-arithmetic
  - group-of-intervals
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a commutative (abelian) group?"
  - "Why are all musical interval groups commutative?"
---

# Quick Definition

A group in which the order of combining elements does not matter (x * y = y * x for all x, y), also called an abelian group. All musical interval groups in this text are commutative.

# Core Definition

A group (G, *) is commutative (or abelian) if for all x, y in G, x * y = y * x. By convention, the operation symbol + is reserved for commutative group operations. In additive notation, the inverse of x is written -x and x + (-y) is abbreviated x - y (Wright, p. 88).

# Prerequisites

- **Group** — A commutative group is a group with the additional commutativity property

# Key Properties

1. x * y = y * x for all x, y in G
2. Convention: + is used only for commutative operations
3. In additive notation: inverse of x is -x; x - y means x + (-y)
4. All cyclic groups are commutative
5. Not all commutative groups are cyclic

# Construction / Recognition

## To Verify Commutativity
1. Check if x * y = y * x for all elements x, y in the group
2. If the operation is denoted +, commutativity is assumed by convention

# Context & Application

Commutativity of interval composition means going up a third then a fourth gives the same result as going up a fourth then a third. This property is essential for the identification of modular chromatic intervals with Z_12, where order of composition does not matter.

# Examples

**Example 1** (p. 88): (Z, +), (R, +), (R+, *), and (Z_m, +) are all commutative.

**Example 2** (p. 88): (F(R), compose) is NOT commutative: f(x) = x^2 and g(x) = x + 1 give f(g(x)) = (x+1)^2 but g(f(x)) = x^2 + 1.

**Example 3**: In Z_12: [3] + [5] = [8] = [5] + [3] (minor third + fourth = fourth + minor third).

# Relationships

## Builds Upon
- **Group** — A commutative group is a group with commutativity

## Enables
- **Group of intervals** — Musical interval groups are commutative

## Related
- **Monoid** — Commutative monoids satisfy commutativity without requiring inverses
- **Modular arithmetic** — Z_m is a commutative group

# Common Errors

- **Error**: Using + for a non-commutative operation
  **Correction**: By convention, + is reserved for commutative operations; use * or compose for non-commutative ones

# Common Confusions

- **Confusion**: Thinking "commutative group" and "abelian group" are different concepts
  **Clarification**: They are the same; "abelian" is named after Niels Henrik Abel

- **Confusion**: Believing all groups are commutative
  **Clarification**: Non-commutative groups exist (e.g., matrix groups, function composition groups)

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," p. 88 (Commutativity section).

# Verification Notes

- Definition source: Direct from Wright, p. 88
- Confidence rationale: High — explicit definition
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: function composition counterexample, + convention, Abel attribution
