---
# === CORE IDENTIFICATION ===
concept: Beat-Class Space
slug: beat-class-space

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
section: "2.2.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Example 2.2.2"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
  - equivalence-class
  - time-point-space
extends:
  - time-point-space
related:
  - pitch-class-space
  - generalized-interval-system
contrasts_with:
  - time-point-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I construct a GIS from a musical space?"
---

# Quick Definition

Beat-class space is a GIS of N beat classes (metric positions within a measure) arranged on an N-hour clock, with intervals measured as clockwise hours modulo N.

# Core Definition

"The musical space is the preceding one [time-point space], wrapped around the face of an N-hour clock. We can imagine this as modeling the imposition of an N-unit meter on the earlier space, so that barlines appear regularly every N pulses. The present space has N members, which we shall call 'beat classes,' labeling them by numbers from 0 through N-1" (Lewin, Example 2.2.2, p. 52). S = N beat classes, IVLS = ZN (integers under addition mod N).

# Prerequisites

- **Group** — IVLS = (ZN, +) is a group
- **Equivalence Class** — beat classes are equivalence classes of time points under metric equivalence
- **Time-Point Space** — beat-class space is its modular reduction

# Key Properties

1. S = {0, 1, 2, ..., N-1} (N beat classes)
2. IVLS = ZN = integers under addition mod N
3. int(s, t) = clockwise distance from s to t on N-hour clock
4. Beat-class 0 = all downbeats (barline pulses)
5. This is the rhythmic analog of pitch-class space

# Construction / Recognition

## To Construct:
1. Choose a meter with N beats per measure
2. Wrap the time points around an N-hour clock
3. Define int(s, t) = clockwise hours from s to t

## To Recognize:
1. N metric positions (beat classes) as elements
2. Intervals in ZN
3. Modular arithmetic for composition

# Context & Application

Beat-class space models metric position abstractly, independent of which measure. A dancing master calling "ONE-two-three" assigns beat classes. Conductors know beat classes kinetically through hand positions. Milton Babbitt developed a system of 12 beat classes paralleling twelve-tone pitch-class theory.

# Examples

**Example 1** (p. 52): In 12/8 meter (N = 12): int(beat-class 10, beat-class 5) = 7.

**Example 2** (p. 52): Babbitt's 12 beat-class system behaves formally like the 12-tone pitch-class system.

**Example 3** (p. 52): Conductors associate beat classes with spatial hand positions; intervals correspond to gestural paths.

# Relationships

## Builds Upon
- **Time-Point Space** — beat-class space wraps time-point space mod N

## Related
- **Pitch-Class Space** — the tonal analog (12 pitch classes on a clock)

## Contrasts With
- **Time-Point Space** — finite vs. infinite; modular vs. integer intervals

# Common Errors

- **Error**: Using the wrong modulus N for the meter.
  **Correction**: N depends on the meter: 12 for 12/8, 4 for 4/4, 3 for 3/4.

# Common Confusions

- **Confusion**: Thinking beat class 0 must be "beat 1."
  **Clarification**: Beat classes are labeled 0 through N-1, with beat-class 0 at the barline.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.2, Section 2.4, pp. 52-53, 54.

# Verification Notes

- Definition source: direct from Example 2.2.2 and Section 2.4
- Confidence rationale: explicit example with full GIS specification
- Re-extracted from v2 card; preserved: Babbitt reference, conducting gesture interpretation
