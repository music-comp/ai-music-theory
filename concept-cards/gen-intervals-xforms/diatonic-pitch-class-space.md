---
concept: Diatonic Pitch-Class Space
slug: diatonic-pitch-class-space

category: generalized-interval-systems
subcategory: musical-spaces
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "2.1.4"

extraction_confidence: high

aliases:
  - "Example 2.1.4"

prerequisites:
  - group
  - equivalence-relation
extends:
  - diatonic-pitch-space
related:
  - pitch-class-space
  - generalized-interval-system
contrasts_with:
  - pitch-class-space

answers_questions:
  - "How do I construct a GIS from a musical space?"
---

# Quick Definition

Diatonic pitch-class space is a GIS of seven pitch classes (the seven mode degrees) arranged on a seven-hour clock, with intervals measured as clockwise hours modulo 7.

# Core Definition

"The musical space comprises seven pitch-classes, corresponding to the seven mode degrees of system 2.1.1. If we wrap the scale around the face of a seven-hour clock, then int(s, t) is the number of hours that we traverse on that clock, in proceeding clockwise from s to t" (Lewin, Example 2.1.4, p. 47). S = 7 diatonic pitch classes, IVLS = Z7 (integers under addition modulo 7).

# Prerequisites

- **Group** — IVLS = (Z7, +) is a group
- **Equivalence Relation** — diatonic pitch classes are equivalence classes under octave equivalence in diatonic space

# Key Properties

1. S = 7 diatonic pitch classes (mode degrees)
2. IVLS = Z7 = integers under addition mod 7
3. int(s, t) = clockwise distance from s to t on a 7-hour clock
4. This is the modular version of diatonic pitch space (Example 2.1.1)
5. Analogous to pitch-class space but with 7 elements instead of 12

# Construction / Recognition

## To Construct:
1. Take the seven mode degrees of a diatonic scale
2. Wrap them around a 7-hour clock
3. Define int(s, t) = clockwise hours from s to t

## To Recognize:
1. Seven pitch-class elements
2. Intervals in Z7 (0 through 6)

# Context & Application

This GIS models scale-degree relationships in diatonic contexts. Lewin notes that analogs can be produced for other scales (e.g., octatonic scale with an 8-hour clock). The example illustrates that the GIS framework is not limited to 12-note chromatic systems.

# Examples

**Example 1** (p. 47): int(D, D) = 0, int(D, E) = 1, int(D, C) = 6.

**Example 2**: One could derive an octatonic analog: 8 pitch classes on an 8-hour clock with IVLS = Z8.

# Relationships

## Builds Upon
- **Diatonic Pitch Space** — this is its modular reduction

## Related
- **Pitch-Class Space** — the 12-note chromatic analog

## Contrasts With
- **Pitch-Class Space** — 7 elements with Z7 vs. 12 elements with Z12

# Common Errors

- **Error**: Confusing diatonic pitch-class intervals with chromatic pitch-class intervals.
  **Correction**: In diatonic pitch-class space, int(C, E) = 2 (two scale steps), not 4 (four semitones).

# Common Confusions

- **Confusion**: Thinking only 12-note systems form GIS structures.
  **Clarification**: Any modular system (7-note, 8-note, etc.) can form a GIS with the appropriate Zn group.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.4, p. 47.

# Verification Notes

- Definition source: direct from Example 2.1.4
- Confidence rationale: explicit example in source
- New card (no prior version existed)
