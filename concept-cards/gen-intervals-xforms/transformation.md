---
# === CORE IDENTIFICATION ===
concept: Transformation
slug: transformation

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
section: "1.3.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - function
extends:
  - function
related:
  - semigroup-of-transformations
  - identity-transformation
contrasts_with:
  - operation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What mathematical concepts must I know before understanding GIS?"
---

# Quick Definition

A transformation on a set S is a function from S into S itself -- it maps elements of S to other elements of S.

# Core Definition

"A function from a family S into S itself will be called a transformation on S" (Lewin, Definition 1.3.1, p. 34). A transformation is distinguished from a general function by the requirement that the domain and codomain are the same family. If the transformation is additionally 1-to-1 and onto, it is called an operation.

# Prerequisites

- **Function** — a transformation is a special case of a function

# Key Properties

1. Domain equals codomain: f: S -> S
2. Transformations can always be composed with other transformations on the same S
3. A transformation need not be 1-to-1 or onto
4. If a transformation is both 1-to-1 and onto, it is an operation

# Construction / Recognition

## To Construct:
1. Fix a set S
2. Define a rule that maps each element of S to some element of S

## To Recognize:
1. Check that the function maps S into S itself (not into a different set)
2. It is a transformation regardless of whether it is 1-to-1 or onto

# Context & Application

Transformations are the core objects of study in Lewin's theory. Musical transformations include transposition, inversion, and many others. The key property is self-referentiality: transformations map musical objects (pitches, pitch classes, chords, time points) back to objects of the same type, allowing chains of transformations and algebraic structure.

# Examples

**Example 1** (p. 34): On pitch classes, Tn (transposition by n) maps each pitch class to another pitch class in the same set. This is a transformation on the set of 12 pitch classes.

**Example 2** (p. 34): The identity operation 1 on S, defined by 1(s) = s, is a transformation.

**Non-example**: The function mapping pitches to their pitch classes is NOT a transformation on pitches because the codomain (pitch classes) differs from the domain (pitches).

# Relationships

## Builds Upon
- **Function** — a transformation is a function with S = S'

## Enables
- **Operation** — an operation is a bijective transformation
- **Semigroup of Transformations** — a closed collection of transformations
- **Identity Transformation** — the "do nothing" transformation

## Contrasts With
- **Operation** — an operation is a transformation that is additionally 1-to-1 and onto

# Common Errors

- **Error**: Assuming all transformations are invertible.
  **Correction**: Only operations (1-to-1, onto transformations) have inverses. A transformation collapsing all pitches to C has no inverse.

# Common Confusions

- **Confusion**: Using "transformation" and "operation" interchangeably.
  **Clarification**: In Lewin's technical usage, every operation is a transformation, but not every transformation is an operation. The distinction matters for group theory.

# Source Reference

Chapter 1: Mathematical Preliminaries, Definition 1.3.1, p. 34.

# Verification Notes

- Definition source: direct from Definition 1.3.1
- Confidence rationale: explicit, concise definition in source
- Re-extracted from v2 card; preserved: pitch-class non-example, emphasis on self-mapping
