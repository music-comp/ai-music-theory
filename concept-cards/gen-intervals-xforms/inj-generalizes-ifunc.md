---
concept: INJ Generalizes IFUNC Theorem
slug: inj-generalizes-ifunc

category: generalized-set-theory
subcategory: injection-function
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.7.1"

extraction_confidence: high

aliases:
  - "Theorem 6.7.1"

prerequisites:
  - inj-function
  - ifunc
  - transposition-operation
extends:
  - inj-function
  - ifunc
related:
  - inj-transformation-theorem
contrasts_with: []

answers_questions:
  - "How does IFUNC relate to INJ?"
  - "In what sense does INJ generalize IFUNC?"
---

# Quick Definition
In any GIS, IFUNC is a special case of INJ: IFUNC(X, Y)(i) = INJ(X, Y)(T_i). This shows INJ is the more fundamental concept, with IFUNC arising when the transformation is a transposition.

# Core Definition
Theorem 6.7.1 (Lewin, p. 179): "Let (S, IVLS, int) be a GIS. Then for each interval i and for all sets X and Y, IFUNC(X, Y)(i) = INJ(X, Y)(T_i)." The proof shows that counting pairs (s, t) with s in X, t in Y, int(s, t) = i is the same as counting elements s of X with T_i(s) in Y.

# Prerequisites
- **INJ Function** — The generalizing function
- **IFUNC** — The function being generalized
- **Transposition Operation** — T_i is the specific transformation connecting the two

# Key Properties
1. IFUNC(X, Y)(i) = INJ(X, Y)(T_i) — exact equality
2. INJ handles all transformations; IFUNC only handles transpositions
3. All IFUNC theorems become special cases of INJ theorems
4. INJ works without a GIS; IFUNC requires one

# Construction / Recognition
## To Apply:
1. In a GIS, any IFUNC computation can be done via INJ with transpositions
2. Conversely, INJ with transpositions reduces to IFUNC

## To Recognize:
1. When analyzing transposition-based relationships, IFUNC and INJ are equivalent

# Context & Application
This theorem justifies INJ as the "master function." Lewin visualizes: imagine X and Y as point configurations in a plane; interval i as a vector. IFUNC asks "how many arrows of that vector go from X to Y?" INJ asks "if I translate X by that vector, how many points coincide with Y?" These are the same question differently phrased.

# Examples
**Example 1** (p. 179): Geometric visualization — X and Y as point configurations. "To the right and up 30 degrees for 5 inches" as an interval. IFUNC counts distinct arrows; INJ counts coincidences after translation. They are equal.

**Example 2**: IFUNC({C, E}, {G, B})(7) = 2 = INJ({C, E}, {G, B})(T_7).

# Relationships
## Builds Upon
- **IFUNC** — Shows IFUNC is a special case of INJ
- **INJ Function** — INJ is the more general concept

## Enables
- **INJ Transformation Theorem** — Generalizes IFUNC transformation theorems
- **System Modulation** — Extends IFUNC questions to arbitrary transformations

# Common Errors
- **Error**: Thinking INJ only works for transpositions
  **Correction**: INJ works for ANY transformation; the theorem shows transpositions yield IFUNC as a special case

# Common Confusions
- **Confusion**: Thinking IFUNC and INJ are redundant
  **Clarification**: INJ is strictly more general — it handles non-GIS contexts and non-operation transformations that IFUNC cannot

# Source Reference
Chapter 6: Generalized Set Theory (2), Theorem 6.7.1, p. 179.

# Verification Notes
- Definition source: Direct from Theorem 6.7.1 with proof
- Confidence rationale: Explicit theorem with proof and geometric visualization
- Re-extraction notes: Re-extracted from v2 card; preserved: geometric visualization, pitch-class example. Added v3.1 structure.
