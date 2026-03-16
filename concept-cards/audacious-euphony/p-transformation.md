---
# === CORE IDENTIFICATION ===
concept: "P Transformation (Parallel)"
slug: p-transformation

# === CLASSIFICATION ===
category: transformations
subcategory: null
tier: intermediate

# === PROVENANCE ===
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Hexatonic Cycles"
chapter_number: 2
pdf_page: 35
section: "Hexatonic Progressions, Tonnetz Representations, and Triadic Transformations"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Parallel"
  - "parallel transformation"
  - "mode change"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - single-semitonal-displacement
  - minimal-work-relation
extends: []
related:
  - l-transformation
  - h-transformation
  - lp-transformation
  - hexatonic-cycle
  - chromatic-versus-diatonic-semitone
contrasts_with:
  - l-transformation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the P transformation?"
  - "How does P differ from L?"
---

# Quick Definition
The P (Parallel) transformation converts a major triad to its parallel minor or vice versa by moving the third by one chromatic semitone while preserving the root and fifth as common tones.

# Core Definition
Following Brian Hyer's 1989 adaptation from Lewin, Cohn uses "the letter **P** (parallel major/minor) to indicate the motion between triads that share two common tones and a common root" (p. 47). P preserves the perfect-fifth dyad (root and fifth) and displaces the third by chromatic semitone. It is an involution: "two consecutive applications produce an identity" (p. 47). P produces a chromatic semitone (same letter name, e.g., E to Eb), appearing as vertical motion (0 degrees) on the Tonnetz (Table 2.1, p. 48).

# Prerequisites
- **single-semitonal-displacement** — P involves one voice moving one semitone
- **minimal-work-relation** — P is one of two transformations realizing the minimal-work relation

# Key Properties
1. Voice-leading work: 1 semitone
2. Common tones: 2 (the perfect fifth)
3. Root motion: none (same root)
4. Semitonal species: chromatic (same letter name)
5. Tonnetz angle: 0 degrees (vertical)
6. Involution: P(P(X)) = X

# Construction / Recognition
- P(C major) = c minor: (C, **E**, G) -> (C, **Eb**, G)
- P(c minor) = C major: (C, **Eb**, G) -> (C, **E**, G)
- P(F# major) = f# minor: (F#, **A#**, C#) -> (F#, **A**, C#)

In a hexatonic cycle, P alternates with L to generate the complete cycle.

# Context & Application
P transformations are ubiquitous in tonal music as mode mixture or modal mutation. In hexatonic progressions, P alternates with L to generate smooth chromatic paths. On the Tonnetz, P appears as vertical motion perpendicular to the fifth axis.

# Examples
**Example 1** (p. 42): The Tarnhelm progression begins with P transformations.

**Example 2** (pp. 44-45): Beethoven's "Spring" Sonata: Bb major mutates to bb minor; Schubert's Piano Trio Op. 100: "The unmelodied accompaniment mutates a major chord to its parallel minor."

# Relationships
## Builds Upon
- **single-semitonal-displacement** — P realizes a single semitonal displacement
- **minimal-work-relation** — P is one of two minimal-work transformations

## Enables
- **hexatonic-cycle** — P alternates with L to generate the cycle
- **h-transformation** — H = LP = PL
- **lp-transformation** — LP combines L and P

## Related
- **chromatic-versus-diatonic-semitone** — P produces the chromatic species

## Contrasts With
- **l-transformation** — P preserves the fifth, L preserves the minor third; P produces chromatic semitone, L produces diatonic semitone

# Common Errors
- **Error**: Confusing P with R (relative)
  **Correction**: P changes mode with same root (C major -> c minor); R changes mode with shared major-third dyad (C major -> a minor)

# Common Confusions
- **Confusion**: Thinking P always means "moving down" (E to Eb)
  **Clarification**: P on minor triads moves the third up (Eb to E); the direction depends on the starting mode

# Source Reference
Chapter 2: Hexatonic Cycles, pp. 47-48. Table 2.1 on p. 48.

# Verification Notes
- Re-extracted from v2 card; preserved: the Hyer attribution, the Table 2.1 data, the musical examples
- Confidence: HIGH — explicitly defined in Table 2.1 with clear notation
