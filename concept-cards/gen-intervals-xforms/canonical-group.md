---
concept: Canonical Group
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
A canonical group is a designated group of operations on the space S that determines which sets are considered "equivalent" for analytical purposes.

# Formal Definition
Definition 5.2.1: In certain connections we shall fix a group of operations on S and call it "the canonical group." It will be denoted CANON. Sets X and X' will be called "canonically equivalent" if there exists some canonical operation A such that X' = A(X).

The choice of canonical group is context-dependent and reflects analytical priorities.

# Mathematical Formulation
CANON is a group of operations on S satisfying:
1. IDENT is in CANON (identity operation)
2. If A is in CANON, then A^-1 is in CANON (closure under inverses)
3. If A and B are in CANON, then BA is in CANON (closure under composition)

Canonical equivalence is an equivalence relation:
- Reflexive: X = IDENT(X), so X is equivalent to itself
- Symmetric: If X' = A(X), then X = A^-1(X')
- Transitive: If X' = A(X) and X'' = B(X'), then X'' = (BA)(X)

# Musical Context/Application
Different choices of CANON yield different notions of equivalence. In Forte's atonal set theory, CANON typically includes transpositions and inversions, making major and minor triads equivalent. If CANON contains only transpositions, major and minor triads are distinct set classes.

The canonical group can include interval-preserving operations, transpositions, inversions, and other operations (like M5/M7 transformations) depending on analytical goals.

# Examples
Standard pitch-class GIS with X = {C, E, G}:

CANON = {T_0, T_1, ..., T_11} (transpositions only):
- /X/ = {major triads} (12 sets)
- Minor triads form a different set class

CANON = {T_i, I_j : all i, j} (transpositions and inversions):
- /X/ = {major and minor triads} (24 sets)
- All consonant triads are equivalent

CANON = {interval-preserving operations}:
- In commutative GIS, same as transpositions
- In non-commutative GIS, may differ from transpositions

# Related Concepts
- Set Class
- Canonical Equivalence
- Interval-Preserving Operations
- Transposition Operations
- EMB (Embedding Function)

# Common Confusions
There is no single "correct" canonical group. The choice depends on what musical relationships the analyst considers structurally significant. Different canonical groups produce different partitions of sets into equivalence classes, and these different partitions may all be analytically useful in different contexts.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Definition 5.2.1
