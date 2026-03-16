---
# === CORE IDENTIFICATION ===
concept: Mean-Tone Space
slug: mean-tone-space

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
section: "Discussion following Example 2.1.6"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "quarter-comma mean-tone space"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - modular-harmonic-space
  - equivalence-relation
  - quotient-group
extends:
  - modular-harmonic-space
related:
  - pitch-class-space
  - just-intonation-pitch-space
  - temperament
contrasts_with:
  - modular-harmonic-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How is mean-tone space derived from modular harmonic space?"
  - "What equivalence relation collapses the two-dimensional harmonic map to a one-dimensional chain?"
  - "What happens when enharmonic equivalence is further imposed on mean-tone space?"
---

# Quick Definition
Mean-tone space is a one-dimensional GIS derived from the two-dimensional modular harmonic space (Figure 2.2) by declaring pitch classes with the same letter name but different subscripts to be equivalent, collapsing the mediant dimension and producing an infinite chain of fifths.

# Core Definition
"We could reduce the system of 2.1.6 farther if we considered pitch classes to be equivalent when they shared the same letter name, differing only by subscript. Then C_-1, C_0, C_1, C_2, ... would all mean the same thing. ... In this case, moving one square north on the game board of figure 2.2 would be functionally equivalent to moving four squares east. The north/south dimension of the board would functionally disappear, and we could reduce our map to a one-dimensional east/west succession of dominant-related pitch classes ... Eb, Bb, F, C, G, D, A, E, ..." (Lewin, Ch. 2, following Example 2.1.6, p. 48).

# Prerequisites
- **Modular harmonic space** — The two-dimensional GIS being reduced
- **Equivalence relation** — Declaring subscript variants of the same letter name equivalent
- **Quotient group** — The reduction produces a quotient of the original interval group

# Key Properties
1. Space S = infinite chain of pitch classes by fifths: ... Eb, Bb, F, C, G, D, A, E, B, F#, ...
2. IVLS = integers under addition (counting fifths)
3. One step north = four steps east (syntonic comma tempered out)
4. Models quarter-comma mean-tone temperament: four fifths exactly equal a major third
5. Without enharmonic equivalence, the space is infinite (all sharps/flats distinct)

# Construction / Recognition
## To Construct:
1. Begin with modular harmonic space (Figure 2.2)
2. Declare all subscript variants of a letter name equivalent (C_-1 ~ C_0 ~ C_1 ~ ...)
3. Observe the north/south dimension collapses (one north = four east)
4. Result: a one-dimensional chain ordered by fifths
## To Recognize:
1. A one-dimensional pitch-class space ordered by fifths
2. Intervals measured by counting steps along the chain of fifths

# Context & Application
"Because of the equivalence relation that led to this series, we may as well consider the reduced pitch classes to represent pitches in quarter-comma mean-tone temperament: Four new 'fifths' (that were steps east on figure 2.2) are pitch-class equivalent to a 'major third' (that was a step north on figure 2.2)" (Lewin). Mean-tone temperament was a standard tuning in the Renaissance and Baroque.

# Examples
**Example 1** (p. 48): The chain ... Eb, Bb, F, C, G, D, A, E, B, F#, C#, G#, D#, A# ...
- int(C, G) = 1 (one fifth)
- int(C, D) = 2 (two fifths)
- int(C, E) = 4 (four fifths = one major third in mean-tone)

**Example 2** (p. 48): Further reduction with enharmonic equivalence (Gb = F#, Db = C#, etc.) wraps the chain into a circle: "we find ourselves back at the system of 2.1.3, only now measuring intervals-modulo-the-octave by (equally tempered) fifths rather than by semitones."

# Relationships
## Builds Upon
- **Modular harmonic space** — Mean-tone is a quotient reduction of this two-dimensional space
## Enables
- **Pitch-class space** — Further enharmonic reduction yields the 12-pc system measured by fifths
## Related
- **Just intonation pitch space** — Mean-tone tempers the syntonic comma from just intonation
## Contrasts With
- **Modular harmonic space** — In harmonic space, subscripted pitch classes remain distinct

# Common Errors
- **Error**: Treating mean-tone space as derived from chromatic pitch-class space (2.1.3)
  **Correction**: It is derived from modular harmonic space (2.1.6), not chromatic space

# Common Confusions
- **Confusion**: Thinking mean-tone space is finite
  **Clarification**: Without enharmonic equivalence, the chain of fifths extends infinitely in both directions; only with enharmonic equivalence does it wrap into 12 pitch classes

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, discussion following Example 2.1.6, page 48.

# Verification Notes
- Definition source: Direct quotation from discussion following Example 2.1.6
- Confidence rationale: Explicitly constructed and discussed by Lewin
- Re-extraction notes: Re-extracted from v2 card; preserved: derivation from Figure 2.2, enharmonic further reduction, temperament connection
