---
# === CORE IDENTIFICATION ===
concept: EMB Probability Theorem
slug: emb-probability

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
section: "5.3.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Theorem 5.3.4"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - emb-function
  - set-class
extends:
  - emb-function
related:
  - ifunc-probability
  - emb-decomposition-theorem
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can EMB be interpreted probabilistically?"
---

# Quick Definition
EMB can be interpreted as a probability: the chance of randomly extracting a set of class /X/ from Y equals EMB(/X/, Y) divided by COMB(M, N), where M = card(X) and N = card(Y).

# Core Definition
Theorem 5.3.4: "Let the cardinality of Y be N. Let M be a positive integer less than N. Pull M members of Y at random. Then the probability that you have pulled a set of class /X/ is given by the number EMB(/X/, Y)/COMB(M, N)" (Lewin, p. 140). Here COMB(M, N) = N!/(M!(N-M)!) is the binomial coefficient.

# Prerequisites
- **EMB Function** — The function being given probabilistic interpretation
- **Set Class** — The classes whose embeddings are being counted

# Key Properties
1. P(random M-subset of Y is in class /X/) = EMB(/X/, Y) / COMB(M, N)
2. Sum over all M-element classes /X/ of probabilities = 1
3. COMB(M, N) = total number of M-element subsets of Y
4. Provides a statistical baseline for measuring compositional choices

# Construction / Recognition
## To Compute:
1. Determine M = card(X), N = card(Y)
2. Compute COMB(M, N) = N!/(M!(N-M)!)
3. EMB probability = EMB(/X/, Y) / COMB(M, N)

## To Recognize:
1. Probabilistic statements about set-class frequency within a larger set

# Context & Application
Like IFUNC's probability interpretation, EMB probability provides a null hypothesis for random selection. In compositional analysis, deviations from these expected frequencies reveal structural choices. The theorem is also essential for proving the EMB Decomposition Theorem (5.3.5.2).

# Examples
**Example 1** (derived from discussion, p. 140): Y = C major scale (7 notes), M = 3. COMB(3, 7) = 35 total trichords. If CANON = transpositions + inversions: EMB(3-11, Y) = 6, so P(harmonic triad) = 6/35 = 17.1%.

# Relationships
## Builds Upon
- **EMB Function** — Probability is derived from EMB values

## Enables
- **EMB Decomposition Theorem** — Uses probability theory in its proof

## Related
- **IFUNC Probability** — Analogous probabilistic interpretation for IFUNC

# Common Errors
- **Error**: Interpreting EMB probabilities as predictions of what a composer will write
  **Correction**: Probabilities model random selection; deviations from these reveal compositional intent

# Common Confusions
- **Confusion**: Thinking high EMB probability means the set class is musically important
  **Clarification**: Statistical frequency is a baseline, not a measure of musical significance

# Source Reference
Chapter 5: Generalized Set Theory (1), Theorem 5.3.4, p. 140.

# Verification Notes
- Definition source: Direct from Theorem 5.3.4
- Confidence rationale: Explicit theorem with proof
- Re-extraction notes: Re-extracted from v2 card; preserved: major scale example. Added v3.1 structure.
