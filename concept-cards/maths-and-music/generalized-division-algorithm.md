---
concept: Generalized Division Algorithm
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
unit: null
---

# Quick Definition
An extension of the Division Algorithm that allows the dividend to be any real number (not just an integer), guaranteeing a unique decomposition into an integer quotient and a non-negative real remainder.

# Formal Definition
Given $m \in \mathbb{Z}^+$ and $x \in \mathbb{R}$, there exist unique $q \in \mathbb{Z}$ and $r \in \mathbb{R}$ with $0 \leq r < m$ such that $x = qm + r$. The proof uses the Well-Ordering Principle (WOP.4): the set $S = \{\ell \in \mathbb{Z} \mid \ell m \leq x\}$ has a largest element $q$, giving $qm \leq x < (q+1)m$, so $r = x - qm$ satisfies $0 \leq r < m$.

# Mathematical Context
The generality over the standard Division Algorithm is that $x$ may be any real number, not just an integer. In fact, the divisor $m$ can also be any positive real number, not just a positive integer. This theorem guarantees that every equivalence class in $\mathbb{R}/{\sim}$ has exactly one representative in $[0, m)$, which is essential for the wrapping function and the parameterization of modular equivalence classes by the circle.

# Musical Context
The Generalized Division Algorithm justifies representing any interval by its unique modular equivalent in $[0, m)$. When $m = 12$ (semitones), any interval measured in semitones (even non-integer values representing microtonal intervals) has a unique octave-equivalent representative between 0 and 12. This underpins the geometric model of wrapping the real line of intervals around the chromatic circle.

# Examples
- $m = 8$, $x = 13.5$: $13.5 = 1 \cdot 8 + 5.5$, so $q = 1$, $r = 5.5$
- $m = 12$, $x = -7.3$: $-7.3 = (-1) \cdot 12 + 4.7$, so $q = -1$, $r = 4.7$
- $m = 12$, $x = 25$: $25 = 2 \cdot 12 + 1$, so $q = 2$, $r = 1$

# Related Concepts
- Division Algorithm
- Well-Ordering Principle
- Modular Equivalence on the Real Numbers
- Wrapping Real Line Around Circle

# Common Confusions
- Unlike the standard Division Algorithm, the remainder $r$ here can be any real number in $[0, m)$, not just a non-negative integer
- The theorem guarantees BOTH existence and uniqueness of $q$ and $r$
- The proof works even when $x$ is negative; the quotient $q$ will be negative in such cases

# Source Reference
Chapter 7, "Generalized Division Algorithm" section, p. 82 (PDF)
