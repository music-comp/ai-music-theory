---
concept: Combination of Inversion and Interval-Preserving Operations
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
When an interval-preserving operation P and an inversion I_u^v are composed, the result is another inversion. The composition formulas show how P moves one of the inversion parameters while leaving the other fixed.

# Formal Definition
**Theorem 3.5.7:** For any interval-preserving operation P and any inversion I_u^v:

(A): P I_u^v = I_u^w where w = P(v)
(B): I_u^v P = I_x^v where x = P^(-1)(u)
(C): P commutes with I_u^v iff P = Tc for some transposition Tc such that c is central and cc = e

# Mathematical Formulation
**Proof sketch:**
Setting n = int(ref, P(ref)), we write P = Pn. The LABEL manipulations parallel those for Theorem 3.5.6.

**Part (C):** P I_u^v = I_u^v P requires I_u^w = I_x^v.
By Theorem 3.5.3, w = I_x^v(u) and int(u, x) = j^(-1)nj must be central.
One shows j^(-1)nj is central iff n is central.
When n is central, P = Pn = Tn (by Theorem 3.4.8).
The condition reduces to that of Theorem 3.5.6(C).

# Musical Context/Application
This theorem parallels Theorem 3.5.6 for transpositions, showing that interval-preserving operations combine with inversions in a structured way. The commutation condition (C) shows that the P operations that commute with inversions are precisely the transpositions with central, self-inverse intervals.

In commutative GIS, P = T always, so this reduces to the previous theorem. In non-commutative GIS, it distinguishes the roles of P and T.

# Examples
**Pitch-class GIS (commutative):**
Since P = T, this reduces to Theorem 3.5.6.
- P5 I_C^C = I_C^F
- I_C^C P5 = I_Ab^C

**Non-commutative time-span GIS:**
- P(h,u) I_{(c,z)}^{(d,w)} = I_{(c,z)}^{P(h,u)(d,w)}
- The "u" parameter (c, z) stays fixed
- The "v" parameter (d, w) is transformed to P(h,u)(d, w) = (h + ud, uw)

**Commutation:**
P commutes with some inversion iff P is a transposition Tc with c central and cc = e. In time-span GIS, only the identity satisfies this.

# Related Concepts
- Interval-Preserving Operation (Pi)
- Inversion Operation (I_u^v)
- Transposition Operation (Ti)
- Central Element
- Combination of Inversion and Transposition

# Common Confusions
1. **P vs. T in compositions:** In commutative GIS these coincide. In non-commutative GIS, P I gives I_u^{P(v)} while T I gives I_{T(u)}^v. The moving parameter differs!

2. **Order matters:** P I_u^v moves v to P(v); I_u^v P moves u to P^(-1)(u). Both result in inversions, but with different parameter changes.

3. **Condition (C):** P commutes with inversions only when P is a transposition Tc with c central and self-inverse. This is more restrictive than just being interval-preserving.

4. The group-theoretic proof uses the fact that j^(-1)nj is central iff n is central--a lemma about conjugates of central elements.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.5.7, pp. 87-88
