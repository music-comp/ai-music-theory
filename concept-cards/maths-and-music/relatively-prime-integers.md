---
# === CORE IDENTIFICATION ===
concept: Relatively Prime Integers
slug: relatively-prime-integers

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
  - "coprime"
  - "mutually prime"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - greatest-common-divisor
extends:
  - greatest-common-divisor
related:
  - euler-phi-function
  - generating-interval
  - prime-numbers
  - m-on-n-polyrhythmic-patterns
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What does it mean for two integers to be relatively prime?"
  - "How does relative primality determine generating intervals?"
---

# Quick Definition

Two integers $m$ and $n$ are relatively prime (coprime) if $\gcd(m, n) = 1$, meaning their only common divisors are $\pm 1$. This condition determines which intervals generate a chromatic scale and which polyrhythmic patterns complete fully.

# Core Definition

"To say that $\gcd(m, n) = 1$ is to say that the only common divisors of $m$ and $n$ in $\mathbb{Z}$ are $\pm 1$. In this case we say that $m$ and $n$ are relatively prime" (Wright, Ch. 8, p. 103). By Bezout's identity, $m$ and $n$ are relatively prime iff there exist integers $h, k$ with $hm + kn = 1$.

# Prerequisites

- **Greatest Common Divisor** -- Relative primality is defined in terms of the GCD

# Key Properties

1. $\gcd(m, n) = 1$ iff $m\mathbb{Z} + n\mathbb{Z} = \mathbb{Z}$
2. Equivalent to the existence of $h, k \in \mathbb{Z}$ with $hm + kn = 1$
3. If $p$ is prime and $n \in \mathbb{Z}$, then either $p \mid n$ or $\gcd(p, n) = 1$
4. $\phi(m)$ counts the integers in $\{1, \ldots, m-1\}$ that are relatively prime to $m$

# Construction / Recognition

## To determine if m and n are relatively prime:
1. Compute $\gcd(m, n)$ using the Euclidean algorithm
2. If $\gcd(m, n) = 1$, they are relatively prime
3. Alternatively, check that $m$ and $n$ share no common prime factor

# Context & Application

Relative primality determines three musical properties: (1) which chromatic intervals generate all note classes ($[m]$ generates $\mathbb{Z}_n$ iff $\gcd(m, n) = 1$); (2) which m-on-n polyrhythmic patterns run for exactly $mn$ units before repeating; (3) the "completeness" of interval circles.

# Examples

**Example 1** (p. 105): $\gcd(5, 12) = 1$: the fourth generates all 12 note classes.

**Example 2** (p. 106): $\gcd(3, 4) = 1$: the 3-on-4 pattern in "In the Mood" takes 12 notes to complete.

**Example 3** (p. 107): $\gcd(3, 5) = 1$: the 3-on-5 pattern in "Rhapsody in Blue" takes 15 notes.

**Example 4**: $\gcd(4, 12) = 4 \neq 1$: 4 and 12 are NOT relatively prime; major thirds cycle through only 3 note classes.

# Relationships

## Builds Upon
- **Greatest Common Divisor** -- Relative primality is the condition $\gcd(m, n) = 1$

## Enables
- **Euler Phi Function** -- $\phi(m)$ counts integers relatively prime to $m$
- **M on N Polyrhythmic Patterns** -- Require $\gcd(m, n) = 1$ to work fully

## Related
- **Generating Interval** -- Generating intervals correspond to coprime pairs
- **Prime Numbers** -- A prime $p$ is relatively prime to every integer it does not divide

# Common Errors

- **Error**: Assuming "relatively prime" means at least one of the numbers is prime
  **Correction**: $\gcd(8, 15) = 1$ even though neither 8 nor 15 is prime

# Common Confusions

- **Confusion**: Thinking two consecutive integers might share a common factor
  **Clarification**: Consecutive integers are always relatively prime: $\gcd(n, n+1) = 1$

# Source Reference

Chapter 8: "Algebraic Properties of the Integers," "Greatest Common Divisor" section, p. 103. Also introduced in Chapter 6, p. 74.

# Verification Notes

- Definition source: Direct quote from p. 103
- Confidence rationale: Explicit definition in source
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: musical examples, consecutive integers fact
