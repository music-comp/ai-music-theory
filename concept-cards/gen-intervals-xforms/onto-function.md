---
concept: Onto Function
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
An onto function (surjection) is a function where every member of the codomain S' is the value of at least one argument from the domain S.

# Formal Definition
The function f from S into S' is onto S' if every member of S' is the value of some argument. Equivalently, every member of S' appears at least once in the right-hand column of the function table.

# Mathematical Formulation
- f: S -> S' is onto (surjective) if:
  - For every s' in S', there exists at least one s in S such that f(s) = s'
  - The image of f equals the codomain: f(S) = S'
- When f is onto, we write f: S -> S' is "onto S'" or "f maps S onto S'"

# Musical Context/Application
In musical transformations, onto functions ensure that every element in the target space is reachable. For transposition on pitch classes, every pitch class can be reached from some other pitch class, making it onto. When analyzing transformational networks, onto-ness ensures complete coverage of the musical space.

# Examples
Musical example: Transposition T5 on the 12 pitch classes is onto. Every pitch class y can be written as T5(x) for some pitch class x (specifically, x = y - 5 mod 12).

Musical example: The function mapping pitches to their pitch classes is onto the 12 pitch classes: every pitch class is represented by some pitch.

Non-example: f(x) = x^2 on positive integers is not onto the positive integers, because there is no positive integer x such that f(x) = 3.

# Related Concepts
- Function
- One-to-One Function
- Operation
- Inverse Function
- Homomorphism

# Common Confusions
- "Onto" concerns whether all targets are hit, while "1-to-1" concerns whether targets are hit uniquely
- A function being onto S' depends on what S' is defined to be
- The phrase "into S'" means S' is the codomain; "onto S'" means f is surjective

# Source Reference
Chapter 1: Mathematical Preliminaries, Definition 1.2.6.1
