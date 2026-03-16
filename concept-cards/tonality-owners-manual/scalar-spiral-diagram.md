---
concept: Scalar Spiral Diagram
slug: scalar-spiral-diagram

category: analysis
subcategory: geometrical-models
tier: advanced

source: "Tonality: An Owner's Manual"
source_slug: tonality-owners-manual
authors: "Dmitri Tymoczko"
chapter: "Modulation"
chapter_number: 8
pdf_page: 361
section: "Two models of key distance"

extraction_confidence: high

aliases:
  - 7-in-12 spiral diagram
  - scale space spiral

prerequisites:
  - basic-voice-leading
  - spiral-diagram-derivation
extends:
  - voice-leading-geometry
related:
  - basic-voice-leading-of-scales
  - hierarchical-transposition
  - leading-tone-lowering
  - fourth-scale-degree-raising
contrasts_with: []

answers_questions:
  - "What is the scalar spiral diagram and how does it represent diatonic scales?"
  - "How do slides and loops on the diagram correspond to different types of modulation?"
  - "How does the diagram show that enharmonicism is chordal inversion at the scalar level?"
---

# Quick Definition

A spiral representation of seven-note diatonic scales within twelve-tone chromatic space, where sliding along the spiral represents scale-degree-preserving modulations and loops around the spiral represent scale-degree-changing modulations that produce enharmonic equivalence.

# Core Definition

The scalar spiral diagram places seven-note diatonic scales in twelve-tone chromatic space, using the same geometrical framework previously applied to chords within scales. Sliding clockwise from C to F passes seven points for **T**_{-7} while passing twelve o'clock four times for t_4. The combination **T**_{-7}t_4 shifts B to B-flat while keeping all other melodic slots fixed -- this is the basic voice leading (p. 358, Fig. P8.2). Slides along the spiral represent scale-degree-preserving voice leadings; loops around the spiral represent scale-degree-changing voice leadings that alter abstract voices' roles. Complete loops produce enharmonic equivalence, "just as loops in chord space produce chordal inversion" -- indeed, "enharmonicism just is chordal inversion at the scalar level" (p. 368). The diagram allows us to see when repeating contrapuntal patterns can return each voice to its starting pitch: only when the pattern moves purely radially, requiring multiple chords at the same angular position (possible for 3-in-12 triads since 3 divides 12, but not for 7-in-12 scales since 7 does not divide 12).

# Prerequisites

- **Basic voice leading** -- The general concept of minimal voice leading, applied here to scales
- **Spiral diagram derivation** -- Understanding how spiral diagrams are constructed from T and t operations

# Key Properties

1. Seven-note diatonic scales placed in twelve-tone chromatic space
2. Clockwise motion adds flats (flatward modulation)
3. Counterclockwise motion adds sharps (sharpward modulation)
4. Slides = scale-degree-preserving voice leadings
5. Loops = scale-degree-changing voice leadings (enharmonic equivalence)
6. Enharmonicism is chordal inversion at the scalar level
7. Because 7 does not divide 12, no repeated modulatory move returns voices to starting pitches
8. Fifth-related scales are found approximately 30 degrees apart on the circle

# Construction / Recognition

## To Use the Scalar Spiral Diagram:
1. Place all twelve major diatonic scales on the spiral, separated by 30-degree angular steps
2. Clockwise steps correspond to adding one flat (lowering leading tone)
3. Counterclockwise steps correspond to adding one sharp (raising fourth degree)
4. A complete clockwise loop (360 degrees) represents twelve fifth-related modulations -- a nontrivial voice leading that shifts every scale degree down by one
5. Paths that retrace themselves (going out and back) produce trivial voice leadings

# Context & Application

The diagram unifies the treatment of chords and scales under a single geometrical framework. It dates back to the eighteenth-century work of Heinichen and Kellner, whose "Musicalische Circul" is one of the earliest geometrical representations of musical structure, antedating the Tonnetz by almost a decade (p. 363, Fig. 8.1.6). The diagram can be extended to incorporate minor keys using superimposed circles for diatonic, acoustic (melodic minor ascending), and harmonic minor scales (Fig. 8.3.3). A Pythagorean (non-tempered) version would not close, with B-sharp major slightly displaced from C major (Fig. 8.2.4).

# Examples

**Example 1** (p. 358, Fig. P8.2): The spiral diagram for diatonic scales, showing the basic voice leading **T**_{-7}t_4 calculated by sliding clockwise from C to F.

**Example 2** (p. 367, Fig. 8.2.2): Two modulatory sequences graphed in 7-in-12 space -- one making a complete clockwise loop (nontrivial, producing enharmonic equivalence), the other retracing its path (trivial, returning voices to starting positions).

**Example 3** (p. 373, Fig. 8.3.3): Extended models incorporating diatonic, acoustic, and harmonic minor scales, with minor keys shown as spatially extended across multiple radial positions.

# Relationships

## Builds Upon
- **Voice-leading geometry** -- The scalar spiral is a specific instance of the general geometrical framework
- **Spiral diagram derivation** -- Uses the same T and t operations

## Enables
- **Basic voice leading of scales** -- The diagram provides the tool for calculating and visualizing scalar voice leading
- **Hierarchical transposition** -- The scalar spiral forms one level of the nested spiral diagram pair

## Related
- **Leading-tone lowering** -- Clockwise steps on the diagram
- **Fourth-scale-degree raising** -- Counterclockwise steps on the diagram

## Contrasts With
- (Not the same as the circle of fifths, though related -- the circle of fifths orders keys by tonic; the spiral orders scales by voice-leading proximity)

# Common Errors

- **Error**: Confusing the scalar spiral diagram with the circle of fifths
  **Correction**: The spiral diagram for scales is related but distinct -- it represents voice-leading proximity and distinguishes slides from loops, which the circle of fifths does not

# Common Confusions

- **Confusion**: Thinking merely notational enharmonic respellings correspond to genuine scalar voice leading
  **Clarification**: "Merely notational" respellings (e.g., G-sharp major spelled as A-flat major) do not correspond to any scalar voice leading and have no observable consequences; genuine enharmonic equivalence involves an observable change in scale-degree roles (p. 368)

# Source Reference

Prelude to Chapter 8, pp. 357-359; Chapter 8: Modulation, sections 1-2, pp. 361-371.

# Verification Notes

- Definition source: Direct from Prelude to Ch. 8 and Ch. 8, sections 1-2
- Confidence rationale: Central geometrical tool with precise mathematical formulation
- Cross-reference status: Verified against basic voice leading of scales, leading-tone lowering, fourth-scale-degree raising cards
- Re-extraction notes: Re-extracted from v2 card; preserved: **T**_{-7}t_4 formula, slides vs. loops distinction, enharmonicism-as-inversion insight, Pythagorean non-closure, Heinichen/Kellner reference, 7-does-not-divide-12 property
