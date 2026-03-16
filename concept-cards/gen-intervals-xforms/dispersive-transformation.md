---
concept: Dispersive Transformation
slug: dispersive-transformation

category: generalized-set-theory
subcategory: injection-function
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.4"

extraction_confidence: high

aliases: []

prerequisites:
  - inj-function
extends: []
related:
  - progressive-transformation
  - external-transformation
  - internal-transformation
contrasts_with:
  - progressive-transformation

answers_questions:
  - "What is a dispersive transformation?"
---

# Quick Definition
A dispersive transformation f (for the progression X to Y) has minimal INJ(X, Y)(f) — it maps X largely outside Y, the opposite of a progressive transformation.

# Core Definition
Section 6.4 (Lewin, p. 174): A dispersive transformation makes "the value of INJ(X, Y)(f) minimal or relatively small." When INJ(X, Y)(f) = 0, f is maximally dispersive: no element of X maps into Y. Progressive followed by Y-external tends to be dispersive; X-internal followed by dispersive tends to be dispersive.

# Prerequisites
- **INJ Function** — Dispersive is defined via low INJ(X, Y)(f) values

# Key Properties
1. f is dispersive for X->Y if INJ(X, Y)(f) is near zero
2. When INJ(X, Y)(f) = 0: T_f(X) has no common tones with Y
3. Progressive + Y-external -> tends dispersive
4. X-internal + dispersive -> tends dispersive

# Construction / Recognition
## To Identify:
1. Compute INJ(X, Y)(f) for transformations of interest
2. Those with lowest values are dispersive

## To Recognize:
1. Transformed X shares no (or minimal) common tones with Y

# Context & Application
In Schoenberg's op. 19 no. 6, measure 8 accumulates dispersive transpositions of the rh chord, creating maximum "distance" from lh before the reprise. Six transpositions T_i satisfy INJ(rh, lh)(T_i) = 0; four appear in measure 8.

# Examples
**Example 1** (pp. 174-175, Figure 6.10): Schoenberg op. 19 no. 6. Dispersive transpositions: i = 0, 2, 4, 5, 7, 9 satisfy INJ(rh, lh)(T_i) = 0. Measure 8 embeds T_2(rh), T_5(rh), T_7(rh), T_9(rh), creating dense dispersive texture before the T_0(rh) = rh return at measure 9.

# Relationships
## Builds Upon
- **INJ Function** — Defined through INJ values

## Contrasts With
- **Progressive Transformation** — High vs. low INJ(X, Y)(f)

## Related
- **External Transformation** — External: low INJ(X, X); dispersive: low INJ(X, Y)

# Common Errors
- **Error**: Equating dispersive with external
  **Correction**: Dispersive concerns X's relationship to Y; external concerns X's relationship to itself

# Common Confusions
- **Confusion**: Thinking dispersive means "unstructured"
  **Clarification**: Dispersive transpositions can be highly structured, as in op. 19 no. 6 where they create a systematic buildup

# Source Reference
Chapter 6: Generalized Set Theory (2), section 6.4 and Figure 6.10, pp. 174-175.

# Verification Notes
- Definition source: Direct from section 6.4
- Confidence rationale: Explicit definition with analytical example
- Re-extraction notes: Re-extracted from v2 card; preserved: Schoenberg op.19 no.6 example. Added v3.1 structure.
