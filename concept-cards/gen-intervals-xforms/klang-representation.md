---
# === CORE IDENTIFICATION ===
concept: Klang Representation
slug: klang-representation

# === CLASSIFICATION ===
category: transformation-theory
subcategory: klang-operations
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.1.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - Klang
  - "ordered pair (p, sign)"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - fundamental-bass-intervals
extends: []
related:
  - dom-transformation
  - med-transformation
  - par-transformation
  - rel-transformation
  - lt-transformation
  - slide-transformation
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a Klang?"
  - "How does the Klang formalism solve the problem of fundamental bass intervals?"
---

# Quick Definition
A formal representation of a chord as an ordered pair (p, sign), where p is a pitch class (the root) and sign is + for major or - for minor, yielding a space of 24 Klangs that supports mode-sensitive transformations.

# Core Definition
"Each Klang is an ordered pair (p, sign), where p is a pitch class and sign takes on the values + and - for major and minor respectively. The Klang models a harmonic object with p as root or tonic, an object whose modality is determined by the sign" (Lewin, 8.1.1, p. 175). Transposition preserves sign: "(C, +) transposed by 5/6 is (A, +)." Right orthography is used: "(Klang)f" rather than "f(Klang)," so that "(Klang)fg = ((Klang)f)g" -- the order of composition matches left-to-right reading (pp. 175-176).

# Prerequisites
- **Fundamental bass intervals** — The inadequacy of root-intervals for distinguishing mode motivates Klangs

# Key Properties
1. Each Klang is an ordered pair (p, sign) with p a pitch class and sign in {+, -}
2. 12 pitch classes x 2 modes = 24 Klangs in equal temperament
3. Transposition preserves sign: (p, sign)T_i = (p+i, sign)
4. Right orthography is used: transformations are written to the right of their arguments
5. The composition order is reversed under right orthography: (Klang)fg means f first, then g
6. Derived from and modifying Riemann's function theory ideas

# Construction / Recognition
## To Construct:
1. Identify the root pitch class p of a chord
2. Determine the mode (+ for major, - for minor)
3. Write as ordered pair (p, sign)
## To Recognize:
1. An ordered pair notation with pitch class and mode indicator
2. Transformations written in right orthography

# Context & Application
The Klang formalism solves the problem that "the numerical ratio does not tell us that the A harmony is minor rather than major" (p. 176). By pairing pitch class with mode, Klangs enable transformations that change mode (PAR, REL, LT, SLIDE) alongside those that preserve it (DOM, SUBD). Lewin adopts and modifies "some ideas from the function theories of Hugo Riemann" (p. 175).

# Examples
**Example 1** (p. 175): (C, +) = C major, (C, -) = C minor, (A, -) = A minor, (G, +) = G major.

**Example 2** (p. 176): (C, +) transposed by 5/6 yields (A, +), not (A, -). This is why the A-minor tail of Figure 7.9 requires a transformation "other than harmonic-transposition-by-(5/6)."

# Relationships
## Builds Upon
- **Fundamental bass intervals** — Klang representation resolves their inadequacy
## Enables
- **DOM transformation** — Operates on Klangs
- **MED transformation** — Operates on Klangs with mode change
- **PAR transformation** — Changes mode, preserves root
- **REL transformation** — Relative major/minor on Klangs
- **LT transformation** — Leading-tone exchange on Klangs
- **SLIDE transformation** — Preserves third while changing mode
## Related
- **Right orthography** — The notation convention for Klang transformations
- **Non-intervallic transformations** — Klang transformations that go beyond GIS structure

# Common Errors
- **Error**: Confusing Klangs with specific chord voicings
  **Correction**: A Klang is an ordered pair, not a pitched chord; it specifies root and mode only

# Common Confusions
- **Confusion**: Thinking all 24 Klangs are needed in every analysis
  **Clarification**: The relevant Klang space depends on the musical context; some analyses use subsets
- **Confusion**: Assuming Klangs require equal temperament
  **Clarification**: The definition works in any tuning system, though the space may differ in just intonation (infinite Klangs possible)

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.1.1, pages 175-176.

# Verification Notes
- Definition source: Direct quotation from 8.1.1
- Confidence rationale: Explicit formal definition in source
- Re-extraction notes: Re-extracted from v2 card; preserved: right orthography note, Riemann connection, 24-Klang count
