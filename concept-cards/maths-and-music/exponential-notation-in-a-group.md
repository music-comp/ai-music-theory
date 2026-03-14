---
concept: Exponential Notation in a Group
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
The convention of writing $x^n$ for the $n$-fold composition of a group element with itself, extending to negative exponents via inverses, with familiar rules of exponents holding in any group.

# Formal Definition
Let $(G, \cdot)$ be a group, $x \in G$, $n \in \mathbb{Z}$. Define: $x^n = x \cdot x \cdots x$ ($n$ times) for $n > 0$; $x^0 = e$; $x^{-n} = x^{-1} \cdot x^{-1} \cdots x^{-1}$ ($n$ times) for $n > 0$. The rules of exponents hold: $x^{n+m} = x^n \cdot x^m$ and $(x^n)^m = x^{nm}$. In additive notation (commutative groups), $nx$ replaces $x^n$, and the rules become $(n+m)x = nx + mx$ and $m(nx) = (nm)x$.

# Mathematical Context
These rules are crucial for the theory of cyclic groups. If $t$ generates a cyclic group of order $m$, then $t^n = t^r$ where $r$ is the remainder of $n$ divided by $m$ (by the Division Algorithm). The rules of exponents allow us to determine when $t^n$ is also a generator: precisely when $\gcd(n, m) = 1$.

# Musical Context
In the group of modular chromatic intervals $\mathbb{Z}_{12}$, the additive notation $n \cdot [k]$ represents iterating the interval $[k]$ a total of $n$ times. For example, $3 \cdot [7] = [21] = [9]$ means three fifths compose to a major sixth (modulo octave). The rules of exponents ensure that iterating intervals is consistent with group operations.

# Examples
- In $\mathbb{Z}_{12}$: $7 \cdot [5] = [35] = [11]$ (seven fourths = major seventh mod octave)
- In a cyclic group of order 8 with generator $t$: $t^3$ is also a generator since $\gcd(3,8) = 1$
- Additive rule: $(n+m) \cdot [k] = n \cdot [k] + m \cdot [k]$
- $(x^n)^m = x^{nm}$: iterating $m$ times an interval that is $n$ iterations of $x$ equals $nm$ iterations of $x$

# Related Concepts
- Group
- Cyclic Group and Generator
- Order of an Element
- Modular Arithmetic

# Common Confusions
- In a commutative group with additive notation, $nx$ means $x + x + \cdots + x$ ($n$ times), not $n \cdot x$ in the usual sense of multiplication; the "multiplication" $n \cdot x$ mixes the integer $n$ with the group element $x$
- The rules $x^{n+m} = x^n \cdot x^m$ hold in any group, but $x^n \cdot y^n = (xy)^n$ requires commutativity
- $x^0 = e$ by definition, even when $x$ is not the identity

# Source Reference
Chapter 7, "Exponential Notation in a Group" section, p. 82 (PDF)
