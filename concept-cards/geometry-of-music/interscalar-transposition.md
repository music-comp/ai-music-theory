---
# === CORE IDENTIFICATION ===
concept: Interscalar Transposition
slug: interscalar-transposition

# === CLASSIFICATION ===
category: scales-modes
subcategory: operations
tier: intermediate-advanced

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Scales"
chapter_number: 4
pdf_page: 160
section: "4.8-4.9"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "interscalar mapping"
  - "cross-scale transposition"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - scalar-transposition
  - modulation-as-voice-leading
extends:
  - scalar-transposition
related:
  - strongly-crossing-free-voice-leading
  - combining-scalar-chromatic-transposition
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is interscalar transposition?"
  - "How does it generalize scalar transposition?"
  - "Why is it connected to efficient voice leading?"
---

# Quick Definition
Interscalar transposition moves a musical pattern from one scale to another while preserving scalar intervals. If scalar transposition moves patterns along a single scale, interscalar transposition moves them between different scales. It is always strongly crossing-free and represents a maximally efficient voice leading.

# Core Definition
Interscalar transposition generalizes scalar transposition by allowing the starting and ending scales to differ. Where scalar transposition shifts a pattern by n steps within the same scale, interscalar transposition shifts a pattern by n steps from one scale into another. For example, a motive in D harmonic minor can be related to the "same" motive in F major by zero-step interscalar transposition (root maps to root), even though the scales are different. The number of steps depends on the arbitrary choice of first scale degree, but the existence of an interscalar relationship is independent of this choice. Crucially, interscalar transpositions are always "strongly crossing-free" voice leadings — they remain crossing-free regardless of how voices are distributed in register. This connects interscalar transposition directly to the problem of finding efficient voice leadings.

# Prerequisites
- Scalar transposition
- Modulation as voice leading

# Key Properties
1. Moves patterns from one scale to another preserving scalar intervals
2. Always strongly crossing-free
3. For scales of the same size, can be modeled as voice leadings between pitch-class sets
4. The most efficient voice leading between any two chords is always an interscalar transposition
5. Reduces the search space for efficient voice leadings dramatically (by a factor of n for n-note chords)
6. For n-note chords, only n interscalar transpositions need to be checked (one for each possible root-to-degree mapping)

# Construction / Recognition
## To Apply:
1. Number the degrees of both scales
2. Map degree d of the first scale to degree (d+n) of the second scale
3. The mapping is completely determined by the choice of n
## To Find Efficient Voice Leadings:
1. Given two chords of size n, check the n interscalar transpositions
2. Choose the one with the smallest total voice motion

# Context & Application
Interscalar transposition is one of the chapter's most powerful concepts, connecting scale theory to the general problem of efficient voice leading. It explains how composers intuitively find efficient voice leadings: rather than searching all possible voice-leading mappings, they need only consider the small number of interscalar transpositions. The concept also explains the pedagogical practice of forbidding voice crossings — this restriction naturally privileges interscalar transpositions, drastically simplifying the search for efficient voice leading.

# Examples
**Example 1** (p. 160): Three forms of the motive in Bach's D minor invention: the first two (in D harmonic minor and F major) are related by zero-step interscalar transposition; the first and third by two-step interscalar transposition (Figure 4.8.5).
**Example 2** (p. 161): Interscalar transposition in Debussy's "Fetes" (diatonic to acoustic), Stravinsky's Rite of Spring, and Shostakovich's A major fugue (treating triads as scales) — Figure 4.8.6.
**Example 3** (p. 164-165): The minimal voice leading between C major and A minor triads is a one-step ascending interscalar transposition; between C major and C minor triads, a zero-step interscalar transposition (Figure 4.9.4).

# Relationships
## Builds Upon
- **scalar-transposition** — Generalized by interscalar transposition
- **modulation-as-voice-leading** — Interscalar transposition is a specific type of scale voice leading
## Enables
- **combining-scalar-chromatic-transposition** — The final synthesis
- **strongly-crossing-free-voice-leading** — Interscalar transpositions are always strongly crossing-free
## Related
- **voice-leading-lattices** — Lattice edges can be understood as interscalar transpositions

# Common Errors
- **Error**: Thinking the specific "number of steps" is musically significant
  **Correction**: The number depends on the arbitrary choice of first scale degree; what matters is the existence of an interscalar relationship

# Common Confusions
- **Confusion**: Can interscalar transposition connect scales of different sizes?
  **Clarification**: Yes — "interscalar transposition in pitch space" connects scales of different sizes (Figure 4.8.7), but these introduce octave-dependent changes and cannot be modeled as pitch-class voice leadings

# Source Reference
Chapter 4: Scales, Sections 4.8-4.9, pages 160-167.

# Verification Notes
- Definition source: From Sections 4.8 and 4.9
- Confidence rationale: High — central concept with extensive examples and formal treatment
- Cross-reference status: Verified against Bach, Debussy, Stravinsky, Shostakovich examples
