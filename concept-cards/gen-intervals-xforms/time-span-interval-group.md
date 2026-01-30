---
concept: Time-Span Interval Group
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
The time-span interval group IVLS consists of pairs (i, p) with a non-commutative composition law. The first component measures relative temporal position (in span-lengths), the second measures duration ratio.

# Formal Definition
**Lemma 4.1.3.1:** Let IVLS be the family of pairs (i, p) where i is real and p is positive real. Then IVLS forms a group under:

(i, p)(j, q) = (i + pj, pq)

Identity: (0, 1)
Inverse: (i, p)^(-1) = (-i/p, 1/p)

This group is non-commutative.

# Mathematical Formulation
**Verification of group axioms:**

Closure: If (i, p) and (j, q) are in IVLS, so is (i + pj, pq).

Associativity:
((i,p)(j,q))(k,r) = (i + pj, pq)(k, r) = (i + pj + pqk, pqr)
(i,p)((j,q)(k,r)) = (i,p)(j + qk, qr) = (i + p(j + qk), pqr) = (i + pj + pqk, pqr)

Identity: (i,p)(0,1) = (i + p*0, p*1) = (i, p)
         (0,1)(i,p) = (0 + 1*i, 1*p) = (i, p)

Inverse: (i,p)(-i/p, 1/p) = (i + p*(-i/p), p*(1/p)) = (0, 1)
        (-i/p, 1/p)(i,p) = (-i/p + (1/p)*i, (1/p)*p) = (0, 1)

**Non-commutativity proof:**
(i, p)(j, q) = (i + pj, pq)
(j, q)(i, p) = (j + qi, qp)
These differ when i + pj =/= j + qi, e.g., (1, 2)(0, 3) = (1, 6) but (0, 3)(1, 2) = (3, 6).

# Musical Context/Application
The group structure reflects how rhythmic relationships compound:
- First component: temporal offset in "first-span units"
- Second component: duration ratio

The non-commutativity captures that "scaling then shifting" differs from "shifting then scaling" in temporal music.

# Examples
**Composition calculation:**
(2, 3)(4, 5) = (2 + 3*4, 3*5) = (14, 15)
Interpretation: 2 spans + 3*(4 spans) = 14 spans; 3*5 = 15 times duration

**Inverse calculation:**
(2, 3)^(-1) = (-2/3, 1/3)
Check: (2, 3)(-2/3, 1/3) = (2 + 3*(-2/3), 3*(1/3)) = (0, 1)

**Order matters:**
(1, 2)(3, 1) = (1 + 2*3, 2*1) = (7, 2)
(3, 1)(1, 2) = (3 + 1*1, 1*2) = (4, 2)
Different results!

**Identity interpretation:**
(0, 1) means "same attack time" (0 span-lengths later) and "same duration" (1 times as long).

# Related Concepts
- Time-Span GIS
- Non-Commutative Groups
- Group Composition
- Central Interval (only (0,1) is central)
- Time-span Transposition

# Common Confusions
1. **The formula (i + pj, pq):** The first component is NOT i + j. The p factor scales j before adding.

2. **Order of composition:** (i, p) then (j, q) gives (i + pj, pq). Think: first interval (i, p), then measure the second interval relative to the scaled context.

3. **Units in first component:** i and j are measured in span-lengths, not absolute time. That's why p scales j.

4. **Why non-commutative?** Scaling (p factor) affects subsequent temporal measurements. "Scale then shift" differs from "shift then scale."

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Lemma 4.1.3.1, pp. 106
