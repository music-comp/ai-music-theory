---
concept: Monoid
slug: monoid

category: algebra-in-music
subcategory: groups
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
section: "Monoid"

extraction_confidence: high

aliases: []

prerequisites: []
extends: []
related:
  - group
  - commutative-group
  - modular-integers
contrasts_with:
  - group

answers_questions:
  - "What is a monoid?"
  - "What is a group in abstract algebra?"
  - "How does a monoid differ from a group?"
---

# Quick Definition

An algebraic structure consisting of a set with an associative binary operation and an identity element, serving as the foundation upon which groups are built.

# Core Definition

A monoid is a set M with a law of composition (binary operation) denoted by some symbol (e.g., *) satisfying (Wright, pp. 87-88):
1. **Associativity**: For any x, y, z in M, (x * y) * z = x * (y * z).
2. **Identity**: There exists an element e in M such that for all x in M, x * e = e * x = x.

The identity element e is unique (proved by: if e' is another identity, then e = e * e' = e'). A monoid is always non-empty (contains e). A monoid is **commutative** if x * y = y * x for all x, y. By convention, + is used only for commutative operations.

# Prerequisites

This is a foundational algebraic concept introduced without prerequisites within this source.

# Key Properties

1. The operation is associative: parentheses can be dropped without ambiguity
2. The identity element is unique
3. A monoid is always non-empty (it contains e)
4. Commutativity is a separate, optional property
5. Elements need not have inverses (unlike in a group)
6. Notation (M, *) specifies both the set and its operation

# Construction / Recognition

## To Verify a Monoid
1. Check that the operation is closed (combining two elements gives an element of M)
2. Verify associativity: (x * y) * z = x * (y * z) for all x, y, z
3. Identify the identity element e satisfying x * e = e * x = x for all x
4. If all three hold, the structure is a monoid

# Context & Application

The set of musical intervals under composition forms a monoid (and in fact a group). The identity element is the unison interval. The monoid structure captures the idea that composing two intervals yields another interval and that composition is associative. Monoids are the stepping stone to groups, which add the requirement of inverses.

# Examples

**Example (a)** (p. 87): (R, *) — real numbers under multiplication. Identity is 1. A monoid but not a group (0 has no inverse).

**Example (b)** (p. 87): (Z, +) — integers under addition. Identity is 0. A commutative monoid and group.

**Example (c)** (p. 87): (F(S), compose) — functions from S to S under composition. Identity is id_S. Not commutative in general: f(x) = x^2 and g(x) = x + 1 give f(g(x)) = (x+1)^2 != x^2 + 1 = g(f(x)).

**Example (d)** (p. 87): (Z_m, +) — modular integers under addition. Identity is [0]. Commutative monoid and group.

# Relationships

## Builds Upon
This is foundational; no prior concepts required.

## Enables
- **Group** — A group is a monoid where every element has an inverse
- **Commutative group** — A commutative monoid with inverses

## Related
- **Modular integers** — Z_m is an example of a commutative monoid (and group)

## Contrasts With
- **Group** — A group additionally requires every element to have an inverse

# Common Errors

- **Error**: Assuming every monoid is a group
  **Correction**: A monoid does not require inverses; (R, *) is a monoid but not a group because 0 has no multiplicative inverse

# Common Confusions

- **Confusion**: Thinking associativity implies commutativity
  **Clarification**: Associativity ((x*y)*z = x*(y*z)) and commutativity (x*y = y*x) are independent properties; function composition is associative but not commutative

- **Confusion**: Believing a monoid can be empty
  **Clarification**: A monoid always contains at least the identity element e

# Source Reference

Chapter 7: "Octave Identification and Modular Arithmetic," pp. 87-88 (Monoid section). See examples (a)-(d) and the proof of uniqueness of e.

# Verification Notes

- Definition source: Direct from Wright, pp. 87-88
- Confidence rationale: High — formal definition with axioms explicitly stated
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all four examples, non-commutativity of function composition example, uniqueness proof
