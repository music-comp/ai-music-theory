---
# === CORE IDENTIFICATION ===
concept: String Fretting and Rational Intervals
slug: string-fretting-and-rational-intervals

# === CLASSIFICATION ===
category: rational-intervals
subcategory: just-intervals
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "The Rational Numbers As Musical Intervals"
chapter_number: 11
pdf_page: 138
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - rational interval construction by string division

# === TYPED RELATIONSHIPS ===
prerequisites:
  - rational-interval
extends: []
related:
  - just-interval
  - consonance-and-small-integer-ratios
contrasts_with:
  - irrationality-of-equally-tempered-intervals

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can rational intervals be produced on a string?"
  - "Why were rational intervals accessible to ancient musicians?"
---

# Quick Definition

Rational intervals can be accurately produced on a vibrating string using compass and straightedge to divide the string length, a technique accessible to ancient mathematicians, unlike the irrational intervals of equal temperament.

# Core Definition

For a vibrating string of length L with fundamental frequency f, fretting at distance (m/n)*L from one end produces a vibrating length of (m/n)*L, yielding frequency f*(n/m). Wright explains: "If n/m >= 1, its rational interval is obtained with the string's fundamental frequency by fretting the string at distance (m/n)*L (<= L) from one end" (p. 139). Since "any interval in the real number line can be divided into n equal subintervals using compass and rule," this technique was accessible to ancient geometers.

# Prerequisites

- **Rational interval** -- Understanding what makes an interval rational (ratio in Q+)

# Key Properties

1. To produce ratio n/m, fret at distance (m/n)*L from one end (note the inversion)
2. Shorter string = higher pitch: (m/n)*L < L when n > m
3. Only rational divisions of a string are constructible with compass and straightedge
4. Irrational intervals like 2^(1/12) (tempered semitone) were not accessible to ancient mathematicians
5. The geometric construction explains the historical priority of rational intervals

# Construction / Recognition

## To Produce a Rational Interval n/m on a String

1. Divide the string of length L into n equal parts using compass and straightedge
2. Mark the point at distance m parts from one end (i.e., at (m/n)*L)
3. Fret at this point
4. The vibrating portion (m/n)*L produces frequency f * (n/m)

## Common Fret Positions

- Octave (2/1): fret at L/2
- Just fifth (3/2): fret at 2L/3
- Just fourth (4/3): fret at 3L/4
- Just major third (5/4): fret at 4L/5

# Context & Application

This technique explains how ancient musicians could produce precise just intervals without modern measurement tools. A luthier could divide a string into halves, thirds, quarters, fifths, etc., using only geometric methods. By contrast, the equal-tempered semitone requires placing a fret at 2^(-1/12)*L ~ 0.9439*L, a distance not constructible by compass and straightedge. The historical priority of rational intervals is partly a consequence of this constructibility.

# Examples

**Example 1** (p. 139): The just fifth 3/2 is obtained by fretting at 2L/3.

**Example 2** (p. 139): The tempered semitone (2^(1/12)) "would necessitate, as we have seen, finding the distance 2^(-1/12)*L -- a technique not accessible to ancient mathematicians."

**Example 3** (p. 139): A diagram shows compass-and-straightedge division of a line segment into 5 equal parts, illustrating the geometric technique.

# Relationships

## Builds Upon
- **Rational interval** -- String fretting is the physical realization of rational intervals

## Enables
- **Just interval** -- Just intervals are the musically important rational intervals achievable by fretting

## Related
- **Consonance and small integer ratios** -- The consonant intervals are precisely those achievable by simple string divisions

## Contrasts With
- **Irrationality of equally tempered intervals** -- Equal-tempered fret positions are irrational and were not geometrically constructible in antiquity

# Common Errors

- **Error**: Fretting at (n/m)*L instead of (m/n)*L to produce ratio n/m
  **Correction**: Shorter string = higher pitch; to produce n/m, the vibrating length must be (m/n)*L, which is shorter than L when n > m

# Common Confusions

- **Confusion**: Thinking modern guitar frets use rational string divisions
  **Clarification**: Modern fretted instruments use equal-tempered fret placement (irrational distances), not rational divisions

- **Confusion**: Believing that geometric constructibility means rational intervals are "more correct"
  **Clarification**: Constructibility explains their historical priority, not their superiority; equal temperament has its own advantages

# Source Reference

Chapter 11: "The Rational Numbers As Musical Intervals," pp. 138-139.

# Verification Notes

- Definition source: Direct from pp. 138-139
- Confidence rationale: Explicit description with diagram reference
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: modern guitar caveat, constructibility vs. correctness distinction
