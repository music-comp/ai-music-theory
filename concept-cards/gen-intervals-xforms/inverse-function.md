---
# === CORE IDENTIFICATION ===
concept: Inverse Function
slug: inverse-function

# === CLASSIFICATION ===
category: mathematical-foundations
subcategory: functions
tier: foundational

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
section: "1.2.6.3-1.2.6.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - function
  - one-to-one-function
  - onto-function
extends:
  - function
related:
  - operation
  - inverse-element
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

The inverse function f^(-1) of a 1-to-1 onto function f reverses the mapping: if f(s) = s', then f^(-1)(s') = s.

# Core Definition

"Let f be a 1-to-1 function from S onto S'. Then f^(-1), the inverse function of f, is defined as the family of pairs (s', s) within S' x S such that (s, s') is a member of f" (Lewin, Definition 1.2.6.3, p. 33). The inverse f^(-1) is itself a 1-to-1 function from S' onto S, and the inverse of f^(-1) is f (Theorem 1.2.6.4). Furthermore, if f and f' satisfy f'f(s) = s for all s and ff'(s') = s' for all s', then f and f' are both 1-to-1, onto, and inverse to each other (Theorem 1.2.6.5).

# Prerequisites

- **Function** — inverse is defined for functions
- **One-to-One Function** — the function must be 1-to-1 for the inverse to exist
- **Onto Function** — the function must be onto for the inverse to exist

# Key Properties

1. f^(-1) exists only when f is both 1-to-1 and onto
2. f^(-1)(f(s)) = s for all s in S
3. f(f^(-1)(s')) = s' for all s' in S'
4. (f^(-1))^(-1) = f
5. f^(-1) is itself 1-to-1 and onto

# Construction / Recognition

## To Construct:
1. Verify f is 1-to-1 and onto
2. For each s' in S', find the unique s such that f(s) = s'
3. Define f^(-1)(s') = s

## To Recognize:
1. Given f and f', check if f'(f(s)) = s for all s (Condition A of Theorem 1.2.6.5)
2. Check if f(f'(s')) = s' for all s' (Condition B)
3. If both hold, f' = f^(-1)

# Context & Application

Inverse functions allow musical transformations to be "undone." The inverse of transposition by n semitones is transposition by -n. The inverse of inversion about a pitch p is inversion about the same pitch p. Understanding inverse functions is essential for group structure (every operation must have an inverse) and for analyzing retrograde and symmetrical musical structures.

# Examples

**Example 1** (p. 33): T5 (transpose up 5) has inverse T7 (= T(-5) mod 12). If T5(C) = F, then T7(F) = C.

**Example 2**: Inversion I about C is its own inverse: I(I(E)) = I(Ab) = E. So I^(-1) = I.

**Example 3**: If f(s) = s + 3 on integers, then f^(-1)(s') = s' - 3. f(5) = 8 and f^(-1)(8) = 5.

# Relationships

## Builds Upon
- **One-to-One Function** — required for inverse existence
- **Onto Function** — required for inverse existence

## Enables
- **Operation** — operations are characterized by having inverses
- **Group** — groups require every element to have an inverse

## Related
- **Inverse Element** — the abstract algebraic generalization of inverse functions

# Common Errors

- **Error**: Attempting to invert a function that is not 1-to-1.
  **Correction**: If f(a) = f(b) with a different from b, there is no unique preimage; f^(-1) does not exist.

- **Error**: Confusing f^(-1) with 1/f (the reciprocal).
  **Correction**: f^(-1) is the functional inverse (reversing the mapping), not the multiplicative reciprocal.

# Common Confusions

- **Confusion**: Thinking (fg)^(-1) = f^(-1)g^(-1).
  **Clarification**: The inverse of a composition reverses order: (fg)^(-1) = g^(-1)f^(-1).

# Source Reference

Chapter 1: Mathematical Preliminaries, Definitions 1.2.6.3, Theorems 1.2.6.4-1.2.6.5, pp. 33-34.

# Verification Notes

- Definition source: direct from Definitions 1.2.6.3 and Theorems 1.2.6.4-1.2.6.5
- Confidence rationale: explicit definitions and theorems with proofs noted
- Re-extracted from v2 card; preserved: T5/T7 example, inversion self-inverse example, composition reversal note
