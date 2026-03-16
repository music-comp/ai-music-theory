---
# === CORE IDENTIFICATION ===
concept: Continuity and Discontinuity
slug: continuity-and-discontinuity

# === CLASSIFICATION ===
category: harmonics-and-timbre
subcategory: periodic-functions
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Timbre and Periodic Functions"
chapter_number: 10
pdf_page: 118
section: "Piecewise Definitions and Continuity"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "continuous function"
  - "discontinuity"

# === TYPED RELATIONSHIPS ===
prerequisites: []
extends: []
related:
  - piecewise-functions
  - periodic-functions
  - fourier-series
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What does it mean for a function to be continuous?"
  - "What is a discontinuity?"
---

# Quick Definition

A function is continuous at a point if its value does not "jump" there -- small changes in input produce small changes in output. A discontinuity is a point where this condition fails, represented intuitively as a "jump" in the graph.

# Core Definition

"A function $y = f(x)$ is defined to be continuous at $x = a$ if given any $\epsilon > 0$ there exists $\delta > 0$ such that $|f(x) - f(a)| < \epsilon$ whenever $|x - a| < \delta$" (Wright, Ch. 10, p. 119). This means $f(x)$ will be arbitrarily close to $f(a)$ when $x$ is sufficiently close to $a$.

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. Continuity requires that $f(x)$ approaches $f(a)$ as $x$ approaches $a$
2. A "jump" discontinuity occurs when left and right limits differ
3. Jump discontinuities cannot be "fixed" by reassigning the function value at that point
4. A function continuous on an interval has a graph with no "jumps"

# Construction / Recognition

## To check continuity at x = a:
1. Evaluate $f(a)$
2. Check the left limit: $\lim_{x \to a^-} f(x)$
3. Check the right limit: $\lim_{x \to a^+} f(x)$
4. If both limits equal $f(a)$, the function is continuous at $a$
5. If the limits differ, or differ from $f(a)$, there is a discontinuity

# Context & Application

Physical vibrations are continuous (position cannot jump instantaneously), but mathematical models of sound waves may include discontinuities as idealizations of very rapid transitions. The Fourier theorem accommodates functions with finitely many discontinuities per period, interpreting them as moments of near-instantaneous change.

# Examples

**Example 1** (p. 118): $g(x) = x$ for $x \leq 1$, $g(x) = 1$ for $x > 1$: continuous everywhere (no jump at $x = 1$ since both sides approach 1).

**Example 2** (p. 119): $h(x) = x$ for $x \leq 1$, $h(x) = 2$ for $x > 1$: discontinuous at $x = 1$ (jump from 1 to 2). For $\epsilon = 1/2$, no $\delta$ makes $|h(x) - h(1)| < 1/2$ for all $x$ near 1.

**Example 3** (p. 119): $h_2(x)$ with $h_2(1) = 3$: reassigning $h(1)$ does not fix the discontinuity.

# Relationships

## Enables
- **Piecewise Functions** -- Piecewise functions may or may not be continuous
- **Fourier Series** -- The Fourier theorem requires finitely many discontinuities per period

## Related
- **Periodic Functions** -- Musical waveforms are periodic and may have discontinuities

# Common Errors

- **Error**: Assuming all discontinuities can be removed by choosing the "right" value
  **Correction**: Jump discontinuities (where left and right limits differ) cannot be repaired by reassigning a single function value

# Common Confusions

- **Confusion**: Thinking a discontinuity means the function is undefined
  **Clarification**: A discontinuity means the function fails to be continuous; the function may still be defined at that point (just with a "jump")

# Source Reference

Chapter 10: "Timbre and Periodic Functions," "Piecewise Definitions and Continuity" section, pp. 118-120.

# Verification Notes

- Definition source: Direct quote of epsilon-delta definition from p. 119
- Confidence rationale: Explicit formal definition with examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: h_2 example showing non-removable discontinuity
