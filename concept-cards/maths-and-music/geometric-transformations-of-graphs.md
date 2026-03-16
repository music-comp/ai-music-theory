---
# === CORE IDENTIFICATION ===
concept: Geometric Transformations of Graphs
slug: geometric-transformations-of-graphs

# === CLASSIFICATION ===
category: mathematical-foundations
subcategory: functions
tier: foundational

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Transformations of Graphs"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "graph transformations"
  - "shifts and stretches"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - functions-and-graphs
extends: []
related:
  - translation
  - transposition
  - retrogression
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are the four geometric transformations of function graphs?"
  - "How do graph transformations relate to musical transformations?"
  - "What happens when you shift or stretch a function's graph?"
---

# Quick Definition

Four operations -- vertical shift, horizontal shift, vertical stretch, and horizontal stretch -- that systematically move or deform the graph of a function, forming the mathematical basis for melodic transformations in music.

# Core Definition

Let $c \in \mathbb{R}$ and $y = f(x)$ be a function. Wright defines four geometric transformations (p. 16):
1. **Vertical shift**: $y = f(x) + c$ shifts the graph upward by $c$
2. **Horizontal shift**: $y = f(x - c)$ shifts the graph to the right by $c$
3. **Vertical stretch**: $y = cf(x)$ stretches the graph vertically by factor $c$
4. **Horizontal stretch**: $y = f(x/c)$ stretches the graph horizontally by factor $c$ (where $c \neq 0$)

# Prerequisites

- **Functions and Graphs** — Transformations operate on graphs of functions

# Key Properties

1. When $c < 0$ in shifts, the direction reverses (upward becomes downward, rightward becomes leftward)
2. When $0 < c < 1$ in stretches, the transformation is a compression
3. When $c < 0$ in vertical stretch, a flip about the $x$-axis occurs
4. When $c < 0$ in horizontal stretch, a flip about the $y$-axis occurs
5. Transformations compose and can be applied sequentially

# Construction / Recognition

## To apply a transformation:

1. Identify the base function $f(x)$
2. Determine the type of transformation (shift or stretch, horizontal or vertical)
3. Apply the appropriate formula: $f(x) + c$, $f(x - c)$, $cf(x)$, or $f(x/c)$
4. Note sign and magnitude of $c$ to determine direction/scale

# Context & Application

These transformations have direct musical analogues:
- **Horizontal shift** corresponds to translation (repetition in time)
- **Vertical shift** corresponds to transposition (shifting pitch up or down)
- **Horizontal stretch** corresponds to augmentation or diminution (changing tempo/duration)
- **Vertical stretch** relates to changes in amplitude (dynamics)
- **Horizontal reflection** ($c < 0$ in horizontal stretch) corresponds to retrogression

# Examples

- $y = x^2 + 1$ is $y = x^2$ shifted up by 1 (p. 16)
- $y = (x - 3)^2$ is $y = x^2$ shifted right by 3 (p. 16)
- $y = 2x^2$ is $y = x^2$ stretched vertically by factor 2 (p. 17)
- Musical transposition up a fourth corresponds to a vertical shift of the pitch-vs-time graph

# Relationships

## Builds Upon
- **Functions and Graphs** — Transformations modify function graphs

## Enables
- **Translation** — Musical horizontal shift is a graph transformation
- **Transposition** — Musical vertical shift is a graph transformation
- **Retrogression** — Musical reflection is a graph transformation

## Related
- **Translation** — Direct musical application of horizontal shift
- **Transposition** — Direct musical application of vertical shift

# Common Errors

- **Error**: Using $f(x + c)$ for a rightward shift
  **Correction**: Rightward shift uses $f(x - c)$; the sign is counterintuitive
- **Error**: Using $f(cx)$ for horizontal stretch by factor $c$
  **Correction**: Horizontal stretch uses $f(x/c)$, not $f(cx)$

# Common Confusions

- **Confusion**: Thinking "stretch" always means enlargement
  **Clarification**: When $0 < c < 1$, a stretch is actually a compression
- **Confusion**: Expecting the sign convention for horizontal transformations to match vertical ones
  **Clarification**: Horizontal transformations have counterintuitive signs: $f(x - c)$ shifts right, $f(x/c)$ stretches horizontally

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Transformations of Graphs" section, pp. 16-17 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 16-17
- Confidence rationale: High — explicit definitions with all four cases enumerated
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: musical analogues (translation, transposition, retrogression), sign convention confusions
