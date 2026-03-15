---
# === CORE IDENTIFICATION ===
concept: Scalar Model of Key Distance
slug: scalar-model-of-key-distance

# === CLASSIFICATION ===
category: geometric-theory
subcategory: key-relations
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Functional Harmony"
chapter_number: 7
pdf_page: 264
section: "7.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "voice-leading model of key distance"
  - "scale-based key distance"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - efficient-voice-leading
extends: []
related:
  - two-lattices
  - circle-of-thirds-harmonic
contrasts_with:
  - weber-chart-of-regions

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can key distance be measured using voice leading between scales?"
  - "Why do minor keys modulate to the relative major more often than to the dominant?"
  - "How does the scalar model improve on Weber's chart of the regions?"
---

# Quick Definition
A model of key distance based on voice-leading distance between associated scales, where keys are close when their scales can be connected by small semitonal shifts — this better explains actual modulation patterns than the traditional Weber chart.

# Core Definition
Tymoczko proposes measuring key distance by the voice-leading distance between associated scales rather than by the acoustic or chordal relationships between tonic notes. For major keys, this simply reduces to the circle of fifths (each step changes one note by semitone). For minor keys, which use three scale forms (natural, harmonic, melodic), he averages the distances between the scales belonging to each key. The resulting distances predict that: major keys are closest to V, IV, and vi; minor keys are closest to III (relative major), with VII in second place; and there is a particularly close relation between a major key and its supertonic minor (ii). Statistical analysis of modulation frequencies in Bach, Haydn, Mozart, and Beethoven confirms these predictions better than the traditional Weber model, achieving correlations of .91-.96 compared to .77-.84 for Weber.

# Prerequisites
- Voice-leading distance between scales (the scale lattice from Chapter 4)

# Key Properties
1. For major keys: closest keys are V, IV, vi, with ii close behind
2. For minor keys: closest key is III (relative major), then VII
3. Predicts more modulation to supertonic minor than parallel minor (confirmed by data)
4. Explains why minor keys favor relative major over dominant minor
5. Correlations of .91-.96 with actual modulation data, vs .77-.84 for Weber

# Construction / Recognition
## To Construct/Create:
1. For major-to-major: count semitones needed to transform one diatonic scale into another
2. For major-to-minor: average the distances from the major scale to each of the three minor scales
3. For minor-to-minor: average the distances in the most efficient pairing of three scales to three scales

## To Identify/Recognize:
1. Keys with small average scale distances are predicted to be frequent modulatory destinations
2. Cross-reference with actual modulation patterns in the repertoire

# Context & Application
The scalar model matters because it suggests different generalizations than the Weber model. In Chapter 9, twentieth-century composers exploit efficient voice leading between non-traditional scales. If we recognize scale-to-scale voice leading as the mechanism behind classical modulation, we can understand these twentieth-century practices as generalizations of traditional techniques rather than departures from them. This has important implications for music history and pedagogy.

# Examples
**Example 1** (Fig. 7.4.5, p. 268): Key distance table showing C major closest to F major and G major (distance 1), then A minor and D minor (distance 1 and 1.33).

**Example 2** (Fig. 7.4.7, p. 269): Modulation frequency data — for all four composers, major keys favor V, IV, vi, ii; minor keys favor III and VII, confirming scalar predictions over Weber.

**Example 3** (Fig. 7.4.6, p. 268): C major is particularly close to D minor because two of the three D minor scales can be linked to C major by single-semitone voice leading.

# Relationships
## Builds Upon
- **efficient-voice-leading** — Key distance is measured by voice-leading distance between scales
## Enables
- **two-lattices** — The scale lattice parallels the chord lattice
- Understanding of twentieth-century modulatory practices
## Contrasts With
- **weber-chart-of-regions** — The traditional model based on chordal/tonic relationships

# Common Errors
- **Error**: Assuming the scalar model explains everything about modulation
  **Correction**: Other factors matter too — parallel keys share a tonic note (acoustic relationship), and individual composers have preferences

# Common Confusions
- **Confusion**: Thinking scale distance and chord distance measure the same thing
  **Clarification**: Scale distance measures how many semitones transform one seven-note scale into another; chord distance measures voice leading between three- or four-note chords

# Source Reference
Chapter 7: Functional Harmony, Section 7.4, pages 264-270, Figures 7.4.3-7.4.7.

# Verification Notes
- Definition source: Directly from Section 7.4 with quantitative validation
- Confidence rationale: High — extensively argued with statistical evidence
- Cross-reference status: Verified against scale lattice from Chapter 4 and modulation data
