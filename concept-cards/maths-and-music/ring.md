---
concept: Ring
category: theory
source: "Mathematics and Music"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
authors: "David Wright"
unit: null
---

# Quick Definition
An algebraic structure with two operations (addition and multiplication) where addition forms a commutative group, multiplication forms a monoid, and multiplication distributes over addition.

# Formal Definition
A ring is a non-empty set $R$ endowed with two associative operations $+$ and $\cdot$ such that: (1) $(R, +)$ is a commutative group with identity $0$; (2) $(R, \cdot)$ is a monoid with identity $1$; (3) distributivity holds: $a \cdot (b + c) = a \cdot b + a \cdot c$ and $(b + c) \cdot a = b \cdot a + c \cdot a$ for all $a, b, c \in R$. A ring is **commutative** if $a \cdot b = b \cdot a$ for all $a, b \in R$.

# Mathematical Context
From the ring axioms one can derive $(-1) \cdot x = -x$ and $0 \cdot x = 0$ for any $x \in R$. The monoid $(R, \cdot)$ is generally not a group since $0$ has no multiplicative inverse. The only ring where $(R, \cdot)$ is a group is the trivial ring $R = \{0\}$ (where $0 = 1$). The text deals exclusively with commutative rings.

# Musical Context
The ring $\mathbb{Z}$ provides the algebraic framework for studying integer intervals (those with positive integer frequency ratios). The ring $\mathbb{Z}_m$ captures both the additive structure of modular interval composition and the multiplicative structure needed for the theory of generators and units. Ring theory connects the group structure of intervals to number-theoretic properties of the integers.

# Examples
- $\mathbb{Z}$ with usual addition and multiplication: commutative ring, integral domain, units are $\{1, -1\}$
- $\mathbb{R}$ with usual addition and multiplication: commutative ring, integral domain, units are $\mathbb{R} \setminus \{0\}$
- $\mathbb{Q}$ with usual addition and multiplication: commutative ring, integral domain, units are $\mathbb{Q} \setminus \{0\}$
- $\mathbb{Z}_m$ with $[k] + [\ell] = [k + \ell]$ and $[k] \cdot [\ell] = [k\ell]$: commutative ring

# Related Concepts
- Group
- Monoid
- Units in a Ring
- Integral Domain
- Ideals and Principal Ideals
- Modular Integers

# Common Confusions
- A ring has TWO operations; a group has one. The additive structure is always a commutative group, but the multiplicative structure is only a monoid
- "Commutative ring" means multiplication is commutative; the addition is always commutative by definition
- Not every element has a multiplicative inverse; those that do are called units

# Source Reference
Chapter 8, "Ring" section, p. 100 (PDF)
