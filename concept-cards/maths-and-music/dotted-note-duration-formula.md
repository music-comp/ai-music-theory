---
concept: Dotted Note Duration Formula
slug: dotted-note-duration-formula

category: rhythm-and-form
subcategory: duration
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
  - "dotted notes"
  - "dot duration formula"

prerequisites:
  - note-durational-values
  - geometric-series-and-summation
extends:
  - note-durational-values
related:
  - ties-and-slurs
  - meter-and-time-signatures
  - rests
contrasts_with: []

answers_questions:
  - "How does a dot affect a note's duration?"
  - "What is the formula for the duration of a multiply-dotted note?"
  - "What is the limiting duration as the number of dots increases?"
---

# Quick Definition

A dot after a note extends its duration by half, a second dot adds a quarter, and in general $m$ dots multiply the original duration $d$ by $[2 - (1/2)^m]$, approaching but never reaching $2d$.

# Core Definition

A note of duration $d$ followed by $m$ dots has duration (Wright, p. 33, equation 2.1):
$$d_m = d\left[2 - \left(\frac{1}{2}\right)^m\right]$$

This is derived from the geometric series:
$$d_m = d\left(1 + \frac{1}{2} + \frac{1}{2^2} + \cdots + \frac{1}{2^m}\right) = d\sum_{i=0}^{m}\left(\frac{1}{2}\right)^i$$

Equivalently: $d_m = d\left[1 + \frac{2^m - 1}{2^m}\right]$.

# Prerequisites

- **Note Durational Values** — The base duration $d$ comes from the power-of-2 system
- **Geometric Series and Summation** — The formula uses the finite geometric series sum

# Key Properties

1. One dot multiplies duration by $3/2$
2. Two dots multiply by $7/4$
3. Three dots multiply by $15/8$
4. The formula uses $r = 1/2$ in the geometric series
5. As $m \to \infty$, $d_m \to 2d$ (but never reaches it)
6. The value $d_m$ is always strictly less than $2d$ for any finite $m$
7. Dots apply to rests as well as notes

# Construction / Recognition

## To calculate a dotted note's duration:

1. Determine the base duration $d$ of the undotted note
2. Count the number of dots $m$
3. Apply the formula: $d_m = d[2 - (1/2)^m]$
4. Alternatively: first dot adds $d/2$, second adds $d/4$, third adds $d/8$, etc.

# Context & Application

In practice, more than two dots are rarely used. A dotted quarter note in $\frac{4}{4}$ time has duration $\frac{3}{2}$ beats, a dotted half note has 3 beats. The convergence to $2d$ as $m \to \infty$ connects to the infinite geometric series $\sum_{i=0}^{\infty}(1/2)^i = 2$, illustrating the concept of limit from calculus.

# Examples

- Dotted sixteenth (4 beats/whole note): $d = \frac{1}{4}$, $m = 1$: $d_1 = \frac{1}{4} \cdot \frac{3}{2} = \frac{3}{8}$ beats (p. 32)
- Double-dotted sixteenth: $d_2 = \frac{1}{4} \cdot \frac{7}{4} = \frac{7}{16}$ beats (p. 33)
- Triply dotted sixteenth (2 beats/whole note): $d = \frac{1}{8}$, $m = 3$: $d_3 = \frac{1}{8}[2 - \frac{1}{8}] = \frac{15}{64}$ beats (p. 34)
- Dotted half note in $\frac{4}{4}$: $d = 2$ beats, $d_1 = 3$ beats

# Relationships

## Builds Upon
- **Note Durational Values** — Dots modify the power-of-2 duration system
- **Geometric Series and Summation** — The formula is a geometric series with $r = 1/2$

## Enables
- Complex rhythmic patterns that cannot be expressed with simple note values alone

## Related
- **Ties and Slurs** — Ties provide an alternative way to extend duration
- **Rests** — Dots apply to rests as well

# Common Errors

- **Error**: Thinking each dot adds half of the original duration
  **Correction**: Each dot adds half of the *previous* addition: first dot adds $d/2$, second adds $d/4$, third adds $d/8$

# Common Confusions

- **Confusion**: Believing a dotted note can reach double the original duration
  **Clarification**: $d_m < 2d$ for all finite $m$; the limit $2d$ is never reached
- **Confusion**: Thinking dots only apply to notes
  **Clarification**: Dots apply to rests as well as notes

# Source Reference

Chapter 2: "Horizontal Structure", "Dots" section, pp. 32-34 (PDF); equation (2.1).

# Verification Notes

- Definition source: Direct from source, equation (2.1), p. 33
- Confidence rationale: High — explicit formula with derivation and worked examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all worked examples, limit interpretation, dots-on-rests note
