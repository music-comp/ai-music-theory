---
concept: Pitch-Class Space
slug: pitch-class-space

category: generalized-interval-systems
subcategory: musical-spaces
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "2.1.3"

extraction_confidence: high

aliases:
  - "Example 2.1.3"
  - twelve-tone pitch-class space

prerequisites:
  - group
  - equivalence-relation
  - quotient-group
extends:
  - chromatic-pitch-space
related:
  - generalized-interval-system
  - beat-class-space
  - diatonic-pitch-class-space
contrasts_with:
  - chromatic-pitch-space

answers_questions:
  - "How do I construct a GIS from a musical space?"
---

# Quick Definition

Pitch-class space is a GIS of 12 pitch classes (octave-equivalent pitches) arranged on a clock face, with intervals measured as clockwise hours modulo 12.

# Core Definition

"The musical space comprises the twelve pitch-classes under equal temperament. If we arrange the pitch classes around the face of a clock following the order of a chromatic scale, then int(s, t) is the number of hours that we traverse in proceeding clockwise from s to t" (Lewin, Example 2.1.3, p. 47). S = 12 pitch classes, IVLS = integers under addition modulo 12, and int(s, t) = clockwise distance from s to t. Notably, int(s, t) does not depend on which pitch class is positioned at 12 o'clock.

# Prerequisites

- **Group** — IVLS = (Z12, +) is a group
- **Equivalence Relation** — pitch classes are equivalence classes under octave equivalence
- **Quotient Group** — Z12 is the quotient of Z by 12Z

# Key Properties

1. S = {C, C#, D, D#, E, F, F#, G, G#, A, A#, B} (12 pitch classes)
2. IVLS = Z12 = integers under addition mod 12
3. int(s, t) = clockwise distance from s to t on a 12-hour clock
4. int(s, t) is always in {0, 1, 2, ..., 11}
5. This is a finite GIS (12 elements), unlike the infinite chromatic pitch space

# Construction / Recognition

## To Construct:
1. Arrange 12 pitch classes on a clock face
2. Define int(s, t) = number of clockwise hours from s to t

## To Recognize:
1. 12 pitch classes as elements
2. Intervals in Z12 (0 through 11)
3. Modular arithmetic for interval composition

# Context & Application

This is the foundational GIS for twelve-tone and pitch-class set theory. Octave equivalence collapses all octave-related pitches into single classes. This is one of few GIS examples that is finite and practically complete -- every element is easily accessible in music.

# Examples

**Example 1** (p. 47): int(E, E) = 0, int(E, F) = 1, int(F, E) = 11.

**Example 2** (p. 47): If s is at 8 o'clock and t is at 1 o'clock, int(s, t) = 5.

**Example 3**: Condition (A): int(C, E) = 4, int(E, G) = 3, int(C, G) = 7, and 4 + 3 = 7 mod 12.

# Relationships

## Builds Upon
- **Chromatic Pitch Space** — pitch-class space is the mod-12 reduction of chromatic pitch space

## Enables
- **Commutative vs. Non-Commutative GIS** — this is a commutative GIS example

## Related
- **Beat-Class Space** — the rhythmic analog (mod N)
- **Diatonic Pitch-Class Space** — the mod-7 analog for diatonic contexts

## Contrasts With
- **Chromatic Pitch Space** — infinite vs. finite; integers vs. integers mod 12

# Common Errors

- **Error**: Using negative intervals in mod 12.
  **Correction**: int(F, E) = 11, not -1. In mod 12, all intervals are non-negative.

# Common Confusions

- **Confusion**: Thinking the clock orientation matters.
  **Clarification**: int(s, t) does not depend on which pitch class sits at 12 o'clock; only relative positions matter.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.3, Section 2.4, pp. 47, 53.

# Verification Notes

- Definition source: direct from Example 2.1.3 and Section 2.4
- Confidence rationale: explicit example with full GIS specification
- Re-extracted from v2 card; preserved: clock-independence remark, int(E,E)=0/int(E,F)=1/int(F,E)=11 examples
