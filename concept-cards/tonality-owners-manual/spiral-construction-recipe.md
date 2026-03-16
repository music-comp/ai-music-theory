---
# === CORE IDENTIFICATION ===
concept: Spiral Construction Recipe
slug: spiral-construction-recipe

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
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - transposition-along-a-collection
  - spiral-diagrams-for-chord-space
extends: []
related:
  - slide-along-spiral
  - loop-on-spiral
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do you construct a spiral diagram for any chord in any scale?"
---

# Quick Definition

A step-by-step procedure for constructing a spiral diagram to represent any n-note chord in any o-note scale, applicable to any hierarchical level of musical structure.

# Core Definition

The recipe constructs "a simple and intuitive geometry of hierarchically nested transposition, equally applicable to voice in chord, chord in scale, and scale in chromatic aggregate" (p. 42). Step A: Draw a spiral with n loops (n = chord size), starting clockwise at 12 o'clock, moving inward to 9 o'clock for the nth time, then connecting end to beginning. Step B: Mark o equally spaced points (o = scale size) by dividing the circle into o equal pie slices and placing a point at every nth slice border. Label with consecutive scale tones, typically C at 12 o'clock, descending clockwise (pp. 42-43).

# Prerequisites

- **Transposition Along a Collection** -- Understanding what the diagram represents
- **Spiral Diagrams for Chord Space** -- Understanding how to read the resulting diagram

# Key Properties

1. Completely general: works for any n-note chord in any o-note scale
2. The number of loops = chord size (n)
3. The number of points = scale size (o)
4. Points placed at every nth slice border
5. Convention: C at innermost 12 o'clock, descending clockwise
6. Requires no mathematics beyond counting

# Construction / Recognition

## To Construct:
1. Determine n (chord size) and o (scale size)
2. Draw a spiral with n loops, connecting end to beginning
3. Divide the circle into o equal pie slices
4. Place points at every nth slice border, moving along the spiral
5. Label points with consecutive scale tones (C at 12 o'clock, descending clockwise)

## To Verify:
1. Count loops -- should equal chord size
2. Count points -- should equal scale size
3. Check that angular position = sum of pitch classes (mod scale size)

# Context & Application

The recipe is used throughout the book to construct different diagrams: 3-in-12 for chromatic triads (rock), 2-in-7 and 3-in-7 for diatonic dyads and triads (early music), 7-in-12 for diatonic scales (modulation), and 2-in-3 for two-note triadic subsets (melody). "With a little practice, the reader can construct and manipulate the spiral diagrams for any chord in any scale" (p. 46).

# Examples

**Example 1** (p. 43, Figure P2.10 left): 3-in-12 -- major triads in chromatic scale. 3 loops, 12 points at every 3rd clock position.

**Example 2** (p. 43, Figure P2.10 right): 7-in-12 -- diatonic collections in chromatic scale. 7 loops, 12 points at every 7th slice.

**Example 3** (p. 42, Figure P2.9): Spirals for 1, 2, 3, and 7-note chords shown for comparison.

# Relationships

## Builds Upon
- **Spiral Diagrams for Chord Space** -- The recipe produces spiral diagrams

## Enables
- **Slide Along Spiral** -- Navigating the constructed diagram
- **Loop on Spiral** -- Navigating the constructed diagram

# Common Errors

- **Error**: Confusing the number of loops with the number of points
  **Correction**: Loops = chord size; points = scale size

# Common Confusions

- **Confusion**: Thinking the clockwise-descending convention is mathematically necessary
  **Clarification**: It is a convention; both directions are defaults (descending musical motion, clockwise circular motion)

# Source Reference

Prelude to Chapter 2, pp. 42-44, Figures P2.9-P2.10. Described as requiring "no mathematics" and "no particular music-theoretical skill."

# Verification Notes

- Definition source: Direct from pp. 42-43
- Confidence: HIGH -- explicit step-by-step procedure
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: specific diagram examples, "no mathematics" note
