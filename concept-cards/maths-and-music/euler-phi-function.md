---
# === CORE IDENTIFICATION ===
concept: Euler Phi Function
slug: euler-phi-function

# === CLASSIFICATION ===
category: algebra-in-music
subcategory: number-theory
tier: advanced

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
section: "Euler Phi Function"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "Euler's totient function"
  - "phi function"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - relatively-prime-integers
  - units-in-a-ring
extends:
  - greatest-common-divisor
related:
  - generating-interval
  - modular-integers-ring-structure
  - prime-numbers
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I compute the Euler phi function?"
  - "How many generating intervals does an m-chromatic scale have?"
  - "What does the Euler phi function count?"
---

# Quick Definition

The Euler phi function $\phi(m)$ counts the number of positive integers less than $m$ that are relatively prime to $m$. It simultaneously gives the number of units in $\mathbb{Z}_m$, the number of group generators of $\mathbb{Z}_m$, and the number of generating intervals in the $m$-chromatic scale.

# Core Definition

"For any $m \in \mathbb{Z}^+$, we have defined the *Euler phi function* $\phi(m)$ to be the number of positive integers $n$ with $1 \leq n < m$ which are relatively prime to $m$. According to the above theorem, $\phi(m)$ also counts the number of elements in $\mathbb{Z}_m^*$, and the number of group generators for $(\mathbb{Z}_m, +)$. By virtue of the latter, $\phi(m)$ counts the number of generating intervals in the $m$-chromatic scale" (Wright, Ch. 8, p. 105).

# Prerequisites

- **Relatively Prime Integers** -- $\phi(m)$ counts integers relatively prime to $m$
- **Units in a Ring** -- $\phi(m) = |\mathbb{Z}_m^*|$

# Key Properties

1. $\phi(m) = |\{n \in \mathbb{Z}^+ \mid 1 \leq n < m, \gcd(n, m) = 1\}|$
2. $\phi(m) = |\mathbb{Z}_m^*|$ (number of units in the ring)
3. $\phi(m)$ = number of generators of $(\mathbb{Z}_m, +)$
4. $\phi(m)$ = number of generating intervals in the $m$-chromatic scale
5. $\phi(p) = p - 1$ for any prime $p$

# Construction / Recognition

## To compute phi(m):
1. List integers $1, 2, \ldots, m-1$
2. For each, compute $\gcd(n, m)$
3. Count those with $\gcd(n, m) = 1$
4. Alternatively, use the formula based on prime factorization: if $m = p_1^{a_1} \cdots p_k^{a_k}$, then $\phi(m) = m \prod_{i=1}^{k}(1 - 1/p_i)$

# Context & Application

$\phi(m)$ tells how many fundamentally different interval cycles exist in an $m$-chromatic scale that visit every note class. For the standard 12-chromatic scale, $\phi(12) = 4$, corresponding to the semitone, fourth, fifth, and major seventh.

# Examples

**Example 1** (p. 105): $\phi(12) = 4$, since the numbers $1, 5, 7, 11$ are precisely the positive integers $< 12$ which are relatively prime to $12$. These correspond to the four generating intervals: semitone, fourth, fifth, and major seventh.

**Example 2**: $\phi(7) = 6$, since every integer from 1 to 6 is relatively prime to 7 (because 7 is prime). In a 7-note scale, every non-trivial interval is a generator.

**Example 3**: $\phi(p) = p - 1$ for any prime $p$.

# Relationships

## Builds Upon
- **Greatest Common Divisor** -- $\phi$ counts integers coprime to $m$
- **Units in a Ring** -- $\phi(m) = |\mathbb{Z}_m^*|$

## Enables
- **Generating Interval** -- $\phi(m)$ counts generating intervals

## Related
- **Prime Numbers** -- $\phi(p) = p - 1$ for primes
- **Modular Integers Ring Structure** -- $\phi$ characterizes the unit group

# Common Errors

- **Error**: Including $m$ itself when counting integers relatively prime to $m$
  **Correction**: $\phi(m)$ counts integers strictly less than $m$: $1 \leq n < m$

# Common Confusions

- **Confusion**: Thinking larger $m$ gives larger $\phi(m)$
  **Clarification**: $\phi(12) = 4$ but $\phi(11) = 10$; primes have large $\phi$ values relative to their size

# Source Reference

Chapter 8: "Algebraic Properties of the Integers," "Euler Phi Function" section, p. 105. First introduced in Chapter 6, p. 74.

# Verification Notes

- Definition source: Direct quote from p. 105
- Confidence rationale: Explicit definition with multiple equivalent characterizations
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: musical interpretation, phi(12) = 4 example
