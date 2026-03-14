---
concept: Left Orthography
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Mathematical Preliminaries"
chapter_number: 1
pdf_page: 32
unit: null
authors: David Lewin
---

# Quick Definition
Left orthography is the notational convention of writing function names to the left of arguments, as in f(s), where the composition f'f means "apply f first, then f'."

# Formal Definition
In left orthography, one writes "f(s)" for "the resulting value when function f is applied to argument s." The composition function f'f means "apply f first, then apply f' to the result," so that (f'f)(s) = f'(f(s)). The function written leftmost in a composition is applied last.

# Mathematical Formulation
- f(s) denotes: apply f to s
- (f'f)(s) = f'(f(s)): in the composition f'f, f is applied first
- For three functions: (f''f'f)(s) = f''(f'(f(s)))
- Reading order: functions apply from right to left in a composition

# Musical Context/Application
Left orthography is the standard notation in Lewin's work and most music theory literature. When analyzing a chain of transformations, one reads from right to left: T3I means "invert, then transpose by 3" (apply I first, then T3). This convention is familiar from standard mathematical notation.

# Examples
From Chapter 1: "IT2 = J" means: Given any pitch class s, invert about C the 2-transpose of s, obtaining the inversion about B of the given s.

Breakdown: IT2(s) = I(T2(s)). First transpose s by 2, then invert about C.

Musical calculation: IT2(C) = I(D) = Bb (where I inverts about C).
Directly: J(C) = Bb (where J inverts about B).
The equation IT2 = J holds.

# Related Concepts
- Right Orthography
- Composition of Functions
- Function
- Transformation

# Common Confusions
- In f'f, despite f' appearing first (leftmost), f is applied first
- This can seem counterintuitive; the notation reflects "f' of f of s"
- Confusing left and right orthography leads to reversed composition orders
- Lewin uses left orthography almost exclusively in the book

# Source Reference
Chapter 1: Mathematical Preliminaries, Section 1.2.4
