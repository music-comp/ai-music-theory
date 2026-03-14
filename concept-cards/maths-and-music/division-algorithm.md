---
concept: Division Algorithm
category: theory
source: "Mathematics and Music"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
authors: "David Wright"
unit: null
---

# Quick Definition
A fundamental theorem of arithmetic stating that any integer can be divided by a positive integer to produce a unique quotient and remainder.

# Formal Definition
Given $m \in \mathbb{Z}^+$ and $n \in \mathbb{Z}$, there exist $q, r \in \mathbb{Z}$ with $0 \leq r < m$ such that $n = qm + r$.

# Mathematical Context
The Division Algorithm is not actually an algorithm but an existence theorem. The integer $q$ is the quotient and $r$ is the remainder. The remainder satisfies $0 \leq r < m$, ensuring uniqueness. This principle underlies modular arithmetic, which becomes central to the mathematical treatment of octave equivalence and interval arithmetic. The proof relies on the Well-Ordering Principle.

# Musical Context
The Division Algorithm is the mathematical foundation for modular arithmetic in music. When counting semitones modulo 12 (for octave equivalence), dividing a semitone count $n$ by $m = 12$ yields a remainder $r$ that identifies the note class. For example, 17 semitones above C gives $17 = 1 \cdot 12 + 5$, so the note class is 5 semitones above C, which is F.

# Examples
- $m = 9, n = 123$: $123 = 13 \cdot 9 + 6$, so $q = 13, r = 6$
- $m = 12, n = -37$: $-37 = (-4) \cdot 12 + 11$, so $q = -4, r = 11$
- Exercise 1(d): $m = 7, n = 14k + 23$ where $k$ is an integer

# Related Concepts
- Well-Ordering Principle
- Sets and Number Systems
- Octave Equivalence
- Note Classes

# Common Confusions
- The Division Algorithm works for negative $n$ as well; the remainder $r$ is always non-negative ($0 \leq r < m$), even when $n$ is negative
- Despite the name, it is a theorem (existence statement), not a computational procedure

# Source Reference
Chapter 1, "Some Properties of Integers" section, p. 14 (PDF)
