---
concept: Integral Domain
slug: integral-domain

category: algebra-in-music
subcategory: rings
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
section: "Cancellation"

extraction_confidence: high

aliases: []

prerequisites:
  - ring
extends:
  - ring
related:
  - cancellation-property
  - principal-ideal-domain
  - unique-prime-factorization
  - prime-numbers
contrasts_with: []

answers_questions:
  - "What is an integral domain?"
  - "Why is the no-zero-divisors property important for number theory?"
---

# Quick Definition

An integral domain is a ring in which the product of two non-zero elements is always non-zero. This property eliminates "zero divisors" and enables the cancellation property essential for unique factorization.

# Core Definition

"A ring $R$ is called an *integral domain* if whenever $a, b \in R$ with $ab = 0$, then $a = 0$ or $b = 0$" (Wright, Ch. 8, p. 100). Equivalently, $R$ has no zero divisors. The Cancellation Property follows as a consequence.

# Prerequisites

- **Ring** -- An integral domain is a ring satisfying an additional condition

# Key Properties

1. No zero divisors: $ab = 0$ implies $a = 0$ or $b = 0$
2. The cancellation property holds: $a \neq 0$ and $ab = ac$ implies $b = c$
3. $\mathbb{Z}$, $\mathbb{Q}$, and $\mathbb{R}$ are integral domains
4. $\mathbb{Z}_n$ is an integral domain if and only if $n$ is prime

# Construction / Recognition

## To verify a ring R is an integral domain:
1. Confirm $R$ is a ring
2. Check that $R$ is not the trivial ring (optional, usually assumed)
3. Verify: for all $a, b \in R$, if $ab = 0$ then $a = 0$ or $b = 0$
4. Equivalently, find no counterexample $ab = 0$ with both $a \neq 0$ and $b \neq 0$

# Context & Application

The integral domain property of $\mathbb{Z}$ underpins the unique factorization of integers into primes, which in turn ensures the unique decomposition of musical intervals into prime intervals. When $n$ is prime, $\mathbb{Z}_n$ being an integral domain means every non-zero element is a unit, so every non-trivial interval is a generating interval.

# Examples

**Example 1** (p. 100): $\mathbb{Z}$ is an integral domain: if $ab = 0$ with $a, b \in \mathbb{Z}$, then $a = 0$ or $b = 0$.

**Example 2** (p. 100): $\mathbb{R}$ and $\mathbb{Q}$ are integral domains.

**Example 3**: $\mathbb{Z}_6$ is NOT an integral domain: $[2] \cdot [3] = [6] = [0]$, but $[2] \neq [0]$ and $[3] \neq [0]$.

**Example 4**: $\mathbb{Z}_7$ IS an integral domain since 7 is prime.

# Relationships

## Builds Upon
- **Ring** -- An integral domain is a ring with the no-zero-divisors property

## Enables
- **Cancellation Property** -- Follows directly from the integral domain property
- **Principal Ideal Domain** -- A PID is an integral domain where every ideal is principal
- **Unique Prime Factorization** -- Requires the integral domain property

## Related
- **Prime Numbers** -- $\mathbb{Z}_p$ is an integral domain iff $p$ is prime

# Common Errors

- **Error**: Assuming $\mathbb{Z}_n$ is always an integral domain
  **Correction**: $\mathbb{Z}_n$ is an integral domain only when $n$ is prime

# Common Confusions

- **Confusion**: Thinking "integral domain" refers to integrals (calculus)
  **Clarification**: The name comes from the connection to the integers, which are the prototypical integral domain

- **Confusion**: Thinking the cancellation property requires multiplicative inverses
  **Clarification**: Cancellation is weaker than invertibility; it holds in $\mathbb{Z}$ even though most elements lack inverses

# Source Reference

Chapter 8: "Algebraic Properties of the Integers," "Cancellation" section, p. 100.

# Verification Notes

- Definition source: Direct quote from p. 100
- Confidence rationale: Explicit definition in source
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Z_6 counterexample, musical interpretation
