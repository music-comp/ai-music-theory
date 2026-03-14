---
concept: Cancellation Property
category: theory
source: "Mathematics and Music"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
authors: "David Wright"
---

# Quick Definition
The property that in an integral domain, if $a \neq 0$ and $ab = ac$, then $b = c$, allowing "division" by non-zero elements even when multiplicative inverses do not exist.

# Formal Definition
**Proposition (Cancellation):** If $R$ is an integral domain, and $a, b, c \in R$ with $a \neq 0$ and $ab = ac$, then $b = c$. Proof: $0 = ab - ac = a(b - c)$. Since $a \neq 0$ and $R$ is an integral domain, $b - c = 0$, i.e., $b = c$.

# Mathematical Context
The cancellation property is weaker than having multiplicative inverses. In $\mathbb{Z}$, we cannot "divide" $6$ by $4$ to get an integer, but if $4a = 4b$ for integers $a, b$, then $a = b$. This property is equivalent to the absence of zero divisors. It fails in rings like $\mathbb{Z}_6$: $[2] \cdot [3] = [2] \cdot [0]$ but $[3] \neq [0]$.

# Musical Context
The cancellation property in $\mathbb{Z}$ supports the uniqueness arguments in prime factorization, which in turn ensures that the decomposition of rational intervals into prime intervals is unique. It also underlies proofs about generators of $\mathbb{Z}_m$ and the structure of chromatic scales.

# Examples
- In $\mathbb{Z}$: $3 \cdot 7 = 3 \cdot 7$ trivially; more usefully, if $5x = 5y$ then $x = y$
- In $\mathbb{Z}_7$ (integral domain): if $[3] \cdot [a] = [3] \cdot [b]$, then $[a] = [b]$
- In $\mathbb{Z}_6$ (NOT integral domain): $[2] \cdot [3] = [0] = [2] \cdot [0]$, but $[3] \neq [0]$; cancellation fails

# Related Concepts
- Integral Domain
- Ring
- Units in a Ring

# Common Confusions
- Cancellation does not mean you can "divide" in the ring; $ab = ac$ implies $b = c$, but $a$ need not have a multiplicative inverse
- The condition $a \neq 0$ is essential; you cannot cancel zero
- Cancellation fails in rings with zero divisors, like $\mathbb{Z}_6$

# Source Reference
Chapter 8, "Cancellation" section, p. 100 (PDF)
