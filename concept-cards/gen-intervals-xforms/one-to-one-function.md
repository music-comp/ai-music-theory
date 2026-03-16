---
concept: One-to-One Function
slug: one-to-one-function

category: mathematical-foundations
subcategory: functions
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.2.6.2"

extraction_confidence: high

aliases:
  - 1-to-1 function
  - injection
  - injective function

prerequisites:
  - function
extends:
  - function
related:
  - onto-function
  - inverse-function
  - operation
contrasts_with:
  - onto-function

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

A function f from S into S' is one-to-one (1-to-1) if no two distinct arguments share the same value -- each output comes from exactly one input.

# Core Definition

"The function f from S into S' is 1-to-1 if no two distinct arguments share the same value. (No member of S' appears more than once in the right-hand column of the function table.)" (Lewin, Definition 1.2.6.2, p. 33).

# Prerequisites

- **Function** — one-to-one is an additional property of a function

# Key Properties

1. If f(s1) = f(s2), then s1 = s2
2. Equivalently: if s1 differs from s2, then f(s1) differs from f(s2)
3. A 1-to-1 function preserves distinctness of elements
4. A function can be 1-to-1 without being onto, or vice versa

# Construction / Recognition

## To Construct:
1. Define f: S -> S'
2. Ensure that distinct elements of S always map to distinct elements of S'

## To Recognize:
1. Check whether any two different elements of S produce the same value
2. If f(s1) = f(s2) with s1 different from s2, the function is not 1-to-1

# Context & Application

One-to-one functions preserve distinctness, ensuring no information is lost. In music theory, transposition and inversion on pitch classes are both one-to-one: different pitch classes always map to different pitch classes. This property is essential for operations (bijective transformations) and for the existence of inverse functions.

# Examples

**Example 1** (p. 33): Transposition T2 on pitch classes is 1-to-1: T2(C) = D, T2(C#) = D#, etc. No two different pitch classes map to the same result.

**Example 2**: The function mapping all pitches to their pitch class is NOT 1-to-1, because C4 and C5 both map to pitch class C.

# Relationships

## Builds Upon
- **Function** — one-to-one is a property of functions

## Enables
- **Operation** — an operation is a transformation that is both 1-to-1 and onto
- **Inverse Function** — requires both 1-to-1 and onto
- **Isomorphism** — an isomorphism is a 1-to-1 homomorphism

## Contrasts With
- **Onto Function** — one-to-one concerns uniqueness; onto concerns coverage

# Common Errors

- **Error**: Confusing "one-to-one" with "onto."
  **Correction**: One-to-one means each value has at most one preimage; onto means each value has at least one preimage.

# Common Confusions

- **Confusion**: Assuming a one-to-one function is automatically onto.
  **Clarification**: f(s) = 2s on positive integers is one-to-one but not onto (odd integers are not values). Both properties are needed for an inverse to exist.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.2.6.2, p. 33.

# Verification Notes

- Definition source: direct from Definition 1.2.6.2
- Confidence rationale: explicit definition in source
- Re-extracted from v2 card; preserved: transposition example, pitch-to-pitch-class non-example
