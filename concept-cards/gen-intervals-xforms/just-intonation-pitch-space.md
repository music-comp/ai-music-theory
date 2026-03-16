---
# === CORE IDENTIFICATION ===
concept: Just Intonation Pitch Space
slug: just-intonation-pitch-space

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: musical-spaces
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "2.1.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Example 2.1.5"
  - harmonic pitch space

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
  - function
extends: []
related:
  - modular-harmonic-space
  - generalized-interval-system
  - harmonic-intuition
contrasts_with:
  - chromatic-pitch-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I construct a GIS from a musical space?"
---

# Quick Definition

Just intonation pitch space is a GIS where pitches are related by pure frequency ratios, and intervals are measured as quotients FQ(t)/FQ(s) of the form 2^a * 3^b * 5^c.

# Core Definition

"This musical space, harmonic rather than melodic, comprises pitches available from a given pitch using just intonation. If we write FQ(s) to denote the fundamental frequency of the pitch s, then int(s, t) is the quotient FQ(t)/FQ(s). That quotient will be some number of the form 2^a * 3^b * 5^c, where a, b, and c are integers" (Lewin, Example 2.1.5, p. 48). IVLS is the multiplicative group of such ratios.

# Prerequisites

- **Group** — IVLS is a multiplicative group of rationals
- **Function** — int is a function from S x S into IVLS

# Key Properties

1. S = pitches available through just intonation from a given pitch
2. IVLS = {2^a * 3^b * 5^c : a, b, c in Z} under multiplication
3. int(s, t) = FQ(t)/FQ(s) (frequency ratio)
4. Identity: int(s, s) = 1
5. Inverse: int(t, s) = 1/int(s, t)
6. This is a harmonic (not melodic) space

# Construction / Recognition

## To Construct:
1. Start with a given pitch
2. Generate all pitches reachable by octaves (2), fifths (3/2), and thirds (5/4)
3. Define int(s, t) = FQ(t)/FQ(s)

## To Recognize:
1. Intervals are frequency ratios (multiplicative, not additive)
2. Ratios factor into powers of 2, 3, and 5

# Context & Application

Lewin devotes extensive discussion to the nature of harmonic intuition in this example. He argues that our intuitions of harmonic intervals are "highly conditioned by cultural factors" and that we intuit chains of basic harmonic moves (octave, fifth, third) rather than directly perceiving complex ratios like 45/32. The ratio 45/32 (C4 to F#4) arises as 2 * (5/4) * (3/4) * (3/4), reflecting the chain: octave above the mediant of the dominant of the dominant.

# Examples

**Example 1** (p. 48): int(C4, C5) = 2 (octave), int(C4, G4) = 3/2 (perfect fifth), int(C4, E4) = 5/4 (major third).

**Example 2** (p. 49, Figure 2.1): int(C4, F#4) = 45/32. The chain of intuitions: C4 -> G3 (dominant, 3/4) -> D3 (dominant, 3/4) -> F#3 (mediant, 5/4) -> F#4 (octave, 2). Product: 2 * (5/4) * (3/4) * (3/4) = 45/32.

# Relationships

## Builds Upon
- **Group** — uses a multiplicative group of rationals

## Enables
- **Modular Harmonic Space** — reducing this space modulo octaves

## Related
- **Harmonic Intuition** — Lewin's philosophical discussion of how we perceive harmonic intervals

## Contrasts With
- **Chromatic Pitch Space** — additive (semitones) vs. multiplicative (ratios)

# Common Errors

- **Error**: Computing intervals by addition instead of multiplication.
  **Correction**: Intervals are ratios; they compose by multiplication: int(r,s) * int(s,t) = int(r,t).

# Common Confusions

- **Confusion**: Thinking the "natural" mathematical factorization of a ratio reflects the intuited harmonic path.
  **Clarification**: 45/32 = 3^2 * 5 / 2^5 is the mathematical factorization, but the intuited path is 2 * (5/4) * (3/4) * (3/4), reflecting octave, mediant, and dominant relationships.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.5, Section 2.4, Figure 2.1, pp. 48-51, 53.

# Verification Notes

- Definition source: direct from Example 2.1.5
- Confidence rationale: explicit example with extended philosophical discussion
- Re-extracted from v2 card; preserved: 45/32 chain analysis, cultural conditioning discussion
