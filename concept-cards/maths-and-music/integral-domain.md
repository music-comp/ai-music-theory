---
concept: Integral Domain
category: theory
source: "Mathematics and Music"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
authors: "David Wright"
unit: null
---

# Quick Definition
A ring in which the product of two non-zero elements is always non-zero, enabling the cancellation property that is fundamental to algebraic reasoning about integers and intervals.

# Formal Definition
A ring $R$ is an integral domain if whenever $a, b \in R$ with $ab = 0$, then $a = 0$ or $b = 0$. Equivalently, $R$ has no "zero divisors" (non-zero elements whose product is zero). The Cancellation Property follows: if $a \neq 0$ and $ab = ac$, then $b = c$.

# Mathematical Context
Proof of cancellation: $ab = ac$ implies $a(b - c) = 0$. Since $a \neq 0$ and $R$ is an integral domain, $b - c = 0$, so $b = c$. The integers $\mathbb{Z}$, rationals $\mathbb{Q}$, and reals $\mathbb{R}$ are all integral domains. The ring $\mathbb{Z}_n$ is an integral domain precisely when $n$ is prime.

# Musical Context
The integral domain property of $\mathbb{Z}$ underpins the unique factorization of integers, which in turn relates to the decomposition of musical intervals into prime intervals. When $n$ is prime, $\mathbb{Z}_n$ being an integral domain means the corresponding n-chromatic scale has special algebraic properties: every non-zero element is a unit, so every non-trivial interval is a generating interval.

# Examples
- $\mathbb{Z}$ is an integral domain: if $ab = 0$ with $a, b \in \mathbb{Z}$, then $a = 0$ or $b = 0$
- $\mathbb{R}$ and $\mathbb{Q}$ are integral domains
- $\mathbb{Z}_6$ is NOT an integral domain: $[2] \cdot [3] = [6] = [0]$, but $[2] \neq [0]$ and $[3] \neq [0]$
- $\mathbb{Z}_7$ IS an integral domain (since 7 is prime)

# Related Concepts
- Ring
- Cancellation Property
- Prime Numbers
- Unique Prime Factorization
- Units in a Ring

# Common Confusions
- "Integral domain" does not mean "a domain of integrals"; the name comes from its connection to the integers
- $\mathbb{Z}_n$ is an integral domain if and only if $n$ is prime; for composite $n$, there are zero divisors
- The cancellation property requires $a \neq 0$; dividing by zero is never valid, even in an integral domain

# Source Reference
Chapter 8, "Cancellation" section, p. 100 (PDF)
