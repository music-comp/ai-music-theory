---
concept: Function
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
A function (or mapping) from set S into set S' assigns to each element of S exactly one corresponding element in S'.

# Formal Definition
A function or mapping from S into S' is a subfamily f of S x S' which has this property: Given any s in S, there is exactly one pair (s, s') within the family f which has the given s as the first entry of the pair. The element s' is called the value of the function f for the argument s, written f(s) = s'.

# Mathematical Formulation
- f: S -> S' denotes a function from S into S'
- For each s in S, there exists a unique s' in S' such that (s, s') is in f
- f(s) = s' means s' is the value of f at argument s
- Visualized as a table: each member of S appears exactly once in the left column
- Members of S' may appear multiple times or not at all in the right column

# Musical Context/Application
Functions are foundational to transformation theory in music. Musical transformations (transposition, inversion, retrograde) are all functions that map musical elements to other musical elements. For example, transposition by 2 semitones is a function from pitch classes to pitch classes: T2(C) = D, T2(D) = E, etc.

# Examples
From Chapter 1: Consider S, S', and S" all to be the family of positive integers. Let f1(s) = s + 3, f2(s) = 2s. These are functions from integers to integers.

Musical example: Let S and S' both be the family of twelve pitch classes. Let f(s) = s transposed by 2. This function maps each pitch class to another pitch class: f(C) = D, f(C#) = D#, etc.

# Related Concepts
- One-to-One Function
- Onto Function
- Composition of Functions
- Transformation
- Operation
- Inverse Function

# Common Confusions
- A function requires exactly one output for each input (not zero, not multiple)
- The same output may result from different inputs (unless the function is 1-to-1)
- "Function" and "mapping" are synonymous in this context
- The domain S must be fully covered (every element has a value), but the codomain S' need not be

# Source Reference
Chapter 1: Mathematical Preliminaries, Definition 1.2.1
