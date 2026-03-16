---
# === CORE IDENTIFICATION ===
concept: Functional Equality
slug: functional-equality

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
section: "1.2.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - equality of functions

# === TYPED RELATIONSHIPS ===
prerequisites:
  - function
extends: []
related:
  - composition-of-functions
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
  - "When are two functions considered the same?"
---

# Quick Definition

Two functions f and g from S into S' are equal if they produce the same table -- that is, if f(s) = g(s) for every s in S.

# Core Definition

"Given families S and S', we shall say that the functions f and g from S into S' are the same, writing f = g, if f and g are the same subsets of S x S', that is if they produce the same table" (Lewin, Definition 1.2.2, p. 32). Two functions are equal when they assign identical values to every argument, regardless of how they are defined or computed.

# Prerequisites

- **Function** — must understand what a function is before defining when two are equal

# Key Properties

1. Equality is determined by input-output behavior, not by definition method
2. Two functions with different formulas can be equal if they produce the same values
3. Functional equality is the basis for verifying functional equations like f'f = f"

# Construction / Recognition

## To Construct:
1. Not applicable (equality is a relation, not a construction)

## To Recognize:
1. Given two functions f and g with the same domain S and codomain S'
2. Check that f(s) = g(s) for every s in S
3. If all values match, f = g; otherwise f and g are distinct

# Context & Application

Lewin stresses this definition because functional equality is the basis for verifying all functional equations in the text. When Lewin writes f'f = f", this means: for every s in S, computing f'(f(s)) gives the same result as computing f"(s). This principle underlies the verification of composition equations throughout the algebraic framework.

# Examples

**Example 1** (p. 32): Let f1(s) = s + 3, f2(s) = 2s, f3(s) = 2s, and f4(s) = s + 6 on positive integers. The equation f2f1 = f4f3 asserts that for any integer s, doubling (s + 3) equals adding 6 to (2s). Both give 2s + 6, confirming equality.

**Example 2** (p. 32): Let f(s) = s transposed by 2, f'(s) = s inverted about C, and f"(s) = s inverted about B on pitch classes. The equation f'f = f" asserts that for every pitch class s, inverting about C the 2-transpose of s equals the inversion about B of s.

# Relationships

## Builds Upon
- **Function** — equality is defined between functions

## Enables
- **Composition of Functions** — functional equations about compositions rely on this definition

## Related
- **Composition of Functions** — composition equations are verified via functional equality

# Common Errors

- **Error**: Concluding two functions are different because their formulas look different.
  **Correction**: Compare actual output values for all arguments, not symbolic expressions.

# Common Confusions

- **Confusion**: Thinking functional equality requires identical definitions.
  **Clarification**: f(s) = 2(s + 3) and g(s) = 2s + 6 define the same function because they produce the same values for every input.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.2.2, p. 32. Related discussion in Section 1.2.5 on verifying functional equations.

# Verification Notes

- Definition source: direct from Definition 1.2.2
- Confidence rationale: explicit definition with clear emphasis in source text
- New card (no prior version existed)
