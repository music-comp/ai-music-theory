---
concept: Central Interval
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
A central interval is an element of IVLS that commutes with every other interval in the group. Central intervals determine when transpositions equal interval-preserving operations and when inversions with different parameters are equivalent.

# Formal Definition
An interval i in IVLS is called central if for every j in IVLS:

ij = ji

In group theory terminology, the central elements form the "center" of the group, denoted Z(IVLS).

# Mathematical Formulation
**Definition (from 1.8.2):**
i is central in IVLS if ij = ji for all j in IVLS

**Key theorem (3.4.8):** The following are equivalent:
- Ti preserves intervals
- Ti = Pi for some (equivalently, any) choice of ref
- i is central in IVLS

**Corollary 3.4.9:**
- In commutative GIS: every interval is central, so Ti = Pi for all i
- In non-commutative GIS: some i are not central, so Ti =/= Pi for such i

**Role in inversion theory (3.5.3):**
I_u^v = I_x^w iff w = I_u^v(x) AND int(x, u) is central

# Musical Context/Application
Central intervals are the "well-behaved" intervals whose transpositions act like familiar pitch-class transpositions (preserving intervals). In non-commutative GIS, most intervals are not central, leading to transpositions that distort intervallic relationships.

The center of IVLS measures "how commutative" the group is:
- Commutative group: center = entire group
- "Highly non-commutative" group: center = {identity only}

# Examples
**Commutative pitch-class group:**
All 12 intervals are central. Z(Z/12Z) = Z/12Z.

**Time-span interval group (non-commutative):**
Only (0, 1) is central.
Proof: If (i, p)(j, q) = (j, q)(i, p) for all (j, q), then:
(i + pj, pq) = (j + qi, qp) for all j, q.
This requires (p-1)j = (q-1)i for all j, q.
Taking j = 1, q = 1: p - 1 = 0, so p = 1.
Then (q-1)i = 0 for all q, so i = 0.
Thus only (0, 1) is central.

**Consequence in time-span GIS:**
Only T(0,1) = identity preserves intervals. All other transpositions distort intervals.

# Related Concepts
- Transposition and Interval Preservation
- Inversion Equivalence Conditions
- Commutative vs. Non-commutative Groups
- Group Center

# Common Confusions
1. **Central vs. identity:** The identity e is always central, but in commutative groups, every element is central.

2. **Centrality is a property of the element:** Whether i is central depends on the group structure, not on any particular GIS realization.

3. **Transposition behavior:** Ti preserves intervals iff i is central. Students from commutative settings may assume all transpositions preserve intervals.

4. **Inversion equivalence:** The centrality condition in Theorem 3.5.3 is why inversion equivalence is more restrictive in non-commutative GIS.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.4.8 and related discussions, pp. 81-86
