---
concept: Monoid
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
An algebraic structure consisting of a set with an associative binary operation and an identity element, serving as the foundation upon which groups and rings are built.

# Formal Definition
A monoid is a set $M$ with a law of composition (binary operation) $\cdot : M \times M \to M$ satisfying:
1. **Associativity**: For any $x, y, z \in M$, $(x \cdot y) \cdot z = x \cdot (y \cdot z)$.
2. **Identity**: There exists an element $e \in M$ such that for all $x \in M$, $x \cdot e = e \cdot x = x$.

The identity element $e$ is unique. A monoid is always non-empty since it contains $e$. A monoid is called **commutative** if $x \cdot y = y \cdot x$ for all $x, y \in M$.

# Mathematical Context
The notation $(M, \cdot)$ specifies both the set and its operation. A monoid differs from a group in that elements need not have inverses. Key examples include $(\mathbb{R}, \cdot)$ (multiplication, identity 1), $(\mathbb{Z}, +)$ (addition, identity 0), $(\mathbb{Z}_m, +)$ (modular addition, identity $[0]$), and $(\mathcal{F}(S), \circ)$ (function composition on a set $S$, identity $\text{id}_S$). By convention, the symbol $+$ is used only for commutative operations.

# Musical Context
The set of musical intervals under composition forms a monoid (and in fact a group). The identity element is the unison interval. The monoid structure captures the fundamental idea that composing two intervals yields another interval, and that composition is associative: going up a third then a fourth then a fifth is the same regardless of how you group the operations.

# Examples
- $(\mathbb{R}, \cdot)$: monoid but not a group (0 has no multiplicative inverse)
- $(\mathbb{Z}, +)$: commutative monoid and group (identity is 0)
- $(\mathbb{Z}_m, +)$: commutative monoid and group, with $[k] + [\ell] = [k + \ell]$ well-defined
- $(\mathcal{F}(\mathbb{R}), \circ)$: non-commutative monoid; $f(x) = x^2$ and $g(x) = x + 1$ give $(f \circ g)(x) = (x+1)^2 \neq x^2 + 1 = (g \circ f)(x)$

# Related Concepts
- Group
- Commutative Group
- Ring
- Modular Integers

# Common Confusions
- A monoid is not necessarily a group; the key difference is that monoid elements need not have inverses
- Associativity allows dropping parentheses ($x \cdot y \cdot z$ is unambiguous), but commutativity is a separate property
- $(\mathbb{R}, \cdot)$ is a monoid but not a group because $0$ lacks a multiplicative inverse; removing $0$ yields the group $(\mathbb{R} \setminus \{0\}, \cdot)$

# Source Reference
Chapter 7, "Monoid" section, p. 82 (PDF)
