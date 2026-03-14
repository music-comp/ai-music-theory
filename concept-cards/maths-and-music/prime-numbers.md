---
concept: Prime Numbers
category: theory
source: "Mathematics and Music"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
authors: "David Wright"
---

# Quick Definition
Positive integers greater than 1 divisible only by 1 and themselves, serving as the fundamental building blocks of all integers through unique prime factorization, and corresponding to "prime intervals" in music.

# Formal Definition
A positive integer $p$ is prime if it is divisible in $\mathbb{Z}$ by precisely two positive integers: $1$ and $p$. (Note that $1$ is not prime.) Key property: if $p$ is prime and $p \mid mn$, then $p \mid m$ or $p \mid n$. More generally, if $p$ divides a product $m_1 m_2 \cdots m_s$, then $p$ divides at least one $m_i$.

# Mathematical Context
The proof of the key property: if $p \nmid m$, then $\gcd(m, p) = 1$, so $1 = hm + kp$ for some integers $h, k$. Multiplying by $n$: $n = hmn + kpn$. Since $p \mid mn$, both terms on the right are divisible by $p$, so $p \mid n$. This property is essential for proving the Unique Factorization Theorem. If $p$ is prime, then $\mathbb{Z}_p$ is an integral domain.

# Musical Context
Prime numbers correspond to prime intervals: intervals whose frequency ratio is a prime integer. The interval ratio 2 (octave), ratio 3 (approximately an octave-and-a-fifth), ratio 5 (approximately two octaves and a major third), and ratio 7 (approximately two octaves and a minor seventh) are the most musically relevant prime intervals. All rational intervals can be decomposed into compositions of prime intervals and their opposites.

# Examples
- The first ten primes: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29
- $1$ is NOT prime (divisible by precisely one positive integer, not two)
- If $p = 5$ and $5 \mid 30$, then since $30 = 5 \cdot 6$, we have $5 \mid 5$ (confirming the property)
- $\mathbb{Z}_p$ is an integral domain for prime $p$: in $\mathbb{Z}_7$, $[a] \cdot [b] = [0]$ implies $[a] = [0]$ or $[b] = [0]$

# Related Concepts
- Sieve of Eratosthenes
- Unique Prime Factorization
- Relatively Prime Integers
- Prime Intervals
- Integral Domain

# Common Confusions
- $1$ is not prime; primality requires exactly two positive divisors, and $1$ has only one
- "Prime" in mathematics has a precise meaning different from "prime row" in twelve-tone theory
- There are infinitely many primes (proved by considering a prime factor of $p_1 p_2 \cdots p_n + 1$)

# Source Reference
Chapter 8, "Prime Numbers" section, p. 100 (PDF)
