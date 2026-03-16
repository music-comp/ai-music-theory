---
concept: Voice-Leading Size
slug: voice-leading-size

category: voice-leading
subcategory: measurement
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 91
section: "3.4"

extraction_confidence: high

aliases:
  - "voice-leading distance"
  - "path length in chord space"

prerequisites:
  - generalized-line-segments
  - two-note-chord-space
extends:
  - efficient-voice-leading
related:
  - harmonic-consistency-and-efficient-voice-leading
  - nearly-even-chords
contrasts_with: []

answers_questions:
  - "How is the size of a voice leading measured geometrically?"
  - "What determines how 'efficient' a voice leading is?"
---

# Quick Definition
The size of a voice leading corresponds to the length of its generalized line segment in chord space. Shorter paths represent more efficient voice leadings, where individual voices move smaller distances.

# Core Definition
Voice-leading size is measured by the length of the generalized line segment representing the voice leading in chord space. This corresponds to the total amount of motion in all voices combined. For a "reasonable" measure of voice-leading size, removing voice crossings never makes the voice leading larger, and the most efficient voice leading between any two chords can always be found among the crossing-free (interscalar transposition) options. The geometric representation makes efficiency comparisons visual: shorter line segments mean smaller voice leadings, and the global structure of chord space reveals which chord pairs can be connected by short paths.

# Prerequisites
- Generalized line segments as representations of voice leadings
- Two-note chord space

# Key Properties
1. Size = length of the generalized line segment in chord space
2. Shorter paths = more efficient voice leadings
3. Removing voice crossings never increases voice-leading size
4. The most efficient voice leading is always crossing-free
5. Different paths between the same two points have different sizes

# Construction / Recognition
## To Measure:
1. Plot the voice leading as a generalized line segment
2. Calculate the total length of the path, including any boundary interactions
3. Alternatively, sum the absolute values of the individual voice motions (for common measures)

# Context & Application
Voice-leading size is the fundamental quantity that connects chord space geometry to musical practice. Composers who seek "efficient" voice leading are implicitly searching for short paths in chord space. The geometric perspective reveals that nearly even chords (near the center of the space) have the most efficient connections to their transpositions, explaining why major thirds, perfect fifths, and similar consonant intervals are so useful in Western counterpoint.

# Examples
**Example 1** (p. 91-92): The voice leading (C, E) -> (Eb, G), which moves each voice up by 3 semitones, has a specific path length represented by a horizontal line segment.
**Example 2** (p. 93-94): The four voice leadings in Figure 3.4.3 connect the same two chords but have different path lengths, showing that some realizations are more efficient than others.

# Relationships
## Builds Upon
- **generalized-line-segments** — Size is the length of these paths
- **two-note-chord-space** — The space in which measurement occurs
## Enables
- **harmonic-consistency-and-efficient-voice-leading** — The central question of combining these two constraints
## Related
- **nearly-even-chords** — These have the shortest connections to their transpositions
- **efficient-voice-leading** — Defined as small voice-leading size

# Common Errors
- **Error**: Assuming all paths between two chords have the same length
  **Correction**: Different voice leadings between the same two chords can have very different sizes

# Common Confusions
- **Confusion**: Confusing voice-leading size with the number of voices that move
  **Clarification**: Size measures total distance moved, not how many voices participate; a large motion in one voice can be "larger" than small motions in many voices

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.4, pages 91-94.

# Verification Notes
- Definition source: From Section 3.4 discussion of path length
- Confidence rationale: High — geometric definition directly stated
- Cross-reference status: Verified against Chapter 2 discussion of voice-leading size
