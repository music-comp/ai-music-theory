---
concept: Unique Prime Factorization
category: theory
source: "Mathematics and Music"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
authors: "David Wright"
unit: null
---

# Quick Definition
The fundamental theorem that every positive integer can be expressed as a product of prime numbers in exactly one way (up to ordering), guaranteeing the unique decomposition of integer intervals into prime intervals.

# Formal Definition
**Theorem (Unique Factorization):** Let $n \geq 1$ be an integer. Then $n$ can be factored as $n = p_1^{\alpha_1} p_2^{\alpha_2} \cdots p_r^{\alpha_r}$ where $r \geq 0$, $p_1, p_2, \ldots, p_r$ are distinct primes, and $\alpha_1, \alpha_2, \ldots, \alpha_r \geq 1$. This factorization is unique up to reordering of the primes.

# Mathematical Context
The proof has two parts. **Existence:** By contradiction using the Well-Ordering Principle: if some $n$ has no prime factorization, take the smallest such $n$. Since $n$ is not prime, $n = m\ell$ with $1 < m, \ell < n$. Both $m$ and $\ell$ have factorizations by minimality, so $n$ does too -- contradiction. **Uniqueness:** If $p_1^{\alpha_1} \cdots p_r^{\alpha_r} = q_1^{\beta_1} \cdots q_t^{\beta_t}$, then $p_1 \mid q_1^{\beta_1} \cdots q_t^{\beta_t}$, so $p_1 \mid q_i$ for some $i$ (since $p_1$ is prime), giving $p_1 = q_i$. Cancel and repeat.

# Musical Context
Unique factorization means every integer interval (one with a positive integer frequency ratio) decomposes uniquely into prime intervals. Since rational intervals have ratios $m/n$ with $m, n \in \mathbb{Z}^+$, they decompose as compositions of prime intervals and their opposites. This decomposition is unique, reflecting the fundamental role of prime intervals (octave, twelfth, etc.) in the structure of rational harmony.

# Examples
- $110 = 2 \cdot 5 \cdot 11$
- $792 = 2^3 \cdot 3^2 \cdot 11$
- $343 = 7^3$
- The case $r = 0$ gives $n = 1$ (the empty product)
- $n = 1$ has the "trivial" factorization with no prime factors

# Related Concepts
- Prime Numbers
- Sieve of Eratosthenes
- Integral Domain
- Prime Intervals
- Integral Intervals

# Common Confusions
- The theorem says the factorization is unique UP TO ORDERING; $12 = 2^2 \cdot 3 = 3 \cdot 2^2$ are the "same" factorization
- The case $n = 1$ is included with $r = 0$ (empty product of primes equals 1)
- Uniqueness relies crucially on the property that if a prime divides a product, it divides one of the factors

# Source Reference
Chapter 8, "Unique Factorization" section, p. 100 (PDF)
