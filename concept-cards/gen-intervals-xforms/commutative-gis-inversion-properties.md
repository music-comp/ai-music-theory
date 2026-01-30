---
concept: Inversion Properties in Commutative GIS
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
In a commutative GIS, inversions have special properties: every inversion is self-inverse (I = I^(-1)), and the composition IT equals T^(-1)I. These properties simplify calculations and match familiar pitch-class inversion behavior.

# Formal Definition
**Corollary 3.5.10:** Let T and I be any transposition operation and any inversion operation in a commutative GIS. Then:

(A): I^(-1) = I (every inversion is self-inverse/involutory)
(B): IT = T^(-1)I (inversion conjugates transposition to its inverse)

# Mathematical Formulation
**Proof of (A):**
From Corollary 3.5.9, (I_u^v)^(-1) = I_v^u.
In commutative GIS, I_u^v = I_v^u (Corollary 3.5.5).
Therefore I^(-1) = I.

**Proof of (B):**
Set J = IT. By Theorem 3.5.6(B), J is an inversion operation.
By (A), J = J^(-1).
So IT = J = J^(-1) = (IT)^(-1) = T^(-1)I^(-1) = T^(-1)I.

**Alternative form of (B):**
Multiplying both sides of IT = T^(-1)I by T on the left:
TIT = I, or equivalently, TI = IT^(-1).

# Musical Context/Application
These properties are familiar from standard pitch-class theory:
- Inversions are involutions (applying twice returns to the original)
- Transposition "passes through" inversion with sign change

Property (B) explains why, in pitch-class analysis, we can rewrite ITn as T(-n)I: the transposition index negates when moved past an inversion.

# Examples
**Pitch-class example:**
- I_0(x) = -x (mod 12) is self-inverse: I_0(I_0(x)) = -(-x) = x
- I_0 T_5 = T_(-5) I_0 = T_7 I_0
- Verification: I_0 T_5(0) = I_0(5) = 7; T_7 I_0(0) = T_7(0) = 7

**Commutator interpretation:**
Property (B) can be rewritten: I T I^(-1) = T^(-1).
This says inversion conjugates transposition to its inverse.
In group theory terms: I normalizes the transposition group.

**Row operations:**
In twelve-tone theory, these properties explain why:
- Applying I twice returns the original row
- RI = IR (retrograde and inversion commute in effect)

# Related Concepts
- Inversion Operation (I_u^v)
- Transposition Operation (Ti)
- Involutory Transformation
- Commutative GIS
- Group Conjugation

# Common Confusions
1. **These properties fail in non-commutative GIS:** In time-span GIS, I =/= I^(-1) generally, and IT =/= T^(-1)I.

2. **I^(-1) = I does NOT mean I = identity:** Self-inverse means I*I = identity, not I = identity. Inversions are non-trivial involutions.

3. **Formula (B) IT = T^(-1)I:** The transposition index "negates" when passing through an inversion. In additive notation: I Tn = T(-n) I.

4. Students familiar with only commutative GIS may assume these properties hold universally. They are specific to the commutative case.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Corollary 3.5.10, pp. 89
