---
concept: Wallpaper Periodicity
slug: wallpaper-periodicity

category: geometric-theory
subcategory: chord-spaces
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 86
section: "3.1"

extraction_confidence: high

aliases:
  - "periodic tiling"
  - "musical wallpaper"

prerequisites:
  - ordered-pitch-space
extends: []
related:
  - parable-of-the-ant
  - two-note-chord-space
  - three-note-chord-space
contrasts_with: []

answers_questions:
  - "Why does ordered pitch space have a repeating structure?"
  - "How does octave equivalence create periodicity in pitch space?"
  - "What is the relationship between adjacent tiles in musical space?"
---

# Quick Definition
The repeating, tile-like structure of ordered pitch space, where each tile contains exactly one representative of every unordered set of pitch classes, analogous to a wallpaper pattern.

# Core Definition
When octave equivalence is considered, ordered pitch space exhibits a periodic structure: like wallpaper, it consists of a single pattern (or "tile") repeated to cover the infinite plane. Each tile contains precisely one point for every unordered pair of pitch classes. Adjacent tiles in the diagonal direction are related by octave transposition in one voice. Adjacent tiles related by reflection across their shared border represent pairs with the same pitch content but in reversed order. This periodicity is the key insight that enables "folding" the infinite space into a compact chord space by identifying equivalent points across tiles.

# Prerequisites
- Ordered pitch space and its coordinate system
- Octave equivalence as a musical concept

# Key Properties
1. The space consists of identical tiles, each containing one representative of every two-note chord
2. Diagonal neighbors are related by octave transposition in one voice
3. Reflected neighbors (across horizontal or vertical shared borders) contain the same notes in reversed order
4. The pattern can be symbolized using a right-side-up face: adjacent tiles may be upside-down or mirrored
5. Any single tile suffices to represent all possible two-note chords

# Construction / Recognition
## To Construct:
1. Draw the extended two-dimensional ordered pitch space (Figure 3.1.5)
2. Identify the repeating unit by finding four tiles that cover all pitch-class combinations
3. Note the reflection and translational symmetries between tiles
## To Recognize:
1. Any space showing repeating identical regions where movement between tiles corresponds to octave shifts or order reversal

# Context & Application
Wallpaper periodicity is the conceptual bridge between the infinite ordered pitch space and the compact Mobius strip of chord space. Understanding the wallpaper pattern is essential for grasping why chord space has the particular topology it does. The concept generalizes to higher dimensions: three-note chord space is a three-dimensional periodic tiling, and so on.

# Examples
**Example 1** (p. 86): Figure 3.1.5 shows four complete tiles of two-dimensional ordered pitch space, with the lower-left and upper-right related by octave transposition and the lower-left and upper-left related by reflection (order reversal).
**Example 2** (p. 86-87): Figure 3.1.6 uses a human face to symbolize the tile relationships: the face appears right-side-up, upside-down, and mirror-reflected in different tiles.

# Relationships
## Builds Upon
- **ordered-pitch-space** — The wallpaper pattern is a property of ordered pitch space
## Enables
- **parable-of-the-ant** — The ant walks on this wallpaper
- **two-note-chord-space** — Formed by "folding" the wallpaper into a single tile
## Related
- **three-note-chord-space** — Higher-dimensional analogue with 3D "tiles"

# Common Errors
- **Error**: Thinking adjacent tiles are identical copies
  **Correction**: Adjacent tiles are related by specific transformations (octave shift or order reversal), not simple translation

# Common Confusions
- **Confusion**: Confusing the wallpaper's periodicity with simple translational symmetry
  **Clarification**: The periodicity involves both translations (octave shifts) and reflections (order reversal), which is why the resulting chord space is a Mobius strip rather than a torus

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.1, pages 86-87.

# Verification Notes
- Definition source: Directly from the extended discussion in Section 3.1 and the face metaphor
- Confidence rationale: High — central concept with detailed figures and explanation
- Cross-reference status: Verified against Parable of the Ant (Section 3.2)
