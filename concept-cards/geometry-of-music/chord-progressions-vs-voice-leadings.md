---
# === CORE IDENTIFICATION ===
concept: Chord Progressions vs Voice Leadings
slug: chord-progressions-vs-voice-leadings

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
pdf_page: 91
section: "3.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "progression vs voice leading"
  - "teleportation vs path"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - two-note-chord-space
extends:
  - voice-leading-definition
related:
  - generalized-line-segments
  - voice-leading-size
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the geometric difference between a chord progression and a voice leading?"
  - "Why can the same chord progression be realized by many different voice leadings?"
---

# Quick Definition
In chord space, a chord progression is a pair of points (no specific path between them), while a voice leading is a specific path (generalized line segment) connecting those points. A progression specifies "where" but not "how"; a voice leading specifies both.

# Core Definition
A chord progression such as {C, E} => {F, A} is represented geometrically by simply identifying the initial and final points in chord space. It does not specify how the individual notes move from one chord to the next — "it is as if the music magically teleports, disappearing at {C, E} and instantaneously reappearing at {F, A}." A voice leading, by contrast, specifies a particular path between the two points, corresponding to continuous glissandi in each voice. The size of the voice leading corresponds to the length of the path. Because there can be infinitely many paths between any two points (differing in their interactions with the boundaries), the same chord progression can be realized by infinitely many different voice leadings.

# Prerequisites
- Two-note chord space as a Mobius strip
- Concept of voice leading from Chapter 2

# Key Properties
1. A chord progression is an unordered pair of points in chord space
2. A voice leading is a directed path (generalized line segment) between two points
3. The same two points can be connected by infinitely many different paths
4. The length of a path corresponds to the size of the voice leading
5. The shortest path represents the most efficient voice leading

# Construction / Recognition
## To Recognize a Chord Progression:
1. Identify two chords (two points in chord space)
2. No path between them is specified
## To Recognize a Voice Leading:
1. Identify two chords AND the specific motion of each voice
2. This determines a unique path (generalized line segment) in the space

# Context & Application
This distinction is central to Tymoczko's framework. Many traditional music-theory discussions conflate progressions with voice leadings, but the geometric model makes the distinction vivid and precise. The distinction is especially important when discussing efficient voice leading: a chord progression merely names two chords, whereas efficient voice leading requires finding a short path between them.

# Examples
**Example 1** (p. 91): The progression {C, E} => {F, A} is just two points. The voice leading (C, E) -> (Eb, G) moves each voice up by three semitones and is represented by a horizontal line segment (Figure 3.4.1).
**Example 2** (p. 94): Figure 3.4.3 shows four distinct voice leadings between {C, E} and {D, F}, each following a different path through chord space, with different boundary interactions.

# Relationships
## Builds Upon
- **two-note-chord-space** — The space in which this distinction is visualized
## Enables
- **generalized-line-segments** — The paths that represent voice leadings
- **voice-leading-size** — Measured by path length
## Related
- **voice-leading-definition** — The Chapter 2 definition that is here geometrized

# Common Errors
- **Error**: Treating a chord progression as if it uniquely determines a voice leading
  **Correction**: A single progression can be realized by infinitely many voice leadings

# Common Confusions
- **Confusion**: Thinking the "shortest" path is always the obvious one
  **Clarification**: Due to the Mobius strip topology, the shortest path may involve wrapping around or bouncing off boundaries, producing voice leadings that are not immediately obvious

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.4, pages 91-94.

# Verification Notes
- Definition source: Directly from the opening of Section 3.4
- Confidence rationale: High — explicitly stated distinction with clear geometric interpretation
- Cross-reference status: Verified against Chapter 2 definitions
