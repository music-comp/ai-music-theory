---
# === CORE IDENTIFICATION ===
concept: M-on-N Polyrhythmic Patterns
slug: m-on-n-polyrhythmic-patterns

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
section: "Patterns of m on n in Music"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "m on n patterns"
  - "polyrhythm"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - greatest-common-divisor
  - relatively-prime-integers
extends:
  - generating-interval
related:
  - cyclic-group-and-generator
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do m-on-n polyrhythmic patterns work mathematically?"
  - "Why does a 3-on-4 pattern take 12 beats to complete?"
---

# Quick Definition

A compositional technique where a pattern of $m$ elements is superimposed against a pattern of $n$ elements. When $\gcd(m, n) = 1$, the combined pattern takes exactly $m \times n$ units to complete, creating musical tension and fulfillment.

# Core Definition

"Composers sometimes create ingenious musical passages by imposing a pattern of $m$ notes or beats against a pattern of $n$ such, where $\gcd(m, n) = 1$. This technique exploits (perhaps unknowingly by the composer) the fact that $[m]$ is a generator in $\mathbb{Z}_n$ (and vice versa)" (Wright, Ch. 8, p. 106). The double pattern completes after exactly $m \times n$ units and not before.

# Prerequisites

- **Greatest Common Divisor** -- The technique depends on $\gcd(m, n) = 1$
- **Relatively Prime Integers** -- Coprimality ensures full cycle completion

# Key Properties

1. Requires $\gcd(m, n) = 1$ for the full effect
2. The double pattern completes after exactly $mn$ units
3. Each $m$-cycle starts on a different element of $\mathbb{Z}_n$
4. Perfect symmetry: the pattern can be analyzed from either cycle's perspective
5. This polyrhythm is fundamentally different from tuplets

# Construction / Recognition

## How the pattern works mathematically:
1. Label beat positions $1, 2, \ldots, mn$
2. Every $m$th position marks a cycle point
3. The sequence of positions modulo $n$ is $[m], [2m], [3m], \ldots, [nm] = [0]$
4. Since $\gcd(m, n) = 1$, $[m]$ generates $\mathbb{Z}_n$, so all elements appear
5. The cycles align again only at position $mn$

# Context & Application

Composers use m-on-n patterns to create tension and a sense of fulfillment when the double cycle completes. The technique gives the listener the choice of counting beats in groups of $m$ or groups of $n$, creating rhythmic ambiguity.

# Examples

**Example 1** (p. 106): "In the Mood" -- 3 on 4. Three pitches ($C_4$, $E_4^\flat$, $A_4^\flat$) cycled through a four-eighth-note swing rhythm. Both cycles complete at 12 eighth notes. The multiples of $[3]$ in $\mathbb{Z}_4$ are $[3], [2], [1], [0]$, exhausting all elements.

**Example 2** (p. 107): "Rhapsody in Blue" -- 3 on 5. Three pitches ($D_4^\sharp$, $D_4$, $C_4^\sharp$) against a five-note rhythmic figure. Completes after $3 \times 5 = 15$ notes, spanning three measures.

**Example 3** (p. 107): "Ain't No Sunshine" -- 3 on 16. A rhythmic figure of 3 sixteenth notes repeated in 4/4 time (16 sixteenth notes per measure). Runs for $3 \times 16 = 48$ sixteenth notes = 3 measures.

# Relationships

## Builds Upon
- **Greatest Common Divisor** -- Pattern completion depends on $\gcd(m, n) = 1$
- **Relatively Prime Integers** -- Coprimality is the essential condition

## Enables
- Compositional understanding of polyrhythmic techniques

## Related
- **Cyclic Group and Generator** -- $[m]$ generating $\mathbb{Z}_n$ underlies the pattern

# Common Errors

- **Error**: Assuming any $m$-on-$n$ pattern takes $mn$ units
  **Correction**: If $\gcd(m, n) > 1$, the pattern completes in fewer than $mn$ units

# Common Confusions

- **Confusion**: Thinking m-on-n is the same as a tuplet
  **Clarification**: Tuplets subdivide beats unequally; m-on-n polyrhythm involves two independent cycles running simultaneously

- **Confusion**: Thinking the technique only works for pitch cycling
  **Clarification**: It can involve pitch cycling against rhythm (as in "In the Mood") or rhythmic figure cycling against meter (as in "Ain't No Sunshine")

# Source Reference

Chapter 8: "Algebraic Properties of the Integers," "Patterns of m on n in Music" section, pp. 106-108.

# Verification Notes

- Definition source: Direct from pp. 106-108
- Confidence rationale: Explicit discussion with three detailed musical examples
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all three musical examples, mathematical analysis
