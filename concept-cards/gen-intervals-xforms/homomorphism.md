---
concept: Homomorphism
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
A homomorphism is a function between semigroups (or groups) that preserves the algebraic structure: the image of a product equals the product of the images.

# Formal Definition
A function f from a semigroup (X, BIN) into a semigroup (X', BIN') is a homomorphism if it satisfies the law: BIN'(f(x1), f(x2)) = f(BIN(x1, x2)) for all x1 and all x2 in X. Colloquially: "The combination of the images is the image of the combination." In multiplicative notation: f(x1)f(x2) = f(x1x2).

# Mathematical Formulation
- f: (X, BIN) -> (X', BIN') is a homomorphism if f(x1)f(x2) = f(x1x2)
- For groups: f(e) = e' (identity maps to identity)
- For groups: f(x^(-1)) = f(x)^(-1) (inverse maps to inverse)
- Homomorphic image of a group is a group (Theorem 1.12.2)
- Natural map to quotient is always a homomorphism: C(x1)C(x2) = C(x1x2)

# Musical Context/Application
Homomorphisms formalize structure-preserving relationships between musical systems. The map from chromatic pitch intervals to pitch-class intervals (reducing mod 12) is a homomorphism. Maps between different GIS structures that preserve interval relationships are homomorphisms. Understanding homomorphisms helps identify when two musical systems share the same abstract structure.

# Examples
The natural map C from integers to integers mod 12:
- C(x1) + C(x2) = C(x1 + x2)
- C(5) + C(8) = C(13) = C(1) = C(5 + 8)
- This is a homomorphism from (Z, +) onto (Z12, +)

From Theorem 1.12.1: If f is a homomorphism and e is the identity in (X, BIN), then f(e) is the identity in (X', BIN'). If x has inverse x^(-1), then f(x^(-1)) is the inverse of f(x).

Section 1.11.3: Any homomorphic image of a semigroup is isomorphic to some quotient semigroup.

# Related Concepts
- Isomorphism
- Anti-homomorphism
- Congruence
- Quotient Group
- Natural Map
- Structure-Preserving Map

# Common Confusions
- A homomorphism need not be 1-to-1 (that would be an isomorphism)
- A homomorphism need not be onto
- The key property is f(xy) = f(x)f(y), not preservation of individual elements
- Anti-homomorphisms satisfy f(xy) = f(y)f(x) (reversed order)

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.11.1, Theorems 1.12.1-1.12.2
