---
concept: Voice Leading
slug: voice-leading

category: voice-leading
subcategory: geometric-voice-leading
tier: foundational

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Implicit Musical Knowledge"
chapter_number: 1
pdf_page: 1
section: null

extraction_confidence: high

aliases:
  - voice-leading geometry

prerequisites: []
extends: []
related:
  - pitch-space-and-voice-leading-geometry
  - spiral-diagrams-for-chord-space
  - transposition-along-a-collection
  - lp-and-pl-voice-leading
  - efficient-voice-leading
contrasts_with: []

answers_questions:
  - "What is voice-leading geometry, and how does it represent musical relationships spatially?"
  - "How does voice leading connect harmony and melody?"
---

# Quick Definition

The manner in which individual voices move from one chord to the next, understood geometrically as paths through higher-dimensional chord spaces where each voice is represented by its own dimension.

# Core Definition

Voice leading describes the mapping between the notes of successive chords, specifying how each note moves to a note in the following chord. "The theory of voice leading, pioneered by Richard Cohn, uses geometry and mathematics to elucidate the connection between harmony and melody, revealing fascinating structure in specific pieces, in the work of individual composers, and across genres" (p. 3). In Tymoczko's spiral diagram framework, voice leadings are decomposed into combinations of transposition along the chord (t_x) and transposition along the scale (T_y), where efficient voice leadings result from these two operations nearly counteracting each other.

# Prerequisites

This is a foundational concept with no prerequisites within this source.

# Key Properties

1. Voice leading is the fundamental link between harmony and counterpoint
2. It can be modeled geometrically, with each voice as a dimension
3. Voice leadings decompose into chordal transposition (t_x) and scalar transposition (T_y)
4. Efficient voice leading results when t_x and T_y nearly counteract each other
5. In rock music, voice leading is partially constructed by the listener from strummed chords, unlike classical notation
6. Surface voices (concrete melodies) must be distinguished from abstract voices (scale degrees)

# Construction / Recognition

## To Analyze Voice Leading:
1. Identify which note in chord 1 maps to which note in chord 2
2. Determine the interval each voice travels
3. On spiral diagrams, decompose the path into sliding (T_y) and looping (t_x) components
4. Assess efficiency by checking whether the center of gravity is approximately preserved

# Context & Application

Voice leading operates differently at different levels of the quadruple hierarchy: at the chord-in-scale level, it connects successive harmonies; at the scale-in-aggregate level, it connects keys (modulation). The book argues that counterpoint is not fundamentally about avoiding parallels or balancing independent lines, but about understanding "the interdependence between harmonic and melodic forces" (p. 33). General linear impulses like the preference for descending stepwise melodies generate different harmonic progressions in different contexts -- modal in one situation, functional in another.

# Examples

**Example 1** (p. 50, Figure 2.1.4): The LP voice leading from C major to E major: C moves to B, E stays fixed, G moves to G# (T_4 t_{-1}).

**Example 2** (p. 51, Figure 2.1.5): The PL voice leading (reverse): G# to G, E fixed, B to C (T_{-4} t_1).

**Example 3** (pp. 4-5, Figure 1.1.1): Gesualdo's trick as a voice-leading pattern with two parallel voices and one alternating.

# Relationships

## Enables
- **Spiral Diagrams for Chord Space** -- Voice leadings are represented as paths on spiral diagrams
- **Efficient Voice Leading** -- A key property of voice leading studied through geometry
- **LP and PL Voice Leading** -- Specific voice-leading types named in neo-Riemannian theory

## Related
- **Transposition Along a Collection** -- Voice leadings decompose into transpositions along chord and scale
- **Pitch Space and Voice-Leading Geometry** -- The geometric framework for studying voice leading

# Common Errors

- **Error**: Assuming voice leading always involves literal stepwise motion in every voice
  **Correction**: Efficient voice leading minimizes total voice displacement, which may involve some voices holding while others move by step or leap

# Common Confusions

- **Confusion**: Thinking voice leading in rock works the same as in classical music
  **Clarification**: In rock, voice leading is partially "constructed by the listener" from strummed chords rather than embodied in written notation (p. 65)

- **Confusion**: Equating voice leading with "avoiding parallel fifths"
  **Clarification**: Voice leading is a broader principle about the interdependence of harmony and melody; parallel-avoidance is one historical manifestation

# Source Reference

Chapter 1, pp. 3-4; detailed treatment throughout Chapters 1-2 and Preludes. The theory is attributed to Richard Cohn, with geometrical formalization by Tymoczko.

# Verification Notes

- Definition source: Direct from p. 3, with spiral diagram formalization from Prelude and Ch. 2
- Confidence: HIGH -- central concept of the entire book
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: note about rock voice leading being listener-constructed
