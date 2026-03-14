---
concept: Euler Phi Function
category: theory
source: "Mathematics and Music"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
authors: "David Wright"
unit: null
---

# Quick Definition
A number-theoretic function that counts how many positive integers less than $n$ are relatively prime to $n$, directly giving the number of generating intervals in the n-chromatic scale.

# Formal Definition
The Euler phi function $\phi: \mathbb{Z}^+ \to \mathbb{Z}^+$ is defined by $\phi(n) = |\{m \in \mathbb{Z}^+ \mid 1 \leq m < n \text{ and } \gcd(m, n) = 1\}|$. Equivalently, $\phi(n)$ counts the number of generators of the cyclic group $\mathbb{Z}_n$, the number of units in the ring $\mathbb{Z}_n$ (i.e., $|\mathbb{Z}_n^*|$), and the number of generating intervals in the n-chromatic scale.

# Mathematical Context
The three equivalent characterizations of $\phi(m)$ follow from the theorem that for $[n] \in \mathbb{Z}_m$, the following are equivalent: (1) $\gcd(m, n) = 1$; (2) $[n]$ is a generator of $(\mathbb{Z}_m, +)$; (3) $[n]$ is a unit in the ring $\mathbb{Z}_m$. Thus $\phi(m)$ simultaneously counts generators of the group, units of the ring, and integers relatively prime to $m$.

# Musical Context
$\phi(n)$ tells a musician how many fundamentally different interval cycles exist in an n-chromatic scale that visit every note class. For the standard 12-chromatic scale, $\phi(12) = 4$, corresponding to the four generating intervals: semitone, fourth, fifth, and major seventh.

# Examples
- $\phi(12) = 4$: the integers $1, 5, 7, 11$ are relatively prime to 12
- $\phi(14) = 6$: the integers $1, 3, 5, 9, 11, 13$ are relatively prime to 14
- $\phi(7) = 6$: every integer from 1 to 6 is relatively prime to 7 (since 7 is prime)
- $\phi(p) = p - 1$ for any prime $p$

# Related Concepts
- Greatest Common Divisor
- Relatively Prime Integers
- Generating Interval
- Cyclic Group and Generator
- Units in a Ring
- Prime Numbers

# Common Confusions
- $\phi(1) = 1$ by convention (the empty product); the only positive integer $\leq 1$ satisfying $\gcd(m, 1) = 1$ is vacuously counted
- $\phi(n)$ counts integers strictly less than $n$ that are relatively prime to $n$; it does not count $n$ itself
- A larger $n$ does not necessarily mean a larger $\phi(n)$; for example, $\phi(12) = 4$ but $\phi(11) = 10$

# Source Reference
Chapter 8, "Euler Phi Function" section, p. 100 (PDF); first introduced in Chapter 6, p. 74 (PDF)
