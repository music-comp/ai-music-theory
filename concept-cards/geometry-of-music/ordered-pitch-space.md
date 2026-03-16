---
concept: Ordered Pitch Space
slug: ordered-pitch-space

category: geometric-theory
subcategory: chord-spaces
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 83
section: "3.1"

extraction_confidence: high

aliases:
  - "two-dimensional pitch space"
  - "2D ordered pitch space"

prerequisites:
  - basic-musical-objects
extends: []
related:
  - two-note-chord-space
  - wallpaper-periodicity
  - voice-leading-as-line-segment
contrasts_with:
  - circular-pitch-class-space

answers_questions:
  - "How can pairs of pitches be represented geometrically?"
  - "What is the difference between representing two notes as two points in one dimension versus one point in two dimensions?"
---

# Quick Definition
A two-dimensional geometric space in which each point represents an ordered pair of pitches, with one axis for each voice. Voice leadings appear as line segments in this plane.

# Core Definition
Ordered pitch space is the foundational geometric construction from which Tymoczko derives the higher-dimensional chord spaces. An ordered pair of notes (x, y) is represented as a single point in a two-dimensional plane, where the horizontal axis represents the first pitch and the vertical axis represents the second. Each voice has one "degree of freedom," and a two-voice texture therefore requires two dimensions. This space is infinite and continuous, containing every conceivable pair of pitches. Voice leadings between ordered pairs correspond to line segments in the plane, with horizontal lines representing motion in just the first voice, vertical lines representing motion in just the second, and diagonal lines representing simultaneous motion in both voices.

# Prerequisites
- Basic musical objects (ordered sequences of pitches) from Chapter 2
- Familiarity with coordinate geometry (x-y plane)

# Key Properties
1. The space is infinite and two-dimensional, with one axis per voice
2. After rotating 45 degrees clockwise, horizontal motion represents parallel motion and vertical motion represents contrary motion
3. The space exhibits wallpaper periodicity when octave equivalence is considered
4. Oblique motion (one voice stationary) lies along 45-degree diagonals in the rotated space

# Construction / Recognition
## To Construct:
1. Assign the first pitch of the ordered pair to the horizontal axis
2. Assign the second pitch to the vertical axis
3. Plot the point at their intersection
4. Rotate 45 degrees clockwise so parallel motion becomes horizontal
## To Recognize:
1. Any two-dimensional plane with axes representing individual pitches is an ordered pitch space
2. Voice leadings appear as directed line segments between points

# Context & Application
This is the starting point for the chapter's geometric program. By subsequently "folding" this space to ignore octave and order, Tymoczko derives the Mobius strip topology of two-note chord space. The rotated version is particularly useful because it separates the parallel (transpositional) and contrary components of voice leading into orthogonal directions.

# Examples
**Example 1** (p. 84): The ordered pair (C4, E4) is represented either as two colored circles on a one-dimensional line or as a single point in two-dimensional space (Figure 3.1.1).
**Example 2** (p. 84): The voice leading (C4, E4) -> (E4, C4), in which voices trade notes, is represented as a line segment in the plane (Figure 3.1.2).
**Example 3** (p. 85): A passage from Josquin's Missa l'homme arme is plotted as a series of line segments (Figure 3.1.3).

# Relationships
## Builds Upon
- **basic-musical-objects** — Ordered sequences of pitches from Chapter 2
## Enables
- **wallpaper-periodicity** — The periodic structure of ordered pitch space
- **two-note-chord-space** — Formed by "folding" ordered pitch space
## Related
- **voice-leading-as-line-segment** — Voice leadings are line segments in this space
## Contrasts With
- **circular-pitch-class-space** — The one-dimensional circular model from Chapter 2

# Common Errors
- **Error**: Thinking the rotation changes the space itself
  **Correction**: The 45-degree rotation is purely a change of perspective; the space is identical, but parallel and contrary motion become easier to see

# Common Confusions
- **Confusion**: Confusing ordered pitch space with chord space
  **Clarification**: Ordered pitch space distinguishes (C, E) from (E, C) and treats pitches in different octaves as different points. Chord space ignores both order and octave.

# Source Reference
Chapter 3: A Geometry of Chords, Section 3.1, pages 83-86.

# Verification Notes
- Definition source: Directly from Section 3.1 opening discussion
- Confidence rationale: High — explicitly described and illustrated with multiple figures
- Cross-reference status: Verified as foundational to all subsequent chord space constructions
