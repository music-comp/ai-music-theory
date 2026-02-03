---
concept: "Group of Interval-Preserving Operations (PSVS)"
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
The interval-preserving operations form a group (PSVS) that is isomorphic (not anti-isomorphic) to the interval group IVLS. The composition Pi Pj = Pij preserves the order of multiplication.

# Formal Definition
**Theorem 3.4.5:** The transformations Pi form a group of operations isomorphic to IVLS under the map f(i) = Pi. In particular, the formula Pi Pj = Pij is valid.

# Mathematical Formulation
**Group structure:**
- Closure: Pi Pj = Pij (result is interval-preserving)
- Identity: Pe = identity operation
- Inverses: (Pi)^(-1) = P(i^(-1))
- Associativity: inherited from composition

**Isomorphism properties:**
- f: IVLS -> PSVS defined by f(i) = Pi
- f is 1-to-1 (Pi = Pj implies i = j)
- f is onto (every interval-preserving op is some Pi)
- f(ij) = Pij = Pi Pj = f(i)f(j) (order preserved!)

**Proof of Pi Pj = Pij:**
LABEL(Pi(Pj(s))) = i * LABEL(Pj(s))
                 = i * (j * LABEL(s))
                 = (ij) * LABEL(s)
                 = LABEL(Pij(s))
So Pi Pj(s) = Pij(s) for all s, hence Pi Pj = Pij.

# Musical Context/Application
PSVS captures all transformations that preserve intervallic relationships. In commutative GIS, PSVS = TNSPS (the same group). In non-commutative GIS, these are distinct groups with different algebraic relationships to IVLS.

The isomorphism (versus anti-isomorphism for transpositions) reflects the different ways T and P operations interact with the GIS structure.

# Examples
**Pitch-class operations (commutative):**
- PSVS = TNSPS (identical)
- P5 P3 = P8 = P3 P5

**Time-span interval-preserving operations:**
- P(h,u) defined by LABEL(P(h,u)(a,x)) = (h,u) * (a,x) = (h + ua, ux)
- P(h,u) P(k,v) = P((h,u)(k,v)) = P(h + uk, uv)
- Order is preserved: first parameter (h,u), second (k,v), result is their product

**Contrast with transpositions:**
- TNSPS: Ti Tj = Tji (anti-isomorphism)
- PSVS: Pi Pj = Pij (isomorphism)
- In non-commutative case, Tji =/= Tij generally

# Related Concepts
- Interval-Preserving Operation (Pi)
- Interval Group (IVLS)
- Isomorphism
- Group of Transpositions (TNSPS)
- PETEY Group

# Common Confusions
1. **Isomorphism vs. anti-isomorphism:** PSVS is isomorphic to IVLS (order preserved); TNSPS is anti-isomorphic (order reversed).

2. **In commutative GIS:** PSVS = TNSPS, and both are isomorphic (= anti-isomorphic when group is abelian) to IVLS.

3. **Why the difference?** Pi is defined by LABEL(Pi(s)) = i * LABEL(s) (left multiplication). Ti is defined by int(s, Ti(s)) = i. These create opposite relationships to composition.

4. **Reference-dependence:** The specific Pi depends on ref, but the group PSVS as a whole is the same regardless of ref choice.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.4.5, pp. 79-80
