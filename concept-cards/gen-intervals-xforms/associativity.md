---
concept: Associativity
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
Associativity is the property that grouping doesn't matter: x(yz) = (xy)z for all elements. The order of applying operations is independent of how they are parenthesized.

# Formal Definition
A binary composition on X is associative if BIN(x, BIN(y, z)) = BIN(BIN(x, y), z) for all x, y, and z. In multiplicative notation: x(yz) = (xy)z. Transformational composition is always associative: f(gh) = (fg)h, meaning applying f to the (gh)-transform of s gives the same result as applying (fg) to the h-transform of s.

# Mathematical Formulation
- Associative Law: x(yz) = (xy)z for all x, y, z in X
- For transformations: f(gh) = (fg)h
- Allows unambiguous notation xyz without parentheses
- Required for semigroup and group structure
- Composition of functions is always associative

# Musical Context/Application
Associativity ensures that chains of musical transformations can be computed in any order. When analyzing T3T5T2, we can compute (T3T5)T2 = T8T2 = T10, or T3(T5T2) = T3T7 = T10 - same result. This property is essential for building transformation networks where paths can be evaluated step by step in any grouping.

# Examples
From Chapter 1: For any sample s, the result of applying f to the (gh)-transform of s is the same as applying (fg) to the h-transform of s.

Associative examples:
- Integer addition: (2 + 3) + 4 = 5 + 4 = 9 = 2 + 7 = 2 + (3 + 4)
- Transformation composition: T3(T5T2) = T3T7 = T10 = T8T2 = (T3T5)T2

Non-associative example: Exponentiation is NOT associative:
- 3^(2^3) = 3^8 = 6561
- (3^2)^3 = 9^3 = 729
- 6561 != 729

# Related Concepts
- Binary Composition
- Semigroup
- Group
- Composition of Functions
- Commutativity

# Common Confusions
- Associativity concerns grouping (parentheses), not order (commutativity)
- xy and yx may differ even when (xy)z = x(yz)
- Composition of functions is always associative, even when not commutative
- Non-associative operations (like exponentiation) cannot form semigroups

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.4.2-1.4.3, Section 1.3.5
