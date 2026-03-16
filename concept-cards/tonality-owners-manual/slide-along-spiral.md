---
# === CORE IDENTIFICATION ===
concept: Slide Along Spiral
slug: slide-along-spiral

# === CLASSIFICATION ===
category: voice-leading
subcategory: geometric-voice-leading
tier: intermediate

# === PROVENANCE ===
source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Prelude: Transposition Along a Collection"
chapter_number: null
pdf_page: 37
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - sliding along the spiral

# === TYPED RELATIONSHIPS ===
prerequisites:
  - spiral-diagrams-for-chord-space
extends: []
related:
  - loop-on-spiral
  - radial-motion-on-spiral
  - transposition-along-a-collection
contrasts_with:
  - loop-on-spiral

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What does sliding along the spiral represent musically?"
---

# Quick Definition

Movement along the spiral path itself, corresponding to transposition along the larger scale-like collection (T_y), shifting all of a chord's notes by the same number of scale steps.

# Core Definition

"Sliding along the spiral corresponds to transposition along the larger scale-like collection, shifting the chord's notes upward or downward by the same number of scale steps. In this book clockwise motion descends and counterclockwise motion ascends. One needs to move n slices clockwise to transpose an n-note chord down by scale step" (p. 44). This is Rule 1 of the three rules for navigating the spiral.

# Prerequisites

- **Spiral Diagrams for Chord Space** -- Understanding the spatial framework

# Key Properties

1. Corresponds to transposition along the enclosing scale (T_y)
2. Clockwise = descending; counterclockwise = ascending
3. n slices clockwise = one scale step down for an n-note chord
4. Sliding alone does not produce efficient voice leading between nearby chords
5. Every path on the spiral can be decomposed into slides and loops

# Construction / Recognition

## To Slide:
1. Start at a point on the spiral
2. Move along the spiral line itself (not jumping between rings)
3. Count touched points (not counting start) to determine the transposition T_x
4. Clockwise = negative (descending); counterclockwise = positive (ascending)

# Context & Application

Sliding represents chromatic or scalar transposition -- parallel motion of all voices by the same interval. For the 3-in-12 major triad diagram, a one-step clockwise slide (**T**_{-1}) transposes all notes down by one chromatic semitone. Slides are the simpler of the two operations; the musically interesting voice leadings arise from combining slides with loops.

# Examples

**Example 1** (p. 44, Figure P2.11, path a): Sliding along the spiral transposes the chord along the scale.

**Example 2** (p. 52, Figure 2.1.6): Two-step clockwise motion from C to Bb lowers each voice by major second; three intermediate chords (B, Eb, G) lie between them.

# Relationships

## Builds Upon
- **Spiral Diagrams for Chord Space** -- Slides are one of three operations on spiral diagrams

## Related
- **Transposition Along a Collection** -- Sliding = transposition along the enclosing scale

## Contrasts With
- **Loop on Spiral** -- Loops transpose along the chord; slides transpose along the scale

# Common Errors

- **Error**: Thinking a slide produces efficient voice leading between nearby chords
  **Correction**: Efficient voice leading requires combining a slide with a loop so they partially cancel

# Common Confusions

- **Confusion**: Thinking sliding and looping are independent alternatives
  **Clarification**: Most musically significant voice leadings combine both; Rule 3 says every path can be decomposed into slides and loops

# Source Reference

Prelude to Chapter 2, Rule 1, p. 44, Figure P2.11 path a.

# Verification Notes

- Definition source: Direct quotation from p. 44
- Confidence: HIGH -- explicitly stated as Rule 1
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: all key properties
