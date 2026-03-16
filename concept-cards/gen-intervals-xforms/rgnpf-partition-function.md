---
concept: "RGNPF (Regener's Generalized Partition Function)"
slug: rgnpf-partition-function

category: generalized-set-theory
subcategory: injection-function
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.9"

extraction_confidence: high

aliases:
  - Regener Partition Function
  - partition function

prerequisites:
  - inj-function
  - canonical-group
  - k-relation-generalized
extends:
  - k-relation-generalized
related:
  - emb-function
contrasts_with: []

answers_questions:
  - "What is the generalized partition function?"
  - "How does RGNPF relate to EMB?"
---

# Quick Definition
RGNPF(X, Y)(N) counts how many canonical operations A satisfy INJ(X, Y)(A) = N, partitioning CANON by INJ values and giving the complete distribution of how X relates to Y under all canonical operations.

# Core Definition
Section 6.9 (Lewin, p. 185): "When CANON is finite, for each integer N between 0 and cardX inclusive, RGNPF(X, Y)(N) is the number of members A of CANON satisfying INJ(X, Y)(A) = N." Named for Eric Regener. Key formula: EMB(X, Y) = RGNPF(X, Y)(cardX) / RGNPF(X, X)(cardX). RGNPF counts operations; EMB counts forms. The denominator corrects for symmetry.

# Prerequisites
- **INJ Function** — RGNPF counts INJ value occurrences
- **Canonical Group** — CANON must be finite
- **K-Relation Generalized** — K/Kh multiplicities are RGNPF values at extremes

# Key Properties
1. RGNPF(X, Y)(N) = |{A in CANON : INJ(X, Y)(A) = N}|
2. Sum over N of RGNPF(X, Y)(N) = |CANON|
3. RGNPF(X, Y)(cardX) = K_1 multiplicity; RGNPF(X, Y)(0) = K_2 multiplicity
4. EMB(X, Y) = RGNPF(X, Y)(cardX) / RGNPF(X, X)(cardX)
5. If X has M symmetries (non-identity operations fixing X), denominator = M + 1

# Construction / Recognition
## To Compute:
1. For each A in CANON, compute INJ(X, Y)(A)
2. Tally results by value N
3. The tally for each N is RGNPF(X, Y)(N)

## To Recognize:
1. A histogram of INJ values over canonical operations

# Context & Application
RGNPF provides the complete picture beyond K/Kh, showing the full distribution of how X relates to Y under canonical operations. For symmetric sets, the EMB/RGNPF formula accounts for operations that produce the same embedded form.

# Examples
**Example 1** (derived from p. 185): X = augmented triad {C, E, G#}, CANON = transpositions. RGNPF(X, X)(3) = 3 (T_0, T_4, T_8 all fix X). EMB(X, X) = 1 (only one form embedded in itself). Formula: 1 = 3/3.

**Example 2**: X = major triad, Y = major scale, CANON = transpositions. RGNPF(X, Y)(3) = 3 (three transpositions embed X fully). Sum over all N = 12 (all 12 transpositions accounted for).

# Relationships
## Builds Upon
- **K-Relation Generalized** — RGNPF gives the full distribution, not just extremes

## Enables
- **EMB Function** — EMB derivable from RGNPF via the formula

# Common Errors
- **Error**: Confusing RGNPF (counts operations) with EMB (counts forms)
  **Correction**: Symmetric sets have multiple operations per form; RGNPF counts operations, EMB counts forms

# Common Confusions
- **Confusion**: Thinking RGNPF works for infinite CANON
  **Clarification**: Finite CANON required; for infinite CANON, measure-theoretic generalizations are needed (section 6.10)

# Source Reference
Chapter 6: Generalized Set Theory (2), section 6.9, p. 185.

# Verification Notes
- Definition source: Direct from section 6.9
- Confidence rationale: Explicit definition with formula
- Re-extraction notes: Re-extracted from v2 card; preserved: EMB derivation formula, augmented triad symmetry example, major triad/scale example. Added v3.1 structure.
