---
concept: Units in a Ring
category: theory
source: "Mathematics and Music"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
authors: "David Wright"
---

# Quick Definition
Elements of a ring that have multiplicative inverses, forming a group under multiplication. In $\mathbb{Z}_m$, the units are precisely the generators of the additive group.

# Formal Definition
An element $x$ in a ring $R$ is called a unit if it has a multiplicative inverse, denoted $x^{-1}$, such that $x \cdot x^{-1} = x^{-1} \cdot x = 1$. The set of units, denoted $R^*$, forms a group under multiplication. For $\mathbb{Z}_m$, the units are exactly those $[n]$ with $\gcd(m, n) = 1$, and $|\mathbb{Z}_m^*| = \phi(m)$.

# Mathematical Context
In general, $(R, \cdot)$ is only a monoid, not a group, because $0$ has no multiplicative inverse (except in the trivial ring $R = \{0\}$). But the subset $R^*$ of units does form a group. Key examples: $\mathbb{Z}^* = \{1, -1\}$; $\mathbb{R}^* = \mathbb{R} \setminus \{0\}$; $\mathbb{Q}^* = \mathbb{Q} \setminus \{0\}$.

# Musical Context
The units of $\mathbb{Z}_{12}$ are $\{[1], [5], [7], [11]\}$, which are precisely the generating intervals of the 12-chromatic scale. The equivalence between being a unit ($[n] \in \mathbb{Z}_m^*$), being a generator of $(\mathbb{Z}_m, +)$, and satisfying $\gcd(m, n) = 1$ ties together ring theory, group theory, and the structure of chromatic scales.

# Examples
- $\mathbb{Z}^* = \{1, -1\}$: only $\pm 1$ have integer multiplicative inverses
- $\mathbb{R}^* = \mathbb{R} \setminus \{0\}$: every non-zero real has a multiplicative inverse
- $\mathbb{Z}_{12}^* = \{[1], [5], [7], [11]\}$: these are the four elements with $\gcd(n, 12) = 1$
- In $\mathbb{Z}_7$: all non-zero elements are units ($\mathbb{Z}_7^* = \{[1], [2], [3], [4], [5], [6]\}$) since 7 is prime

# Related Concepts
- Ring
- GCD Condition for Generators
- Euler Phi Function
- Integral Domain
- Generating Interval

# Common Confusions
- Not every non-zero element of a ring is a unit; in $\mathbb{Z}$, the only units are $\pm 1$
- $0$ is never a unit (except in the trivial ring where $0 = 1$)
- In $\mathbb{Z}_m$, being a unit (multiplicative property) is equivalent to being a group generator (additive property); this is a non-obvious algebraic fact

# Source Reference
Chapter 8, "Units" section, p. 100 (PDF)
