---
# === CORE IDENTIFICATION ===
concept: Modular Diatonic Space
slug: modular-diatonic-space

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: tonal-spaces
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "Example 2.1.4, Section 2.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "diatonic pitch-class space"
  - "seven-hour clock space"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - integers-mod-n
extends: []
related:
  - diatonic-pitch-space
  - pitch-class-space
  - beat-class-space
contrasts_with:
  - pitch-class-space
  - diatonic-pitch-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How is diatonic interval structure modeled as a GIS with seven pitch classes?"
  - "What is the interval group for modular diatonic space?"
  - "How does this relate to other modular scale-based spaces?"
---

# Quick Definition
Modular diatonic space is a GIS of seven pitch classes corresponding to the seven scale degrees of a diatonic system, arranged on a seven-hour clock, with intervals measured as clockwise hours under addition modulo 7.

# Core Definition
"The musical space comprises seven pitch-classes, corresponding to the seven mode degrees of system 2.1.1. If we wrap the scale around the face of a seven-hour clock, then int(s, t) is the number of hours that we traverse on that clock, in proceeding clockwise from s to t" (Lewin, Example 2.1.4, p. 47).

The GIS: "S = the seven mode degrees as indicated, the group IVLS = the integers under addition modulo 7, and the function int(s, t) = number of hours clockwise from s to t on a 7-hour clock" (Lewin, Section 2.4, p. 48).

# Prerequisites
- **Generalized Interval System** — This is an instance of the GIS definition
- **Integers mod N** — IVLS is Z_7, the integers under addition modulo 7

# Key Properties
1. S = {C, D, E, F, G, A, B} (or seven scale degrees)
2. IVLS = Z_7 = integers under addition mod 7
3. int(s, t) = clockwise distance on a 7-hour clock
4. All interval values lie in {0, 1, 2, 3, 4, 5, 6}
5. int(s, t) + int(t, s) = 0 mod 7, i.e., int(t, s) = 7 - int(s, t) when int(s, t) > 0

# Construction / Recognition
## To Construct:
1. Arrange the seven diatonic pitch classes around a 7-hour clock
2. Measure intervals as clockwise hours traversed
3. Reduce all arithmetic modulo 7
## To Recognize:
1. A 7-element pitch-class space with intervals in Z_7
2. Intervals measured in diatonic scale steps, not semitones

# Context & Application
This GIS models diatonic interval relationships under octave equivalence. The mod-7 arithmetic captures relationships like "a third plus a third equals a fifth" correctly: 2 + 2 = 4 in step-class terms. Lewin notes that analogous modular spaces can be constructed for other scales: "we could investigate octatonic-scale space ... we could derive therefrom a modular space of eight pitch-classes, wrapping the octatonic scale around an eight-hour clock and measuring intervals modulo 8" (Example 2.1.4 discussion).

# Examples
**Example 1** (Example 2.1.4, p. 47):
- int(D, D) = 0 (unison)
- int(D, E) = 1 (one step up)
- int(D, C) = 6 (six hours clockwise = one step "down")

**Example 2** (derived):
- int(C, E) = 2 (a "third" = 2 steps)
- int(E, G) = 2 (a "third" = 2 steps)
- int(C, G) = 4 (a "fifth" = 4 steps)
- Verification: 2 + 2 = 4 mod 7

**Analogs** (Example 2.1.4 discussion): Octatonic scale yields 8 pitch classes with IVLS = Z_8; pentatonic scale yields 5 pitch classes with IVLS = Z_5.

# Relationships
## Builds Upon
- **Integers mod N** — Provides the algebraic structure (here N = 7)
## Enables
- **Diatonic analysis** — Models scale-step relationships under octave equivalence
## Related
- **Diatonic pitch space** — The linear (non-modular) version from Example 2.1.1
- **Pitch-class space** — The chromatic analog with 12 pitch classes and Z_12
- **Beat-class space** — Another modular clock-based GIS
## Contrasts With
- **Pitch-class space** — Uses 7 elements and Z_7, not 12 elements and Z_12
- **Diatonic pitch space** — Modular (finite, wrapping) vs. linear (infinite, non-wrapping)

# Common Errors
- **Error**: Writing int(D, C) = -1
  **Correction**: In mod-7 arithmetic, int(D, C) = 6, not -1; all intervals are in {0, 1, 2, 3, 4, 5, 6}

# Common Confusions
- **Confusion**: Conflating scale-step intervals with semitone intervals
  **Clarification**: Intervals in this GIS measure diatonic scale steps, not semitones. The 7-hour clock is entirely distinct from the 12-hour pitch-class clock.

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.4 and Section 2.4, pages 47-48.

# Verification Notes
- Definition source: Direct quotation from Example 2.1.4 and Section 2.4
- Confidence rationale: Explicitly defined with worked examples
- Re-extraction notes: Re-extracted from v2 card; preserved: clock model, Lewin's examples, octatonic/pentatonic analogs
