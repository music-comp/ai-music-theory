---
# === CORE IDENTIFICATION ===
concept: Additive Duration Space
slug: additive-duration-space

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
section: "2.2.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Example 2.2.5"
  - subtractive duration space

# === TYPED RELATIONSHIPS ===
prerequisites:
  - group
  - generalized-interval-system
  - gis-condition-b
extends: []
related:
  - duration-proportion-space
  - duration-class-modular-space
contrasts_with:
  - duration-proportion-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I construct a GIS from a musical space?"
---

# Quick Definition

Additive duration space attempts to measure intervals between durations by subtraction (differences) rather than ratios, but it fails to form a GIS because negative durations are not meaningful, violating Condition (B).

# Core Definition

"The musical space is a family of durations. Int(s, t) is the difference (NB not the quotient) of time units between s and t: Int(s, t) = (t - s) units" (Lewin, Example 2.2.5, p. 55). However, this does NOT form a GIS because Condition (B) fails: for s = 3 and i = -8, there is no duration t with int(s, t) = i, since t = -5 units is a "negative duration," which is meaningless. Lewin stresses that "we cannot conceive... a duration lasting precisely 5 units less than no time at all."

# Prerequisites

- **Group** — the attempted IVLS would be (Z, +)
- **Generalized Interval System** — this example shows what a GIS requires
- **GIS Condition B** — this is the condition that fails

# Key Properties

1. Attempted: S = positive durations, IVLS = integers under addition
2. int(s, t) = t - s (difference in time units)
3. Condition (B) FAILS: no duration t = -5 units exists
4. Contrast with pitch space: supersonic/subsonic pitches are conceivable, but negative durations are not
5. Example 2.2.6 rescues this via modular arithmetic

# Construction / Recognition

## To Construct:
1. Cannot be constructed as a GIS (Condition B fails)

## To Recognize:
1. The space attempts additive intervals between positive durations
2. The failure arises when the interval would require negative durations

# Context & Application

This is the only example in Chapter 2 that does NOT form a GIS. It illustrates the necessity of Condition (B) and the limits of GIS modeling. While we can conceive of arbitrarily high or low pitches, we cannot conceive of a duration lasting "measurably less than no time at all." This non-example motivates the modular version (Example 2.2.6) that rescues the situation.

# Examples

**Example 1** (p. 55): r = 3 units (dotted eighth), s = 4 units (quarter), t = 8 units (half). int(r, s) = 1 unit, int(s, t) = 4 units, int(t, r) = -5 units.

**Example 2** (p. 55): Condition (B) failure: take s = 3 and i = -8. Then t = s + i = -5. But -5 units is not a duration.

**Example 3** (p. 55): Compare with multiplicative intervals (2.2.3): int(r, s) = 4/3, int(s, t) = 2, int(t, r) = 3/8 -- all positive, all meaningful.

# Relationships

## Builds Upon
- **GIS Condition B** — this is the condition that fails

## Enables
- **Duration-Class Modular Space** — modular arithmetic rescues this system

## Related
- **Duration Proportion Space** — the multiplicative alternative that does work

## Contrasts With
- **Duration Proportion Space** — quotients (always positive) vs. differences (can be negative)

# Common Errors

- **Error**: Assuming any additive interval system on durations forms a GIS.
  **Correction**: The inability to conceive negative durations prevents Condition (B) from holding.

# Common Confusions

- **Confusion**: Thinking "negative duration" is like "negative pitch interval."
  **Clarification**: Negative pitch intervals correspond to conceivable low pitches. Negative durations have no conceivable analog.

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.5, Section 2.4, pp. 55-56.

# Verification Notes

- Definition source: direct from Example 2.2.5
- Confidence rationale: explicit discussion of failure with philosophical reasoning
- Re-extracted from v2 card; preserved: specific duration examples, philosophical argument about negative durations, comparison with multiplicative system
