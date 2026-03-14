---
concept: Involutory Elements
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
An involutory element (or involution) in a group is an element i satisfying i * i = e (identity). In GIS theory, transpositions by involutory central elements commute with all inversions.

# Formal Definition
An element n of a group G is involutory if:
n * n = e (equivalently, n = n^(-1))

In the context of GIS transformations, Theorem 3.5.6(C) shows that T_n commutes with all inversions if and only if n is central AND involutory.

# Mathematical Formulation
**Involution condition:**
n^2 = e, or equivalently n = n^(-1)

**Role in GIS (Theorem 3.5.6(C)):**
T_n commutes with I_u^v iff:
1. n is central (commutes with all intervals), AND
2. nn = e (n is involutory)

**In commutative GIS:**
Condition 1 is automatic. So T_n commutes with inversions iff n^2 = e.

**12-tone pitch classes:**
n^2 = 0 (mod 12) iff n = 0 or n = 6.
So only T_0 and T_6 commute with inversions.

# Musical Context/Application
Involutory transpositions have special status: they commute with all inversions. In 12-tone pitch-class theory, T_6 (tritone transposition) is the unique non-trivial transposition with this property.

This explains why the tritone plays a special role in twelve-tone operations: it's the only transposition that can be freely reordered with inversions.

# Examples
**Pitch-class involutions:**
- 0 + 0 = 0 ✓
- 6 + 6 = 12 = 0 ✓
- 3 + 3 = 6 ✗
- 4 + 4 = 8 ✗

Only 0 and 6 are involutory mod 12.

**Commutation:**
T_6 I_0 = I_0 T_6 (for any inversion I in 12-tone)
T_5 I_0 ≠ I_0 T_5 (generally)

**Time-span GIS:**
(i, p)^2 = (i + pi, p^2) = (0, 1) requires:
- p^2 = 1, so p = 1
- i + 1*i = 0, so i = 0

Only (0, 1) is involutory. Thus only the identity transposition commutes with inversions.

**Consequence:**
In time-span GIS, no non-trivial transposition commutes with any inversion.

# Related Concepts
- Combination of Inversion and Transposition
- Central Interval
- Commutation of Operations
- Tritone
- Self-inverse

# Common Confusions
1. **Involutory ≠ central:** An element can be involutory without being central, or central without being involutory. Both conditions are needed for T_n to commute with inversions.

2. **In commutative groups:** All elements are central, so only involutory condition matters.

3. **"Self-inverse":** Another name for involutory. The element is its own inverse.

4. **Why T_6 is special:** It's the unique non-identity involutory element in Z/12Z, hence the unique non-trivial transposition commuting with inversions.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.5.6(C) and discussion, pp. 87-88
