---
# === CORE IDENTIFICATION ===
concept: Basic Voice Leading of Scales
slug: basic-voice-leading-of-scales

# === CLASSIFICATION ===
category: modulation
subcategory: voice-leading
tier: intermediate

# === PROVENANCE ===
source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Modulation"
chapter_number: 8
pdf_page: 361
section: "Two models of key distance"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - scalar basic voice leading
  - diatonic scale basic voice leading

# === TYPED RELATIONSHIPS ===
prerequisites:
  - basic-voice-leading
  - scalar-spiral-diagram
extends: []
related:
  - leading-tone-lowering
  - fourth-scale-degree-raising
  - hierarchical-transposition
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the minimal voice leading connecting two diatonic scales?"
  - "How is the basic voice leading of scales calculated using the spiral diagram?"
  - "Why do seven sharpward modulations map tonic to tonic, but five flatward modulations do not?"
---

# Quick Definition

The minimal voice leading connecting two diatonic scales, which either lowers the leading tone by semitone (flatward/clockwise) or raises the fourth degree by semitone (sharpward/counterclockwise), keeping all other notes fixed.

# Core Definition

The basic voice leading for the 7-in-12 diatonic scale is calculated using the spiral diagram recipe from section 2.1: sliding downward from C to F gives **T**_{-7}, revisiting the initial angular position four times for t_4. The combination **T**_{-7}t_4 (or equivalently **T**_5t_{-3}) lowers a single note -- the leading tone -- by semitone while keeping all other notes fixed (p. 358, Fig. P8.2). This connects fifth-related diatonic scales (e.g., C major to F major by lowering B to B-flat). A series of seven sharpward basic voice leadings maps the tonic of C major to the tonic of C-sharp major; a series of five flatward basic voice leadings maps C major's tonic to D-flat major's leading tone (p. 364, Fig. 8.1.7). The basic voice leading is the fundamental unit of modulation in functional tonality.

# Prerequisites

- **Basic voice leading** -- The general concept of minimal voice leading between chords, applied here to scales
- **Scalar spiral diagram** -- The geometrical framework showing diatonic scales in 7-in-12 space

# Key Properties

1. Flatward (clockwise): lowers the leading tone by semitone (**T**_{-7}t_4)
2. Sharpward (counterclockwise): raises the fourth degree by semitone
3. All other notes remain fixed -- only one pitch changes
4. Connects fifth-related diatonic scales
5. Seven sharpward steps: C major tonic maps to C-sharp major tonic
6. Five flatward steps: C major tonic maps to D-flat major leading tone
7. Any repeated modulatory move returns to its starting scale in a new registral position (because 7 does not divide 12)

# Construction / Recognition

## To Apply the Basic Voice Leading of Scales:
1. Start with any diatonic scale
2. For flatward modulation: lower the leading tone (7th degree) by semitone
3. For sharpward modulation: raise the fourth degree by semitone
4. All other pitch classes remain fixed
5. To model at the piano: use fixed fingering, with each finger tracking an abstract voice (melodic slot)

# Context & Application

Each step on the circle of keys corresponds to one application of this basic voice leading. Understanding modulation as iterated basic voice leading connects it to the same framework used for chord-to-chord progressions, providing a unified treatment of musical motion at different structural levels. The piano fingering exercise makes this concrete: play a C major scale with fixed fingering, then lower one finger's note to modulate flatward. The tonic shifts from one finger to another, tracking how scale-degree roles change (p. 363).

# Examples

**Example 1** (p. 358, Fig. P8.2): The spiral diagram for diatonic scales in chromatic space, showing the basic voice leading **T**_{-7}t_4.

**Example 2** (p. 364, Fig. 8.1.7): Two paths between C major and D-flat/C-sharp major -- five flatward steps (150 degrees clockwise) vs. seven sharpward steps (210 degrees counterclockwise).

**Example 3** (p. 363): Playing scales at the piano with fixed fingering: each flatward modulation lowers the note played by the fourth right-hand finger while the tonic shifts to a new finger.

# Relationships

## Builds Upon
- **Basic voice leading** -- The scalar basic voice leading extends the chordal concept to larger collections
- **Scalar spiral diagram** -- Provides the geometrical framework for calculating and visualizing

## Enables
- **Leading-tone lowering** -- The flatward direction of the basic voice leading
- **Fourth-scale-degree raising** -- The sharpward direction of the basic voice leading
- **Hierarchical transposition** -- The scalar basic voice leading forms one level of hierarchical structure

## Related
- (Related concepts listed above as prerequisites and enabled concepts)

## Contrasts With
- (Contrasts with triadic basic voice leading in that 7 does not divide 12, so repeated moves always change registral position)

# Common Errors

- **Error**: Thinking the basic voice leading connects scales whose tonics are a fifth apart in some abstract sense
  **Correction**: The connection is contrapuntal (minimal voice-leading motion) -- scales that differ by one accidental

# Common Confusions

- **Confusion**: Expecting repeated modulatory moves to return each voice to its starting pitch
  **Clarification**: Because 7 does not divide 12, any repeated modulatory move will return to its starting scale in a new registral position. This is fundamentally different from triadic voice leading (where 3 divides 12), allowing repeating schemas that return each voice to its starting pitch

# Source Reference

Prelude to Chapter 8, pp. 357-359; Chapter 8: Modulation, sections 1-2, pp. 361-371.

# Verification Notes

- Definition source: Direct from Prelude to Ch. 8 and Ch. 8, sections 1-2
- Confidence rationale: Core concept with precise mathematical formulation and multiple examples
- Cross-reference status: Verified against scalar spiral diagram, leading-tone lowering, fourth-scale-degree raising cards
- Re-extraction notes: Re-extracted from v2 card; preserved: **T**_{-7}t_4 formula, 7-sharpward vs. 5-flatward paths, piano fingering exercise, 7-does-not-divide-12 property
