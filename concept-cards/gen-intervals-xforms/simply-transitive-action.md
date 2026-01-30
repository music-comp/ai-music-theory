---
concept: Simply Transitive Action
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
A simply transitive group action means that for any two elements s and t in the space, there is exactly one group element (interval) that takes s to t.

# Formal Definition
Condition (B) of the GIS definition states: For every s in S and every i in IVLS, there exists a unique t in S such that int(s, t) = i. This establishes that IVLS acts simply transitively on S. "Simply" means each interval from s reaches exactly one point; "transitive" means every point can be reached from s by some interval.

# Mathematical Formulation
- IVLS acts on S via: for s in S and i in IVLS, define s.i = t where int(s, t) = i
- Simply transitive = free + transitive
- Transitive: for any s, t in S, some i in IVLS satisfies int(s, t) = i (can reach t from s)
- Free (simple): if int(s, t) = i1 and int(s, t) = i2, then i1 = i2 (unique interval)
- Equivalently: the map i -> s.i is a bijection from IVLS to S (for any fixed s)

# Musical Context/Application
Simple transitivity means the interval group IVLS "parameterizes" the space S perfectly. Given a reference point (like C4 in chromatic space), every other point is reached by exactly one interval. This allows us to identify the space S with the group IVLS once we fix a reference. The musical space is "homogeneous" - it looks the same from every point.

# Examples
Pitch-class space (Z12):
- Fix reference: C = 0
- Every pitch class is reached by exactly one interval: G is interval 7 from C, A is interval 9 from C, etc.
- The map i -> (C + i) mod 12 is a bijection from Z12 to pitch classes

Chromatic pitch space (Z):
- Fix reference: C4
- Every pitch is reached by exactly one integer interval
- The map i -> (C4 + i semitones) is a bijection from Z to chromatic pitches

Condition (B) ensures this works for all GIS structures.

# Related Concepts
- GIS Condition B
- Generalized Interval System
- Group Action
- Bijection
- Homogeneous Space

# Common Confusions
- "Simply transitive" is a technical term from group theory
- It implies |S| = |IVLS| (same cardinality)
- Different from "doubly transitive" or other group action types
- The action is implicit in the GIS definition, not stated separately

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1(B), implicit in discussion
