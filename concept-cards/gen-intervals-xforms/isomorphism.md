---
concept: Isomorphism
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
An isomorphism is a 1-to-1 homomorphism onto another semigroup - a bijective structure-preserving map that establishes two algebraic structures as "essentially the same."

# Formal Definition
A homomorphism is an isomorphism (into) if it is 1-to-1. If f is an isomorphism of (X, BIN) onto (X', BIN'), we say the two semigroups are isomorphic (via f). In that case, the inverse map f^(-1) is an isomorphism of (X', BIN') onto (X, BIN).

# Mathematical Formulation
- f: (X, BIN) -> (X', BIN') is an isomorphism if:
  - f is a homomorphism: f(x1x2) = f(x1)f(x2)
  - f is 1-to-1 (injective)
  - f is onto (surjective)
- If (X, BIN) and (X', BIN') are isomorphic, they have identical algebraic structure
- Notation: (X, BIN) is isomorphic to (X', BIN') means there exists an isomorphism between them

# Musical Context/Application
Isomorphic groups have the same abstract structure even if their elements look different. The group of pitch-class transpositions (T0 through T11) is isomorphic to Z12 (integers mod 12). This means anything we can prove about Z12 automatically applies to transpositions. Recognizing isomorphisms helps transfer results between different musical domains.

# Examples
From Section 1.11.3: The homomorphic image semigroup (X', BIN') is isomorphic to the quotient semigroup (X, BIN)/CONG. This means every quotient construction can be understood as a homomorphic image and vice versa.

Isomorphic examples:
- Transpositions {T0, T1, ..., T11} under composition is isomorphic to (Z12, +)
- The isomorphism sends Tn to n, and TmTn = Tm+n corresponds to m + n in Z12

From Section 1.11.4: For transpositions Ti and interval-preserving operations Pi, the map i -> Pi is an isomorphism while i -> Ti is an anti-isomorphism.

# Related Concepts
- Homomorphism
- Anti-isomorphism
- One-to-One Function
- Onto Function
- Quotient Group

# Common Confusions
- Isomorphism requires being both 1-to-1 AND onto AND a homomorphism
- Isomorphic structures are "the same" algebraically, just with different labels
- The inverse of an isomorphism is also an isomorphism
- Anti-isomorphism reverses the operation order: f(xy) = f(y)f(x)

# Source Reference
Chapter 1: Mathematical Preliminaries, Definition 1.11.2, Section 1.11.3
