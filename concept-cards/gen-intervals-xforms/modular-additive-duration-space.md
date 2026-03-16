---
# === CORE IDENTIFICATION ===
concept: Modular Additive Duration Space
slug: modular-additive-duration-space

# === CLASSIFICATION ===
category: generalized-interval-systems
subcategory: rhythmic-spaces
tier: intermediate

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "Example 2.2.6, Section 2.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "duration-class space"
  - "modular duration space"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - integers-mod-n
extends: []
related:
  - additive-duration-space
  - beat-class-space
  - pitch-class-space
contrasts_with:
  - additive-duration-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does modular arithmetic rescue additive duration intervals?"
  - "What is a duration-class?"
  - "How does this space relate to the failed Example 2.2.5?"
---

# Quick Definition
Modular additive duration space is a GIS that rescues additive duration intervals by wrapping them around an M-hour clock, giving meaning to "negative" duration-classes and forming a valid GIS where the non-modular additive system (Example 2.2.5) failed.

# Core Definition
"We restrict our attention to the durations of 2.2.5 that are exactly the positive integral multiples of some basic small duration, which we take as the temporal unit. We wrap these durations around an M-hour clock, accordingly reducing the system to a modular system. The modular space comprises M duration-classes: Two durations belong to the same duration-class if their lengths differ by some integral multiple of M" (Lewin, Example 2.2.6, p. 48).

"Example 2.2.6 can be regarded as one means of salvaging example 2.2.5 in this connection, by providing a convention that attaches meaning to the concept of a negative duration-class. E.g. we can think of duration-class '-5' as that class containing all durations lasting just 5 units less than some multiple of the modulus duration" (Lewin, Section 2.4, p. 49).

# Prerequisites
- **Generalized Interval System** — This is an instance of the GIS definition
- **Integers mod N** — IVLS is the additive group of integers modulo M

# Key Properties
1. S = {0, 1, 2, ..., M-1} (M duration-classes)
2. IVLS = Z_M = integers under addition mod M
3. int(s, t) = (t - s) mod M
4. Negative duration-classes are meaningful: -5 mod M = M - 5
5. Condition (B) holds (unlike Example 2.2.5), making this a valid GIS

# Construction / Recognition
## To Construct:
1. Fix a modulus M (representing a "measure" or cycle length in time units)
2. Define duration-classes as equivalence classes of positive integral durations mod M
3. Define int(s, t) = (t - s) mod M
## To Recognize:
1. A rhythmic space with M duration-classes on a clock
2. Additive intervals measured modulo M

# Context & Application
This GIS gives mathematical structure to rhythmic analysis with a fixed cycle size M. Example 2.2.5 (non-modular additive durations) failed to form a GIS because negative durations have no meaning -- one "cannot conceive a duration lasting precisely 5 units less than no time at all." Wrapping around an M-hour clock resolves this: "-5" becomes "M - 5," which is a meaningful duration-class.

# Examples
**Example 1** (Example 2.2.6, p. 48): With M = 16 and the time unit as a sixteenth note:
- Duration-class s = 8 (half note, give or take whole notes)
- Duration-class t = 4 (quarter note, give or take whole notes)
- int(s, t) = 4 - 8 = -4 = 12 mod 16
- Interpretation: "A quarter note, tied to an extra whole note for free, is a dotted half longer than a half note."

# Relationships
## Builds Upon
- **Integers mod N** — Provides the algebraic structure for IVLS
## Enables
- **Rhythmic analysis** — Allows set-theoretic methods to apply to duration-classes
## Related
- **Beat-class space** — Analogous modular rhythmic space for time points rather than durations
- **Pitch-class space** — Structural analog: just as pitch classes wrap pitches mod 12, duration-classes wrap durations mod M
## Contrasts With
- **Additive duration space** — Example 2.2.5 fails to form a GIS; this modular version succeeds

# Common Errors
- **Error**: Confusing this with Example 2.2.4 (multiplicative modular duration space)
  **Correction**: Here intervals are additive (t - s mod M); in 2.2.4 they are multiplicative (t/s mod powers of M)

# Common Confusions
- **Confusion**: Thinking "negative duration-classes" are meaningless
  **Clarification**: Under modular arithmetic, -5 mod M = M - 5, which is a perfectly meaningful duration-class representing durations 5 units shorter than a multiple of M

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.6 and Section 2.4, pages 48-49.

# Verification Notes
- Definition source: Direct quotation from Example 2.2.6 and Section 2.4
- Confidence rationale: Explicitly defined with worked example
- Re-extraction notes: Re-extracted from v2 card; preserved: M=16 worked example, contrast with failed Example 2.2.5, rhythmic analog to pitch-class space
