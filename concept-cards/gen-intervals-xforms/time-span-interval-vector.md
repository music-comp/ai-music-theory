---
# === CORE IDENTIFICATION ===
concept: Time-Span Interval Vector
slug: time-span-interval-vector

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: rhythmic-analysis
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "Example 5.4.1, Section 5.6"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "rhythmic interval vector"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - time-span-gis
  - attack-ordered-dyad
  - canonical-group
  - ifunc
extends:
  - interval-vector
related:
  - forwards-oriented-interval
  - interval-preserving-operations
  - embedding-function
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How is Forte's interval vector generalized to the non-commutative time-span GIS?"
  - "What is the role of attack-ordered dyads and forwards-oriented intervals?"
  - "Why must CANON be the interval-preserving operations (not transpositions) for this construction?"
---

# Quick Definition
In the non-commutative GIS of time spans, the interval vector of a set X tabulates how many attack-ordered dyads of each forwards-oriented interval type are embedded within X, providing a powerful generalization of Forte's interval vector to rhythmic analysis.

# Core Definition
"We can develop a very strong formal analog for Forte's interval vector in this particular system (NB). Let X be a set containing more than two members; let D be a dyad; then EMB(D, X), the number of forms of D embedded within X, is equal to the number of ways the forwards-oriented interval (i, p) can be spanned between members of X, where (i, p) is the interval spanning the attack-ordered members of D. In other words, EMB(D, X) = IFUNC(X, X)(i, p)" (Lewin, Ch. 5, p. 113).

# Prerequisites
- **Time-span GIS** — The non-commutative GIS whose elements are time spans (onset, duration)
- **Attack-ordered dyad** — A dyad (s, t) ordered so that s begins before t (or, if simultaneous, s is shorter)
- **Canonical group** — Must be the interval-preserving operations for this construction to work
- **IFUNC** — The interval function on sets, used to count interval occurrences

# Key Properties
1. An attack-ordered dyad (AOD) D = (s, t) has int(s, t) = (i, p) where i >= 0, and if i = 0 then p > 1
2. The interval (i, p) is called "forwards-oriented": t begins i s-durations after s begins, and lasts p times as long
3. Two-element set classes correspond 1-to-1 with forwards-oriented intervals (proved in Section 5.6)
4. For a set X: EMB(D, X) = IFUNC(X, X)(i, p)
5. The interval vector tabulates EMB(D, X) for each dyad class D (i.e., each forwards-oriented interval)

# Construction / Recognition
## To Construct:
1. Model the rhythmic motive as a set X of time spans
2. For each pair of time spans in X, determine the attack-ordered dyad and its forwards-oriented interval (i, p)
3. Tabulate the count of each (i, p) that occurs
## To Recognize:
1. A table mapping forwards-oriented intervals to non-negative integers
2. Counting how many ways each rhythmic dyad type appears within a set

# Context & Application
The crucial choice of CANON = interval-preserving operations (not transpositions) makes dyad classes correspond exactly to forwards-oriented intervals. "Forwards-oriented intervals thus play exactly the same role here that Forte's 'interval classes' play in his atonal theory: They can be used to label the distinct set-classes of dyads" (Lewin, p. 113). This enables rigorous rhythmic set theory using the same formal machinery as pitch-class set theory.

# Examples
**Example 1** (Example 5.4.1, Figures 5.11-5.12, Chopin Sonata):
- Motive (b): interval vector includes (1,1): 3 occurrences (consecutive notes of equal duration), (2,1): 2, (3,1): 2, (4,1): 2, (5,1): 1
- Motive (c): (1,1): 2, fewer repeated patterns, (3,2): 2 occurrences
- Motive (d): maximum diversity; "only one interval appears more than once"

The progressive diversification of interval vectors from (b) through (c) to (d) captures increasing rhythmic complexity.

**Example 2** (p. 114): "The forwards-oriented interval (1,1) labels the set-class of AODs D = (s,t) such that t begins right after s (1 s-length after s begins) and extends the same duration as s (1 times the length of s)." A pair of consecutive quarter notes anywhere in any tempo belongs to this class.

# Relationships
## Builds Upon
- **Time-span GIS** — The underlying non-commutative GIS
- **Attack-ordered dyad** — The basic unit counted by the vector
- **IFUNC** — The counting mechanism: EMB(D, X) = IFUNC(X, X)(i, p)
## Enables
- **Rhythmic set theory** — Comparing rhythmic motives by their internal interval structure
## Related
- **Forwards-oriented interval** — Labels for dyad classes in the time-span GIS
- **Interval-preserving operations** — The canonical group making the construction work
- **Embedding function** — EMB counts forms of D within X

# Common Errors
- **Error**: Using transpositions as CANON instead of interval-preserving operations
  **Correction**: Only interval-preserving operations as CANON make dyad classes correspond 1-to-1 with forwards-oriented intervals; with transpositions as CANON, the theorem fails

# Common Confusions
- **Confusion**: Thinking canonical equivalence of time-span dyads is obvious
  **Clarification**: The theorem that dyad classes correspond to forwards-oriented intervals "is by no means obvious or trivial" (Lewin); a formal proof is given in Section 5.6

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Example 5.4.1, Figures 5.11-5.12, Section 5.6, pages 112-115.

# Verification Notes
- Definition source: Direct quotation from Section 5.4 and Example 5.4.1
- Confidence rationale: Formally defined with proof reference and worked examples
- Re-extraction notes: Re-extracted from v2 card; preserved: Chopin example, CANON choice rationale, EMB = IFUNC identity
