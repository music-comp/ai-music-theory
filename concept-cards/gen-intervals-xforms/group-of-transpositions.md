---
concept: "Group of Transpositions (TNSPS)"
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
The transposition operations in a GIS form a group (TNSPS) that is anti-isomorphic to the interval group IVLS. This means the map from intervals to transpositions reverses the order of multiplication.

# Formal Definition
**Theorem 3.4.2:** Each Ti is an operation (1-to-1 and onto as a transformation on S). The transposition operations form a group of operations on S. That group is anti-isomorphic to the group of intervals. Specifically, the map f(i) = Ti is an anti-isomorphism from IVLS onto TNSPS.

# Mathematical Formulation
**Group structure:**
- Closure: Ti Tj = Tji (result is a transposition)
- Identity: Te = identity operation
- Inverses: (Ti)^(-1) = T(i^(-1))
- Associativity: inherited from composition of functions

**Anti-isomorphism properties:**
- f: IVLS -> TNSPS defined by f(i) = Ti
- f is 1-to-1 (Ti = Tj implies i = j)
- f is onto (every transposition is some Ti)
- f(ij) = Tij but Ti Tj = Tji (order reverses!)

**Proof of Ti Tj = Tji:**
int(s, Ti(Tj(s))) = int(s, Tj(s)) + int(Tj(s), Ti(Tj(s)))
                  = j + i = ji (in IVLS)
So Ti Tj(s) lies interval ji from s, meaning Ti Tj = Tji.

# Musical Context/Application
The anti-isomorphism explains why transposition composition seems to "reverse" the order of intervals. In pitch-class theory with additive notation, this appears as:
T5 followed by T3 = T8 (since 5 + 3 = 8 = 3 + 5 in commutative group)

But in non-commutative groups, the reversal matters:
T(i,p) followed by T(j,q) = T((j,q)(i,p)) not T((i,p)(j,q))

# Examples
**Pitch-class transpositions (commutative):**
- T5 T3 = T8 = T3 T5 (order doesn't matter)
- (T5)^(-1) = T7 = T(-5 mod 12)
- Te = T0 = identity

**Time-span transpositions (non-commutative):**
- T(2,3) T(4,5) = T((4,5)(2,3)) = T(4 + 5*2, 5*3) = T(14, 15)
- T(4,5) T(2,3) = T((2,3)(4,5)) = T(2 + 3*4, 3*5) = T(14, 15)
- (Wait--these are equal! But that's because the specific intervals happen to produce the same result, not because the group is commutative.)

**General computation:**
To find Ti Tj, compute ji in IVLS, then the result is Tji.

# Related Concepts
- Transposition Operation (Ti)
- Interval Group (IVLS)
- Anti-isomorphism
- Interval-Preserving Operations (PSVS)
- PETEY Group

# Common Confusions
1. **Anti-isomorphism vs. isomorphism:** The map f(i) = Ti reverses multiplication order. f(ij) = Tij but f(i)f(j) = Ti Tj = Tji.

2. **In commutative case:** Anti-isomorphism = isomorphism since ij = ji. Students from pitch-class theory may not notice the reversal.

3. **Why reversal?** The interval int(s, Ti(s)) = i defines Ti in terms of the interval FROM s TO its image. Composition creates intervals in reverse order.

4. **TNSPS vs. PSVS:** These are both groups, but TNSPS is anti-isomorphic to IVLS while PSVS is isomorphic to IVLS.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.4.2, pp. 77-79
