---
# === CORE IDENTIFICATION ===
concept: Uniform vs. Individual Operations on Progressions
slug: uniform-vs-individual-operations

# === CLASSIFICATION ===
category: geometric-theory
subcategory: formalism
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.5-2.6"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "uniform vs. individual symmetry"
  - "uniform and individual application"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - optic-symmetries
  - basic-musical-object
extends:
  - optic-symmetries
related:
  - voice-leading-vs-chord-progression
  - voice-leading-in-pitch-space
  - chord-progression
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the difference between uniform and individual application of OPTIC symmetries?"
  - "What is a voice leading vs. a chord progression?"
---

# Quick Definition
When OPTIC symmetries are applied to progressions (pairs of chords), they can be applied uniformly (the same operation to both chords, preserving voice identity) or individually (different operations to each chord, destroying voice identity).

# Core Definition
Progressions are "higher-order" constructions containing multiple objects, so each OPTIC symmetry can be applied in two ways. Uniform application applies the exact same operation to both chords in a progression: for example, uniformly permuting (C, E, G) -> (C, F, A) to get (E, G, C) -> (F, A, C), applying the same reordering to both. Individual application applies different versions of the symmetry to each chord: permuting to get (C, E, G) -> (F, C, A), where different reorderings are applied. The distinction is crucial: uniform permutation yields voice leadings (voice identity preserved); individual permutation and cardinality change yield chord progressions (voice identity destroyed). Since each of five symmetries can be uniform, individual, or absent, there are far more categories of progressions than of individual objects.

# Prerequisites
- **optic-symmetries** — The symmetries being applied
- **basic-musical-object** — The objects in the progression

# Key Properties
1. Uniform: same operation applied to both chords; individual: different operations
2. Uniform P -> voice leadings; individual P+C -> chord progressions
3. Each symmetry has three modes: uniform, individual, absent
4. Creates many more categories for progressions than for single objects
5. Fundamental to the voice leading vs. chord progression distinction

# Construction / Recognition
## To Construct/Create:
1. Take a progression of two chords
2. Choose whether each OPTIC symmetry is applied uniformly, individually, or not at all
3. The choice determines what kind of musical object results
## To Identify/Recognize:
1. If the same transformation is applied to both chords -> uniform
2. If different transformations are applied -> individual

# Context & Application
This distinction is the formal mechanism underlying the difference between voice leadings and chord progressions. It also underlies the concepts of uniformly and individually T-related voice leadings (Section 2.6), which capture important musical relationships like the connection between the openings of Wagner's Tristan, Brahms' Op. 76 No. 4, and Debussy's Prelude to "The Afternoon of a Faun."

# Examples
**Example 1** (p. 59, Fig 2.5.1): Uniform permutation of (C, E, G) -> (C, F, A) produces (E, G, C) -> (F, A, C) — same reordering of both chords. Individual permutation produces (C, E, G) -> (F, C, A) — different reorderings.

**Example 2** (p. 63, Fig 2.5.6): Summary table: voice leading in pitch space = uniform P; voice leading in pitch-class space = uniform OP; chord progression = individual OPC.

# Relationships
## Builds Upon
- **optic-symmetries** — The symmetries being distinguished
## Enables
- **voice-leading-vs-chord-progression** — The most important consequence
## Related
- **voice-leading-in-pitch-space** — Uniform P
- **chord-progression** — Individual OPC
## Contrasts With
- No direct contrast; this IS the contrast between two modes of applying symmetries

# Common Errors
- **Error**: Applying individual operations when uniform is intended (or vice versa)
  **Correction**: Carefully distinguish whether voice identity should be preserved (uniform) or destroyed (individual)

# Common Confusions
- **Confusion**: Thinking "individual" means "applying to only one chord"
  **Clarification**: Individual means applying different versions of the same symmetry to each chord — both chords are transformed, but differently

# Source Reference
Chapter 2: Harmony and Voice Leading, Sections 2.5-2.6, pages 59-67.

# Verification Notes
- Definition source: Direct from Section 2.5, formalized in Figure 2.5.6
- Confidence rationale: High — precisely defined with clear examples
- Cross-reference status: Verified; fundamental distinction used throughout the book
