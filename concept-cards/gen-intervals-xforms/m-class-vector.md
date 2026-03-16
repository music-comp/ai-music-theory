---
# === CORE IDENTIFICATION ===
concept: M-Class Vector
slug: m-class-vector

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: embedding-functions
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.3.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - M-element class vector
  - dyad-type vector
  - trichord-type vector

# === TYPED RELATIONSHIPS ===
prerequisites:
  - emb-function
  - set-class
  - canonical-group
extends:
  - emb-function
related:
  - emb-decomposition-theorem
  - emb-probability
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an M-class vector?"
  - "How does the M-class vector generalize Forte's interval vector?"
---

# Quick Definition
The M-class vector of a set Y lists the values EMB(/X/, Y) as /X/ runs through all set classes whose members have cardinality M. When M=2, this is Forte's interval vector.

# Core Definition
Definition 5.3.3 (Lewin, p. 139): The M-class vector of Y gives EMB(/X/, Y) as /X/ ranges over the M-member set classes. Only finitely many set classes have nonzero EMB values (since Y is finite). The 2-class vector is the "dyad-type vector" (Forte's interval vector); the 3-class vector is the "trichord-type vector," and so on.

# Prerequisites
- **EMB Function** — M-class vector is a collection of EMB values
- **Set Class** — The vector ranges over M-element set classes
- **Canonical Group** — The number and identity of set classes depend on CANON

# Key Properties
1. M-class-vector(Y): {M-element set classes} -> non-negative integers
2. Only finitely many entries are nonzero
3. Sum of all entries = COMB(M, |Y|) = total M-element subsets of Y
4. The 2-class vector = Forte's interval vector (with appropriate CANON)
5. Number of entries depends on CANON: transpositions only gives more classes than transpositions + inversions

# Construction / Recognition
## To Construct:
1. Fix CANON and enumerate all M-element set classes
2. For each class /X/, compute EMB(/X/, Y)
3. The resulting function is the M-class vector of Y

## To Recognize:
1. A tabulation of how many of each M-element pattern type appear within a set

# Context & Application
The interval vector is the best-known special case (M=2). Trichord-type vectors (M=3) are also analytically useful. In the time-span GIS, the 2-class vector (interval vector) has a particularly strong form because EMB(D, X) = IFUNC(X, X)(i, p) for dyads.

# Examples
**Example 1** (p. 139): Y = C major scale, CANON = transpositions + inversions. 2-class vector (interval vector): [2, 5, 4, 3, 6, 1]. 3-class vector lists EMB values for all 12 trichord classes.

**Example 2** (p. 139): With CANON = transpositions only, the 3-class vector has 19 entries instead of 12, because major and minor triads are distinct set classes.

# Relationships
## Builds Upon
- **EMB Function** — Vector entries are EMB values

## Enables
- **EMB Decomposition Theorem** — Relates M-class vectors at different cardinalities

## Related
- **EMB Probability** — Each vector entry has a probabilistic interpretation

# Common Errors
- **Error**: Assuming the M-class vector has a fixed number of entries
  **Correction**: The number of entries depends on both M and the canonical group

# Common Confusions
- **Confusion**: Thinking the interval vector is the only M-class vector
  **Clarification**: It is the M=2 case; higher-cardinality vectors provide additional structural information

# Source Reference
Chapter 5: Generalized Set Theory (1), Definition 5.3.3, p. 139.

# Verification Notes
- Definition source: Direct from Definition 5.3.3
- Confidence rationale: Explicit definition in source
- Re-extraction notes: Re-extracted from v2 card; preserved: interval vector connection, CANON dependence. Added v3.1 structure.
