---
# === CORE IDENTIFICATION ===
concept: Sieve of Eratosthenes
slug: sieve-of-eratosthenes

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
section: "Sieve of Eratosthenes"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - prime-numbers
extends: []
related:
  - unique-prime-factorization
  - euler-phi-function
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I find all prime numbers up to a given limit?"
  - "What is the Sieve of Eratosthenes?"
---

# Quick Definition

An ancient systematic procedure for finding all prime numbers up to a given integer $n$ by iteratively crossing out multiples of each successive prime, attributed to the Greek mathematician Eratosthenes of Cyrene (3rd century BC).

# Core Definition

"A systematic procedure for finding the prime numbers was given by the Greek astronomer and mathematician Eratosthenes of Cyrene (3rd century BC). We conceive of the positive integers as an infinite list $1, 2, 3, 4, 5, 6, \ldots$, then proceed to cross out certain numbers on the list" (Wright, Ch. 8, p. 104). After crossing out $1$, then all higher multiples of each successive prime, the remaining numbers are exactly the primes.

# Prerequisites

- **Prime Numbers** -- Must understand what primes are to appreciate what the sieve finds

# Key Properties

1. Cross out $1$ first (not prime)
2. For each remaining $m > 1$, cross out all multiples of $m$ greater than $m$
3. Only need to check divisors up to $\sqrt{n}$
4. The process is complete and systematic -- it identifies all primes $\leq n$
5. Demonstrates that primes become sparser as numbers grow

# Construction / Recognition

## To sieve primes up to n:
1. List integers $1, 2, 3, \ldots, n$
2. Cross out $1$
3. Starting with $m = 2$: cross out $4, 6, 8, 10, \ldots$ (all multiples of $2$ above $2$)
4. Move to the next uncrossed number ($3$): cross out $9, 15, 21, 27, \ldots$ (multiples of $3$ above $3$, not already crossed)
5. Continue with $5, 7, \ldots$ up to $\lfloor\sqrt{n}\rfloor$
6. All remaining uncrossed numbers are prime

# Context & Application

The sieve identifies which integers are prime, directly relating to prime intervals in music. It also helps determine the Euler phi function $\phi(n)$ by identifying which integers share prime factors with $n$.

# Examples

**Example 1** (p. 104): Sieving up to 30. After crossing out multiples of 2, then 3, then 5 (only need to check up to $\sqrt{30} \approx 5.48$), the remaining primes are: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29 -- ten primes total.

# Relationships

## Builds Upon
- **Prime Numbers** -- The sieve finds primes

## Enables
- **Unique Prime Factorization** -- Knowing the primes enables factorization
- **Euler Phi Function** -- Prime factorization aids computation of $\phi$

# Common Errors

- **Error**: Forgetting to cross out $1$ first
  **Correction**: $1$ is not prime and must be removed at the start

- **Error**: Checking all $m$ up to $n$
  **Correction**: Only need to check up to $\sqrt{n}$; any composite $n$ has a factor $\leq \sqrt{n}$

# Common Confusions

- **Confusion**: Thinking the sieve only finds some primes
  **Clarification**: The sieve is complete -- it identifies all primes $\leq n$

# Source Reference

Chapter 8: "Algebraic Properties of the Integers," "Sieve of Eratosthenes" section, pp. 104-105.

# Verification Notes

- Definition source: Direct from pp. 104-105
- Confidence rationale: Explicit description with worked example
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: sqrt(n) optimization note
