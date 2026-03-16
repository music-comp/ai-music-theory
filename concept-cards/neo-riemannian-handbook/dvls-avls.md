---
# === CORE IDENTIFICATION ===
concept: DVLS and AVLS Voice-Leading Measurements
slug: dvls-avls

# === CLASSIFICATION ===
category: transformations
subcategory: voice-leading-metrics
tier: advanced

# === PROVENANCE ===
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Dmitri Tymoczko"
chapter: "Inversional Symmetry and Voice Leading"
chapter_number: 8
pdf_page: null
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "Displacement Voice-Leading Size"
  - "Average Voice-Leading Size"
  - "directed voice-leading sum"
  - "absolute voice-leading sum"
  - "voice-leading distance measures"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - voice-leading-graph
  - plr-transformations
extends: []
related:
  - voice-leading-efficiency-principle
  - weitzmann-region-analysis
  - parsimonious-trichords
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are DVLS and AVLS and how do they quantify voice-leading efficiency?"
  - "How do neo-Riemannian transformations compare in terms of voice-leading distance?"
  - "What is a Weitzmann region and how is it defined by AVLS?"
---

# Quick Definition

Quantitative measures of voice-leading efficiency: DVLS (Displacement/Directed Voice-Leading Size) sums the semitone distances moved by all voices (with or without direction), while AVLS (Average/Absolute Voice-Leading Size) divides this by the number of voices to give average motion per voice.

# Core Definition

Tymoczko (Ch. 8) develops DVLS and AVLS as part of a geometric approach to voice leading. **DVLS** = |d1| + |d2| + ... + |dn| where di is the displacement in semitones of voice i. **AVLS** = DVLS / n where n is the number of voices. These metrics quantify distances in "voice-leading space" where points represent chords, distances represent voice-leading effort, and efficient progressions correspond to short paths.

Rings (Ch. 18) uses these measures analytically, noting that DVLS can also be "directed" (distinguishing up from down) while AVLS measures "absolute" distance regardless of direction -- what Joseph Straus calls total voice-leading "work" or "exertion" (p. 490).

# Prerequisites

- **Voice-leading graphs**: The geometric spaces in which DVLS/AVLS measure distances.
- **PLR transformations**: The operations whose efficiency these metrics quantify.

# Key Properties

1. **Non-negative**: Both measures are always >= 0
2. **Zero only for identity**: DVLS = 0 only when chords are identical (or contrary motion cancels)
3. **Inversion-preserving**: I-related voice leadings have equal DVLS/AVLS
4. **Normalization**: AVLS allows comparison across different chord cardinalities

# Construction / Recognition

For neo-Riemannian transformations on triads:
| Transformation | DVLS | AVLS |
|---------------|------|------|
| P | 1 | 0.33 |
| L | 1 | 0.33 |
| R | 2 | 0.67 |

P and L are "maximally smooth" -- achieving minimum DVLS for distinct triads. AVLS = 2 defines membership in a **Weitzmann region** (Rings, Ch. 18, p. 491).

# Context & Application

Rings (Ch. 18) uses DVLS/AVLS to analyze Schubert's Gb Impromptu, mm. 78-82. The chromatically extraordinary passage shows AVLS = 2 throughout, revealing that all progressions stay within a single Weitzmann region. DVLS alternates +2/-2 until the cadential resolution where DVLS = 0 marks the "wrenching" return to functional tonal syntax via contrary motion (p. 490).

# Examples

**P transformation** (C major to C minor): C->C: 0, E->Eb: 1, G->G: 0. DVLS = 1, AVLS = 0.33.

**R transformation** (C major to A minor): C->C: 0, E->E: 0, G->A: 2. DVLS = 2, AVLS = 0.67.

**Schubert Impromptu analysis** (Rings, Ch. 18, p. 490): Gb+ to B-: DVLS = +2, AVLS = 2. B- to D+: DVLS = -2, AVLS = 2. Pattern: DVLS alternates +2/-2 while AVLS = 2 throughout the chromatic passage, incrementing to 3 only at the confirming cadence.

**Why chromatic thirds recur**: C major to Ab major has DVLS = 2, more efficient than fifth-related progressions. This explains the prevalence of major-third-related triads in chromatic music.

# Relationships

## Builds Upon
- Voice-leading graph theory and geometric chord spaces

## Enables
- Weitzmann region analysis: AVLS = 2 defines these regions
- Systematic comparison of voice-leading efficiency across repertoires

## Related
- Voice-leading efficiency principle (Rings): Uses DVLS/AVLS as analytical tools
- Parsimonious trichords: Defined by minimal DVLS connections

## Contrasts With
- Functional harmonic analysis (measures harmonic distance, not voice-leading distance)

# Common Errors

- **Error**: Confusing DVLS with number of moving voices.
  **Correction**: DVLS counts total semitones moved, not number of active voices.

# Common Confusions

- **Confusion**: Thinking low DVLS means harmonically close.
  **Clarification**: Voice-leading distance and harmonic/functional distance are independent measures. V-I has relatively high DVLS but is harmonically fundamental.

# Source Reference

Tymoczko, Dmitri. "Inversional Symmetry and Voice Leading." Ch. 8. Rings, Steven. "Riemannian and Neo-Riemannian Analysis." Ch. 18, pp. 489-492. In *The Oxford Handbook of Neo-Riemannian Music Theories*.

# Verification Notes

Re-extracted from v2 card; preserved: formulas, PLR comparison table, all calculation examples, Schubert Impromptu analysis from Rings, chromatic-third explanation. Enhanced with Rings's directed/absolute distinction and Weitzmann region connection. Confidence high.
