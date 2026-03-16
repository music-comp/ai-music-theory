---
concept: Operation
slug: operation

category: mathematical-foundations
subcategory: functions
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.3.1, 1.3.3.3"

extraction_confidence: high

aliases:
  - bijective transformation

prerequisites:
  - transformation
  - one-to-one-function
  - onto-function
extends:
  - transformation
related:
  - inverse-function
  - group-of-operations
contrasts_with:
  - transformation

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

An operation on a set S is a transformation that is both 1-to-1 and onto -- a bijective self-mapping that preserves the structure of the space completely and has an inverse.

# Core Definition

"If the function is 1-to-1 and onto, it will be called an operation on S" (Lewin, Definition 1.3.1, p. 34). Equivalently, "a transformation f on S is an operation if and only if there exists a transformation f' on S satisfying the functional equations f'f = 1; ff' = 1. If this be the case then f' is the inverse operation of f" (Theorem 1.3.3.3, p. 34).

# Prerequisites

- **Transformation** — an operation is a special type of transformation
- **One-to-One Function** — an operation must be 1-to-1
- **Onto Function** — an operation must be onto

# Key Properties

1. An operation f: S -> S is both 1-to-1 and onto (bijective)
2. Every operation has a unique inverse operation f^(-1)
3. f^(-1)f = ff^(-1) = 1 (the identity)
4. Composition of two operations is an operation
5. Operations form the building blocks of groups

# Construction / Recognition

## To Construct:
1. Define a transformation f: S -> S
2. Verify it is 1-to-1 (distinct inputs give distinct outputs)
3. Verify it is onto (every element of S is an output)

## To Recognize:
1. Check for an inverse: if f' exists with f'f = 1 and ff' = 1, then f is an operation
2. Alternatively, verify 1-to-1 and onto directly

# Context & Application

Operations are the most important transformations in music theory because they are invertible. Transposition and inversion on pitch classes are operations. Groups of operations form the algebraic structures that underlie GIS theory. When we analyze music using transformation theory, we typically work with operations because no musical information is lost.

# Examples

**Example 1** (p. 34): T5 on pitch classes is an operation. It is 1-to-1 (different pitch classes map to different pitch classes) and onto (every pitch class is T5 of something). Its inverse is T7.

**Example 2**: I0 (inversion about C) on pitch classes is an operation that is its own inverse: I0I0 = 1.

**Non-example**: The "constant" transformation f(s) = C for all pitch classes s is a transformation but NOT an operation (it is neither 1-to-1 nor onto).

# Relationships

## Builds Upon
- **Transformation** — an operation is a bijective transformation

## Enables
- **Group of Operations** — groups are collections of operations satisfying closure and inverse conditions

## Contrasts With
- **Transformation** — a transformation need not be 1-to-1 or onto; an operation must be both

# Common Errors

- **Error**: Using "operation" in the colloquial sense (any procedure or computation).
  **Correction**: In Lewin's technical sense, "operation" specifically means a bijective transformation on S.

# Common Confusions

- **Confusion**: Thinking "transformation" and "operation" are synonymous.
  **Clarification**: Every operation is a transformation, but not vice versa. The distinction is that operations are bijective and therefore invertible.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.3.1, Theorem 1.3.3.3, p. 34.

# Verification Notes

- Definition source: direct from Definition 1.3.1 and Theorem 1.3.3.3
- Confidence rationale: explicit definitions with clear equivalence theorem
- Re-extracted from v2 card; preserved: T5/T7 example, constant-function non-example
