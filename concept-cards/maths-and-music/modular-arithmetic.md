---
concept: Modular Arithmetic
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
The arithmetic of equivalence classes in $\mathbb{Z}_m$, where operations are performed on representatives and the result is reduced modulo $m$, providing the computational framework for interval arithmetic under octave equivalence.

# Formal Definition
The group $\mathbb{Z}_m$ is called a modular group, and operations involving its law of composition are called modular arithmetic. Addition in $\mathbb{Z}_m$ is defined by $[k] + [\ell] = [k + \ell]$, and multiplication by $[k] \cdot [\ell] = [k\ell]$. Both are well-defined (independent of representative choice). Modular arithmetic can be visualized as rotations on the m-hour clock.

# Mathematical Context
Modular arithmetic combines the group structure $(\mathbb{Z}_m, +)$ with the ring structure $(\mathbb{Z}_m, +, \cdot)$. The addition on the modular clock is computed by rotating clockwise by $k$ positions then $\ell$ positions, and the result is where the top position lands. The Generalized Division Algorithm ensures every result has a unique representative in $\{0, 1, \ldots, m-1\}$.

# Musical Context
Modular arithmetic is the algebra of intervals under octave equivalence. Every chromatic interval computation reduces to modular arithmetic in $\mathbb{Z}_{12}$. Creating twelve-tone row charts uses modular arithmetic extensively: the entry at position $(i,j)$ is $a_j - a_i$ in $\mathbb{Z}_{12}$. Non-standard n-tone row charts use $\mathbb{Z}_n$ arithmetic.

# Examples
- $[6] + [13] = [19] = [1]$ in $\mathbb{Z}_9$ (since $19 = 2 \cdot 9 + 1$)
- $[7] + [7] = [14] = [2]$ in $\mathbb{Z}_{12}$ (two fifths = whole step)
- $[5] + [7] = [12] = [0]$ in $\mathbb{Z}_{12}$ (fourth + fifth = unison)
- $[4] - [10] = [-6] = [6]$ in $\mathbb{Z}_{12}$ (used to compute row chart entries)

# Related Concepts
- Modular Integers
- Modular Chromatic Intervals
- Ring
- Row Chart
- Modular Clock

# Common Confusions
- Modular arithmetic is not "rounding" or "approximation"; it is exact arithmetic on equivalence classes
- The result is always an equivalence class, not a specific integer; writing $[6] + [13] = [1]$ means the equivalence class $[1]$, which contains $\ldots, -8, 1, 10, 19, \ldots$
- Subtraction in $\mathbb{Z}_m$ is well-defined: $[k] - [\ell] = [k] + [-\ell] = [k - \ell]$

# Source Reference
Chapter 7, "Modular Arithmetic" section, p. 82 (PDF)
