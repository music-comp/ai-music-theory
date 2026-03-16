---
# === CORE IDENTIFICATION ===
concept: "IFUNC (Interval Function)"
slug: ifunc

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
section: "5.1.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - interval function
  - X/Y interval function

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - set-in-gis
  - interval-group-ivls
extends: []
related:
  - ifunc-symmetry-theorem
  - ifunc-interval-preserving
  - ifunc-transposition-theorem
  - ifunc-inversion-theorem
  - ifunc-probability
  - convolution-interpretation
contrasts_with:
  - inj-function
  - emb-function

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the IFUNC (interval function)?"
  - "How do I compute IFUNC between two sets?"
  - "How does IFUNC relate to Forte's interval vector?"
  - "What must I know before understanding IFUNC?"
---

# Quick Definition
IFUNC(X, Y) is a function that, for each interval i in IVLS, counts the number of distinct ways that interval can be spanned from members of set X to members of set Y within a GIS.

# Core Definition
Definition 5.1.3: "Given a GIS, given sets X and Y, then the X/Y interval function is a function IFUNC(X, Y) which maps the group IVLS into the family of non-negative integers as follows: For each interval i in IVLS, the value of the function, IFUNC(X, Y)(i), counts the number of distinct pairs (s, t) in S x S such that s is in X, t is in Y, and int(s, t) = i" (Lewin, p. 119).

IFUNC(X, Y)(i) tells us in how many different ways the interval i can be spanned between (members of) X and (members of) Y.

# Prerequisites
- **Generalized Interval System** — IFUNC is defined within the framework of a GIS (S, IVLS, int)
- **Set in a GIS** — X and Y must be finite subsets of S
- **Interval Group IVLS** — The domain of IFUNC is the group of intervals

# Key Properties
1. IFUNC(X, Y): IVLS -> {0, 1, 2, 3, ...}
2. IFUNC(Y, X)(i) = IFUNC(X, Y)(i^{-1}) — exchanging sets "inverts" the interval (Theorem 5.1.4)
3. IFUNC(P(X), P(Y)) = IFUNC(X, Y) for any interval-preserving operation P (Theorem 5.1.5)
4. IFUNC(T_n(X), Y)(i) = IFUNC(X, Y)(ni) (Theorem 5.1.6A)
5. IFUNC(X, T_n(Y))(i) = IFUNC(X, Y)(in^{-1}) (Theorem 5.1.6B)
6. For commutative GIS with inversion I: IFUNC(I(X), I(Y)) = IFUNC(Y, X) (Theorem 5.1.7)
7. IFUNC(X, Y)(i)/(MN) gives the probability of randomly drawing interval i from X to Y, where M = card(X), N = card(Y) (Theorem 5.1.8)

# Construction / Recognition
## To Compute IFUNC(X, Y):
1. For each interval i in IVLS, initialize count to 0
2. For each element s in X and each element t in Y, compute int(s, t)
3. Increment the count for interval int(s, t)
4. The resulting counts give IFUNC(X, Y)(i) for each i

## To Recognize:
1. A table or function mapping intervals to non-negative integers
2. The sum of all values equals card(X) * card(Y)
3. The function is defined on the entire group IVLS

# Context & Application
IFUNC generalizes and extends the interval vector from traditional atonal set theory. While Forte's interval vector counts interval classes within a single set (essentially IFUNC(X, X) with additional symmetry reductions), IFUNC can measure intervallic relationships between two different sets. This captures melodic progressions (antecedent/consequent), harmonic successions, contrapuntal relationships, and many other musical structures. IFUNC also serves as a probability distribution (Theorem 5.1.8) and can be interpreted mathematically as a convolution of characteristic functions on a locally compact group.

# Examples
**Example 1** (p. 120, Figure 5.1): X1 = {E, Bb}, Y1 = {F, A, C#}. IFUNC(X1, Y1)(i) = 0 if i is even, = 1 if i is odd. Different pairs (X2, Y2), (X3, Y4) produce the same IFUNC values despite being different sets of different cardinalities.

**Example 2** (pp. 121-128, Figures 5.2-5.6): In Webern's op. 7 no. 3, X = {Ab, Bb, Eb} and Y is a 7-note melodic set. IFUNC(X, Y)(3) = 3 (maximum) indicates T_3(X) can be embedded in Y. IFUNC(X, Y)(8) = 3 also indicates T_8(X) embeds in Y as a bounding frame.

**Example 3** (pp. 131-132, Figure 5.8): In Schoenberg's Violin Fantasy op. 47, IFUNC reveals that "scarce" even intervals between piano set X and violin set Y bind the spatio-temporal frame of the phrase, with intervals 4 and 8 connecting boundary tones.

# Relationships
## Builds Upon
- **Generalized Interval System** — IFUNC requires the int function from a GIS
- **Set in a GIS** — Arguments X and Y are sets

## Enables
- **EMB Function** — In certain GIS structures, EMB(D, X) = IFUNC(X, X)(i, p) for dyad D
- **Z-Relation Generalized** — Z-related sets share identical IFUNC self-values
- **INJ Generalizes IFUNC** — INJ(X, Y)(T_i) = IFUNC(X, Y)(i) (Theorem 6.7.1)

## Related
- **Convolution Interpretation** — IFUNC as convolution of characteristic functions
- **IFUNC Probability** — IFUNC as probability distribution

## Contrasts With
- **INJ Function** — INJ generalizes IFUNC to arbitrary transformations, not just transpositions
- **EMB Function** — EMB counts forms of a set class embedded in another set

# Common Errors
- **Error**: Computing IFUNC using interval classes instead of directed intervals
  **Correction**: IFUNC counts directed intervals from X to Y; int(s, t) = i is directional

- **Error**: Expecting IFUNC(X, Y) = IFUNC(Y, X)
  **Correction**: IFUNC(Y, X)(i) = IFUNC(X, Y)(i^{-1}); the functions are related by interval inversion, not equal

# Common Confusions
- **Confusion**: IFUNC is the same as Forte's interval vector
  **Clarification**: The interval vector is essentially IFUNC(X, X) with additional reductions for interval-class equivalence. IFUNC operates between two potentially different sets and counts all intervals in IVLS, not just interval classes.

- **Confusion**: IFUNC can capture all set-theoretic relationships
  **Clarification**: IFUNC cannot engage inversional relationships directly. The Injection Function (Chapter 6) generalizes IFUNC to handle inversions, wedges, and other non-transposition transformations.

# Source Reference
Chapter 5: Generalized Set Theory (1), Definition 5.1.3, Theorems 5.1.4-5.1.8, Figures 5.1-5.8, pp. 119-133.

# Verification Notes
- Definition source: Direct from Definition 5.1.3
- Key properties: Theorems 5.1.4-5.1.8 explicitly stated and proved
- Confidence rationale: Explicit definition with extensive examples and proofs
- Re-extraction notes: Re-extracted from v2 card; preserved: Webern and Schoenberg examples, key property formulas. Added typed relationships, prerequisites, competency questions, construction steps, all v3.1 sections.
