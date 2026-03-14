---
concept: Cyclic Group and Generator
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
unit: null
---

# Quick Definition
A group in which every element can be expressed as a power (or multiple) of a single element called a generator. The group $\mathbb{Z}_m$ is the prototypical finite cyclic group.

# Formal Definition
Given a group $G$ and an element $t \in G$, $t$ is a generator for $G$ if $\{t^n \mid n \in \mathbb{Z}\} = G$. A group is cyclic if it has a generator. If the set $S = \{n \in \mathbb{Z}^+ \mid t^n = e\}$ is empty, then $G \cong \mathbb{Z}$. If $S \neq \emptyset$, its smallest element $m$ (the order of $t$) gives $G = \{e, t, t^2, \ldots, t^{m-1}\}$, a group with exactly $m$ elements.

# Mathematical Context
The Division Algorithm proves uniqueness: writing $n = qm + r$, we get $t^n = t^r$ with $0 \leq r < m$. A cyclic group of order $m$ has multiple generators. If $t$ has order $m$, then $t^n$ is also a generator if and only if $\gcd(n, m) = 1$. Hence there are $\phi(m)$ generators. All cyclic groups are commutative. Every cyclic group of order $m$ is isomorphic to $\mathbb{Z}_m$.

# Musical Context
The group $\mathbb{Z}_{12}$ of modular chromatic intervals is cyclic: $[1]$ (the semitone) generates it since successive semitones produce all 12 note classes. The generators of $\mathbb{Z}_{12}$ are precisely the generating intervals of the 12-chromatic scale: $[1]$, $[5]$, $[7]$, $[11]$.

# Examples
- $\mathbb{Z}_m$ is cyclic with generator $[1]$ of order $m$ (since $m$ is the smallest $n$ with $n \cdot [1] = [0]$)
- In a cyclic group of order 8 with generator $t$: $u = t^3$ is also a generator since $\gcd(3, 8) = 1$; the powers of $u$ give $u^2 = t^6$, $u^3 = t$, $u^4 = t^4$, etc.
- $(\mathbb{Z}, +)$ is an infinite cyclic group with generators $1$ and $-1$
- $(\mathbb{R}, +)$ is NOT cyclic

# Related Concepts
- Group
- Order of an Element
- Generating Interval
- Euler Phi Function
- GCD Condition for Generators
- Modular Integers

# Common Confusions
- A cyclic group usually has more than one generator; $\mathbb{Z}_{12}$ has four generators ($[1], [5], [7], [11]$)
- "Cyclic" does not mean the group operation is cyclic in some informal sense; it means every element is a power of one element
- An infinite cyclic group has exactly 2 generators ($t$ and $t^{-1}$), while a finite cyclic group of order $m$ has $\phi(m)$ generators

# Source Reference
Chapter 7, "Generators and Cyclic Groups" section, p. 82 (PDF)
