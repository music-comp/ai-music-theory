---
concept: Voicing as Set Class
slug: voicing-as-set-class

category: voice-leading
subcategory: configurations
tier: advanced

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Prelude: The Tinctoris Transform"
chapter_number: null
pdf_page: 87
section: null

extraction_confidence: high

aliases:
  - intrinsic spacing as set class

prerequisites:
  - configuration-and-center-of-gravity
  - intrinsic-vs-enclosing-scale
extends: []
related:
  - tinctoris-transform
contrasts_with: []

answers_questions:
  - "How are voicings formalized as set classes?"
  - "What is the relationship between figured-bass voicings and set theory?"
---

# Quick Definition

A voicing is a pattern of spacing measured in chordal steps along the intrinsic scale, formalizable as a pitch-set set class -- close position is 012, open position is 024, linking figured-bass theory, guitar voicings, and modern set theory.

# Core Definition

"We can formalize the intuitive notion of a voicing as a pattern of spacing in chordal steps: a 'close position' voicing is a voicing in which each note is one chordal step above its lower neighbor (e.g., C4-E4-G4) while an 'open position' voicing places each note two steps above its lower neighbor, as in C4-G4-E5" (p. 94). Any chord "presents multiple set classes simultaneously: a chromatic set class determined by its pitch classes, a diatonic set class determined by the extrinsic scale, and an intrinsic pitch-set set class determined by its voicing" (p. 94). "In different musical situations different sets can be more or less salient, with pitch-class content often more perceptible for small chords in close position, and intrinsic spacing more important as chords grow larger or are voiced in unusual ways" (p. 94).

# Prerequisites

- **Configuration and Center of Gravity** -- Voicings are configurations measured in intrinsic-scale steps
- **Intrinsic vs. Enclosing Scale** -- The intrinsic scale provides the ruler for measuring voicing

# Key Properties

1. Close position = set class 012 (each note one chordal step above neighbor)
2. Open position = set class 024 (each note two chordal steps above neighbor)
3. Every chord presents three set classes: chromatic, diatonic, and intrinsic
4. Pitch-class content is more salient for small close-position chords
5. Intrinsic spacing is more important for large or unusually voiced chords
6. Links figured-bass, guitar voicings ("drop 2"), jazz theory, and set theory

# Construction / Recognition

## To Determine a Voicing's Set Class:
1. Identify the chord's notes
2. Compress to a single octave to form the intrinsic scale
3. Measure each note's position in steps along the intrinsic scale from the bass
4. The resulting numbers form the voicing's set class (e.g., 012, 024)

# Context & Application

The concept connects three historically separate traditions: figured-bass pedagogy (describing hand shapes), guitar manuals (categorizing voicings as "drop 2" etc.), and formal set theory (general vocabulary of musical shapes). All share the factoring out of transposition, leaving relative relationships: "an interval, a figured-bass hand position, a voicing, a 'set class'" (p. 94). The connection was obscured by set theory's indifference to register, lack of concern with voice leading, and assumption of chromatic space.

# Examples

**Example 1** (p. 94, Figure P3.10): Bill Evans's "So What" chord and Schoenberg's "Farben" chord are both voiced as open-position pentachords (024 in intrinsic steps), despite very different pitch-class content.

# Relationships

## Builds Upon
- **Configuration and Center of Gravity** -- Voicings are a specific type of configuration
- **Intrinsic vs. Enclosing Scale** -- The intrinsic scale provides the measurement ruler

## Related
- **Tinctoris Transform** -- Groups progressions by shared voicings/configurations

# Common Errors

- **Error**: Describing "close" and "open" position as vague register descriptions
  **Correction**: They are specific set classes (012 and 024 for triads) measured in intrinsic-scale steps

# Common Confusions

- **Confusion**: Thinking set theory only deals with pitch-class content
  **Clarification**: Voicings add registral information through intrinsic spacing, complementing pitch-class set classes

# Source Reference

Prelude to Chapter 3, pp. 94-95, Figure P3.10.

# Verification Notes

- Definition source: Direct quotation from p. 94
- Confidence: HIGH -- explicitly defined with example
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: Evans/Schoenberg example, three-set-class insight, "drop 2" connection
