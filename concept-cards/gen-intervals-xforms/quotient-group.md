---
concept: Quotient Group
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
A quotient group is formed by taking a group and "dividing out" by a congruence - the congruence classes become the elements of a new group.

# Formal Definition
Let CONG be a congruence on the semigroup (X, BIN). Then the quotient family X/CONG becomes a semigroup itself under the binary composition (BIN/CONG)(C1, C2) = C3, where C3 is the unique congruence class containing x1x2 whenever x1 is in C1 and x2 is in C2. If the original structure is a group, the quotient semigroup is also a group (Theorem 1.12.3).

# Mathematical Formulation
- Given congruence CONG on group G, the quotient G/CONG is a group
- Elements of G/CONG are congruence classes C(x)
- Operation: C(x1)C(x2) = C(x1x2)
- Identity: C(e) where e is the identity of G
- Inverses: C(x)^(-1) = C(x^(-1))
- The natural map C: G -> G/CONG is a homomorphism

# Musical Context/Application
The integers mod 12 form a quotient group of the integers under addition, modeling pitch-class intervals. The pitch-class interval group Z12 is the quotient of Z by the subgroup 12Z. When we work with pitch classes instead of pitches, we are implicitly working in a quotient structure. Duration-classes mod M (as in beat-class theory) also form quotient groups.

# Examples
Example 1.10.4.1: The integers under addition, modulo 12:
- Congruence classes: C(0), C(1), ..., C(11)
- Addition: C(5) + C(8) = C(13) = C(1)
- Identity: C(0)
- Inverse of C(5) is C(7) since C(5) + C(7) = C(12) = C(0)
- This is Z12, the pitch-class interval group

Example 1.10.4.2: Frequency ratios mod powers of 2 give pitch-class intervals in just intonation.

Theorem 1.12.3: Any quotient semigroup of a group is itself a group.

# Related Concepts
- Group
- Congruence
- Homomorphism
- Natural Map
- Modular Arithmetic
- Integers Mod N

# Common Confusions
- Quotient groups require the equivalence relation to be a congruence
- The quotient of a group is always a group (not just a semigroup)
- Different congruences on the same group give different quotient groups
- The natural map to a quotient group is always a homomorphism

# Source Reference
Chapter 1: Mathematical Preliminaries, Theorems 1.10.3, 1.12.3, Example 1.10.4.1
