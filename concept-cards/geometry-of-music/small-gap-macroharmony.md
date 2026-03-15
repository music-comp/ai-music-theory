---
# === CORE IDENTIFICATION ===
concept: Small-Gap Macroharmony (2-Gap)
slug: small-gap-macroharmony

# === CLASSIFICATION ===
category: scales-modes
subcategory: macroharmony
tier: intermediate

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Macroharmony and Centricity"
chapter_number: 5
pdf_page: 174
section: "5.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "2-gap macroharmony"
  - "gapless macroharmony"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - macroharmony
  - harmonic-vs-macroharmonic-consistency
extends: []
related:
  - fundamental-theorem-of-jazz
  - large-gap-macroharmony
  - near-evenness
contrasts_with:
  - large-gap-macroharmony

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What distinguishes 'small-gap' from 'large-gap' macroharmony?"
  - "Why do diatonic and acoustic scales support so many chord types?"
  - "What are the 2-gap macroharmonies in twelve-tone equal temperament?"
---

# Quick Definition
A 2-gap macroharmony is a pitch-class collection whose successive notes are at most two semitones apart, guaranteeing that every note in the macroharmony can serve as the root of both a triad and a fourth chord.

# Core Definition
A macroharmony is classified by its maximum gap size — the largest interval between consecutive pitch classes. A "2-gap macroharmony" has no gap larger than two semitones. This property is crucial because harmonic terms like "triad" and "fourth chord" allow variation in interval size (a third can be 3 or 4 semitones). In a 2-gap macroharmony, for each chord tone there are two semitonally adjacent options, guaranteeing that at least one will lie within the collection. This means every note can serve as the root of a triad, a fourth chord, and many other chord types. The 2-gap property is distinct from near-evenness: a collection can be nearly even while having large gaps (e.g., the major triad), and can have small gaps while being uneven.

# Prerequisites
- **macroharmony** — Understanding the concept of aggregate pitch-class collections
- **harmonic-vs-macroharmonic-consistency** — Understanding why combining consistencies requires certain collections

# Key Properties
1. Maximum gap between consecutive pitch classes is 2 semitones
2. Every note in the collection can serve as the root of a triad and a fourth chord
3. Any out-of-collection note can be moved into the collection by shifting up OR down by semitone
4. Distinct from near-evenness — small gaps and even distribution are related but independent
5. Only a small number of 2-gap collections exist in twelve-tone equal temperament

# Construction / Recognition
## To Construct/Create:
1. Choose 6-8 pitch classes from the chromatic scale
2. Verify that no two consecutive pitch classes are more than 2 semitones apart
3. Check that the collection wraps around (the gap from the last PC back to the first is also at most 2)
## To Identify/Recognize:
1. Arrange the pitch classes in ascending order
2. Calculate the interval between each consecutive pair (including wraparound)
3. If no interval exceeds 2 semitones, the collection is a 2-gap macroharmony

# Context & Application
The 2-gap macroharmonies are precisely the collections that best combine harmonic and macroharmonic consistency. In twelve-tone equal temperament: there is one 6-note 2-gap collection (whole-tone), three 7-note collections (diatonic, acoustic, "whole-tone-plus-one"), and eight 8-note collections (including the octatonic). The prevalence of diatonic and acoustic scales across Western music is partly explained by their 2-gap property — they are large enough to offer harmonic variety but constrained enough to maintain macroharmonic identity.

# Examples
**Example 1** (p. 176, Figure 5.2.4): The complete inventory of 2-gap macroharmonies: 1 six-note (whole-tone), 3 seven-note (diatonic, acoustic, C-C#-D-E-F#-G#-Bb), 8 eight-note (including octatonic).

**Example 2** (p. 175, Figure 5.2.2): In a 2-gap macroharmony, any out-of-scale note can be corrected by moving a semitone in EITHER direction — both chromatic neighbors of an out-of-scale note are guaranteed to be in the collection.

**Example 3** (p. 175): Near-evenness vs. small gaps illustrated: chords can be transposed along a nearly even scale with minimal distortion, while in a gapless scale, chords outside the scale can be "squeezed" in with minimal distortion.

# Relationships
## Builds Upon
- **macroharmony** — A specific type of macroharmony
- **harmonic-vs-macroharmonic-consistency** — 2-gap property enables combining both consistencies
## Enables
- **fundamental-theorem-of-jazz** — The theorem depends on the 2-gap property
## Related
- **near-evenness** — Related but distinct property
## Contrasts With
- **large-gap-macroharmony** — 3-gap collections are more restrictive

# Common Errors
- **Error**: Assuming 2-gap macroharmonies must be nearly even
  **Correction**: The eight-note collection {C, C#, D, D#, E, F#, G#, Bb} has 2-gap property but distributes its semitones unevenly

# Common Confusions
- **Confusion**: Conflating "small gaps" with "near evenness"
  **Clarification**: Near evenness means scalar transposition approximates chromatic transposition; small gaps means any chromatic note can be squeezed into the collection. A major triad is nearly even but has large gaps; some 8-note 2-gap collections are quite uneven.

# Source Reference
Chapter 5: Macroharmony and Centricity, Section 5.2, pages 174-176, Figure 5.2.4.

# Verification Notes
- Definition source: Section 5.2, explicit discussion and enumeration
- Confidence rationale: High — formally defined with complete enumeration provided
- Cross-reference status: Connects to Chapter 4 discussion of near-evenness
