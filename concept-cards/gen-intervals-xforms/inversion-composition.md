---
concept: Composition of Inversion Operations
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
The composition of two inversion operations yields an operation of the form PT (interval-preserving followed by transposition). This result is fundamental for understanding the group structure of transformations in a GIS.

# Formal Definition
**Theorem 3.5.8:** Fix ref, and let the LABELs of v, u, w, and x be respectively i, j, k, and m. Then:

I_u^v I_x^w = P_{im^(-1)} T_{k^(-1)j}

The composition of two inversions is an interval-preserving operation (with interval im^(-1)) followed by a transposition (by interval k^(-1)j).

# Mathematical Formulation
**Proof:**
LABEL(I_u^v I_x^w(s)) = i(LABEL(I_x^w(s)))^(-1)j
                       = i(k * LABEL(s)^(-1) * m)^(-1)j
                       = i(m^(-1) * LABEL(s) * k^(-1))j
                       = (im^(-1)) * LABEL(s) * (k^(-1)j)
                       = LABEL(P_{im^(-1)} T_{k^(-1)j}(s))

**Corollary 3.5.9:** I_v^u is the inverse operation to I_u^v.

Proof: Take x = v, w = u in Theorem 3.5.8. Then m = i, k = j, so im^(-1) = e and k^(-1)j = e. Thus I_u^v I_v^u = P_e T_e = identity.

# Musical Context/Application
This theorem explains why the composition of two inversions returns to a "direct" transformation (PT). In pitch-class theory, this manifests as: I_a I_b = T_{b-a} (with appropriate index calculations).

The theorem also establishes that the set of all inversions, combined with transpositions and interval-preserving operations, forms a closed group.

# Examples
**Pitch-class calculations:**
In 12-tone GIS with ref = C (so LABEL = pitch-class integer):
- I_0 I_0 = T_0 (identity)
- I_0 I_6 = T_6
- Generally: I_a I_b = T_{b-a} (since P = T in commutative case)

**Inverse of an inversion:**
- (I_C^E)^(-1) = I_E^C
- But in commutative GIS, I_C^E = I_E^C, so inversions are self-inverse
- Formula: I^(-1) = I (Corollary 3.5.10(A) for commutative case)

**Non-commutative case:**
- In time-span GIS: (I_s^t)^(-1) = I_t^s, which may differ from I_s^t
- The composition formula involves both P and T components

# Related Concepts
- Inversion Operation (I_u^v)
- Interval-Preserving Operation (Pi)
- Transposition Operation (Ti)
- PETEY Group
- PETINV Group

# Common Confusions
1. **The formula involves both P and T:** Even in commutative GIS where P = T, the formula structure reveals the two separate roles.

2. **Corollary 3.5.9:** The inverse of I_u^v is I_v^u, NOT I_u^v itself (unless the GIS is commutative and Corollary 3.5.10(A) applies).

3. **In commutative GIS:** I I = T (transposition), and inversions are self-inverse. This is the familiar pitch-class situation.

4. **Order of composition:** I_u^v I_x^w computes as "apply I_x^w first, then I_u^v." The formula gives P_{im^(-1)} T_{k^(-1)j}.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.5.8 and Corollary 3.5.9, pp. 88-89
