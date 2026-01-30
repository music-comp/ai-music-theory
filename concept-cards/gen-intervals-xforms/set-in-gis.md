---
concept: Set in a GIS
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
A set in the context of a Generalized Interval System (GIS) is defined as a finite unordered subfamily of the space S of musical elements.

# Formal Definition
Definition 5.1.1: Given a GIS(S, IVLS, int), a "set" means a finite unordered subfamily of S. This definition restricts attention to finite collections of elements, though the space S itself may be infinite.

# Mathematical Formulation
Let (S, IVLS, int) be a GIS. A set X is:
- A finite collection {s1, s2, ..., sN} where each si is in S
- Unordered (the listing order is immaterial)
- The cardinality of X, written card(X) or |X|, equals N

For a mapping f: S -> S and a set X:
- f(X) = {f(s) : s in X}
- If f is 1-to-1, then card(f(X)) = card(X)
- If f is not 1-to-1, then card(f(X)) may be less than card(X)

# Musical Context/Application
Sets in GIS theory generalize the notion of pitch-class sets from traditional atonal theory. While pitch-class sets are subsets of the 12 chromatic pitch classes, sets in a GIS can be collections of any musical objects: pitches, time points, time spans, durations, or elements of any space S that admits a GIS structure.

# Examples
From Webern's Piano Variations analysis (Chapter 4), time spans form sets in a non-commutative GIS. A melodic motive can be modeled as a set of time spans where each span (a, x) indicates an event beginning at time point a with duration x.

In the standard pitch-class GIS:
- X = {C, E, G} is a 3-element set (C-major triad)
- Y = {Bb, A, C#, B, F, G} is a 6-element set (hexachord)

# Related Concepts
- GIS (Generalized Interval System)
- IFUNC (Interval Function)
- Set Class
- Canonical Equivalence
- EMB (Embedding Function)

# Common Confusions
Students may confuse the technical definition of "set" here with the broader mathematical term. In GIS theory, sets must be finite, even when S is infinite. Also, sets are unordered collections, not sequences or series, even though we often list elements in some convenient order for discussion.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Definition 5.1.1
