---
# === CORE IDENTIFICATION ===
concept: "Z-Relation (Generalized)"
slug: z-relation-generalized

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: interval-functions
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: null

# === CONFIDENCE ===
extraction_confidence: medium

# === VARIANTS ===
aliases:
  - Z-related sets
  - Z-relation

# === TYPED RELATIONSHIPS ===
prerequisites:
  - ifunc
  - canonical-equivalence
  - set-class
extends:
  - ifunc
related:
  - convolution-interpretation
  - ifunc-interval-preserving
contrasts_with:
  - canonical-equivalence

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the generalized Z-relation?"
  - "Under what conditions do non-equivalent sets share identical interval content?"
---

# Quick Definition
Two sets are Z-related if they have identical IFUNC self-values (same internal interval structure) but are not canonically equivalent. This generalizes Forte's Z-relation to any GIS setting.

# Core Definition
In Forte's theory, "pitch-class sets X1 and X2 which are not transposed or inverted forms of each other are Z-related if and only if IFUNC(X1, X1) = IFUNC(X2, X2), as a function on IVLS" (Lewin, p. 134). In a general GIS setting, Lewin poses this as an open research question: under what conditions on X1 and X2 does IFUNC(X1, X1) = IFUNC(X2, X2)? One known sufficient condition: if X2 = P(X1) for an interval-preserving operation P, then IFUNC(X2, X2) = IFUNC(X1, X1) by Theorem 5.1.5. In non-commutative GIS, a transposed set T_n(X1) may NOT share self-IFUNC values with X1.

# Prerequisites
- **IFUNC** — Z-relation is defined through identity of IFUNC self-values
- **Canonical Equivalence** — Z-related sets must NOT be canonically equivalent
- **Set Class** — Z-related sets belong to distinct set classes

# Key Properties
1. Z-related sets share identical IFUNC(X, X) but belong to different set classes
2. In commutative GIS, transposed sets always share self-IFUNC (so Z-relation only distinguishes non-transpositionally related sets)
3. In non-commutative GIS, even transposed sets may have different self-IFUNC values
4. The phenomenon shows that IFUNC does not completely determine set-class membership
5. The question generalizes further: when does IFUNC(X1, Y1) = IFUNC(X2, Y2) for four arbitrary sets?

# Construction / Recognition
## To Identify Z-Related Sets:
1. Compute IFUNC(X1, X1) and IFUNC(X2, X2)
2. If the functions are identical, check whether X2 is a form of X1
3. If not canonically equivalent but same self-IFUNC, the sets are Z-related

## To Recognize:
1. Different set classes with identical interval vectors (in the Fortean sense)

# Context & Application
Lewin describes the Z-relation as "a vast open ground for mathematical and musical inquiry, even in atonal set-theory" (p. 134). The phenomenon is connected to convolution theory: Z-related sets have identical self-convolutions of their characteristic functions. This links the music-theoretic question to deep problems in harmonic analysis on groups.

# Examples
**Example 1** (from standard atonal theory): {0, 1, 4, 6} (Forte 4-Z15) and {0, 1, 3, 7} (Forte 4-Z29) both have interval vector [1, 1, 1, 1, 1, 1] but are not related by transposition or inversion.

**Example 2** (p. 120, Figure 5.1): Different pairs of sets produce identical IFUNC values: IFUNC(X1, Y1) = IFUNC(X1, Y2) = IFUNC(X2, Y3) = IFUNC(X3, Y4), with sets of varying cardinalities.

# Relationships
## Builds Upon
- **IFUNC** — Z-relation is defined through IFUNC equality
- **Set Class** — Z-related sets belong to distinct set classes

## Enables
- **Convolution Interpretation** — Z-sets share identical self-convolutions

## Contrasts With
- **Canonical Equivalence** — Canonically equivalent sets always share IFUNC; Z-related sets share IFUNC without being equivalent

# Common Errors
- **Error**: Assuming sets with the same interval vector must be related by transposition or inversion
  **Correction**: Z-related sets share interval vectors without being canonically equivalent

# Common Confusions
- **Confusion**: Thinking the Z-relation means sets are "the same"
  **Clarification**: Z-related sets share internal interval distributions but may sound quite different and serve different compositional functions

# Source Reference
Chapter 5: Generalized Set Theory (1), discussion following Theorem 5.1.8, pp. 133-135.

# Verification Notes
- Definition source: Synthesized from Lewin's discussion of Forte's Z-relation in generalized context
- Confidence rationale: Medium — presented as an open research question rather than a theorem
- Re-extraction notes: Re-extracted from v2 card; preserved: Forte 4-Z15/4-Z29 example, emphasis on open research questions, non-commutative caveat. Added v3.1 structure.
