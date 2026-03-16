---
concept: Retrogression
slug: retrogression

category: rhythm-and-form
subcategory: melody
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
section: "Retrogression"

extraction_confidence: high

aliases:
  - "retrograde"
  - "melodic retrogression"
  - "horizontal reflection"

prerequisites:
  - geometric-transformations-of-graphs
  - melody
extends:
  - geometric-transformations-of-graphs
related:
  - translation
  - transposition
  - symmetry-in-music
contrasts_with: []

answers_questions:
  - "What is retrogression in music?"
  - "How does retrogression relate to mathematical reflection?"
  - "What is a palindromic melody?"
---

# Quick Definition

The reversal of the order of notes in a melodic sequence, analogous to horizontal reflection of a function's graph, creating a mirror image of the original melody.

# Core Definition

*Retrogression* is the inversion of the order of notes in a sequence, producing a reflection of the initial melody. It is analogous to horizontal reflection, exemplified by replacing $y = f(x)$ with $y = f(-x)$, which reflects the graph around the $y$-axis. In music, retrogression means "inverting the order of notes" so that the resulting sequence forms a reflection of the initial one (Wright, p. 40).

# Prerequisites

- **Geometric Transformations of Graphs** — Retrogression is the musical application of horizontal reflection
- **Melody** — Retrogression operates on melodic sequences

# Key Properties

1. Retrogression is an involution: applying it twice returns the original sequence
2. If a melody is $(p_1, p_2, \ldots, p_n)$, the retrograde is $(p_n, p_{n-1}, \ldots, p_1)$
3. A palindromic melody is its own retrograde
4. Retrogression preserves the set of intervals but reverses their order and negates direction
5. Used extensively in serial (twelve-tone) composition as one of four basic row operations

# Construction / Recognition

## To construct a retrograde:

1. Write out the melody as a sequence of pitches
2. Reverse the order: last note becomes first, etc.
3. The result is the retrograde

## To recognize retrogression:

1. Look for approximate symmetry around a central point
2. Check if the second half mirrors the first half's pitch sequence in reverse

# Context & Application

When a melody has approximate palindromic structure, the listener perceives symmetry around a central point. Retrogression is used extensively in serial composition as one of four basic row operations (prime, retrograde, inversion, retrograde inversion). It also appears in freer tonal contexts as a device for creating coherence.

# Examples

- In "Raindrops Keep Falling On My Head," the melody exhibits symmetry around a central point marked with $\wedge$: pitches on either side form an approximate retrograde (p. 40)
- A simple retrograde: C-E-G becomes G-E-C
- In serial music, the tone row $\{0, 3, 7, 2, 11, \ldots\}$ reversed gives the retrograde row

# Relationships

## Builds Upon
- **Geometric Transformations of Graphs** — Retrogression is horizontal reflection
- **Melody** — Retrogression operates on melodic sequences

## Enables
- **Symmetry in Music** — Retrogression creates reflective symmetry

## Related
- **Translation** — Another transformation type (shift rather than reflection)
- **Transposition** — Can be combined with retrogression

# Common Errors

- **Error**: Confusing retrogression with inversion
  **Correction**: Retrogression reverses time order of pitches; inversion flips intervals vertically — they operate in different dimensions

# Common Confusions

- **Confusion**: Thinking retrogression means harmonic backward motion
  **Clarification**: In Wright's usage, retrogression is pitch-order reversal; in some contexts "retrogression" refers to chords moving in reverse functional order
- **Confusion**: Expecting perfect palindromes in music
  **Clarification**: Exact palindromic melodies are rare; approximate retrogression is more common

# Source Reference

Chapter 2: "Horizontal Structure", "Retrogression" section, p. 40 (PDF).

# Verification Notes

- Definition source: Direct from source, p. 40
- Confidence rationale: High — explicit definition with musical example
- Uncertainties: Wright writes $y = -f(x)$ but the musical operation corresponds to $y = f(-x)$ (horizontal reflection); noted in old card
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Raindrops example, involution property, serial music connection, $f(x)$ vs $f(-x)$ note
