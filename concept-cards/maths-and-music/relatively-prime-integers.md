---
concept: Relatively Prime Integers
category: theory
source: "Mathematics and Music"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
authors: "David Wright"
unit: null
---

# Quick Definition
Two integers whose greatest common divisor is 1, meaning they share no common factor other than $\pm 1$. This condition determines generating intervals and the effectiveness of m-on-n musical patterns.

# Formal Definition
Integers $m$ and $n$ are relatively prime (or coprime) if $\gcd(m, n) = 1$, equivalently, if the only positive integer dividing both $m$ and $n$ is 1. This means $m\mathbb{Z} + n\mathbb{Z} = \mathbb{Z}$, so there exist integers $h, k$ with $hm + kn = 1$ (Bezout's identity).

# Mathematical Context
Relative primality is the key condition in three equivalent characterizations: $[n]$ generates $\mathbb{Z}_m$ iff $[n]$ is a unit in $\mathbb{Z}_m$ iff $\gcd(m, n) = 1$. If $p$ is prime and $n \in \mathbb{Z}$, then either $p \mid n$ or $\gcd(p, n) = 1$. The number of integers in $\{1, 2, \ldots, m-1\}$ relatively prime to $m$ is $\phi(m)$.

# Musical Context
Relative primality determines: (1) which chromatic intervals generate all note classes (those $[m]$ with $\gcd(m, 12) = 1$ in the 12-chromatic scale); (2) which m-on-n polyrhythmic patterns run for exactly $mn$ units before repeating (those with $\gcd(m, n) = 1$); (3) the "completeness" of interval circles.

# Examples
- $\gcd(5, 12) = 1$: 5 and 12 are relatively prime; the fourth generates all 12 note classes
- $\gcd(3, 4) = 1$: the 3-on-4 pattern in "In the Mood" takes $12$ notes to complete
- $\gcd(3, 5) = 1$: the 3-on-5 pattern in "Rhapsody in Blue" takes $15$ notes
- $\gcd(4, 12) = 4 \neq 1$: 4 and 12 are NOT relatively prime; iterating major thirds cycles through only 3 note classes

# Related Concepts
- Greatest Common Divisor
- GCD Condition for Generators
- Euler Phi Function
- Prime Numbers
- Generating Interval
- M on N Polyrhythmic Patterns

# Common Confusions
- "Relatively prime" does not mean either number is prime; $\gcd(8, 15) = 1$ even though neither 8 nor 15 is prime
- If $p$ is prime, then $p$ is relatively prime to every integer it does not divide
- Two consecutive integers are always relatively prime: $\gcd(n, n+1) = 1$

# Source Reference
Chapter 8, "Greatest Common Divisor" section, p. 100 (PDF); introduced in Chapter 6, p. 74 (PDF)
