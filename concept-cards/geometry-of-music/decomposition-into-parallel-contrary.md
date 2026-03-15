---
# === CORE IDENTIFICATION ===
concept: Decomposition into Parallel and Contrary Motion
slug: decomposition-into-parallel-contrary

# === CLASSIFICATION ===
category: geometric-theory
subcategory: voice-leading
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 99
section: "3.7"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "vector decomposition of voice leading"
  - "parallel/contrary components"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - horizontal-vertical-motion
  - two-note-chord-space
extends: []
related:
  - cross-sections-of-chord-space
  - harmonic-consistency-and-efficient-voice-leading
  - individual-t-relatedness
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can any voice leading be decomposed into parallel and contrary components?"
  - "What is the musical significance of separating these components?"
---

# Quick Definition
Any voice leading can be mathematically decomposed into two components: a pure parallel component (both voices moving the same direction by the same amount) and a pure contrary component (voices moving by equal amounts in opposite directions). This is the musical analogue of decomposing a vector into x and y components.

# Core Definition
Given the voice leading (x1, x2) ->(d1, d2) (y1, y2), the pure parallel component transposes both voices by (d1+d2)/2 semitones, while the pure contrary component moves them by (d1-d2)/2 semitones in opposite directions. Geometrically, the parallel component is horizontal and the contrary component is vertical in the rotated chord space. The decomposition is musically significant because it separates the relative motion among voices (the contrary component, which creates counterpoint) from the shared transpositional motion (the parallel component, which merely shifts pitch level). By restricting attention to just the contrary component, we can focus on the voice-leading relationships that are shared by all individually T-related voice leadings.

# Prerequisites
- Horizontal and vertical motion in chord space
- Two-note chord space geometry

# Key Properties
1. Every voice leading has a unique decomposition into parallel + contrary
2. Parallel component = (d1+d2)/2 in each voice
3. Contrary component = (d1-d2)/2 with opposite signs
4. Parallel is horizontal; contrary is vertical in chord space
5. Individually T-related voice leadings share the same contrary component
6. The contrary component stays within a vertical cross section of chord space

# Construction / Recognition
## To Decompose:
1. Given voice leading with paths d1, d2
2. Parallel component: transpose first chord by (d1+d2)/2 semitones
3. Contrary component: the remaining motion, moving voices equally in opposite directions

# Context & Application
This decomposition is "nothing other than high school vector analysis." Its importance lies in allowing us to study the contrapuntally significant aspects of voice leading (the contrary component) separately from mere transposition (the parallel component). When we restrict attention to a cross section of chord space, we are effectively examining only the contrary components of voice leadings, which reveals the essential voice-leading relationships shared by all individually T-related voice leadings.

# Examples
**Example 1** (p. 100): The voice leading (E, B) -> (F#, B) is decomposed into the parallel (E, B) -> (F, C) (both up by semitone) and the contrary (F, C) -> (F#, B) (semitone in opposite directions) — Figure 3.7.1.
**Example 2** (p. 100): Pure contrary motion stays within a vertical cross section containing dyads whose pitch classes sum to the same value.

# Relationships
## Builds Upon
- **horizontal-vertical-motion** — The coordinate system for the decomposition
- **two-note-chord-space** — The space in which decomposition occurs
## Enables
- **cross-sections-of-chord-space** — Isolating the contrary component
- **individual-t-relatedness** — Voice leadings sharing the same contrary component
## Related
- **harmonic-consistency-and-efficient-voice-leading** — The contrary component determines efficient connections

# Common Errors
- **Error**: Thinking the decomposition changes the voice leading
  **Correction**: The decomposition is purely analytical — the original voice leading is the sum of the two components

# Common Confusions
- **Confusion**: Why focus on the contrary component?
  **Clarification**: The parallel component is "trivial" — it is mere transposition. The contrary component captures the contrapuntally interesting relative motion among voices.

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.7, pages 99-103.

# Verification Notes
- Definition source: Directly from Section 3.7, including the algebraic formulas (footnote 14)
- Confidence rationale: High — mathematically precise with explicit formula
- Cross-reference status: Verified against the cross-section discussion and Figure 3.7.1
