---
# === CORE IDENTIFICATION ===
concept: Distance in Music
slug: distance-in-music

# === CLASSIFICATION ===
category: geometric-theory
subcategory: measurement
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Harmony and Voice Leading"
chapter_number: 2
pdf_page: 46
section: "2.1-2.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "musical distance"
  - "intervallic distance"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-space
extends: []
related:
  - pitch-class-space
  - transposition
  - inversion
  - voice-leading-size
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How is distance measured in music?"
  - "What is pitch space vs. pitch-class space?"
---

# Quick Definition
Musical distance is measured in semitones using subtraction in pitch space (|p - q|) or shortest path in pitch-class space, converting the multiplicative frequency ratios of acoustics into additive differences.

# Core Definition
Musicians are primarily sensitive not to absolute frequencies but to the ratios between them. The logarithmic mapping from frequency to pitch space converts these ratios into distances measured by subtraction: the distance between pitches p and q is |p - q| semitones. In pitch-class space, distance is defined as the shortest distance between any two pitches belonging to those pitch classes (always between 0 and 6 semitones). Transposition and inversion are the only distance-preserving transformations of these spaces, making distance a central organizing concept for music theory.

# Prerequisites
- **pitch-space** — Distance is first defined in pitch space

# Key Properties
1. In pitch space: distance = |p - q| (subtraction)
2. In pitch-class space: distance = shortest path around the circle (0 to 6 semitones)
3. Measured in semitones (continuous, not just integers)
4. Transposition preserves distances; inversion preserves distances
5. Listeners are more attuned to distances between notes than to absolute positions
6. Distance is fundamental to voice-leading measurement

# Construction / Recognition
## To Construct/Create:
1. Assign numerical values to pitches using the logarithmic mapping
2. Compute distance by subtraction (pitch space) or shortest path (pitch-class space)
## To Identify/Recognize:
1. Large distance = large interval; small distance = small interval
2. Zero distance = same pitch (or same pitch class)

# Context & Application
Distance is the foundation for measuring voice-leading size, comparing transpositions, and defining the geometry of chord spaces. The emphasis on distance-preserving transformations (transposition and inversion) motivates the OPTIC symmetry framework.

# Examples
**Example 1** (p. 47): A whistler transposes a tune from frequencies f, g, h to cf, cg, ch, preserving frequency ratios and hence pitch-space distances.

**Example 2** (p. 49): "Pitch class E is four semitones away from C" — the shortest path between these two pitch classes on the circle.

# Relationships
## Builds Upon
- **pitch-space** — The space in which distance is first defined
## Enables
- **transposition** — A distance-preserving transformation
- **inversion** — A distance-preserving transformation
- **voice-leading-size** — Measured using distances between individual voices
## Related
- **pitch-class-space** — Provides a second notion of distance (on the circle)
## Contrasts With
- No direct contrast within this source

# Common Errors
- **Error**: Using frequency difference rather than frequency ratio as a measure of musical distance
  **Correction**: Musical distance corresponds to frequency ratios, which become additive differences after logarithmic mapping

# Common Confusions
- **Confusion**: Confusing pitch-space distance with pitch-class distance
  **Clarification**: Pitch-space distance can be any positive real number; pitch-class distance is always between 0 and 6 semitones

# Source Reference
Chapter 2: Harmony and Voice Leading, Sections 2.1-2.2, pages 47-51.

# Verification Notes
- Definition source: Direct from Sections 2.1-2.2
- Confidence rationale: High — precisely defined mathematically
- Cross-reference status: Verified; foundational to all subsequent discussions
