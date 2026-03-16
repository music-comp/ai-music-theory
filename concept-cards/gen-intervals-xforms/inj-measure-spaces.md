---
concept: INJ for Measure Spaces
slug: inj-measure-spaces

category: generalized-set-theory
subcategory: injection-function
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.10"

extraction_confidence: medium

aliases:
  - measure-theoretic INJ
  - continuous INJ

prerequisites:
  - inj-function
extends:
  - inj-function
related:
  - shadow-function
  - time-span-gis
contrasts_with: []

answers_questions:
  - "How can INJ be generalized to infinite sets using measure theory?"
---

# Quick Definition
INJ can be generalized to infinite sets using measure theory: INJ(X, Y)(f) = mes(X intersect f^{-1}(Y)), where mes is a measure on a sigma-field of subsets of S. This replaces cardinality with measure, answering "how much of X maps into Y?"

# Core Definition
Section 6.10 (Lewin, pp. 186-189, marked optional): Given a family S, a field FLD of subsets, a measure mes on FLD, sets X and Y of finite measure, and a measurable transformation f: INJ(X, Y)(f) = mes(X intersect f^{-1}(Y)). This generalizes the finite case where mes = counting measure. Theorem 6.5.1 and subsequent formulas require "measure-preserving" operations (mesOP(X) = mesX).

# Prerequisites
- **INJ Function** — The finite version being generalized

# Key Properties
1. INJ(X, Y)(f) = mes({s in X : f(s) in Y}) = mes(X intersect f^{-1}(Y))
2. When mes = counting measure on finite sets: reduces to ordinary INJ
3. Operations must be measure-preserving for Theorem 6.5.1 to apply
4. Measure-scaling operations (mesOP(X) = scale * mesX) allow more flexible formulas
5. RGNPF generalizes when CANON has a "good" measure

# Construction / Recognition
## To Apply:
1. Choose S, FLD, and mes appropriate to the musical context
2. Identify sets X, Y of finite measure and measurable transformation f
3. Compute INJ(X, Y)(f) = mes(X intersect f^{-1}(Y))

## To Recognize:
1. Questions about "how much" (not "how many") of one set maps into another

# Context & Application
Measure-theoretic INJ enables questions like: "Of the time the violin plays above high C, how much of that time maps to clarinet pianissimo moments 5 seconds later?" Also applicable to the Seurat painting analogy (area measure), time-span analysis (P-invariant or T-invariant measures), and continuous models generally.

# Examples
**Example 1** (pp. 187-188): Seurat painting — S = Euclidean plane, mes = area, X = animal regions, Y = plant regions, f = "shift 3cm at 45 degrees." INJ(X, Y)(f)/areaX = fraction of animal area 3cm below-left of plants.

**Example 2** (pp. 188-189): Time-point continuum — S = real numbers, mes = duration, X = times violin plays above high C, Y = times clarinet plays pp, f = T_5 (shift 5 seconds). INJ(X, Y)(f)/mesX answers the question about temporal coincidence.

**Example 3** (pp. 189-190): Time-span half-plane — with P-invariant or T-invariant measures, one can analyze continuous time-span sets with INJ.

# Relationships
## Builds Upon
- **INJ Function** — Generalizes the finite version

## Enables
- **Shadow Function** — SHADOW sets used in time-span measure contexts

## Related
- **Time-Span GIS** — Time-span measures connect to the non-commutative GIS

# Common Errors
- **Error**: Applying finite-set INJ formulas to continuous settings without verifying measure-preservation
  **Correction**: Check that operations are measure-preserving before applying Theorem 6.5.1 analogs

# Common Confusions
- **Confusion**: Thinking measure-theoretic INJ is needed for most analyses
  **Clarification**: Most musical applications use finite sets; this extension is for advanced continuous models

# Source Reference
Chapter 6: Generalized Set Theory (2), section 6.10 (optional), pp. 186-190.

# Verification Notes
- Definition source: Direct from section 6.10
- Confidence rationale: Medium — presented as optional/advanced material
- Re-extraction notes: Re-extracted from v2 card; preserved: Seurat example, time-point example, P-invariant/T-invariant measures. Added v3.1 structure.
