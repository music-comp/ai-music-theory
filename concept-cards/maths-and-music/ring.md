---
# === CORE IDENTIFICATION ===
concept: Ring
slug: ring

# === CLASSIFICATION ===
category: algebra-in-music
subcategory: rings
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
section: "Ring"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "commutative ring"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - units-in-a-ring
  - integral-domain
  - ideals-and-principal-ideals
  - modular-integers-ring-structure
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a ring?"
  - "What algebraic structure does the set of integers have?"
  - "What is the difference between a ring and a group?"
---

# Quick Definition

A ring is a set equipped with addition and multiplication where addition forms a commutative group, multiplication forms a monoid, and multiplication distributes over addition. In this text, "ring" always means commutative ring.

# Core Definition

"A non-empty set $R$ endowed with two associative laws of composition $+$ and $\cdot$ is called a ring if $(R, +)$ is a commutative group, $(R, \cdot)$ is a monoid, and for any $a, b, c \in R$ we have $a \cdot (b + c) = a \cdot b + a \cdot c$ and $(b + c) \cdot a = b \cdot a + c \cdot a$" (Wright, Ch. 8, p. 100). The ring is commutative if $(R, \cdot)$ is commutative. The text deals exclusively with commutative rings.

# Prerequisites

This is a foundational concept with no prerequisites within this source beyond basic familiarity with groups and monoids from earlier chapters.

# Key Properties

1. $(R, +)$ is a commutative group with identity element $0$
2. $(R, \cdot)$ is a monoid with identity element $1$
3. Distributivity holds: $a \cdot (b + c) = a \cdot b + a \cdot c$ and $(b + c) \cdot a = b \cdot a + c \cdot a$
4. From the axioms one can derive $(-1) \cdot x = -x$ and $0 \cdot x = 0$ for any $x \in R$
5. $(R, \cdot)$ is generally not a group since $0$ has no multiplicative inverse
6. The only ring where $(R, \cdot)$ is a group is the trivial ring $R = \{0\}$ (where $0 = 1$)

# Construction / Recognition

## To verify a set R with operations +, * is a ring:
1. Check $(R, +)$ is a commutative group (closure, associativity, identity $0$, inverses, commutativity)
2. Check $(R, \cdot)$ is a monoid (closure, associativity, identity $1$)
3. Verify left and right distributivity
4. For commutativity, additionally check $a \cdot b = b \cdot a$ for all $a, b$

# Context & Application

The ring structure provides the algebraic framework for studying integer intervals and modular arithmetic in music. The ring $\mathbb{Z}$ captures the multiplicative structure of integer frequency ratios, while the ring $\mathbb{Z}_m$ captures both the additive structure of modular interval composition and the multiplicative structure needed for the theory of generators and units.

# Examples

**Example 1** (p. 100): $\mathbb{Z}$ with usual addition and multiplication is a commutative ring and an integral domain. The group of units is $\mathbb{Z}^* = \{1, -1\}$.

**Example 2** (p. 100): $\mathbb{R}$ with usual operations is a commutative ring and integral domain with $\mathbb{R}^* = \mathbb{R} \setminus \{0\}$.

**Example 3** (p. 100): $\mathbb{Q}$ is an integral domain where all non-zero elements are units.

**Example 4** (p. 100): $\mathbb{Z}_m$ with $[k] + [\ell] = [k + \ell]$ and $[k] \cdot [\ell] = [k\ell]$ is a commutative ring.

# Relationships

## Builds Upon
- **Group** -- The additive structure $(R, +)$ must be a commutative group

## Enables
- **Units in a Ring** -- Units are defined as elements with multiplicative inverses in the ring
- **Integral Domain** -- An integral domain is a ring with no zero divisors
- **Ideals and Principal Ideals** -- Ideals are special subsets of rings

## Related
- **Modular Integers Ring Structure** -- $\mathbb{Z}_m$ provides the key musical example of a ring

## Contrasts With
- **Group** -- A ring has two operations; a group has one

# Common Errors

- **Error**: Assuming every non-zero element of a ring has a multiplicative inverse
  **Correction**: Only units have multiplicative inverses; in $\mathbb{Z}$, only $\pm 1$ are units

- **Error**: Trying to prove $(R, \cdot)$ is a group
  **Correction**: $(R, \cdot)$ is only a monoid; $0$ never has a multiplicative inverse (except in the trivial ring)

# Common Confusions

- **Confusion**: Thinking "commutative ring" means both operations are commutative
  **Clarification**: Addition is always commutative in a ring by definition; "commutative ring" specifically means multiplication is also commutative

- **Confusion**: Conflating a ring with a field
  **Clarification**: A field is a ring where every non-zero element is a unit; $\mathbb{Z}$ is a ring but not a field

# Source Reference

Chapter 8: "Algebraic Properties of the Integers," "Ring" section, pp. 100-101.

# Verification Notes

- Definition source: Direct quote from p. 100
- Confidence rationale: Explicit, formal definition provided in the source
- Uncertainties: None
- Cross-reference status: Verified against existing cards
- Re-extraction notes: Re-extracted from v2 card; preserved: examples, common confusions about commutative terminology
