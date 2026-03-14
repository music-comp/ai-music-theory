---
concept: Semigroup
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
A semigroup is a set with an associative binary operation - elements can be combined, and the combination of three elements doesn't depend on grouping.

# Formal Definition
A semigroup is an ordered pair (X, BIN) comprising a family X and an associative binary composition BIN on X. A binary composition is associative if BIN(x, BIN(y, z)) = BIN(BIN(x, y), z) for all x, y, and z. A closed collection of transformations on S (where fg is in the collection whenever f and g are) is a semigroup of transformations on S.

# Mathematical Formulation
- A semigroup is (X, BIN) where BIN: X x X -> X is associative
- Associative Law: x(yz) = (xy)z for all x, y, z (using multiplicative notation)
- Closure: if x, y are in X, then xy is in X
- No identity element is required (unlike monoids)
- No inverses are required (unlike groups)

# Musical Context/Application
Semigroups model collections of transformations that can be composed but may lack inverses. For example, a collection of transformations that includes "collapse all pitches to C" is a semigroup but not a group (no inverse exists for that transformation). Semigroups are more general than groups, accommodating a wider range of musical transformations.

# Examples
Semigroup of transformations: A collection F of transformations on pitch classes is a semigroup if composing any two transformations in F yields another transformation in F.

Non-associative example (NOT a semigroup): Exponentiation on natural numbers. BIN(3, BIN(2, 3)) = 3^8 = 6561, but BIN(BIN(3, 2), 3) = 9^3 = 729. Since 6561 != 729, exponentiation is not associative.

Abstract example: (integers, addition) and (integers, multiplication) are both semigroups (in fact, groups).

# Related Concepts
- Group
- Binary Composition
- Associativity
- Semigroup of Transformations
- Identity Element

# Common Confusions
- Semigroups need not have an identity (those with identity are called "monoids")
- Semigroups need not have inverses (those with inverses for all elements are groups)
- Not all binary operations are associative (exponentiation is a counterexample)
- The term "semigroup" emphasizes closure and associativity only

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.3.2, 1.4.1-1.4.3
