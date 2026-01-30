---
concept: Transposition and Interval Preservation
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
A transposition Ti preserves intervals if and only if the interval i is central (commutes with every interval in the group). In commutative GIS structures, all transpositions preserve intervals; in non-commutative structures, typically only the identity transposition does.

# Formal Definition
Given an interval i in a GIS, the following conditions are logically equivalent:
- (A): Ti preserves intervals
- (B): For some choice of ref, Ti = Pi
- (C): For any choice of ref, Ti = Pi
- (D): i is central in IVLS (i commutes with every j in IVLS)

# Mathematical Formulation
**Theorem 3.4.8:** Conditions (A)-(D) are equivalent.

**Proof sketch:**
(A) => (C): If Ti preserves intervals, then by Theorem 3.4.7, Ti = Pj for some j. One shows j = i by comparing labels.

(C) => (B): Trivial.

(B) => (D): If Ti = Pi for some ref, then LABEL(s) * i = i * LABEL(s) for all s, implying i commutes with every interval.

(D) => (A): If i is central, then int(Ti(s), Ti(t)) = (LABEL(s)*i)^(-1)(LABEL(t)*i) = i^(-1) * int(s,t) * i = int(s,t).

# Musical Context/Application
This theorem reveals a fundamental dichotomy:

**Commutative case (most familiar GIS):** Every interval is central, so every transposition preserves intervals. This is the familiar situation in pitch-class theory where T5 of an interval-class 3 dyad is still an interval-class 3 dyad.

**Non-commutative case:** Only the identity interval (and possibly a few others) is central. Most transpositions distort intervallic relationships. This occurs in the time-span GIS of Chapter 4.

# Examples
**Pitch-class GIS (commutative):**
- Every Ti = Pi
- T5{C, E} = {F, A}, and int(C, E) = int(F, A) = 4

**12-tone pitch classes and T6:**
- T6 commutes with every Ti (since 6 + n = n + 6 mod 12)
- T6 preserves intervals (which is redundant here since all Ti do)

**Non-commutative time-span GIS:**
- Only (0, 1) is central (the identity interval)
- T(i,p) does not preserve intervals for (i, p) != (0, 1)
- Transposition can change chronological ordering of events

# Related Concepts
- Transposition Operation (Ti)
- Interval-Preserving Operation (Pi)
- Central Element
- Commutative vs. Non-commutative Groups
- LABEL Function

# Common Confusions
1. Students familiar with commutative GIS may assume all transpositions preserve intervals. This fails in non-commutative cases.

2. The centrality condition (D) is the key: an interval that does not commute with all others cannot yield an interval-preserving transposition.

3. Even in non-commutative GIS, the *family* of interval-preserving operations exists (the Pi). It simply does not coincide with the family of transpositions.

4. The identity interval e is always central, so Te = Pe = identity operation always preserves intervals.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.4.8 and Corollaries 3.4.9, pp. 81-82
