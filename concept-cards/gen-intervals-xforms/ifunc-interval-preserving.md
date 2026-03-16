---
# === CORE IDENTIFICATION ===
concept: IFUNC Invariance Under Interval-Preserving Operations
slug: ifunc-interval-preserving

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
section: "5.1.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Theorem 5.1.5"
  - IFUNC P-invariance

# === TYPED RELATIONSHIPS ===
prerequisites:
  - ifunc
  - interval-preserving-operation
extends:
  - ifunc
related:
  - canonical-group
  - canonical-equivalence
  - ifunc-transposition-theorem
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "Why are interval-preserving operations considered canonical?"
  - "What happens to IFUNC when both sets are transformed by an interval-preserving operation?"
---

# Quick Definition
When both sets X and Y are transformed by the same interval-preserving operation P, their IFUNC remains unchanged: IFUNC(P(X), P(Y)) = IFUNC(X, Y).

# Core Definition
Theorem 5.1.5: "Let P be any interval-preserving operation. Then IFUNC(P(X), P(Y)) = IFUNC(X, Y) as a function on IVLS" (Lewin, p. 130). The proof constructs a bijection f(s, t) = (P(s), P(t)) between the relevant pair-sets, using the fact that P preserves int(s, t) = int(P(s), P(t)) and is 1-to-1.

# Prerequisites
- **IFUNC** — The function whose invariance is being established
- **Interval-Preserving Operation** — P must satisfy int(P(s), P(t)) = int(s, t) for all s, t

# Key Properties
1. IFUNC(P(X), P(Y))(i) = IFUNC(X, Y)(i) for every interval i
2. Applies to any interval-preserving operation P
3. In a commutative GIS, interval-preserving operations are exactly the transpositions
4. Provides the formal basis for canonical equivalence: if X' = P(X), then IFUNC(X', X') = IFUNC(X, X)

# Construction / Recognition
## To Apply:
1. Verify that P is interval-preserving: int(P(s), P(t)) = int(s, t) for all s, t
2. Conclude IFUNC(P(X), P(Y)) = IFUNC(X, Y)

## To Recognize:
1. Two sets with identical IFUNC self-values may be related by an interval-preserving operation

# Context & Application
This theorem justifies why interval-preserving operations should be included in any canonical group: they preserve the complete intervallic structure between sets. In a commutative GIS the interval-preserving operations are the transpositions, so transposing both sets preserves all IFUNC values. In a non-commutative GIS, the interval-preserving operations may differ from transpositions, and the choice of which to include in CANON has significant analytical consequences.

# Examples
**Example 1** (derived from discussion, p. 104): In pitch-class space with X = {C, E, G} and Y = {D, F#, A}, applying T_5 to both: IFUNC(T_5(X), T_5(Y)) = IFUNC({F, A, C}, {G, B, D}) = IFUNC(X, Y).

# Relationships
## Builds Upon
- **IFUNC** — Property of the interval function
- **Interval-Preserving Operation** — The class of operations that preserve IFUNC

## Enables
- **Canonical Group** — Justifies including interval-preserving operations in CANON
- **Canonical Equivalence** — P-equivalent sets have identical self-IFUNC values

## Related
- **IFUNC Transposition Theorem** — Different but related: describes effect of transposing one or both sets

# Common Errors
- **Error**: Applying this theorem when only one set is transformed
  **Correction**: Both sets must be transformed by the same P; transforming only one set changes IFUNC (see Theorem 5.1.6)

# Common Confusions
- **Confusion**: Thinking this means all transpositions preserve IFUNC in any GIS
  **Clarification**: In non-commutative GIS structures, transpositions are not interval-preserving. Only interval-preserving operations guarantee IFUNC invariance.

# Source Reference
Chapter 5: Generalized Set Theory (1), Theorem 5.1.5, p. 130.

# Verification Notes
- Definition source: Direct from Theorem 5.1.5 with proof sketch
- Confidence rationale: Explicit theorem with detailed proof in source
- Re-extraction notes: Re-extracted from v2 card; preserved: pitch-class example, emphasis on commutative vs non-commutative distinction. Added v3.1 structure.
