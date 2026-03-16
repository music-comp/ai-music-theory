---
concept: Spiral Diagrams for Chord Space
slug: spiral-diagrams-for-chord-space

category: voice-leading
subcategory: geometric-voice-leading
tier: foundational

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Prelude: Transposition Along a Collection"
chapter_number: null
pdf_page: 37
section: null

extraction_confidence: high

aliases:
  - spiral diagram
  - n-in-o graph

prerequisites:
  - transposition-along-a-collection
extends:
  - pitch-space-and-voice-leading-geometry
related:
  - slide-along-spiral
  - loop-on-spiral
  - radial-motion-on-spiral
  - spiral-construction-recipe
contrasts_with: []

answers_questions:
  - "What is voice-leading geometry, and how does it represent musical relationships spatially?"
  - "How do spiral diagrams represent chord spaces?"
---

# Quick Definition

A two-dimensional geometrical representation of chord space in which each point represents an entire chord, with three types of motion (slide, loop, radial) corresponding to different musical operations and voice leadings.

# Core Definition

A spiral diagram for an n-note chord in an o-note scale has n loops with o equally spaced points. Construction: (A) Draw a spiral with n loops, attaching end to beginning, starting clockwise at 12 o'clock. (B) Mark o equally spaced points by dividing the circle into o equal slices and placing a point at every nth slice border (pp. 42-43). "Each point represents an entire collection of notes, a complete major chord or complete diatonic scale" (p. 43). A chord's angular position corresponds to the sum of its pitch classes (mod scale size). Three rules govern motion: (1) sliding = transposition along the scale; (2) loops = transposition along the chord; (3) two paths represent the same voice leading when they share endpoints and total angular motion (pp. 44-45).

# Prerequisites

- **Transposition Along a Collection** -- The spiral represents the interaction of two levels of transposition

# Key Properties

1. Points represent entire chords, not single notes
2. Number of loops = chord size; number of points = scale size
3. Clockwise = descending; counterclockwise = ascending
4. Angular position = sum of pitch classes (mod scale size)
5. Self-intersection is a 2D projection artifact
6. Slides correspond to T_y; loops correspond to t_x
7. Every path decomposes into slides and loops

# Construction / Recognition

## To Construct:
1. Draw a spiral with n loops (n = chord size)
2. Divide the circle into o equal slices (o = scale size)
3. Place points at every nth slice border, starting at 12 o'clock
4. Label with consecutive scale tones, descending clockwise

## To Use:
1. Locate two chords as points
2. Trace a path between them along the spiral
3. Count touched chords (not counting start) for T_x
4. Count times you pass your initial angular position for t_y
5. The voice leading is T_x t_{-y} or T_{-x} t_y

# Context & Application

The spiral diagrams function as "the music-theoretical equivalent of a consumer product, allowing readers to enjoy the benefits of sophisticated musical geometry without mastering its details -- much as we can use GPS without fully understanding the general-relativistic calculations it performs" (p. 46). The book uses: 3-in-12 (rock harmony), 2-in-7 and 3-in-7 (early music), 7-in-12 (modulation), and 2-in-3 (melody within chords).

# Examples

**Example 1** (p. 43, Figure P2.10): Major triads in chromatic space (3-in-12) and diatonic scales in chromatic space (7-in-12).

**Example 2** (p. 42, Figure P2.9): Spirals for 1, 2, 3, and 7-note chords.

**Example 3** (p. 45, Figure P2.11): Three rules illustrated -- path a (slide), path b (loop), paths c and d (same voice leading, different paths).

# Relationships

## Builds Upon
- **Pitch Space and Voice-Leading Geometry** -- Spiral diagrams simplify higher-dimensional geometry
- **Transposition Along a Collection** -- Slides and loops correspond to the two types of transposition

## Enables
- **Slide Along Spiral** -- Rule 1 for navigating the space
- **Loop on Spiral** -- Rule 2 for navigating the space
- **Radial Motion on Spiral** -- Combining slides and loops produces radial paths

# Common Errors

- **Error**: Interpreting points as single notes
  **Correction**: Each point represents an entire chord; Tymoczko repeatedly emphasizes this

# Common Confusions

- **Confusion**: Being troubled by the spiral line crossing itself
  **Clarification**: Self-intersection is a 2D artifact; the actual geometry does not self-intersect (appendix 2)

# Source Reference

Prelude to Chapter 2, pp. 42-46, Figures P2.9-P2.11. Used throughout the remainder of the book.

# Verification Notes

- Definition source: Direct from pp. 42-45
- Confidence: HIGH -- central visual tool of the book, explicitly constructed
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: GPS analogy, self-intersection clarification, complete construction recipe
