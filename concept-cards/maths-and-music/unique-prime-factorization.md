---
# === CORE IDENTIFICATION ===
concept: Unique Prime Factorization
slug: unique-prime-factorization

# === CLASSIFICATION ===
category: algebra-in-music
subcategory: factorization
tier: advanced

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
section: "Unique Factorization"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "Fundamental Theorem of Arithmetic"
  - "unique factorization theorem"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - prime-numbers
  - integral-domain
extends:
  - principal-ideal-domain
related:
  - prime-intervals
  - integral-intervals
  - powers-of-two-as-exact-keyboard-intervals
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "Can every integer be factored uniquely into primes?"
  - "How does prime factorization determine an interval's character?"
---

# Quick Definition

Every positive integer $n \geq 1$ can be factored as a product of prime powers $n = p_1^{\alpha_1} p_2^{\alpha_2} \cdots p_r^{\alpha_r}$ in exactly one way (up to ordering of the primes). This guarantees the unique decomposition of integer intervals into prime intervals.

# Core Definition

"**Theorem:** Let $n \geq 1$ be an integer. Then $n$ can be factored as $n = p_1^{\alpha_1} p_2^{\alpha_2} \cdots p_r^{\alpha_r}$ where $r \geq 0$, $p_1, p_2, \ldots, p_r$ are distinct primes, and $\alpha_1, \alpha_2, \ldots, \alpha_r \geq 1$. Moreover, this factorization is unique" (Wright, Ch. 8, p. 105).

# Prerequisites

- **Prime Numbers** -- The theorem factors integers into primes
- **Integral Domain** -- The key property that if $p \mid mn$ then $p \mid m$ or $p \mid n$ (used in the uniqueness proof)

# Key Properties

1. Existence: every $n \geq 1$ has a prime factorization (proved by contradiction using WOP)
2. Uniqueness: the factorization is unique up to reordering (proved using the prime divisibility property)
3. The case $r = 0$ gives $n = 1$ (the empty product)
4. The uniqueness proof uses cancellation repeatedly

# Construction / Recognition

## Proof outline (Wright, pp. 105-106):
**Existence:** Suppose some $n$ has no factorization. Take the smallest such $n$ (by WOP). Since $n$ is not prime, $n = m\ell$ with $1 < m, \ell < n$. By minimality, both $m$ and $\ell$ have factorizations, so $n = m\ell$ does too -- contradiction.

**Uniqueness:** If $p_1^{\alpha_1} \cdots p_r^{\alpha_r} = q_1^{\beta_1} \cdots q_t^{\beta_t}$, then $p_1 \mid q_1^{\beta_1} \cdots q_t^{\beta_t}$, so $p_1 \mid q_i$ for some $i$ (since $p_1$ is prime), giving $p_1 = q_i$. Cancel and repeat.

# Context & Application

Unique factorization means every integer interval decomposes uniquely into prime intervals. Since rational intervals have ratios $m/n \in \mathbb{Q}^+$, they decompose as compositions of prime intervals and their inverses. This decomposition is unique, reflecting the fundamental role of prime intervals in rational harmony.

# Examples

**Example 1** (p. 105, exercises): $110 = 2 \cdot 5 \cdot 11$.

**Example 2** (p. 105, exercises): $792 = 2^3 \cdot 3^2 \cdot 11$.

**Example 3** (p. 105, exercises): $343 = 7^3$.

**Example 4**: The case $n = 1$ has the trivial factorization with $r = 0$ (empty product).

# Relationships

## Builds Upon
- **Prime Numbers** -- Primes are the building blocks
- **Principal Ideal Domain** -- The PID property enables the uniqueness proof

## Enables
- **Prime Intervals** -- Prime intervals are the irreducible musical building blocks
- **Powers of Two as Exact Keyboard Intervals** -- Proof uses unique factorization

## Related
- **Integral Intervals** -- Each integer interval decomposes uniquely into prime intervals

# Common Errors

- **Error**: Forgetting the $n = 1$ case
  **Correction**: $n = 1$ is included as the empty product with $r = 0$

# Common Confusions

- **Confusion**: Thinking $12 = 2^2 \cdot 3$ and $12 = 3 \cdot 2^2$ are "different" factorizations
  **Clarification**: Uniqueness is up to ordering; these are considered the same factorization

- **Confusion**: Thinking uniqueness is obvious
  **Clarification**: Uniqueness relies on the non-trivial property that if a prime divides a product, it divides one of the factors

# Source Reference

Chapter 8: "Algebraic Properties of the Integers," "Unique Factorization" section, pp. 105-106.

# Verification Notes

- Definition source: Direct quote of theorem from p. 105
- Confidence rationale: Explicit theorem with complete proof in source
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: proof outline, exercise examples
