---
# === CORE IDENTIFICATION ===
concept: Meter and Time Signatures
slug: meter-and-time-signatures

# === CLASSIFICATION ===
category: rhythm-and-form
subcategory: meter
tier: intermediate

# === PROVENANCE ===
source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Horizontal Structure"
chapter_number: 2
pdf_page: 30
section: "Meter"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS (authority control) ===
aliases:
  - "time signature"
  - "meter"
  - "measures"
  - "bars"
  - "compound time"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - note-durational-values
  - beats-and-tempo
extends: []
related:
  - rhythm
  - dotted-note-duration-formula
  - tuplets
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is meter in music?"
  - "How does a time signature work?"
  - "What is the difference between simple and compound time?"
---

# Quick Definition

Meter organizes music into groups of beats (measures), and the time signature specifies the number of beats per measure and which note value receives one beat, with a special compound interpretation when the top number is divisible by 3 and greater than 3.

# Core Definition

A piece of music is divided into *measures* (or *bars*) of $n$ beats ($n \geq 1$). The *meter* is the number $n$ of beats per measure together with which durational note gets one beat. The *time signature* $\frac{n}{r}$ (written as stacked integers, not a fraction) has two interpretations (Wright, pp. 36-37):

**Usual meaning**: $n$ = beats per measure; $r = 2^m$ designates the $\frac{1}{2^m}$-th note gets one beat.

**Exceptional case** (compound time): when $3 \mid n$ and $n > 3$, the number of beats is $n/3$, and one beat = three $\frac{1}{2^m}$-th notes = a dotted $\frac{1}{2^{m-1}}$-th note.

# Prerequisites

- **Note Durational Values** — The bottom number references the durational note system
- **Beats and Tempo** — Meter organizes beats into measures

# Key Properties

1. The time signature is two stacked integers, NOT a fraction
2. The bottom number $r$ is always a power of 2 (in practice, nearly always 2, 4, or 8)
3. Compound time: when $3 \mid n$ and $n > 3$, beats per measure = $n/3$, beat = dotted note
4. $\frac{3}{4}$ is NOT compound even though 3 divides 3, because the rule requires $n > 3$
5. Time signature appears after the clef and may change within a piece

# Construction / Recognition

## To interpret a time signature $\frac{n}{r}$:

1. If $3 \mid n$ and $n > 3$: compound time — beats = $n/3$, beat unit = dotted $\frac{1}{2^{m-1}}$-th note
2. Otherwise: simple time — beats = $n$, beat unit = $\frac{1}{2^m}$-th note (where $r = 2^m$)
3. The whole note gets $r$ beats in simple time

# Context & Application

Common time signatures include $\frac{4}{4}$ (4 quarter-note beats), $\frac{3}{4}$ (waltz time), $\frac{2}{4}$ (march), and $\frac{6}{8}$ (compound: 2 dotted-quarter beats). The distinction between $\frac{3}{4}$ (simple: 3 quarter-note beats) and $\frac{6}{8}$ (compound: 2 dotted-quarter beats) is fundamental despite both containing the same total duration per measure.

# Examples

- $\frac{4}{4}$: 4 beats per measure, quarter note = 1 beat; whole note = 4 beats (p. 36)
- $\frac{2}{4}$: 2 beats per measure, quarter note = 1 beat (p. 36)
- $\frac{6}{8}$ (compound): $6/3 = 2$ beats per measure, one beat = dotted quarter note (p. 37)
- $\frac{2}{2}$: 2 beats per measure, half note = 1 beat; whole note = 2 beats (p. 34)
- In $\frac{2}{2}$ time, a sixteenth note has duration $\frac{1}{16} \cdot 2 = \frac{1}{8}$ beats (p. 34)

# Relationships

## Builds Upon
- **Note Durational Values** — Bottom number references the duration system
- **Beats and Tempo** — Top number specifies beats per measure

## Enables
- **Rhythm** — Meter provides the framework that rhythm fills

## Related
- **Dotted Note Duration Formula** — Compound time uses dotted notes as beat units
- **Tuplets** — Compound time relates to triplet subdivisions

# Common Errors

- **Error**: Treating the time signature as a fraction
  **Correction**: $\frac{n}{r}$ is two stacked integers, not a mathematical fraction

# Common Confusions

- **Confusion**: Thinking $\frac{6}{8}$ means 6 beats of eighth notes
  **Clarification**: $\frac{6}{8}$ is compound: 2 beats of dotted quarter notes, not 6 beats
- **Confusion**: Treating $\frac{3}{4}$ as compound
  **Clarification**: $\frac{3}{4}$ is simple time (3 quarter-note beats); the compound rule requires $n > 3$
- **Confusion**: Thinking $\frac{3}{4}$ and $\frac{6}{8}$ are equivalent
  **Clarification**: They have the same total duration per measure but different beat structures

# Source Reference

Chapter 2: "Horizontal Structure", "Meter" section, pp. 36-37 (PDF).

# Verification Notes

- Definition source: Direct from source, pp. 36-37
- Confidence rationale: High — explicit definition with both simple and compound cases
- Uncertainties: None
- Cross-reference status: Verified
- Re-extraction notes: Re-extracted from v2 card; preserved: compound time rule ($n > 3$), $\frac{3}{4}$ vs $\frac{6}{8}$ distinction, not-a-fraction caveat
