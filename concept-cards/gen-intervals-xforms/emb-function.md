---
concept: "EMB (Embedding Function)"
slug: emb-function

category: generalized-set-theory
subcategory: embedding-functions
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.3.1-5.3.2"

extraction_confidence: high

aliases:
  - embedding number
  - "EMB(X, Y)"

prerequisites:
  - canonical-group
  - set-class
extends:
  - set-class
related:
  - m-class-vector
  - emb-probability
  - emb-decomposition-theorem
  - cov-function
  - rgnpf-partition-function
contrasts_with:
  - ifunc
  - inj-function

answers_questions:
  - "What is an embedding function (EMB)?"
  - "How does EMB generalize Forte's interval vector?"
---

# Quick Definition
EMB(X, Y) counts the number of forms of X (members of set class /X/) that are included in Y. It generalizes Forte's interval vector to arbitrary set cardinalities and any GIS.

# Core Definition
Definition 5.3.1: "Given sets X and Y, the embedding number of X in Y, EMB(X, Y), is the number of forms of X (i.e. members of /X/) that are included in Y" (Lewin, p. 138). The embedding number depends on CANON though the notation does not show this. Definition 5.3.2 establishes that EMB is well-defined on set classes: EMB(/X/, Y), EMB(X, /Y/), and EMB(/X/, /Y/) are all meaningful.

# Prerequisites
- **Canonical Group** — EMB depends critically on the choice of CANON
- **Set Class** — EMB counts members of /X/ embedded in Y

# Key Properties
1. EMB(X', Y) = EMB(X, Y) for any X' in /X/
2. EMB(X, Y') = EMB(X, Y) for any Y' in /Y/
3. EMB(/X/, /Y/) is therefore well-defined
4. In the time-span GIS with CANON = interval-preserving operations: EMB(D, X) = IFUNC(X, X)(i, p) for dyad D with forwards-oriented interval (i, p)
5. Strictly, one should write EMB(CANON, X, Y) but the notation is already cumbersome

# Construction / Recognition
## To Compute EMB(X, Y):
1. Enumerate all forms A(X) for A in CANON
2. Count how many are subsets of Y
3. This count is EMB(X, Y)

## To Recognize:
1. Any count of how many instances of a pattern type appear within a set

# Context & Application
EMB generalizes Forte's interval vector. The interval vector counts embeddings of dyad classes (2-note set types) within a set. EMB extends this to trichord types, tetrachord types, or any cardinality. The "M-class vector" of Y gives EMB(/X/, Y) as /X/ ranges over all M-element set classes. In the time-span GIS, with interval-preserving operations as CANON, the interval vector is literally EMB for dyads.

# Examples
**Example 1** (p. 138): X = major triad, Y = C major scale. CANON = transpositions only: EMB(X, Y) = 3 (three major triads in the scale). CANON = transpositions + inversions: EMB(X, Y) = 6 (six harmonic triads in the scale).

**Example 2** (p. 114): In the time-span GIS, EMB(D, X) = IFUNC(X, X)(i, p) for dyad D, providing a "very strong formal analog for Forte's interval vector."

# Relationships
## Builds Upon
- **Set Class** — EMB counts forms (members of set classes)
- **Canonical Group** — EMB depends on CANON

## Enables
- **M-Class Vector** — EMB values across all M-element set classes
- **EMB Probability** — Probabilistic interpretation via Theorem 5.3.4
- **EMB Decomposition Theorem** — Relates EMB at different cardinalities

## Related
- **COV Function** — COV(X, Y) counts forms of Y containing X
- **RGNPF Partition Function** — Generalizes EMB counting via INJ

## Contrasts With
- **IFUNC** — IFUNC counts intervals between sets; EMB counts embedded forms
- **INJ Function** — INJ generalizes both IFUNC and EMB

# Common Errors
- **Error**: Forgetting that EMB depends on CANON
  **Correction**: Always specify the canonical group; the same X and Y yield different EMB values under different CANON

# Common Confusions
- **Confusion**: Thinking EMB(X, Y) = EMB(Y, X)
  **Clarification**: EMB is asymmetric — it counts forms of X in Y, not forms of Y in X

# Source Reference
Chapter 5: Generalized Set Theory (1), Definitions 5.3.1-5.3.2, pp. 138-139.

# Verification Notes
- Definition source: Direct from Definitions 5.3.1 and 5.3.2
- Confidence rationale: Explicit definitions with examples
- Re-extraction notes: Re-extracted from v2 card; preserved: major scale examples, CANON dependence emphasis. Added time-span GIS connection, v3.1 structure.
