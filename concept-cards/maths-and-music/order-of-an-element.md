---
concept: Order of an Element
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
unit: null
---

# Quick Definition
The smallest positive integer $m$ such that an element raised to the $m$th power equals the identity, determining how many distinct powers the element produces before cycling.

# Formal Definition
Let $G$ be a group and $t \in G$. Consider the set $S = \{n \in \mathbb{Z}^+ \mid t^n = e\}$. If $S \neq \emptyset$, the smallest element $m \in S$ (existing by the Well-Ordering Principle) is called the order of $t$. In this case, $t$ generates a cyclic subgroup $\{e, t, t^2, \ldots, t^{m-1}\}$ with exactly $m$ distinct elements. If $S = \emptyset$, the element $t$ has infinite order.

# Mathematical Context
The order of $t$ divides the order of the group (Lagrange's theorem). In $\mathbb{Z}_n$, the order of $[k]$ is $n / \gcd(k, n)$. An element $[k] \in \mathbb{Z}_n$ is a generator if and only if its order equals $n$, which happens precisely when $\gcd(k, n) = 1$. The Division Algorithm proves that $t^n = t^r$ where $r$ is the remainder of $n$ divided by $m$.

# Musical Context
The order of a modular chromatic interval $[k]$ in $\mathbb{Z}_n$ tells how many iterations of that interval are needed before returning to the starting note class. For example, in $\mathbb{Z}_{12}$, the minor third $[3]$ has order 4 (since $4 \cdot 3 = 12 \equiv 0$), meaning iterating minor thirds cycles through exactly 4 note classes before repeating.

# Examples
- In $\mathbb{Z}_{12}$: order of $[1]$ is 12 (generator); order of $[3]$ is 4; order of $[4]$ is 3; order of $[6]$ is 2
- In $\mathbb{Z}_8$: $[1]$ has order 8 (generator); $[2]$ has order 4; $[4]$ has order 2
- The order of $[k]$ in $\mathbb{Z}_n$ is $n / \gcd(k, n)$
- In $\mathbb{Z}$: the element $1$ has infinite order (no positive multiple of 1 equals 0)

# Related Concepts
- Cyclic Group and Generator
- GCD Condition for Generators
- Generating Interval
- Group

# Common Confusions
- The "order of an element" is different from the "order of a group" (which is the number of elements in the group)
- An element of order $m$ generates a subgroup with exactly $m$ elements, not $m - 1$
- The order divides the group order; in $\mathbb{Z}_{12}$, element orders can only be 1, 2, 3, 4, 6, or 12

# Source Reference
Chapter 7, "Generators and Cyclic Groups" section, p. 82 (PDF)
