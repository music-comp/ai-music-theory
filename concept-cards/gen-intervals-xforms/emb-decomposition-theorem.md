---
# === CORE IDENTIFICATION ===
concept: EMB Decomposition Theorem
slug: emb-decomposition-theorem

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
section: "5.3.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Theorem 5.3.5.2"
  - EMB factorization

# === TYPED RELATIONSHIPS ===
prerequisites:
  - emb-function
  - emb-probability
  - set-class
extends:
  - emb-function
related:
  - m-class-vector
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can EMB values be decomposed through intermediate set classes?"
---

# Quick Definition
The embedding number EMB(X, Z) can be computed by summing over intermediate-sized set classes /Y/, weighted by EMB(X, /Y/) and EMB(/Y/, Z), with an adjustment factor correcting for overcounting.

# Core Definition
Theorem 5.3.5.2 (Lewin, p. 141): Let L < M < N be positive integers. Let ADJUST = 1/COMB(N-M, N-L). Let Z have cardinality N and X have cardinality L. Then: EMB(X, Z) = ADJUST * SUM over all M-member set-classes /Y/ of [EMB(X, /Y/) * EMB(/Y/, Z)]. The proof uses Lemma 5.3.5.1 (COMB(L,N)/(COMB(L,M)*COMB(M,N)) = 1/COMB(N-M, N-L)) and probabilistic reasoning: pulling an M-member subset from Z, then an L-member subset from that.

# Prerequisites
- **EMB Function** — The function being decomposed
- **EMB Probability** — Probabilistic reasoning used in the proof
- **Set Class** — Intermediate set classes serve as decomposition basis

# Key Properties
1. ADJUST = 1/COMB(N-M, N-L) corrects for overcounting
2. The sum has only finitely many nonzero terms (Y is finite)
3. Relates EMB at different cardinality levels
4. Connected to algebraic topology: decomposing a polytope into faces

# Construction / Recognition
## To Apply:
1. Choose an intermediate cardinality M between L = card(X) and N = card(Z)
2. Compute ADJUST = 1/COMB(N-M, N-L)
3. Sum EMB(X, /Y/) * EMB(/Y/, Z) over all M-element set classes /Y/
4. Multiply by ADJUST to get EMB(X, Z)

## To Recognize:
1. Any situation where EMB values at one cardinality are derived from values at another

# Context & Application
This theorem connects EMB values at different cardinality levels, analogous to decomposing a polyhedron into faces and then edges. Lewin illustrates with a tetrahedral model (Figures 5.9-5.10) where edges of a tetrahedron are counted by summing edges of triangular faces with an adjustment factor of 1/2. The connection to algebraic topology is noted but not pursued.

# Examples
**Example 1** (pp. 139-141, Figures 5.9-5.10): Z = {A, B, C, D} (tetrachord), X = {A, C} (dyad). L=2, M=3, N=4. ADJUST = 1/COMB(1,2) = 1/2. The tetrahedron has 4 triangular faces; each edge belongs to 2 faces. Summing edge counts across faces and dividing by 2 yields the correct edge count.

# Relationships
## Builds Upon
- **EMB Function** — Decomposes EMB into intermediate-level sums
- **EMB Probability** — Proof uses probabilistic reasoning

## Enables
- **M-Class Vector** — Can be computed from lower-cardinality vectors using this theorem

# Common Errors
- **Error**: Forgetting the ADJUST factor
  **Correction**: Without ADJUST, the sum overcounts because the same small set appears in multiple intermediate sets

# Common Confusions
- **Confusion**: Thinking any intermediate cardinality M gives the same result
  **Clarification**: The formula works for any valid M, but different M values may be computationally easier

# Source Reference
Chapter 5: Generalized Set Theory (1), Lemma 5.3.5.1 and Theorem 5.3.5.2, pp. 140-142.

# Verification Notes
- Definition source: Direct from Theorem 5.3.5.2 with proof
- Confidence rationale: Explicit theorem with detailed proof
- Re-extraction notes: Re-extracted from v2 card; preserved: tetrahedron example, algebraic topology reference. Added v3.1 structure.
