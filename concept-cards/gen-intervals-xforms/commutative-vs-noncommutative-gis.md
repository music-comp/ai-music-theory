---
concept: Commutative vs. Non-Commutative GIS
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
A GIS is commutative if its interval group IVLS is commutative (ij = ji for all intervals). Non-commutative GIS have interval groups where order of multiplication matters, leading to fundamentally different behavior of transpositions, inversions, and other operations.

# Formal Definition
A GIS (S, IVLS, int) is commutative if IVLS is an abelian (commutative) group, meaning:

For all i, j in IVLS: ij = ji

If IVLS is non-abelian, the GIS is non-commutative.

# Mathematical Formulation
**Key differences:**

| Property | Commutative GIS | Non-Commutative GIS |
|----------|-----------------|---------------------|
| Ti = Pi | Always | Only when i central |
| Ti preserves intervals | Always | Only when i central |
| I_u^v = I_v^u | Always | Only when int(u,v) central |
| Interval-reversing ops | = Inversions | Don't exist |
| I^(-1) = I | Always | Generally not |
| IT = T^(-1)I | Always | Generally not |

**Corollary 3.4.9:**
(A) In commutative GIS: transpositions = interval-preserving operations
(B) In non-commutative GIS: some transpositions don't preserve intervals; some interval-preserving ops aren't transpositions

# Musical Context/Application
Most familiar music-theoretic GIS are commutative: pitch classes, time-points, diatonic scale degrees. The theory developed for these structures assumes commutativity implicitly.

Non-commutative GIS arise in:
- Time-span theory (attack + duration)
- Certain timbral models
- Any context where order of combining intervals matters

Understanding the commutative/non-commutative distinction is essential for applying GIS theory correctly across different musical domains.

# Examples
**Commutative examples:**
- Pitch classes mod 12 (addition is commutative)
- Time-points (integer addition is commutative)
- Just-intonation ratios (multiplication of positive rationals is commutative)

**Non-commutative example (time-span GIS 4.1.3):**
IVLS consists of pairs (i, p) with composition:
(i, p)(j, q) = (i + pj, pq)

Check: (1, 2)(3, 4) = (1 + 2*3, 2*4) = (7, 8)
       (3, 4)(1, 2) = (3 + 4*1, 4*2) = (7, 8)

Wait--these happen to be equal! But try:
(1, 2)(0, 3) = (1 + 0, 6) = (1, 6)
(0, 3)(1, 2) = (0 + 3, 6) = (3, 6)

These differ, confirming non-commutativity.

# Related Concepts
- Interval Group (IVLS)
- Central Interval
- Transposition and Interval Preservation
- Interval-Reversing Operation
- Time-span GIS

# Common Confusions
1. **Assuming commutativity:** Most pitch-class theory implicitly assumes commutativity. Results like IT = T^(-1)I don't generalize.

2. **Non-commutativity of IVLS, not of GIS operations:** The group IVLS is non-commutative. GIS operations like Ti Pj = Pj Ti still hold (Theorem 3.4.10).

3. **"Transposition" changes meaning:** In non-commutative GIS, Ti may distort intervals and even chronology.

4. **Chapter 4's purpose:** The entire chapter provides a musically significant non-commutative example to illustrate these differences.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Corollary 3.4.9 and related discussions throughout, pp. 77-92
