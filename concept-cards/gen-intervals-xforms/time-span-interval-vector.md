---
concept: Time-Span Interval Vector
category: analysis
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
In the non-commutative GIS of time spans, interval vectors count attack-ordered dyads by their forwards-oriented intervals, providing a powerful generalization of Forte's interval vector to rhythmic analysis.

# Formal Definition
For the GIS of time spans (TMSPS), CANON is taken as the group of interval-preserving operations. An attack-ordered dyad (AOD) D = (s, t) lists two time spans with s beginning before t (or if simultaneous, s shorter). The interval int(s, t) = (i, p) is forwards-oriented: i >= 0, and if i = 0 then p > 1.

The 2-element set classes correspond 1-to-1 with forwards-oriented intervals. For a set X of time spans, EMB(D, X) = IFUNC(X, X)(i, p), where (i, p) is the forwards-oriented interval of D.

# Mathematical Formulation
Forwards-oriented interval (i, p):
- i >= 0 (the second span begins i first-span-durations after the first)
- If i = 0, then p > 1 (the second span is longer)
- p = ratio of durations (second/first)

For attack-ordered dyad D = (s, t) with int(s, t) = (i, p):
EMB(D, X) = number of ways interval (i, p) spans within X

The interval vector of X tabulates EMB(D, X) for each dyad class D.

# Musical Context/Application
This construction allows rigorous rhythmic analysis using the same formal machinery as pitch-class set theory. Rhythmic motives can be compared by their internal interval structure, and relationships between motives can be quantified through IFUNC.

# Examples
From Chopin Sonata analysis (Figures 5.11-5.12), motives (b), (c), and (d):

Motive (b) interval vector includes:
- (1, 1): 3 occurrences (consecutive notes of equal duration)
- (2, 1): 2 occurrences
- (3, 1): 2 occurrences
- etc.

Motive (c) interval vector shows:
- (1, 1): 2 (fewer consecutive equal notes)
- More diverse intervals overall

Motive (d): Maximum diversity, only one interval appears more than once.

This captures the "progressive diversification" from (b) through (c) to (d).

# Related Concepts
- Time-Span GIS
- Attack-Ordered Dyad
- Forwards-Oriented Interval
- Interval-Preserving Operations
- Unrolling Interval Vector

# Common Confusions
The crucial choice of CANON = interval-preserving operations (not transpositions) makes dyad classes correspond exactly to forwards-oriented intervals. This parallels how interval classes work in Forte's theory but requires proof (given in section 5.6 appendix).

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Example 5.4.1 and Figures 5.11-5.12
