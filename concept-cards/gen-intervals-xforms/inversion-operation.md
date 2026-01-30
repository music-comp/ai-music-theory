---
concept: Inversion Operation (I_u^v)
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
The u/v inversion operation I_u^v maps each element s to an element I_u^v(s) that is "balanced" about u and v in a specific intervallic proportion: the interval from v to I_u^v(s) equals the interval from s to u.

# Formal Definition
Given any u in S and any v in S, the operation I_u^v of u/v inversion is defined by the equation:

int(v, I_u^v(s)) = int(s, u)

for all s in S. This captures the intuition that I(s) bears to v the inverse of the intervallic relationship that s bears to u.

# Mathematical Formulation
**Definition 3.5.1:**
I_u^v(s) = t where int(v, t) = int(s, u)

**Theorem 3.5.2:** Fix ref, let i = LABEL(v), j = LABEL(u). Then:
LABEL(I_u^v(s)) = i * LABEL(s)^(-1) * j

**Key properties:**
- I_u^v is 1-to-1 and onto (an operation)
- I_u^v(u) = v and I_u^v(v) = u
- In commutative GIS: I_u^v = I_v^u (Corollary 3.5.5)
- In non-commutative GIS: I_u^v may differ from I_v^u

# Musical Context/Application
Inversion in a GIS generalizes familiar pitch inversion. For pitch classes, I_C^C (inversion about C) maps each pitch class to its "mirror image" across C. The general definition allows inversion about two different points u and v, creating an asymmetric "balance."

The formula int(v, I(s)) = int(s, u) says: the distance from v to the image equals the distance from the original to u. This is the abstract essence of "reflection" or "inversion."

# Examples
**Pitch-class inversion:**
- I_C^C (inversion about C): maps E to Ab, G to F, etc.
- I_C^C = I_F#^F# = I_D^Bb = ... (all equivalent in commutative case)
- For any I, I(s) + s = constant (the "index" of the inversion)

**Determining when I_u^v = I_w^x:**
In commutative GIS: I_u^v = I_w^x iff w = I_u^v(x)
In non-commutative GIS: I_u^v = I_w^x iff w = I_u^v(x) AND int(x, u) is central

**Figure 3.7 visualization:**
s and I(s) are "balanced" about u and v with arrows showing inverse intervallic proportions.

# Related Concepts
- Transposition Operation (Ti)
- Interval-Preserving Operation (Pi)
- Inversion Index
- Interval-Reversing Operations
- LABEL Function

# Common Confusions
1. **I_u^v vs. I_v^u:** In commutative GIS these are the same operation. In non-commutative GIS they may differ, even though both swap u and v.

2. **The formula:** int(v, I_u^v(s)) = int(s, u) places v and s on opposite sides of their respective intervals. This can be confusing--the interval is FROM v TO the image, but FROM the original TO u.

3. **Corollary 3.5.4:** I_u^v = I_v^u iff int(v, u) is central. In non-commutative GIS, this fails for most pairs (u, v).

4. **Inversion is an operation:** Students should verify that I_u^v is indeed 1-to-1 and onto.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Definitions 3.5.1-3.5.5, pp. 82-86
