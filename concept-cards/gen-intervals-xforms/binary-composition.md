---
concept: Binary Composition
slug: binary-composition

category: mathematical-foundations
subcategory: algebraic-structures
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.4.1-1.4.2"

extraction_confidence: high

aliases:
  - binary operation

prerequisites:
  - function
extends: []
related:
  - associativity
  - semigroup
  - commutativity
contrasts_with: []

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

A binary composition on a set X is a function that takes any two elements of X and produces another element of X, written BIN(x, y) or simply xy.

# Core Definition

"A binary composition on X is a function BIN that maps X x X into X. We write BIN(x, y) for the value of BIN on the pair (x, y)" (Lewin, Definition 1.4.1, p. 35). A binary composition is associative if BIN(x, BIN(y, z)) = BIN(BIN(x, y), z) for all x, y, and z (Definition 1.4.2). It is traditional to write "xy" for BIN(x, y) using multiplicative notation.

# Prerequisites

- **Function** — a binary composition is a specific type of function (from X x X into X)

# Key Properties

1. BIN: X x X -> X maps pairs of elements to elements
2. Closure is built into the definition: the result is always in X
3. Not every binary composition is associative
4. Not every binary composition is commutative
5. Multiplicative notation "xy" or additive notation "x + y" may be used

# Construction / Recognition

## To Construct:
1. Fix a set X
2. Define a rule that assigns to each pair (x, y) in X x X an element of X

## To Recognize:
1. Verify the operation takes two inputs from X and produces one output in X
2. Check closure: every pair of elements produces a result that is in X

# Context & Application

Binary composition is the abstract version of how transformations combine. In music theory, composing transpositions uses addition mod 12; combining general transformations uses function composition. The specific rule of combination depends on the context. Lewin warns against carrying intuitions about numerical multiplication into general semigroups.

# Examples

**Example 1** (p. 35): Addition on integers: BIN(3, 5) = 8.

**Example 2** (p. 35): Exponentiation on natural numbers: BIN(x, y) = x^y. This is NOT associative: BIN(3, BIN(2, 3)) = 3^(2^3) = 3^8 = 6561, but BIN(BIN(3, 2), 3) = (3^2)^3 = 9^3 = 729. Since 6561 differs from 729, exponentiation is not associative.

# Relationships

## Builds Upon
- **Function** — a binary composition is a function from X x X into X

## Enables
- **Semigroup** — a semigroup is a set with an associative binary composition
- **Associativity** — a key property a binary composition may or may not have
- **Commutativity** — another key property

# Common Errors

- **Error**: Assuming every binary composition is associative.
  **Correction**: Exponentiation is a standard counterexample of a non-associative binary composition.

# Common Confusions

- **Confusion**: Thinking "binary" refers to the base-2 number system.
  **Clarification**: "Binary" means the composition takes two inputs. It has nothing to do with binary numbers.

- **Confusion**: Conflating multiplicative notation "xy" with actual numerical multiplication.
  **Clarification**: "xy" is abstract notation for BIN(x, y); the actual rule of combination may differ from multiplication.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definitions 1.4.1-1.4.2, p. 35.

# Verification Notes

- Definition source: direct from Definitions 1.4.1-1.4.2
- Confidence rationale: explicit definitions with counterexample
- Re-extracted from v2 card; preserved: exponentiation non-associativity example
