---
concept: Left vs. Right Group Operations
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
In a non-commutative group, left multiplication (g * h) and right multiplication (h * g) generally give different results. This distinction is crucial for understanding why transpositions and interval-preserving operations differ in non-commutative GIS.

# Formal Definition
In a group G with elements g, h:
- Left multiplication by g: L_g(h) = g * h
- Right multiplication by g: R_g(h) = h * g

In a commutative group: L_g = R_g for all g.
In a non-commutative group: L_g =/= R_g for some g.

# Mathematical Formulation
**In GIS context:**
- Interval-preserving operations use left multiplication:
  LABEL(P_i(s)) = i * LABEL(s)

- Transpositions use right multiplication:
  LABEL(T_i(s)) = LABEL(s) * i

**Consequence:**
In commutative GIS: P_i = T_i
In non-commutative GIS: P_i =/= T_i (except for central i)

**Time-span formulas:**
T_(i,p)(a, x) = (a, x) * (i, p) = (a + ix, px)    [right multiply]
P_(h,u)(a, x) = (h, u) * (a, x) = (h + ua, ux)    [left multiply]

These differ!

# Musical Context/Application
The left/right distinction explains why:
- P operations preserve intervals (uniform transformation of entire space)
- T operations may distort intervals (context-dependent transformation)

In the time-span GIS:
- P_(h,u) scales everything uniformly, then shifts (preserves proportions)
- T_(i,p) shifts each span by a multiple of its own duration (depends on span)

# Examples
**Time-span calculations:**
(0, 1) * (2, 3) = (0 + 1*2, 1*3) = (2, 3)    [right multiply]
(2, 3) * (0, 1) = (2 + 3*0, 3*1) = (2, 3)    [left multiply--same for identity]

(1, 2) * (3, 4) = (1 + 2*3, 2*4) = (7, 8)    [right multiply]
(3, 4) * (1, 2) = (3 + 4*1, 4*2) = (7, 8)    [left multiply--same here by chance]

(1, 2) * (0, 3) = (1 + 2*0, 2*3) = (1, 6)    [right multiply]
(0, 3) * (1, 2) = (0 + 3*1, 3*2) = (3, 6)    [left multiply--DIFFERENT!]

**Interpretation:**
T_(0,3)(1, 2) = (1, 2) * (0, 3) = (1, 6)
P_(0,3)(1, 2) = (0, 3) * (1, 2) = (3, 6)

Different operations!

# Related Concepts
- Interval-Preserving Operation (Pi)
- Transposition Operation (Ti)
- Non-Commutative Groups
- Time-Span GIS
- Central Interval

# Common Confusions
1. **Notation ambiguity:** Some texts use L_g for left action, others for the element. Clarify which convention is in use.

2. **Order in products:** (a, x) * (i, p) puts (i, p) on the right; (h, u) * (a, x) puts (h, u) on the left.

3. **Why T uses right:** The definition int(s, T_i(s)) = i places i as the interval FROM s TO its image. Interval composition is contravariant, leading to right multiplication.

4. **Why P uses left:** The definition LABEL(P_i(s)) = i * LABEL(s) directly left-multiplies the label by i.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Notes 4.1.7 and related discussion, pp. 112-114
