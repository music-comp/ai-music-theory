---
concept: Voice-Leading Work
slug: voice-leading-work

category: voice-leading
subcategory: distance-metrics
tier: foundational

source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Mapping the Triadic Universe"
chapter_number: 1
pdf_page: 19
section: "Three Ways to Calculate Triadic Distance"

extraction_confidence: high

aliases:
  - "voice-leading distance"
  - "VL work"
  - "total semitonal displacement"

prerequisites:
  - idealized-voice-leading
extends:
  - voice-leading-approach
related:
  - minimal-work-relation
  - single-semitonal-displacement
  - triadic-distance
contrasts_with:
  - common-tone-approach

answers_questions:
  - "How do I calculate voice-leading work between two triads?"
  - "What is voice-leading work?"
---

# Quick Definition
The sum of the magnitude of all voice motions between two chords connected by idealized voice leading, measured in semitones.

# Core Definition
Voice-leading work quantifies the total semitonal displacement required to transform one chord into another under idealized voice leading. A unit of voice-leading work equals the motion of one voice by one semitone. The work between two triads is calculated by summing the absolute values of all voice motions when tones are optimally paired. For consonant triads, voice-leading work ranges from 1 to 6 semitones, with lower values indicating closer voice-leading proximity (synthesized from Ch. 1, pp. 25-31, and Ch. 2, p. 36).

# Prerequisites
- **idealized-voice-leading** — Voice-leading work is measured under idealized (optimal) voice-leading pairing

# Key Properties
1. Measured in semitones (integer values)
2. Range for consonant triads: 1 (minimum, P or L) to 6 (maximum)
3. Finer-grained than common-tone counting (6 values vs. 3)
4. Independent of root motion direction
5. Measured in pitch-class space, abstracting from actual register
6. One unit of work = one voice moving one semitone

# Construction / Recognition
## To calculate voice-leading work between triads X and Y:
1. List the pitch classes of both triads
2. Find the optimal one-to-one pairing that minimizes total semitonal motion
3. For each pair, calculate the distance in semitones (mod 12, taking the smaller direction)
4. Sum all distances

## Examples:
- C major to c minor (P): E to Eb = **1 unit**
- C major to e minor (L): C to B = **1 unit**
- C major to a minor (R): G to A = **2 units**
- C major to Ab major (LP): C to C, E to Eb, G to Ab = **2 units**
- C major to ab minor (H): C to B, E to Eb, G to Ab = **3 units**
- C major to F major: C to C, E to F, G to A = **3 units**

# Context & Application
Voice-leading work is the central metric for pan-triadic analysis. It provides the basis for defining single semitonal displacement, for adjacency in the hexatonic cycle and Cube Dance, and for measuring triadic proximity independent of root relations. "The voice-leading approach distinguishes between progressions that common-tone counting conflates" (synthesized from Ch. 1).

# Examples
**Example 1** (Ch. 1, Table 1.1): The Schubert B-flat Sonata passage: Bb to Gb = 2 units, Gb to f# = 1 unit, f# to A = 2 units, total = 14 units across the full progression.

**Example 2** (Ch. 2, p. 36): In a hexatonic cycle, "motion between them thus involves a single unit of work," defining the minimal-work relation.

# Relationships
## Builds Upon
- **idealized-voice-leading** — The assumed pairing method
- **voice-leading-approach** — Voice-leading work is the quantitative core of this approach

## Enables
- **minimal-work-relation** — Defined as voice-leading work of 1
- **single-semitonal-displacement** — The transformation producing 1 unit of work
- **hexatonic-cycle** — Built from chains of 1-unit-work connections

## Related
- **triadic-distance** — Voice-leading work is one of three distance metrics

## Contrasts With
- **common-tone-approach** — Common-tone counting provides coarser distance information

# Common Errors
- **Error**: Counting the number of moving voices instead of total semitonal displacement
  **Correction**: Voice-leading work measures total displacement, not the number of moving voices. C to a minor: 1 voice moves 2 semitones = 2 units, not 1 unit

# Common Confusions
- **Confusion**: Thinking voice-leading work applies to actual compositional voicing
  **Clarification**: It measures idealized voice leading in pitch-class space, not the actual registral realization
- **Confusion**: Assuming low work always means more common tones
  **Clarification**: C major to ab minor = 3 units with 0 common tones; C major to G major = 4 units with 1 common tone

# Source Reference
Chapter 1: Mapping the Triadic Universe, pp. 25-31. Glossary, p. 229. Foundational throughout the book.

# Verification Notes
- Re-extracted from v2 card; preserved: the calculation examples, the Schubert analysis reference, the distinction from common-tone counting
- Confidence: HIGH — the concept is explicitly defined and used consistently throughout the book
