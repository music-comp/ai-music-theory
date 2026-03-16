---
concept: Prime Numbers
slug: prime-numbers

category: algebra-in-music
subcategory: factorization
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
section: "Prime Numbers"

extraction_confidence: high

aliases:
  - "primes"

prerequisites: []
extends: []
related:
  - sieve-of-eratosthenes
  - unique-prime-factorization
  - relatively-prime-integers
  - prime-intervals
  - integral-domain
contrasts_with: []

answers_questions:
  - "What is a prime number?"
  - "Why are prime numbers important for music theory?"
  - "How does prime factorization determine an interval's character?"
---

# Quick Definition

A positive integer $p$ is prime if it is divisible by precisely two positive integers: $1$ and $p$ itself. Primes are the multiplicative building blocks of all integers and correspond to musically irreducible "prime intervals."

# Core Definition

"A positive integer $p$ is called *prime* if it is divisible in $\mathbb{Z}$ by precisely two positive integers, namely 1 and $p$. (Note that 1 is not prime by virtue of the word 'precisely.')" (Wright, Ch. 8, p. 103). Key theorem: if $p$ is prime and $p \mid mn$, then $p \mid m$ or $p \mid n$.

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. $1$ is not prime (it has only one positive divisor)
2. If $p$ is prime and $p \mid mn$, then $p \mid m$ or $p \mid n$
3. More generally, if $p \mid m_1 m_2 \cdots m_s$, then $p$ divides at least one $m_i$
4. If $p$ is prime, then $\mathbb{Z}_p$ is an integral domain
5. If $p$ is prime and $n \in \mathbb{Z}$, then either $p \mid n$ or $\gcd(p, n) = 1$
6. There are infinitely many primes

# Construction / Recognition

## To check if p is prime:
1. Verify $p > 1$
2. Check that $p$ has no positive divisors other than $1$ and $p$
3. Equivalently, check no integer in $\{2, 3, \ldots, \lfloor\sqrt{p}\rfloor\}$ divides $p$

# Context & Application

Prime numbers correspond to prime intervals -- musical intervals whose frequency ratio is a prime integer. The interval ratio 2 (octave), ratio 3 (approximately octave-and-a-fifth), ratio 5 (approximately two octaves plus a major third), and ratio 7 (approximately two octaves plus a minor seventh) are the most musically relevant primes. By unique factorization, all rational intervals decompose into compositions of prime intervals and their inverses.

# Examples

**Example 1** (p. 103): The first ten primes: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29.

**Example 2** (p. 104): Proof of the key property: if $p \nmid m$, then $\gcd(m, p) = 1$, so $1 = hm + kp$. Multiplying by $n$: $n = hmn + kpn$. Since $p \mid mn$, both terms on the right are divisible by $p$, so $p \mid n$.

# Relationships

## Enables
- **Sieve of Eratosthenes** -- A method for finding all primes up to $n$
- **Unique Prime Factorization** -- Every integer factors uniquely into primes
- **Prime Intervals** -- Musical intervals with prime ratios

## Related
- **Relatively Prime Integers** -- If $p$ is prime, $p$ is coprime to every integer it does not divide
- **Integral Domain** -- $\mathbb{Z}_p$ is an integral domain iff $p$ is prime

# Common Errors

- **Error**: Claiming $1$ is prime
  **Correction**: Primality requires exactly two positive divisors; $1$ has only one

# Common Confusions

- **Confusion**: Thinking "prime" in number theory is the same as "prime row" in twelve-tone theory
  **Clarification**: These are entirely different uses of the word "prime"

- **Confusion**: Assuming there are finitely many primes
  **Clarification**: There are infinitely many primes; proved by considering a prime factor of $p_1 p_2 \cdots p_n + 1$

# Source Reference

Chapter 8: "Algebraic Properties of the Integers," "Prime Numbers" section, pp. 103-104.

# Verification Notes

- Definition source: Direct quote from p. 103
- Confidence rationale: Explicit definition with theorem and proof
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: proof sketch, musical prime interval connection
