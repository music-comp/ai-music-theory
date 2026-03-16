---
concept: Spacing-Preserving Voice Leading
slug: spacing-preserving-voice-leading

category: fundamentals
subcategory: voice-leading-foundations
tier: intermediate

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Appendix 1: Fundamentals"
chapter_number: null
pdf_page: 533
section: null

extraction_confidence: high

aliases:
  - strongly crossing free
  - crossing-free voice leading

prerequisites:
  - voice-leading
  - voice-exchange
extends:
  - efficient-voice-leading
related:
  - interscalar-transposition
  - spiral-diagram-derivation
  - double-transposition
contrasts_with: []

answers_questions:
  - "What is a spacing-preserving voice leading?"
  - "How do voice leadings factor into voice exchanges and remainders?"
  - "What foundational knowledge is needed for geometric models of voice leading?"
---

# Quick Definition

A voice leading that preserves chordal-step distance between voices and therefore cannot be arranged in register so that voices cross -- the "remainder" left after factoring out voice exchanges from any voice leading.

# Core Definition

"Spacing-preserving voice leadings preserve chordal-step distance and therefore cannot be arranged in register so that their voices cross: if plotted on the pitch-class circle, each voice can glide smoothly from its starting point to its destination without ever sounding the same pitch class as any other voice, except perhaps at its endpoints" (p. 536). Any voice leading whatsoever "can be factored into a voice exchange and a remainder that is spacing-preserving or strongly crossing free" (p. 536). A spacing-preserving voice leading between transpositionally related chords can always be represented as a combination of T (scale transposition) and t (chordal transposition). Between inversionally related chords, it combines I and i.

# Prerequisites

- **Voice leading** -- The basic concept of paths connecting chord notes
- **Voice exchange** -- The component that is factored out

# Key Properties

1. Preserves chordal-step distance between all voice pairs
2. Cannot produce voice crossings when arranged in register
3. Any voice leading = voice exchange + spacing-preserving remainder
4. Between transpositional relatives: combination of T and t
5. Between inversional relatives: combination of I and i
6. Between unrelated chords: called an interscalar transposition
7. Voice crossings never make a voice leading smaller; hence minimal voice leadings are always crossing-free

# Construction / Recognition

## To Factor a Voice Leading:
1. Write the voice leading between two chords
2. Identify any voice crossings (paths that would intersect on the pitch-class circle)
3. Factor out the voice exchanges (contrary-motion pairs that sum to zero)
4. The remainder is the spacing-preserving component
5. Express the remainder as T-t, I-i, or interscalar transposition

# Context & Application

The spiral diagrams represent precisely the spacing-preserving voice leadings between transpositions of one or more chords -- "the possibilities that remain when we ignore voice exchanges" (p. 537). This factoring is fundamental because it separates the "deep" voice-leading structure (represented by spiral diagrams) from the "surface" voice exchanges that can be treated as registral decoration. "Hierarchical self-similarity can arise as the byproduct of the search for efficient voice leading, in both the melodic and harmonic domains" (p. 537).

# Examples

**Example 1** (p. 536): (C, E, G) -> (C, F, A) is spacing-preserving: no crossings possible in any registral arrangement.

**Example 2** (p. 536): (C4, E4) -> (E4, C5) is NOT a voice exchange despite exchanging pitch classes; it is a transposition along the chord.

**Example 3** (p. 536): Voice crossings in (C, E, G) -> (E, C, G) are a voice exchange whose paths sum to zero.

# Relationships

## Builds Upon
- **Voice leading** -- The basic concept being factored
- **Voice exchange** -- The component factored out

## Enables
- **Spiral diagram derivation** -- Diagrams represent spacing-preserving voice leadings
- **Double transposition** -- Spacing-preserving voice leadings between transpositional relatives

## Related
- **Interscalar transposition** -- Spacing-preserving voice leadings between unrelated chords
- **Efficient voice leading** -- Minimal voice leadings are always spacing-preserving

## Contrasts With
- None listed

# Common Errors

- **Error**: Assuming voice exchanges always involve pitch-class swaps
  **Correction**: (C4, E4) -> (E4, C5) swaps pitch classes but is NOT a voice exchange -- it's a transposition. True voice exchanges have paths summing to zero.

- **Error**: Thinking voice crossings can make a voice leading smaller
  **Correction**: "Voice crossings never make a voice leading smaller" (p. 537), so the minimal voice leading between any two chords is always crossing-free

# Common Confusions

- **Confusion**: Conflating Schenkerian "voice exchange" with the technical definition here
  **Clarification**: This is a "minimal and technical" definition referring to specific voice-leading patterns; "Schenkerian theory uses a more robust notion" (p. 536)

- **Confusion**: Thinking the spiral diagram represents ALL voice leadings
  **Clarification**: It represents only the spacing-preserving ones; voice exchanges are represented by curved arrows added to the diagram

# Source Reference

Appendix 1: "Fundamentals," pp. 536-537. See also Figure A1.2.

# Verification Notes

- Definition source: Direct from p. 536
- Confidence rationale: HIGH -- explicitly defined with formal properties
- Cross-reference status: Verified
- Re-extraction notes: New card; no previous version existed
