---
# === CORE IDENTIFICATION ===
concept: Functions and Graphs
slug: functions-and-graphs

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
section: "Functions and graphs"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "real-valued functions"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - sets-and-number-systems
  - interval-notation
extends: []
related:
  - geometric-transformations-of-graphs
  - pitch-and-frequency
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a function and its graph?"
  - "What are domain and range?"
  - "What key functions are used in mathematical music theory?"
---

# Quick Definition

A function maps elements from a domain to a range, and its graph is the visual representation of this mapping in the coordinate plane, with the independent variable on the horizontal axis and the dependent variable on the vertical axis.

# Core Definition

A function from some subset of $\mathbb{R}$ into $\mathbb{R}$ is expressed as $y = f(x)$, where $x$ is the independent variable and $y$ is the dependent variable. When the independent variable parameterizes time, it is often denoted $t$, giving $y = f(t)$. The graph is the set of points $\{(x, f(x))\}$ in the coordinate plane. The concepts of domain and range are assumed (Wright, pp. 15-16).

# Prerequisites

- **Sets and Number Systems** — Functions map subsets of $\mathbb{R}$ to $\mathbb{R}$
- **Interval Notation** — Domains are often intervals of $\mathbb{R}$

# Key Properties

1. Standard notation: $y = f(x)$ with $x$ independent and $y$ dependent
2. When the variable represents time, $t$ is used instead of $x$
3. Key examples: $y = mx + b$ (linear), $y = x^2$ (parabola), $y = \sin x$ and $y = \cos x$ (trigonometric)
4. Two especially relevant functions for music are $y = \sin x$ and $y = \cos x$

# Construction / Recognition

## To identify a function and its graph:

1. Determine the domain (set of valid inputs)
2. Determine the rule $f$ that assigns outputs to inputs
3. Plot points $(x, f(x))$ in the coordinate plane
4. The resulting curve is the graph

# Context & Application

In music, the horizontal axis ($x$ or $t$) typically represents time, and the vertical axis represents pitch or amplitude. Sound waves are modeled by sinusoidal functions. Musical scores themselves function as a type of graph: horizontal position encodes time, vertical position encodes pitch. The sine and cosine functions model pure tones.

# Examples

- $y = mx + b$: linear function with slope $m$ and $y$-intercept $b$ (p. 15)
- $y = x^2$: parabola with vertex at the origin (p. 15)
- $y = \sin x$ and $y = \cos x$: trigonometric functions especially relevant to music (p. 16)
- A musical staff acts analogously to a graph: left-to-right is time, up-down is pitch

# Relationships

## Builds Upon
- **Sets and Number Systems** — Functions operate on subsets of $\mathbb{R}$

## Enables
- **Geometric Transformations of Graphs** — Transformations modify graphs of functions
- **Horizontal Structure** — Time axis in music is the mathematical $x$-axis

## Related
- **Pitch and Frequency** — Sound waves are functions of time

# Common Errors

- **Error**: Confusing the graph (visual representation) with the function (abstract mapping)
  **Correction**: A graph represents a function but is not the function itself

# Common Confusions

- **Confusion**: Thinking $t$ and $x$ are fundamentally different as independent variables
  **Clarification**: The choice between $t$ (time) and $x$ is contextual; they serve the same mathematical role

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Functions and graphs" section, pp. 15-16 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 15-16
- Confidence rationale: High — explicit definitions with standard examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: musical context (staff as graph), confusion about $t$ vs $x$
