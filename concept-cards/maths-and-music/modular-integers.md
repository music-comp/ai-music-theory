---
concept: Modular Integers
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
unit: null
---

# Quick Definition
The elements of $\mathbb{Z}_m$, the set of equivalence classes of integers modulo $m$, which form a group under addition and a ring under addition and multiplication.

# Formal Definition
For $m \in \mathbb{Z}^+$, the modular integers $\mathbb{Z}_m = \{[0], [1], [2], \ldots, [m-1]\}$ form a group under addition defined by $[k] + [\ell] = [k + \ell]$, and a ring with multiplication $[k] \cdot [\ell] = [k\ell]$. Both operations are well-defined (independent of choice of representatives). The additive identity is $[0]$, the multiplicative identity is $[1]$, and the additive inverse of $[k]$ is $[-k] = [m - k]$.

# Mathematical Context
Well-definedness of addition: if $[k'] = [k]$ and $[\ell'] = [\ell]$, then $k' = k + pm$ and $\ell' = \ell + qm$, so $k' + \ell' = k + \ell + (p+q)m$, giving $[k' + \ell'] = [k + \ell]$. The group $(\mathbb{Z}_m, +)$ is cyclic with generator $[1]$ of order $m$. The ring $\mathbb{Z}_m$ is an integral domain precisely when $m$ is prime.

# Musical Context
$\mathbb{Z}_{12}$ is the group of modular chromatic intervals in the standard 12-chromatic scale. Each element represents a note class or interval class: $[0]$ = unison/C, $[1]$ = semitone/C$\sharp$, ..., $[11]$ = major seventh/B. Modular integer arithmetic captures interval composition under octave equivalence. For non-standard scales, $\mathbb{Z}_n$ plays the same role.

# Examples
- In $\mathbb{Z}_{12}$: $[7] + [5] = [12] = [0]$ (fifth + fourth = unison modulo octave)
- In $\mathbb{Z}_{12}$: $[7] + [7] = [14] = [2]$ (two fifths = whole step modulo octave)
- In $\mathbb{Z}_9$: $[6] + [13] = [19] = [1]$ (since $19 \equiv 1 \pmod{9}$)
- The additive inverse of $[5]$ in $\mathbb{Z}_{12}$ is $[7]$ (since $5 + 7 = 12 \equiv 0$)

# Related Concepts
- Modular Equivalence on the Integers
- Modular Arithmetic
- Cyclic Group and Generator
- Ring
- Modular Chromatic Intervals

# Common Confusions
- $\mathbb{Z}_m$ is a finite set with exactly $m$ elements, not an infinite set of integers
- The bracket notation $[k]$ denotes an equivalence class containing infinitely many integers; $[5]$ in $\mathbb{Z}_{12}$ contains $\ldots, -19, -7, 5, 17, 29, \ldots$
- $\mathbb{Z}_m$ is always a ring, but it is an integral domain only when $m$ is prime

# Source Reference
Chapter 7, "Modular Equivalence on the Integers" section, p. 82 (PDF); ring structure in Chapter 8, p. 100 (PDF)
