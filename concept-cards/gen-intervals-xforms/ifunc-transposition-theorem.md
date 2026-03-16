---
# === CORE IDENTIFICATION ===
concept: IFUNC Transposition Theorem
slug: ifunc-transposition-theorem

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
section: "5.1.6"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Theorem 5.1.6"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - ifunc
  - transposition-operation
extends:
  - ifunc
related:
  - ifunc-symmetry-theorem
  - ifunc-interval-preserving
  - ifunc-inversion-theorem
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does transposing one or both sets affect IFUNC?"
  - "What is the effect of transposition on interval functions?"
---

# Quick Definition
When one or both sets are transposed, IFUNC values shift according to specific formulas involving composition of intervals in the group IVLS.

# Core Definition
Theorem 5.1.6 (Lewin, p. 131): For any transposition operation T_n, the following formulas obtain:
- (A): IFUNC(T_n(X), Y)(i) = IFUNC(X, Y)(ni)
- (B): IFUNC(X, T_n(Y))(i) = IFUNC(X, Y)(in^{-1})
- (C): IFUNC(T_n(X), T_n(Y))(i) = IFUNC(X, Y)(nin^{-1})

Formula (A) is proved by constructing a bijection f(s, t) = (T_n(s), t) between relevant pair-sets. Formula (B) follows from (A) and the symmetry theorem 5.1.4. Formula (C) follows from applying (A) then (B).

# Prerequisites
- **IFUNC** — The function whose behavior under transposition is described
- **Transposition Operation** — T_n maps s to the unique t with int(s, t) = n

# Key Properties
1. Transposing X shifts the interval argument by left-multiplication: ni
2. Transposing Y shifts by right-multiplication by inverse: in^{-1}
3. In a commutative GIS with additive notation: IFUNC(T_n(X), Y)(i) = IFUNC(X, Y)(n + i)
4. In a commutative GIS: IFUNC(T_n(X), T_n(Y))(i) = IFUNC(X, Y)(i) — transposing both sets preserves IFUNC
5. In a non-commutative GIS: IFUNC(T_n(X), T_n(Y))(i) = IFUNC(X, Y)(nin^{-1}), which may differ from IFUNC(X, Y)(i)

# Construction / Recognition
## To Apply:
1. Identify the transposition interval n
2. Use the appropriate formula (A), (B), or (C) depending on which sets are transposed
3. In commutative GIS, simplify using additive group operations

## To Recognize:
1. When IFUNC values between transposed sets need to be related to the original IFUNC

# Context & Application
These formulas are essential for understanding how intervallic relationships change under transposition. In atonal theory (commutative GIS), transposing both sets by the same interval preserves all IFUNC values. But in non-commutative settings (e.g., time-span GIS), transposing both sets may genuinely alter IFUNC, showing that transpositions are not always "canonical" in the non-commutative case.

# Examples
**Example 1** (derived from discussion, p. 131): In pitch-class space, IFUNC(T_2(X), Y)(5) = IFUNC(X, Y)(2 + 5) = IFUNC(X, Y)(7). To find interval-5 connections from T_2(X) to Y, look at interval-7 connections from X to Y.

# Relationships
## Builds Upon
- **IFUNC** — The function being analyzed
- **Transposition Operation** — The transformation applied

## Enables
- **IFUNC Interval-Preserving** — In commutative GIS, (C) reduces to invariance
- **INJ Transformation Theorem** — Generalizes these results via INJ

## Related
- **IFUNC Symmetry Theorem** — Used in the proof of formula (B)

# Common Errors
- **Error**: Using additive shift formulas in a non-commutative GIS
  **Correction**: The formulas involve group multiplication; only in commutative groups does nin^{-1} = i

# Common Confusions
- **Confusion**: Assuming transposing both sets always preserves IFUNC
  **Clarification**: This holds only in commutative GIS. In non-commutative GIS, formula (C) shows IFUNC is conjugated, not preserved.

# Source Reference
Chapter 5: Generalized Set Theory (1), Theorem 5.1.6 (A)-(C), pp. 131-132.

# Verification Notes
- Definition source: Direct from Theorem 5.1.6 with proof sketches
- Confidence rationale: Explicit theorem with proofs in source
- Re-extraction notes: Re-extracted from v2 card; preserved: commutative vs non-commutative distinction, pitch-class example. Added v3.1 structure.
