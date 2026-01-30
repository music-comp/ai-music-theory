---
concept: Interval-Preserving Operations (Octatonic)
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups"
chapter_number: B
pdf_page: 282
unit: null
authors: David Lewin
---

# Quick Definition
In octatonic GIS structures, the interval-preserving operations are the dual group to the transposition group: STRANS2 preserves GIS1 intervals, and STRANS1 preserves GIS2 intervals.

# Formal Definition
An operation f on S is interval-preserving for a GIS if int(f(s), f(t)) = int(s, t) for all s, t in S. In the octatonic GIS structures, the interval-preserving operations for GIS1 are exactly the members of STRANS2, and the interval-preserving operations for GIS2 are exactly the members of STRANS1. This dual relationship arises because each group consists of exactly the operations that commute with all members of the other group.

# Mathematical Formulation
For GIS1 = (S, IVLS1, int1):
- Transpositions: STRANS1
- Interval-preserving: STRANS2
- For all f in STRANS2: int1(f(s), f(t)) = int1(s, t)

For GIS2 = (S, IVLS2, int2):
- Transpositions: STRANS2
- Interval-preserving: STRANS1
- For all g in STRANS1: int2(g(s), g(t)) = int2(s, t)

Characterization:
- STRANS2 = {f : f commutes with all g in STRANS1}
- STRANS1 = {g : g commutes with all f in STRANS2}

# Musical Context/Application
Interval-preserving operations transform musical objects while maintaining their intervallic relationships. In the octatonic context, GIS1 and GIS2 offer complementary notions of interval preservation: what preserves intervals in one GIS transforms them in the other.

# Examples
**GIS1 interval preservation:**
"As it turns out, the members of STRANS2 are exactly the interval-preserving operations for GIS1."

"Every member of STRANS2 commutes with every member of STRANS1. In fact, the members of STRANS2 are precisely those transformations on S that commute with every member of STRANS1."

**GIS2 interval preservation:**
"The interval-preserving operations for GIS2 are exactly the members of STRANS1; those are in fact precisely the transformations on S that commute with every member of STRANS2."

**Practical implication:**
In GIS1, applying any STRANS2 operation (Q3, Q9, X1, X2, X4, X5) to a set preserves its GIS1-intervallic structure. The transformed set has the same internal GIS1-intervals as the original.

**INJ function application:**
"More generally, if f is any one of the eight operations in STRANS1, and A is any one of the eight operations in STRANS2, and Y and Z are any sets whatsoever within S, then INJ(Y, Z)(f) = INJ(A(Y), A(Z))(f)."

This expresses how STRANS2 operations preserve GIS1-interval relationships between sets.

# Related Concepts
- GIS1 and GIS2
- STRANS1 Group
- STRANS2 Group
- Dual Simply Transitive Groups
- Commutation
- INJ Function

# Common Confusions
Interval-preserving operations are not the same as transpositions. In a commutative GIS, they coincide (the group is its own dual). In a non-commutative GIS like the octatonic structures, they are different groups. Students should not assume that operations preserving intervals in GIS1 also preserve intervals in GIS2.

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups
