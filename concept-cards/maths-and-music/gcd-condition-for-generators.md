---
concept: GCD Condition for Generators
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
The criterion that an element $[n]$ generates the cyclic group $\mathbb{Z}_m$ if and only if $\gcd(m, n) = 1$, connecting number theory to the structure of chromatic scales.

# Formal Definition
Given $[n] \in \mathbb{Z}_m$, the following three conditions are equivalent:
1. $\gcd(m, n) = 1$ (i.e., $m$ and $n$ are relatively prime)
2. $[n]$ is a generator of the additive group $(\mathbb{Z}_m, +)$
3. $[n]$ is a unit in the ring $\mathbb{Z}_m$ (i.e., $[n] \in \mathbb{Z}_m^*$)

# Mathematical Context
The equivalence of (2) and (3): If $[n]$ generates $(\mathbb{Z}_m, +)$, then $[1] = k \cdot [n] = [k] \cdot [n]$ for some $k$, making $[n]$ a unit. Conversely, if $[n]$ is a unit with inverse $[k]$, then $[\ell] = [\ell] \cdot [1] = [\ell] \cdot [k] \cdot [n] = \ell k \cdot [n]$, so $[n]$ generates $\mathbb{Z}_m$. The equivalence with (1) uses properties of greatest common divisors from Chapter 8.

# Musical Context
This theorem is the precise mathematical reason why certain intervals generate all note classes in a chromatic scale. In the 12-chromatic scale, only the semitone ($[1]$), fourth ($[5]$), fifth ($[7]$), and major seventh ($[11]$) generate all 12 note classes, because only $1, 5, 7, 11$ are relatively prime to 12.

# Examples
- $\gcd(7, 12) = 1$: the fifth generates all 12 note classes (circle of fifths)
- $\gcd(4, 12) = 4 \neq 1$: the major third does NOT generate $\mathbb{Z}_{12}$; iterating it gives only $\{[0], [4], [8]\}$ (augmented triad)
- $\gcd(3, 7) = 1$: in the 7-chromatic scale, $[3]$ generates all 7 intervals
- The number of generators of $\mathbb{Z}_m$ is $\phi(m)$

# Related Concepts
- Cyclic Group and Generator
- Greatest Common Divisor
- Relatively Prime Integers
- Euler Phi Function
- Units in a Ring
- Generating Interval

# Common Confusions
- The condition is $\gcd(m, n) = 1$, not $\gcd(m, n) = m$ or $\gcd(m, n) = n$
- Being a generator of $(\mathbb{Z}_m, +)$ and being a unit of the ring $\mathbb{Z}_m$ are equivalent conditions, despite being algebraically different statements
- The condition applies to any $n \in \mathbb{Z}$, not just $0 \leq n < m$; for example, $[13]$ generates $\mathbb{Z}_{12}$ because $\gcd(13, 12) = 1$

# Source Reference
Chapter 7, "Generators and Cyclic Groups" section, p. 82 (PDF); proof completed in Chapter 8, p. 100 (PDF)
