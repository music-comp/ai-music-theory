---
# === CORE IDENTIFICATION ===
concept: Greatest Common Divisor
slug: greatest-common-divisor

# === CLASSIFICATION ===
category: algebra-in-music
subcategory: number-theory
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
section: "Greatest Common Divisor"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "GCD"
  - "gcd"
  - "highest common factor"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - ideals-and-principal-ideals
  - principal-ideal-domain
extends:
  - principal-ideal-domain
related:
  - relatively-prime-integers
  - euler-phi-function
  - generating-interval
  - m-on-n-polyrhythmic-patterns
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I determine the GCD of two integers?"
  - "What is the greatest common divisor?"
  - "How does the GCD connect to ideals in Z?"
---

# Quick Definition

The greatest common divisor $\gcd(m, n)$ of two integers (not both zero) is the largest positive integer dividing both. It is the unique positive generator of the ideal $m\mathbb{Z} + n\mathbb{Z}$ in $\mathbb{Z}$.

# Core Definition

"Given $m, n \in \mathbb{Z}$, not both zero. The subset $m\mathbb{Z} + n\mathbb{Z}$ [...] is an ideal in $\mathbb{Z}$. Therefore it has a unique positive generator $d$, which divides both $m$ and $n$. If $e$ is any other positive integer which divides both $m$ and $n$, then [...] $e$ divides $d$. Therefore $d \geq e$ and we (appropriately) call $d$ the greatest common divisor of $m$ and $n$" (Wright, Ch. 8, p. 103). Since $d\mathbb{Z} = m\mathbb{Z} + n\mathbb{Z}$, there exist integers $h, k$ such that $d = hm + kn$ (Bezout's identity).

# Prerequisites

- **Ideals and Principal Ideals** -- The GCD is defined as the generator of an ideal
- **Principal Ideal Domain** -- The existence of the GCD depends on $\mathbb{Z}$ being a PID

# Key Properties

1. $d = \gcd(m, n)$ divides both $m$ and $n$
2. If $e$ divides both $m$ and $n$, then $e$ divides $d$ (so $d \geq e$)
3. There exist integers $h, k$ with $d = hm + kn$ (Bezout's identity)
4. $\gcd(m, n)$ is always positive
5. $\gcd(m, n) = 1$ means $m$ and $n$ are relatively prime

# Construction / Recognition

## To compute gcd(m, n):
1. **Via Euclidean algorithm**: Repeatedly apply the Division Algorithm
   - Compute $m = q_1 n + r_1$
   - Then $n = q_2 r_1 + r_2$
   - Continue until remainder is $0$; the last non-zero remainder is $\gcd(m, n)$
2. **Via prime factorization**: Factor both numbers and take the minimum power of each common prime
3. **Via ideal theory**: $\gcd(m, n)$ is the positive generator of $m\mathbb{Z} + n\mathbb{Z}$

# Context & Application

The GCD determines which modular chromatic intervals are generators: $[m]$ generates $\mathbb{Z}_n$ if and only if $\gcd(m, n) = 1$. It also governs m-on-n polyrhythmic patterns: when $\gcd(m, n) = 1$, a pattern of $m$ against $n$ takes exactly $mn$ units to complete.

# Examples

**Example 1** (p. 105): $\gcd(5, 12) = 1$: the fourth $[5]$ is a generating interval in the 12-chromatic scale.

**Example 2** (p. 105): $\gcd(4, 12) = 4$: the major third $[4]$ is not a generator; iterating it visits only 3 of 12 note classes.

**Example 3** (p. 106): $\gcd(3, 4) = 1$: explains why the 3-on-4 pattern in "In the Mood" takes $3 \times 4 = 12$ notes to complete.

**Example 4** (p. 103): $12\mathbb{Z} + 15\mathbb{Z} = 3\mathbb{Z}$, so $\gcd(12, 15) = 3$.

# Relationships

## Builds Upon
- **Principal Ideal Domain** -- GCD existence depends on $\mathbb{Z}$ being a PID

## Enables
- **Relatively Prime Integers** -- Defined as $\gcd(m, n) = 1$
- **Euler Phi Function** -- Counts integers relatively prime to $m$
- **M on N Polyrhythmic Patterns** -- Pattern completion depends on $\gcd(m, n) = 1$

## Related
- **Generating Interval** -- $[m]$ generates $\mathbb{Z}_n$ iff $\gcd(m, n) = 1$

# Common Errors

- **Error**: Forgetting that $\gcd(m, n)$ is defined to be positive
  **Correction**: The GCD is always the unique positive generator, even when $m$ or $n$ is negative

# Common Confusions

- **Confusion**: Thinking the GCD is computed by comparing magnitudes
  **Clarification**: The GCD is defined via divisibility; it is the largest common divisor, determined algebraically via the ideal $m\mathbb{Z} + n\mathbb{Z}$

- **Confusion**: Not realizing $d = hm + kn$ for some integers $h, k$
  **Clarification**: This Bezout identity is a non-trivial consequence of $\mathbb{Z}$ being a PID and is essential for many proofs

# Source Reference

Chapter 8: "Algebraic Properties of the Integers," "Greatest Common Divisor" section, pp. 103-104.

# Verification Notes

- Definition source: Direct quote from p. 103
- Confidence rationale: Explicit definition with proof in source
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: musical examples, Bezout identity
