---
# === CORE IDENTIFICATION ===
concept: Time-Point Space
slug: time-point-space

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: musical-spaces
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "2.2.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Example 2.2.1"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
  - function
extends: []
related:
  - chromatic-pitch-space
  - beat-class-space
  - generalized-interval-system
contrasts_with:
  - duration-proportion-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I construct a GIS from a musical space?"
---

# Quick Definition

Time-point space is a GIS where the elements are regularly pulsing time points, and intervals measure the number of time units by which one point is later than another.

# Core Definition

"The musical space is a succession of time points pulsing at regular temporal distances one time unit apart. Given time points s and t, int(s, t) is the number of temporal units by which t is later than s. (-x units later is x units earlier.)" (Lewin, Example 2.2.1, p. 52). S extends indefinitely in both temporal directions, IVLS = (Z, +).

# Prerequisites

- **Group** — IVLS = (Z, +) is a group
- **Function** — int: S x S -> IVLS is a function

# Key Properties

1. S = time points extending indefinitely backward and forward
2. IVLS = (Z, +), the integers under addition
3. int(s, t) = number of time units that t is later than s
4. Negative intervals mean earlier: -x later = x earlier
5. Structurally identical to chromatic pitch space (both use Z under addition)

# Construction / Recognition

## To Construct:
1. Establish a regular pulse of time points one unit apart
2. Extend indefinitely in both temporal directions
3. Define int(s, t) = number of units that t is later than s

## To Recognize:
1. Elements are time points (instants), not durations (spans)
2. Intervals are integers measuring temporal displacement

# Context & Application

Time-point space provides a GIS framework for rhythmic analysis, modeling discrete equally-spaced attacks or pulses. It is the temporal analog of chromatic pitch space. This space is later modularized into beat-class space (Example 2.2.2) by imposing a meter.

# Examples

**Example 1** (p. 52): If s is beat 5 and t is beat 9, int(s, t) = 4 (t is 4 units later). If t is beat 2, int(s, t) = -3 (3 units earlier).

# Relationships

## Builds Upon
- **Group** — uses (Z, +) as the interval group

## Enables
- **Beat-Class Space** — wrapping time-point space around an N-hour clock

## Related
- **Chromatic Pitch Space** — structurally identical (both use IVLS = Z)

## Contrasts With
- **Duration Proportion Space** — time points measure positions; durations measure lengths

# Common Errors

- **Error**: Confusing time points (positions in time) with durations (lengths of time).
  **Correction**: Time points are instants; durations are spans. Different GIS structures apply to each.

# Common Confusions

- **Confusion**: Thinking the space cannot extend into the past.
  **Clarification**: For Condition (B), the space must extend indefinitely in both directions.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.1, Section 2.4, pp. 52, 54.

# Verification Notes

- Definition source: direct from Example 2.2.1 and Section 2.4
- Confidence rationale: explicit example with full GIS specification
- Re-extracted from v2 card; preserved: structural identity with chromatic pitch space
