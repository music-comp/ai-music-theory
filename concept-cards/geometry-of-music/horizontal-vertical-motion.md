---
# === CORE IDENTIFICATION ===
concept: Horizontal and Vertical Motion (Parallel and Contrary)
slug: horizontal-vertical-motion

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
pdf_page: 85
section: "3.1, 3.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "parallel and contrary motion"
  - "transpositional and contrary components"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - ordered-pitch-space
  - two-note-chord-space
extends: []
related:
  - generalized-line-segments
  - decomposition-into-parallel-contrary
  - voice-leading-size
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What does horizontal motion mean in chord space?"
  - "What does vertical motion mean in chord space?"
  - "How do parallel and contrary motion appear geometrically?"
---

# Quick Definition
In the rotated chord space, horizontal motion represents parallel motion (both voices moving the same direction by the same amount), vertical motion represents perfect contrary motion (voices moving opposite directions by equal amounts), and oblique motion lies along 45-degree diagonals.

# Core Definition
After the 45-degree rotation of ordered pitch space, the geometric directions acquire clear musical meanings. Horizontal motion corresponds to pure parallel motion — transposition of the chord without changing its internal structure. Chords on the same horizontal line are transpositionally related. Vertical motion corresponds to perfect contrary motion — the voices move by equal amounts in opposite directions, preserving the sum of the chord's pitch classes. Chords on the same vertical line sum to the same value. Oblique motion (one voice stationary, the other moving) lies along 45-degree diagonals. These geometric-musical correspondences hold throughout chord space, from the infinite ordered pitch space through the folded Mobius strip.

# Prerequisites
- Ordered pitch space and its 45-degree rotation
- Two-note chord space

# Key Properties
1. Horizontal = parallel motion = transposition
2. Vertical = perfect contrary motion = pitch-class sum preserved
3. 45-degree diagonals = oblique motion (one voice moves)
4. Any voice leading can be decomposed into horizontal and vertical components
5. The horizontal component equals (d1 + d2)/2 semitones of transposition
6. The vertical component equals (d1 - d2)/2 semitones of contrary motion

# Construction / Recognition
## To Calculate:
1. Given voice leading (x1, x2) -> (y1, y2) with paths d1 and d2
2. Horizontal (rightward) component = d1 + d2
3. Vertical (upward) component = d1 - d2 (with direction reversal at each boundary contact)

# Context & Application
This decomposition is a form of vector analysis applied to music. It separates the *relative* motion among voices (the musically interesting contrary component) from the shared parallel component (mere transposition). This separation is fundamental to the book's analytical method and recurs in all dimensions of chord space.

# Examples
**Example 1** (p. 85): Figure 3.1.4 shows the rotation of ordered pitch space, making parallel motion horizontal and contrary motion vertical.
**Example 2** (p. 93): The voice leading (C, E) ->(2,1) (D, F) moves rightward by 2+1=3 and upward by 2-1=1.
**Example 3** (p. 93): The voice leading (B, D) ->(-3,3) (Ab, F) has zero horizontal component (-3+3=0) and moves downward by -3-3=-6, representing pure contrary motion.

# Relationships
## Builds Upon
- **ordered-pitch-space** — The space before rotation
- **two-note-chord-space** — Where these directions are most useful
## Enables
- **decomposition-into-parallel-contrary** — The formal decomposition method
- **cross-sections-of-chord-space** — Vertical cross sections isolate contrary motion
## Related
- **generalized-line-segments** — These line segments have horizontal and vertical components
- **voice-leading-size** — Related to path length in these coordinates

# Common Errors
- **Error**: Confusing "horizontal" in the unrotated vs. rotated space
  **Correction**: In the *rotated* space (which is standard), horizontal = parallel. In the unrotated space, horizontal = motion in voice 1 only.

# Common Confusions
- **Confusion**: Thinking oblique motion has no horizontal or vertical component
  **Clarification**: Oblique motion (one voice stationary) lies along 45-degree diagonals, which have equal horizontal and vertical components

# Source Reference
Chapter 3: A Geometry of Chords, Sections 3.1 and 3.4, pages 85, 93-94.

# Verification Notes
- Definition source: From Section 3.1 (rotation) and Section 3.4 (algebraic formulas)
- Confidence rationale: High — fundamental geometric principle with explicit formulas
- Cross-reference status: Verified against worked examples in Figures 3.4.1 and 3.4.3
