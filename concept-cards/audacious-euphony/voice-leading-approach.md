---
concept: Voice-Leading Approach
slug: voice-leading-approach

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
  - "voice-leading proximity"
  - "voice-leading parsimony"

prerequisites:
  - harmonic-distance
  - idealized-voice-leading
extends:
  - triadic-distance
related:
  - voice-leading-work
  - common-tone-approach
  - root-interval-approach
contrasts_with:
  - root-interval-approach

answers_questions:
  - "What is voice-leading parsimony?"
  - "How does the voice-leading approach measure triadic distance?"
  - "How do I calculate voice-leading work between two triads?"
---

# Quick Definition
A method for calculating triadic distance based on the total semitonal displacement required to transform one triad into another under idealized voice leading, providing the finest gradations among the three distance metrics.

# Core Definition
The voice-leading approach measures triadic proximity by summing the absolute value of semitonal motion in each voice when triads are connected by idealized voice leading. A unit of "voice-leading work" is the motion of one voice by one semitone. This approach attends not only to the number of moving voices but also to the distance each voice travels, providing "finer gradations of distance" than the common-tone metric: 6 distinct values versus 3 (synthesized from Ch. 1, pp. 25-31). The approach has ancient roots: Marchettus of Padua (early 14th c.) articulated a "closest approach" preference for semitonal voice leading, and Zarlino (1558) preferred progressions where voices move by the smallest intervals.

# Prerequisites
- **harmonic-distance** — The voice-leading approach is one specific way of measuring harmonic distance
- **idealized-voice-leading** — The approach depends on optimal pairing of chord tones

# Key Properties
1. Measured in semitones under idealized voice leading
2. Range for consonant triads: 1-6 units
3. Provides 6 distinct distance values (finer than common-tone's 3)
4. Can distinguish between progressions that common-tone counting conflates
5. Independent of root motion direction
6. Underlies the pan-triadic model of harmonic syntax

# Construction / Recognition
For triads X = {x1, x2, x3} and Y = {y1, y2, y3}:
1. Find the pairing of tones that minimizes total semitonal motion
2. Voice-leading work = |x1-y1| + |x2-y2| + |x3-y3| (mod 12, smaller direction)

Examples:
- C major to c minor: 1 unit (E to Eb)
- C major to e minor: 1 unit (C to B)
- C major to a minor: 2 units (G to A)
- C major to Gb major: 2 units (C to Cb/Db, G to Gb)

# Context & Application
This approach underlies the pan-triadic model: triads related by single semitonal displacement form the closest connections in the triadic universe. Capellen (c. 1900) proposed triadic connections based on common tones plus semitonal motions (p. 25). The approach distinguishes between progressions that common-tone counting conflates (e.g., f# minor to Gb major = 1 unit vs. f# minor to A major = 2 units, though both share 2 common tones).

# Examples
**Example 1** (pp. 27-31): The Schubert B-flat Sonata recapitulation has a total voice-leading work of 14 units across four progressions, positioning it "toward the lower/more conjunct end of the range."

**Example 2** (p. 28): Gb major to f# minor = 1 unit of work (Bb to A), demonstrating that enharmonic "conversion" costs only one semitone.

# Relationships
## Builds Upon
- **harmonic-distance** — One specific approach to measuring harmonic distance
- **triadic-distance** — The finest-grained of the three metrics
- **idealized-voice-leading** — The assumed pairing method

## Enables
- **voice-leading-work** — The specific quantitative metric
- **minimal-work-relation** — The minimum voice-leading distance (1 unit)
- **pan-triadic-syntax** — The syntax built on voice-leading proximity

## Related
- **common-tone-approach** — A coarser alternative metric

## Contrasts With
- **root-interval-approach** — Root distance does not predict voice-leading distance

# Common Errors
- **Error**: Measuring voice-leading work in actual registral pitch space
  **Correction**: Voice-leading work is measured in pitch-class space, abstracting from register

# Common Confusions
- **Confusion**: Assuming low voice-leading work always correlates with high common-tone retention
  **Clarification**: C major to ab minor has 3 units of work and 0 common tones; C major to a minor has 2 units and 1 common tone
- **Confusion**: Thinking the metric requires actual semitonal voice leading in the score
  **Clarification**: It measures idealized voice leading, not the composer's actual voicing

# Source Reference
Chapter 1: Mapping the Triadic Universe, pp. 25-31.

# Verification Notes
- Re-extracted from v2 card; preserved: Marchettus and Zarlino historical references, the Schubert and enharmonic examples, the distinction from common-tone counting
- Confidence: HIGH — the approach is explicitly defined and illustrated with examples
