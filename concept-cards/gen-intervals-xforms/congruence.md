---
concept: Congruence
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
A congruence is an equivalence relation on a semigroup that respects the algebraic structure: if x1 ~ y1 and x2 ~ y2, then x1x2 ~ y1y2.

# Formal Definition
An equivalence relation on a semigroup is a congruence if it has this property: Given x1 equivalent to y1 and x2 equivalent to y2, then x1x2 is equivalent to y1y2. This ensures that the quotient of a semigroup by a congruence inherits a well-defined semigroup structure.

# Mathematical Formulation
- A congruence CONG on (X, BIN) satisfies:
  - Reflexive, symmetric, transitive (equivalence relation)
  - If x1 ~ y1 and x2 ~ y2, then x1x2 ~ y1y2 (compatibility with operation)
- For congruence classes C1, C2, there is a unique class C3 such that x1x2 is in C3 whenever x1 is in C1 and x2 is in C2
- This defines a binary operation on X/CONG, making it a semigroup

# Musical Context/Application
Congruence modulo 12 on integers gives the pitch-class interval group Z12. Congruence modulo powers of 2 on frequency ratios gives pitch-class intervals in just intonation (intervals modulo the octave). Congruences allow us to work with "reduced" interval systems where certain distinctions (like octaves) are collapsed.

# Examples
Example 1.10.4.1: On integers under addition, define (x, y) to be congruent if y - x is a multiple of 12. This is a congruence. The quotient is "integers mod 12" with 12 classes C(0), C(1), ..., C(11). The induced operation: C(5) + C(8) = C(1), since 5 + 8 = 13 = 1 mod 12.

Example 1.10.4.2: On rational numbers 2^a * 3^b * 5^c under multiplication, define (x, y) to be congruent if y = x * 2^n for some integer n. The quotient models pitch-class intervals in just intonation.

Verification: If y1 - x1 = 12m and y2 - x2 = 12n, then (y1 + y2) - (x1 + x2) = 12(m + n), confirming closure.

# Related Concepts
- Equivalence Relation
- Quotient Group
- Homomorphism
- Modular Arithmetic
- Natural Map

# Common Confusions
- A congruence is more than just an equivalence relation - it must respect the operation
- The quotient X/CONG inherits a semigroup structure precisely because of compatibility
- Different congruences on the same semigroup yield different quotient structures
- "Congruence mod n" is a specific example of the general concept

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.10.1-1.10.3, Examples 1.10.4.1-1.10.4.2
