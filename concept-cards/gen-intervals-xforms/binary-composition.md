---
concept: Binary Composition
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
A binary composition on a set X is a function that takes two elements of X and produces another element of X, such as addition or multiplication.

# Formal Definition
A binary composition on X is a function BIN that maps X x X into X. We write BIN(x, y) for the value of BIN on the pair (x, y). In multiplicative notation, we write "xy" to signify BIN(x, y).

# Mathematical Formulation
- BIN: X x X -> X
- BIN(x, y) is the result of composing x with y
- Multiplicative notation: xy = BIN(x, y)
- Additive notation: x + y = BIN(x, y)
- Closure is built into the definition: the result is always in X

# Musical Context/Application
Binary composition is how transformations combine. When we compose two transpositions or two inversions, we use a binary composition to determine the result. The specific rule for composition depends on the transformation type: transpositions add (T3T5 = T8), while combining transpositions and inversions follows the T/I group multiplication table.

# Examples
Examples of binary compositions:
- Addition on integers: BIN(3, 5) = 8
- Multiplication on positive reals: BIN(3, 5) = 15
- Composition of transformations: BIN(T3, T5) = T8

Non-example of associativity: Exponentiation BIN(x, y) = x^y is a binary composition on natural numbers, but it is not associative:
- BIN(3, BIN(2, 3)) = 3^(2^3) = 3^8 = 6561
- BIN(BIN(3, 2), 3) = (3^2)^3 = 9^3 = 729

# Related Concepts
- Semigroup
- Associativity
- Group
- Composition of Functions
- Commutativity

# Common Confusions
- Not every binary composition is associative (needed for semigroup)
- Not every binary composition is commutative (xy may differ from yx)
- The term "binary" means two inputs, not binary numbers
- "Composition" here is abstract, not specifically function composition

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.4.1-1.4.2
