---
concept: Interval Notation
slug: interval-notation

category: mathematical-foundations
subcategory: sets-and-relations
tier: foundational

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Basic Mathematical and Musical Concepts"
chapter_number: 1
pdf_page: 14
section: "Intervals of Real Numbers"

extraction_confidence: high

aliases:
  - "real number intervals"
  - "open and closed intervals"

prerequisites:
  - sets-and-number-systems
extends: []
related:
  - functions-and-graphs
  - pitch-and-frequency
contrasts_with:
  - musical-intervals

answers_questions:
  - "What is the standard notation for intervals of real numbers?"
  - "What is the difference between open and closed intervals?"
  - "How do mathematical intervals differ from musical intervals?"
---

# Quick Definition

Standard mathematical notation for subsets of the real number line, using brackets and parentheses to denote closed, open, and half-open intervals.

# Core Definition

For $a, b \in \mathbb{R}$, Wright defines (p. 14):
- $(a, b) = \{x \in \mathbb{R} \mid a < x < b\}$ (open interval)
- $[a, b] = \{x \in \mathbb{R} \mid a \leq x \leq b\}$ (closed interval)
- $(a, b]$ and $[a, b)$ for half-open intervals

# Prerequisites

- **Sets and Number Systems** — Intervals are subsets of $\mathbb{R}$

# Key Properties

1. Open intervals exclude both endpoints; closed intervals include both
2. Half-open intervals include one endpoint and exclude the other
3. The notation $(a, b)$ can also denote an ordered pair; context distinguishes the two uses
4. The term "interval" in this mathematical sense is distinct from the musical use of "interval" (distance between pitches)

# Construction / Recognition

## To identify an interval type:

1. Check the left boundary symbol: `[` means $a$ is included, `(` means $a$ is excluded
2. Check the right boundary symbol: `]` means $b$ is included, `)` means $b$ is excluded
3. Parentheses = strict inequality; brackets = non-strict inequality

# Context & Application

Mathematical intervals model continuous ranges of pitch or frequency. The range of human audibility, approximately $[20, 20000]$ Hz, is a closed interval in $\mathbb{R}^+$. The set of all pitches is $\mathbb{R}^+ = (0, \infty)$. Wright explicitly warns that the mathematical use of "interval" must be distinguished from the musical use (p. 19).

# Examples

- The range of audible frequencies: approximately $[20, 20000]$ in Hz (p. 17)
- The set of pitches is $\mathbb{R}^+$, which can be written as $(0, \infty)$
- The domain of $\sin x$ restricted to one period: $[0, 2\pi)$

# Relationships

## Builds Upon
- **Sets and Number Systems** — Intervals are subsets of $\mathbb{R}$

## Enables
- **Functions and Graphs** — Functions are defined on domains that are often intervals

## Related
- **Pitch and Frequency** — Continuous pitch ranges are modeled as intervals

## Contrasts With
- **Musical Intervals** — Same word "interval" but entirely different meaning; Wright explicitly flags this

# Common Errors

- **Error**: Confusing $(a, b)$ as an ordered pair when an interval is meant
  **Correction**: Context determines whether $(a, b)$ denotes a point in $\mathbb{R}^2$ or a subset of $\mathbb{R}$

# Common Confusions

- **Confusion**: Conflating mathematical intervals (subsets of $\mathbb{R}$) with musical intervals (distances between pitches)
  **Clarification**: Wright explicitly warns about this ambiguity (p. 19); mathematical intervals are sets of real numbers, musical intervals are measured in semitones
- **Confusion**: Thinking open and closed intervals contain the same elements
  **Clarification**: $(a, b)$ excludes $a$ and $b$, while $[a, b]$ includes them — the difference matters at the boundaries

# Source Reference

Chapter 1: "Basic Mathematical and Musical Concepts", "Intervals of Real Numbers" section, p. 14 (PDF).

# Verification Notes

- Definition source: Direct from source, p. 14
- Confidence rationale: High — explicit definitions with standard notation
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: audibility range example, confusion about mathematical vs. musical intervals
