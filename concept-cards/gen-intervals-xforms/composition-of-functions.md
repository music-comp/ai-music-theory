---
concept: Composition of Functions
slug: composition-of-functions

category: mathematical-foundations
subcategory: functions
tier: foundational

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.2.3-1.2.5"

extraction_confidence: high

aliases:
  - function composition

prerequisites:
  - function
  - functional-equality
extends: []
related:
  - left-orthography
  - associativity
contrasts_with: []

answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
  - "How do transformations combine in sequence?"
---

# Quick Definition

Function composition combines two functions by applying one after the other: the composition f'f means first apply f, then apply f' to the result, giving (f'f)(s) = f'(f(s)).

# Core Definition

"Let f be a function from S into S', and let f' be a function from S' into S''. Then the composition function f'f is defined from S into S'' as follows: Given an argument s in S, the value (f'f)(s) is f'(f(s))" (Lewin, Definition 1.2.3, p. 32). In left orthography, f' appears to the left of f in the notation f'f, consistent with the convention of writing the function name to the left of the argument.

# Prerequisites

- **Function** — composition operates on functions
- **Functional Equality** — verifying composition equations requires checking equality of functions

# Key Properties

1. (f'f)(s) = f'(f(s)) for all s in S
2. The composition f'f maps S into S'' when f: S -> S' and f': S' -> S''
3. Composition is always associative: f''(f'f) = (f''f')f
4. Composition is generally not commutative: f'f may differ from ff'
5. In left orthography, the rightmost function is applied first

# Construction / Recognition

## To Construct:
1. Given f: S -> S' and f': S' -> S''
2. For each s in S, compute f(s) to get an element of S'
3. Apply f' to that result: f'(f(s)) gives an element of S''
4. The function s -> f'(f(s)) is the composition f'f

## To Recognize:
1. Identify a chain of two function applications
2. Verify that the codomain of the first matches the domain of the second
3. The composite function maps the domain of the first to the codomain of the second

# Context & Application

Composition is how musical transformations combine into compound transformations. When analyzing a chain of operations (transpose, then invert), the result is a single composition function. Lewin's left orthography convention means that in the expression IT2, the transposition T2 is applied first, then the inversion I.

# Examples

**Example 1** (p. 32): Let f1(s) = s + 3, f2(s) = 2s, f3(s) = 2s, f4(s) = s + 6 on positive integers. The equation f2f1 = f4f3 holds: f2(f1(s)) = 2(s + 3) = 2s + 6 = f4(f3(s)) = f4(2s) = 2s + 6.

**Example 2** (p. 32): On twelve pitch classes, let f(s) = s transposed by 2, f'(s) = s inverted about C, f''(s) = s inverted about B. Then f'f = f'': inverting about C the 2-transpose of any pitch class s gives the same result as inverting about B.

# Relationships

## Builds Upon
- **Function** — composition requires two functions

## Enables
- **Transformation** — transformations compose to form new transformations
- **Semigroup of Transformations** — closure under composition defines semigroups
- **Group of Operations** — groups require composition of operations

## Related
- **Left Orthography** — the notational convention for composition
- **Associativity** — composition of functions is always associative

# Common Errors

- **Error**: Reading f'f as "apply f' first, then f."
  **Correction**: In left orthography, f is applied first (rightmost goes first): (f'f)(s) = f'(f(s)).

- **Error**: Assuming composition is commutative.
  **Correction**: f'f and ff' are generally different; order matters.

# Common Confusions

- **Confusion**: The function written first (leftmost) is applied first.
  **Clarification**: In left orthography, the leftmost function is applied last. f'f means "f' of f of s."

# Source Reference

Chapter 1: Mathematical Preliminaries, Definitions 1.2.3-1.2.4, Section 1.2.5, pp. 32-33.

# Verification Notes

- Definition source: direct from Definition 1.2.3
- Confidence rationale: explicit definition with detailed examples in source
- Re-extracted from v2 card; preserved: integer and pitch-class examples, emphasis on reading order
