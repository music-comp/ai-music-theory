---
concept: Identity Transformation
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
The identity transformation on a set S is the operation that maps every element to itself: 1(s) = s for all s.

# Formal Definition
The identity operation on a family S is that operation 1 on S which assigns the value 1(s) = s to any argument s. For any transformation f on S, the functional equations 1f = f and f1 = f are true.

# Mathematical Formulation
- 1: S -> S defined by 1(s) = s for all s in S
- Left identity property: 1f = f for any transformation f
- Right identity property: f1 = f for any transformation f
- The identity is unique (if it exists)
- 1 is both 1-to-1 and onto, hence an operation

# Musical Context/Application
The identity transformation represents "no change" - leaving musical objects as they are. In pitch-class space, T0 (transposition by 0) is the identity. The identity is essential for defining group structure: every group must contain an identity element. Recognizing the identity helps identify when a chain of transformations returns to the starting point.

# Examples
Musical examples:
- T0 on pitch classes: T0(C) = C, T0(D) = D, etc.
- The "do nothing" transformation
- Any transformation composed with identity yields itself: T5 composed with T0 = T5

Verification: T0T5(C) = T0(F) = F = T5(C), confirming T0T5 = T5.

In any GIS: int(s, s) = e (the identity interval), reflecting that the "interval from s to itself" is always the identity.

# Related Concepts
- Transformation
- Operation
- Group
- Semigroup
- Identity Element (Abstract)

# Common Confusions
- The identity transformation is not "nothing" - it is a specific, well-defined function
- In multiplicative notation, the identity is often written as 1 or e
- T0 and I0I0 both equal the identity, but they are computed differently
- There is exactly one identity transformation on any given set

# Source Reference
Chapter 1: Mathematical Preliminaries, Definitions 1.3.3.1-1.3.3.2
