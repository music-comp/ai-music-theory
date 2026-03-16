---
concept: Geometric Series and Summation
slug: geometric-series-and-summation

category: mathematical-foundations
subcategory: number-systems
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
section: "Dots"

extraction_confidence: high

aliases:
  - "geometric series"
  - "geometric sum formula"

prerequisites:
  - sets-and-number-systems
extends: []
related:
  - dotted-note-duration-formula
  - note-durational-values
contrasts_with: []

answers_questions:
  - "What is the formula for the sum of a finite geometric series?"
  - "When does an infinite geometric series converge?"
  - "How does the geometric series relate to dotted note durations?"
---

# Quick Definition

The formula for summing a finite geometric series, $1 + r + r^2 + \cdots + r^m = \frac{1 - r^{m+1}}{1 - r}$, which underlies the mathematics of dotted note durations and converges to $\frac{1}{1-r}$ when $|r| < 1$.

# Core Definition

For any integer $m \geq 0$ and any real number $r \neq 1$ (Wright, pp. 33-34):
$$\sum_{i=0}^{m} r^i = 1 + r + r^2 + \cdots + r^m = \frac{1 - r^{m+1}}{1 - r}$$

The proof uses the telescoping product $(1 - r)(1 + r + r^2 + \cdots + r^m) = 1 - r^{m+1}$ (Exercise 3).

For $|r| < 1$, the infinite geometric series converges:
$$\sum_{i=0}^{\infty} r^i = \frac{1}{1 - r}$$

# Prerequisites

- **Sets and Number Systems** — Uses real numbers $\mathbb{R}$

# Key Properties

1. The formula requires $r \neq 1$; when $r = 1$, the sum is $m + 1$
2. The proof uses the algebraic identity $(1-r)(1 + r + \cdots + r^m) = 1 - r^{m+1}$
3. For $|r| < 1$, the infinite series converges to $\frac{1}{1-r}$
4. For $|r| \geq 1$ (and $r \neq 1$), the series diverges
5. The concept of limit from calculus is involved in the infinite case

# Construction / Recognition

## To compute a geometric series sum:

1. Identify the common ratio $r$ and number of terms ($m + 1$)
2. Verify $r \neq 1$
3. Apply: $\sum_{i=0}^{m} r^i = \frac{1 - r^{m+1}}{1 - r}$
4. For the infinite case ($|r| < 1$): $\sum_{i=0}^{\infty} r^i = \frac{1}{1 - r}$

# Context & Application

The geometric series with $r = \frac{1}{2}$ directly models dotted note durations. A note of duration $d$ with $m$ dots has duration $d \cdot \sum_{i=0}^{m}(1/2)^i$. The convergence to $2d$ means that no matter how many dots are added, the total duration never reaches twice the original. The infinite sum $\sum_{i=0}^{\infty}(1/2)^i = 2$ (equation 2.2) captures this limit.

# Examples

- $r = \frac{1}{2}, m = 1$: $1 + \frac{1}{2} = \frac{3}{2}$ (single dot multiplier) (p. 33)
- $r = \frac{1}{2}, m = 2$: $1 + \frac{1}{2} + \frac{1}{4} = \frac{7}{4}$ (double dot multiplier) (p. 33)
- $r = \frac{1}{2}, m = 3$: $1 + \frac{1}{2} + \frac{1}{4} + \frac{1}{8} = \frac{15}{8}$ (triple dot multiplier) (p. 34)
- $r = \frac{1}{2}, m \to \infty$: $\sum = 2$ (the limiting duration factor) (equation 2.2)
- General: $\sum_{i=0}^{4} 3^i = \frac{1 - 3^5}{1 - 3} = \frac{-242}{-2} = 121$

# Relationships

## Builds Upon
- **Sets and Number Systems** — Operates on real numbers

## Enables
- **Dotted Note Duration Formula** — The dotted note formula is a geometric series with $r = 1/2$

## Related
- **Note Durational Values** — The power-of-2 system connects to geometric sequences

# Common Errors

- **Error**: Applying the formula when $r = 1$
  **Correction**: The formula requires $r \neq 1$; when $r = 1$, the sum is simply $m + 1$

# Common Confusions

- **Confusion**: Thinking the infinite series always converges
  **Clarification**: Convergence requires $|r| < 1$; for $|r| \geq 1$ the series diverges
- **Confusion**: Confusing the proof method with induction
  **Clarification**: The proof uses the algebraic identity $(1-r)(1 + r + \cdots + r^m) = 1 - r^{m+1}$, though induction also works

# Source Reference

Chapter 2: "Horizontal Structure", "Dots" section, pp. 33-34 (PDF); equation (2.2); Exercise 3.

# Verification Notes

- Definition source: Direct from source, pp. 33-34
- Confidence rationale: High — explicit formula with proof hint and convergence result
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all worked examples, proof by telescoping product, convergence condition
