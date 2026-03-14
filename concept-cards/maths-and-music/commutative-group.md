---
concept: Commutative Group
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
A group in which the order of combining elements does not matter, also called an abelian group. All musical interval groups encountered in this text are commutative.

# Formal Definition
A group $(G, \cdot)$ is commutative (or abelian) if for all $x, y \in G$, $x \cdot y = y \cdot x$. By convention, the operation symbol $+$ is reserved for commutative group operations. In additive notation, the inverse of $x$ is written $-x$ and the expression $x + (-y)$ is abbreviated $x - y$.

# Mathematical Context
All the principal groups in the text are commutative: $(\mathbb{Z}, +)$, $(\mathbb{R}, +)$, $(\mathbb{R}^+, \cdot)$, $(\mathbb{Z}_m, +)$, and the group of intervals. The monoid of functions $(\mathcal{F}(\mathbb{R}), \circ)$ provides a counterexample: $f(x) = x^2$ and $g(x) = x + 1$ give $f \circ g \neq g \circ f$.

# Musical Context
Commutativity of interval composition means that going up a third and then a fourth gives the same result as going up a fourth and then a third. This property is essential for the identification of modular chromatic intervals with $\mathbb{Z}_{12}$, where the order of interval composition does not affect the result.

# Examples
- In $\mathbb{Z}_{12}$: $[3] + [5] = [8] = [5] + [3]$ (minor third + fourth = fourth + minor third)
- $(\mathbb{R}, +)$ is commutative: $a + b = b + a$
- $(\mathbb{R}^+, \cdot)$ is commutative: $xy = yx$
- $(\mathcal{F}(\mathbb{R}), \circ)$ is NOT commutative

# Related Concepts
- Group
- Monoid
- Modular Arithmetic
- Group of Intervals

# Common Confusions
- "Commutative group" and "abelian group" mean the same thing; the latter is named after Niels Henrik Abel
- The symbol $+$ should only be used for commutative operations by convention; using $\cdot$ does not imply non-commutativity
- All cyclic groups are commutative, but not all commutative groups are cyclic

# Source Reference
Chapter 7, "Commutativity" section, p. 82 (PDF)
