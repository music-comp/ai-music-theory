---
concept: Inversion Equivalence Conditions
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
Two inversions I_u^v and I_x^w are the same operation if and only if w = I_u^v(x) and int(x, u) is central. In commutative GIS, the centrality condition is automatic; in non-commutative GIS, it provides an additional constraint.

# Formal Definition
**Theorem 3.5.3:** I_u^v = I_x^w as an operation on S if and only if:
1. w = I_u^v(x), AND
2. The interval int(x, u) is central

In a commutative GIS, condition (2) is automatically satisfied, so the criterion reduces to: I_u^v = I_x^w iff w = I_u^v(x).

# Mathematical Formulation
**Proof outline:**
The condition I_u^v = I_x^w means that for all s:
i * LABEL(s)^(-1) * j = k * LABEL(s)^(-1) * m

where i, j, k, m are the LABELs of v, u, w, x respectively.

This holds for all s iff (k^(-1) * i) * n = n * (m * j^(-1)) for every n in IVLS.

This is equivalent to: k^(-1) * i = m * j^(-1) = c where c is central.

From this one derives: int(v, w) = int(x, u) and int(x, u) is central.

**Corollary 3.5.4:** I_u^v = I_v^u iff int(v, u) is central.

# Musical Context/Application
In familiar pitch-class analysis, many different "index pairs" define the same inversion operation. For example, I_0 (in standard notation) equals I_C^C = I_G^F = I_D^Bb, etc. The theorem explains when such equivalences hold.

In non-commutative GIS, inversions are more rigid: the additional centrality constraint means fewer pairs (u, v) and (x, w) define the same inversion.

# Examples
**Pitch-class equivalences:**
- I_C^C = I_F#^F# since F# = I_C^C(F#) and all intervals are central
- I_C^E = I_A^G since G = I_C^E(A) and int(A, C) = 3 is central (trivially, in commutative group)

**Non-commutative time-span GIS:**
- I_s^t = I_{s'}^{t'} only when s' = s AND t' = t (Corollary in Notes 4.1.7(H))
- This is because only (0, 1) is central in the time-span interval group

**Counting distinct inversions:**
In commutative GIS with n elements: typically n distinct inversion operations
In non-commutative GIS: potentially n^2 distinct inversions (one for each ordered pair)

# Related Concepts
- Inversion Operation (I_u^v)
- Central Element
- Commutative vs. Non-commutative Groups
- Interval Function

# Common Confusions
1. In commutative GIS, students learn one set of rules for inversion equivalence. These rules DO NOT generalize to non-commutative GIS without modification.

2. The condition w = I_u^v(x) is necessary but not sufficient in non-commutative cases. The centrality condition is the additional constraint.

3. Corollary 3.5.4 (I_u^v = I_v^u iff int(v,u) central) shows that "inversion about u and v" may depend on order in non-commutative GIS.

4. The formula involves LABEL, but the final criterion (Theorem 3.5.3) is stated without reference to a particular ref.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.5.3 and Corollaries 3.5.4-3.5.5, pp. 84-86
