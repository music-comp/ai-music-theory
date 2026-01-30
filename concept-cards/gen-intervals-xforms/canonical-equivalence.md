---
concept: Canonical Equivalence
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
Two sets X and X' are canonically equivalent if one can be transformed into the other by some operation in the canonical group CANON.

# Formal Definition
Definition 5.2.1 (continued): Sets X and X' will be called "canonically equivalent" if there exists some canonical operation A such that X' = A(X).

This relation partitions all sets into equivalence classes called "set classes."

# Mathematical Formulation
Canonical equivalence is an equivalence relation on sets:

X ~ X' if and only if there exists A in CANON such that X' = A(X)

Equivalence relation properties:
1. Reflexive: X ~ X via A = IDENT
2. Symmetric: X ~ X' implies X' ~ X via A^-1
3. Transitive: X ~ X' and X' ~ X'' implies X ~ X'' via composition

The equivalence class of X is:
[X] = {A(X) : A in CANON}

If CANON contains interval-preserving operations, then:
- X ~ X' implies IFUNC(X, X) = IFUNC(X', X')
- Equivalent sets have the same internal intervallic structure

# Musical Context/Application
Canonical equivalence generalizes the intuition that transposed or inverted forms of a set "sound the same" in terms of internal interval content. The specific operations deemed canonical reflect which transformations preserve musically relevant features.

# Examples
In pitch-class space:

1. CANON = transpositions only:
   - {C, E, G} ~ {D, F#, A} (both major triads)
   - {C, E, G} is not ~ {C, Eb, G} (major vs. minor)

2. CANON = transpositions and inversions:
   - {C, E, G} ~ {C, Eb, G} (both harmonic triads)
   - {C, E, G} ~ {D, F, A} ~ {C#, E, G#}

3. Two sets are canonically equivalent if and only if they belong to the same "set class" (Forte's terminology).

# Related Concepts
- Canonical Group
- Set Class
- Forms of a Set
- IFUNC Invariance
- Interval-Preserving Operations

# Common Confusions
Canonical equivalence depends entirely on which group is designated as CANON. There is no absolute notion of set equivalence - it is always relative to a chosen canonical group. This is why Lewin uses the term "canonical" rather than simply "equivalent."

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Definition 5.2.1
