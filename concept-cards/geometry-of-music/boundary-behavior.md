---
# === CORE IDENTIFICATION ===
concept: Boundary Behavior in Chord Space
slug: boundary-behavior

# === CLASSIFICATION ===
category: geometric-theory
subcategory: chord-spaces
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 92
section: "3.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "mirror boundaries"
  - "twisted edges"
  - "boundary conditions"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - two-note-chord-space
  - mobius-strip-topology
extends: []
related:
  - generalized-line-segments
  - chord-progressions-vs-voice-leadings
  - parable-of-the-ant
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What happens when a voice leading reaches the edge of chord space?"
  - "Why do some voice leadings appear to bounce?"
  - "What is the difference between mirror boundaries and twisted edges?"
---

# Quick Definition
The two types of edge behavior in chord space: horizontal edges act as mirrors (voice leadings "bounce" off them), while vertical edges are glued with a twist (voice leadings disappear off one side and reappear on the other, shifted vertically).

# Core Definition
Chord space has two distinct types of boundary behavior, both inherited from the wallpaper structure. The horizontal boundaries (top and bottom of the Mobius strip) act as mirrors: a voice leading reaching this boundary appears to reflect, changing the direction of its vertical component while preserving its horizontal component. Musically, this occurs when one voice passes through a unison with the other — what was ascending contrary motion becomes descending contrary motion. The vertical boundaries (left and right edges) are identified with a twist: a voice leading exiting the right edge reappears at a vertically reflected position on the left edge, and vice versa. Contact with any of the four "edges" exchanges "upward" and "downward" directions. These boundary behaviors encode the fundamental symmetries of musical chord space.

# Prerequisites
- Two-note chord space (Mobius strip)
- Voice leadings as line segments in the space

# Key Properties
1. Horizontal boundaries are mirrors — voice leadings reflect off them
2. Vertical boundaries are identified with a twist — voice leadings wrap around
3. Mirror reflection corresponds to two voices passing through a unison
4. The twisted identification corresponds to octave equivalence combined with order equivalence
5. Contact with any boundary reverses the vertical direction (upward becomes downward)
6. Infinitely many generalized line segments can connect any two points, differing by boundary interactions

# Construction / Recognition
## To Construct:
1. Draw a voice leading as a straight line in the Mobius strip
2. When the line reaches a horizontal boundary, reflect it (like a billiard ball)
3. When the line reaches a vertical boundary, continue it from the opposite side at the reflected vertical position
## To Recognize:
1. A voice leading that appears to "bounce" involves a mirror boundary interaction
2. A voice leading that "teleports" across the strip involves a twisted-edge interaction

# Context & Application
Understanding boundary behavior is essential for correctly plotting voice leadings in chord space and for finding all possible voice leadings between two chords. The mirror boundaries are particularly significant because they correspond to the special musical event of voice crossing (or voice convergence to a unison). The twisted edges are important for understanding why certain voice leadings wrap around the space. These behaviors generalize to higher-dimensional chord spaces, where boundaries of the prism (in 3D) or higher-dimensional analogues play corresponding roles.

# Examples
**Example 1** (p. 92): The smooth glissando from (Eb, G) to (F, A) exits the upper-right edge and reappears on the lower-left, demonstrating the twist (Figure 3.4.2).
**Example 2** (p. 93): The glissando from (C, D) to (E, D) — where only one voice moves — bounces off the upper mirror boundary at the unison {D, D}, appearing to change direction (Figure 3.4.2).
**Example 3** (p. 93-94): Figure 3.4.3 shows four different voice leadings between {C, E} and {D, F}, each involving different patterns of boundary interactions.

# Relationships
## Builds Upon
- **two-note-chord-space** — The space whose boundaries we are describing
- **mobius-strip-topology** — The topology that produces these boundaries
## Enables
- **generalized-line-segments** — Paths that interact with boundaries
- **voice-leading-size** — Size relates to path length including boundary interactions
## Related
- **parable-of-the-ant** — Where these behaviors were first introduced intuitively

# Common Errors
- **Error**: Thinking a voice leading that bounces off a mirror involves an actual change in musical direction
  **Correction**: The bounce is an artifact of the representation; musically, the voice continues smoothly in the same direction

# Common Confusions
- **Confusion**: Confusing mirror boundaries with twisted edges
  **Clarification**: Mirror boundaries (horizontal) cause reflection; twisted edges (vertical) cause wrap-around with vertical shift. They correspond to different musical symmetries: unison crossing vs. octave equivalence.

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.4, pages 92-94.

# Verification Notes
- Definition source: Directly from Section 3.4, especially the discussion of Figures 3.4.2 and 3.4.3
- Confidence rationale: High — extensively illustrated with multiple examples
- Cross-reference status: Verified against Parable of the Ant (Section 3.2) and three-note space (Section 3.8)
