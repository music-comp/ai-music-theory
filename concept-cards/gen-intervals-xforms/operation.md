---
concept: Operation
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
An operation on a set S is a transformation that is both 1-to-1 and onto - a bijective self-mapping that has an inverse.

# Formal Definition
A function from a family S into S itself will be called a transformation on S. If the function is 1-to-1 and onto, it will be called an operation on S. Equivalently, a transformation f on S is an operation if and only if there exists a transformation f' on S satisfying f'f = 1 and ff' = 1, where 1 is the identity.

# Mathematical Formulation
- An operation f: S -> S is a bijection (1-to-1 and onto)
- For an operation f, there exists a unique inverse f^(-1) such that:
  - f^(-1)f = 1 (identity)
  - ff^(-1) = 1 (identity)
- Operations are closed under composition: if f, g are operations, fg is an operation
- Operations form the building blocks of groups

# Musical Context/Application
Operations are the most important transformations in music theory because they are invertible - every transformation can be "undone." Transposition and inversion are operations on pitch classes. When we analyze music using transformation theory, we typically work with operations because they preserve the structure of the musical space completely (no information is lost).

# Examples
Operations on pitch classes:
- T5 (transpose by 5): T5^(-1) = T7 (since T5T7 = T0 = identity)
- I0 (invert about C): I0^(-1) = I0 (inversion is its own inverse)
- All 24 transposition/inversion operations form a group

Non-operation example: The "constant" transformation f(s) = C for all pitch classes s is a transformation but NOT an operation (it is neither 1-to-1 nor onto).

# Related Concepts
- Transformation
- One-to-One Function
- Onto Function
- Inverse Function
- Group of Operations
- Identity Transformation

# Common Confusions
- Every operation is a transformation, but not vice versa
- "Operation" in Lewin's technical sense specifically means bijective transformation
- This differs from colloquial use of "operation" (like "arithmetic operation")
- The inverse of an operation is also an operation

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.3.1, 1.3.3.3
