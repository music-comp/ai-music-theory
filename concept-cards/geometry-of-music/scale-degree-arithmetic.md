---
concept: Scale Degree Arithmetic
slug: scale-degree-arithmetic

category: scales-modes
subcategory: operations
tier: intermediate-advanced

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Scales"
chapter_number: 4
pdf_page: 137
section: "4.2"

extraction_confidence: high

aliases:
  - "modular scale arithmetic"
  - "scale degree numbers"

prerequisites:
  - scale-as-ruler
extends: []
related:
  - scalar-transposition
  - scalar-inversion
contrasts_with: []

answers_questions:
  - "How do scale degree numbers work?"
  - "What is scale degree arithmetic?"
---

# Quick Definition
A numbering system for scale notes (starting from 1) with modular arithmetic: for an n-note scale, add or subtract n until the result lies between 1 and n. This is formally identical to pitch-class arithmetic but uses 1-based indexing.

# Core Definition
Scale degree arithmetic assigns sequential numbers to scale notes, starting from 1 for an arbitrarily chosen note. For an n-note scale, arithmetic is modular: when results exceed n or fall below 1, add or subtract n until the result is in range. For a 7-note scale: degree 7 + 1 = degree 1 (wrapping around). This is formally identical to pitch-class arithmetic (which starts from 0), with the trivial difference of a 1-based versus 0-based starting point. Scale degree numbers are dependent on the arbitrary choice of first degree, but scalar distances (the differences between degrees) are not. The traditional terminology for scalar distances is confusing: a "second" = 1 step, a "third" = 2 steps, a "fourth" = 3 steps, etc.

# Prerequisites
- Scale as ruler

# Key Properties
1. 1-based indexing (unlike 0-based pitch-class labels)
2. Modular arithmetic: wrap at the scale's cardinality
3. Choice of first degree is arbitrary and does not imply tonal significance
4. Scalar distances are independent of the choice of first degree
5. Traditional naming convention offsets by 1: "second" = 1 step, "third" = 2 steps, etc.

# Construction / Recognition
## To Assign:
1. Choose any note as scale degree 1
2. Number successive ascending notes as 2, 3, ..., n
3. For arithmetic: add/subtract n to keep results in range [1, n]

# Context & Application
Scale degree arithmetic is the mathematical backbone for scalar transposition and inversion. It also provides the framework for interscalar transposition (Section 4.8), where scale degrees from one scale are mapped to scale degrees of another.

# Examples
**Example 1** (p. 138): In C major (7-note), to transpose (F, G, A) = (4, 5, 6) up by 3: (7, 8, 9) -> subtract 7 from numbers > 7 -> (7, 1, 2) = (B, C, D).
**Example 2** (p. 138): To invert (C, D, F) = (1, 2, 4) around E (degree 3): subtract from 6 -> (5, 4, 2) = (G, F, D).

# Relationships
## Builds Upon
- **scale-as-ruler** — Provides the scale that defines the numbering
## Enables
- **scalar-transposition** — Uses this arithmetic
- **scalar-inversion** — Uses this arithmetic

# Common Errors
- **Error**: Confusing "scale degree 1" with "tonic"
  **Correction**: The choice of degree 1 is arbitrary; it does not designate a tonic

# Common Confusions
- **Confusion**: Why do music theorists start scale degrees from 1 but pitch classes from 0?
  **Clarification**: Historical convention. The two systems are formally identical up to a trivial offset.

# Source Reference
Chapter 4: Scales, Section 4.2, pages 137-138.

# Verification Notes
- Definition source: From Section 4.2
- Confidence rationale: High — formally defined
- Cross-reference status: Verified against footnotes 6-7
