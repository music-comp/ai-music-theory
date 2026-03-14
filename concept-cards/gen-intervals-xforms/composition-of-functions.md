---
concept: Composition of Functions
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
Function composition combines two functions by applying one after the other: the composition f'f means first apply f, then apply f' to the result.

# Formal Definition
Let f be a function from S into S', and let f' be a function from S' into S". Then the composition function f'f is defined from S into S" as follows: Given an argument s in S, the value (f'f)(s) is f'(f(s)).

# Mathematical Formulation
- (f'f)(s) = f'(f(s)) for all s in S
- In left orthography: f' appears to the left of f in the notation f'f
- The composition f'f: S -> S" when f: S -> S' and f': S' -> S"
- Composition is associative: f''(f'f) = (f''f')f

# Musical Context/Application
Composition of functions models sequential musical transformations. If you transpose a pitch class by 2, then invert about C, the result is equivalent to a single compound transformation. Understanding composition is essential for analyzing chains of transformations in twelve-tone music and transformation theory.

# Examples
From Chapter 1: Let S, S', S" all be positive integers. Let f1(s) = s + 3, f2(s) = 2s, f3(s) = 2s, f4(s) = s + 6. Then f2f1 = f4f3: doubling after adding 3 equals adding 6 after doubling.

Musical example from Chapter 1: Let f(s) = s transposed by 2, f'(s) = s inverted about C, and f"(s) = s inverted about B on pitch classes. The equation f'f = f" holds: inverting about C the result of transposing by 2 equals inverting about B directly.

Calculation: f'f(C) = f'(D) = Bb; f"(C) = Bb. Both give the same result.

# Related Concepts
- Function
- Left Orthography
- Right Orthography
- Associativity
- Transformation
- Operation

# Common Confusions
- In f'f, f is applied FIRST, then f' (despite f' appearing on the left)
- This is "left orthography" - function names appear left of arguments
- Composition is associative but NOT generally commutative: f'f may differ from ff'
- The order of composition matters greatly in musical transformations

# Source Reference
Chapter 1: Mathematical Preliminaries, Definition 1.2.3, Section 1.2.4-1.2.5
