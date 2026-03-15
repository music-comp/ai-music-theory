---
# === CORE IDENTIFICATION ===
concept: Generalized Line Segments
slug: generalized-line-segments

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
pdf_page: 93
section: "3.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "paths in chord space"
  - "voice-leading paths"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - two-note-chord-space
  - boundary-behavior
extends: []
related:
  - chord-progressions-vs-voice-leadings
  - voice-leading-size
  - horizontal-vertical-motion
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How are voice leadings represented geometrically in chord space?"
  - "Why are voice-leading paths called 'generalized' line segments?"
---

# Quick Definition
Voice leadings in chord space are "generalized line segments" that can bounce off mirror boundaries and disappear off twisted edges to reappear on the opposite side. There is a one-to-one correspondence between these paths and voice leadings in pitch-class space.

# Core Definition
In chord space, a voice leading is represented by a generalized line segment — a path that starts at one chord and ends at another, but that may interact with the space's boundaries along the way. These paths can "bounce off" mirror boundaries (corresponding to voices passing through a unison) or "disappear" off one twisted edge to reappear on the other (corresponding to octave equivalence). The term "generalized" distinguishes these from ordinary line segments in Euclidean space, which never interact with boundaries. There is a one-to-one correspondence between generalized line segments and voice leadings in pitch-class space. Infinitely many generalized line segments can connect any two chords, each representing a different voice leading (differing in the pattern of reflections and wrappings).

# Prerequisites
- Two-note chord space and its Mobius strip topology
- Boundary behavior (mirrors and twisted edges)

# Key Properties
1. One-to-one correspondence with voice leadings in pitch-class space
2. Can bounce off mirror boundaries (top/bottom) any number of times
3. Can wrap around through twisted edges (left/right) any number of times
4. The length of the path equals the size of the voice leading
5. Infinitely many generalized line segments connect any two points

# Construction / Recognition
## To Construct:
1. Choose a voice leading between two chords
2. Imagine each voice making a continuous glissando
3. Track the resulting path through chord space, noting boundary interactions
## Algebraic Method:
1. Horizontal component = sum of the pitch-class paths in both voices
2. Vertical component = difference of the paths (first minus second)
3. Contact with any boundary reverses the vertical direction

# Context & Application
Generalized line segments are the fundamental objects of analysis in chord space. They allow voice leadings to be visualized, compared, and measured geometrically. The concept is essential for the analytical applications in Section 3.5 and beyond, where musical passages are plotted as sequences of generalized line segments. The algebraic method for computing horizontal and vertical components provides a systematic alternative to the glissando visualization.

# Examples
**Example 1** (p. 93): (C, E) ->(2,1) (D, F) moves rightward by 2+1=3 units and upward by 2-1=1 unit, a simple line segment with no boundary interaction.
**Example 2** (p. 93): (C, E) ->(-7,-2) (F, D) moves leftward by 9 units, wrapping around through the twisted edge (Figure 3.4.3).
**Example 3** (p. 93): (C, E) ->(5,-2) (F, D) moves 7 units upward but hits the mirror boundary after 4 units, reversing direction for the remaining 3 units.

# Relationships
## Builds Upon
- **two-note-chord-space** — The space in which these paths live
- **boundary-behavior** — The behaviors that make them "generalized"
## Enables
- **voice-leading-size** — Measured as path length
## Related
- **chord-progressions-vs-voice-leadings** — Progressions are point pairs; voice leadings are these paths
- **horizontal-vertical-motion** — Decomposition of line segments into components

# Common Errors
- **Error**: Assuming the shortest generalized line segment is always a straight Euclidean line
  **Correction**: Due to boundary interactions, the shortest path may involve bouncing or wrapping

# Common Confusions
- **Confusion**: Thinking boundary interactions represent musical events
  **Clarification**: Boundary reflections and wrappings are artifacts of the compact representation, not real musical phenomena; the underlying voice leading is smooth and continuous

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.4, pages 92-94.

# Verification Notes
- Definition source: Directly from Section 3.4, especially the explicit statement of one-to-one correspondence
- Confidence rationale: High — formally defined with multiple worked examples
- Cross-reference status: Verified against the algebraic method and Figure 3.4.3
