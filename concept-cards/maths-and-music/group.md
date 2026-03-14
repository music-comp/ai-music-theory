---
concept: Group
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
unit: null
---

# Quick Definition
A monoid in which every element has an inverse, providing the algebraic structure that models musical interval composition, octave equivalence, and modular arithmetic.

# Formal Definition
A group is a monoid $(G, \cdot)$ with the additional property: for every $x \in G$, there exists a unique element $x_{\text{inv}}$ (the inverse of $x$) such that $x \cdot x_{\text{inv}} = x_{\text{inv}} \cdot x = e$, where $e$ is the identity element. When the operation is written as $+$ (commutative case), the inverse of $x$ is denoted $-x$, and $x - y$ means $x + (-y)$.

# Mathematical Context
The uniqueness of inverses follows from the proof: if $x'_{\text{inv}}$ is another inverse of $x$, then $x_{\text{inv}} = x_{\text{inv}} \cdot e = x_{\text{inv}} \cdot (x \cdot x'_{\text{inv}}) = (x_{\text{inv}} \cdot x) \cdot x'_{\text{inv}} = e \cdot x'_{\text{inv}} = x'_{\text{inv}}$. Key examples: $(\mathbb{Z}, +)$ with inverse $-k$; $(\mathbb{Z}_m, +)$ with inverse $[-k]$; $(\mathbb{R}^+, \cdot)$ with inverse $x^{-1}$; $(\mathbb{R} \setminus \{0\}, \cdot)$ with inverse $1/x$.

# Musical Context
The set of musical intervals forms a group under composition: the identity is the unison interval, and the inverse of any interval is its opposite (e.g., up a fifth inverted is down a fifth). Under octave equivalence, modular chromatic intervals form the group $\mathbb{Z}_{12}$, where the inverse of [5] (a fourth) is [7] (a fifth), reflecting that a fourth plus a fifth equals an octave (unison modulo octave).

# Examples
- $(\mathbb{Z}, +)$: inverse of $k$ is $-k$
- $(\mathbb{Z}_m, +)$: inverse of $[k]$ is $[-k] = [m - k]$
- $(\mathbb{R}^+, \cdot)$: inverse of $x$ is $1/x$
- $(\mathbb{R}, \cdot)$ is NOT a group: $0$ has no multiplicative inverse
- $\{1, -1\}$ under multiplication is a group isomorphic to $\mathbb{Z}_2$

# Related Concepts
- Monoid
- Commutative Group
- Cyclic Group and Generator
- Group of Intervals
- Modular Chromatic Intervals
- Homomorphism

# Common Confusions
- Not every monoid is a group; the crucial addition is the existence of inverses for every element
- A commutative group is also called an abelian group; all groups in this text are commutative
- The group operation need not be multiplication or addition in the usual sense; it is any associative operation with identity and inverses

# Source Reference
Chapter 7, "Group" section, p. 82 (PDF)
