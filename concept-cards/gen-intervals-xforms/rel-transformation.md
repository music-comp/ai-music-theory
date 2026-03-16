---
concept: REL Transformation
slug: rel-transformation

category: transformation-theory
subcategory: klang-operations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.1.1"

extraction_confidence: high

aliases:
  - "relative transformation"

prerequisites:
  - klang-representation
extends: []
related:
  - par-transformation
  - lt-transformation
  - non-intervallic-transformations
contrasts_with:
  - med-transformation
  - subm-transformation

answers_questions:
  - "How do I apply REL to Klangs?"
  - "How does REL differ from MED?"
---

# Quick Definition
A Klang transformation that takes any Klang into its relative minor/major: (C, +)REL = (A, -) and (C, -)REL = (Eb, +). REL is an involution (self-inverse).

# Core Definition
"We can define REL, the operation that takes any Klang into its relative minor/major. (C, +)REL = (A, -); (C, -)REL = (Eb, +). REL is not the same operation as MED or SUBM: (C, -)REL = (Eb, +) but (C, -)MED = (Ab, +); (C, +)REL = (A, -) but (C, +)SUBM = (E, -)" (Lewin, 8.1.1, p. 177).

# Prerequisites
- **Klang representation** — REL operates on Klangs

# Key Properties
1. (p, +)REL = (p + 9, -) mod 12 [major to relative minor]
2. (p, -)REL = (p + 3, +) mod 12 [minor to relative major]
3. REL^2 = identity (REL is an involution/self-inverse)
4. REL preserves key signature
5. REL differs from MED on minor Klangs: (C, -)REL = (Eb, +) vs. (C, -)MED = (Ab, +)
6. REL differs from SUBM on major Klangs: (C, +)REL = (A, -) vs. (C, +)SUBM = (E, -)

# Construction / Recognition
## To Construct:
1. For major Klang (p, +): move root down a minor third, change to minor
2. For minor Klang (p, -): move root up a minor third, change to major
## To Recognize:
1. The two Klangs share a key signature
2. One is the relative major/minor of the other

# Context & Application
REL captures the relative major/minor relationship fundamental to tonal music. As a non-power-of-MED operation, REL contributes to genuinely non-intervallic Klang networks. Together with PAR and LT, REL forms part of the PLR group central to Neo-Riemannian theory.

# Examples
**Example 1** (p. 177): (C, +)REL = (A, -): C major to A minor. (A, -)REL = (C, +): A minor to C major. (G, +)REL = (E, -): G major to E minor.

# Relationships
## Builds Upon
- **Klang representation** — REL is defined on Klangs
## Related
- **PAR transformation** — Fellow involutory Klang operation
- **LT transformation** — Fellow member of PLR group
- **Non-intervallic transformations** — REL cannot be expressed as a power of MED
## Contrasts With
- **MED transformation** — Agree on major, differ on minor Klangs
- **SUBM transformation** — Agree on neither major nor minor Klangs

# Common Errors
- **Error**: Assuming REL = MED because (C, +)REL = (C, +)MED = (A, -)
  **Correction**: They differ on minor Klangs: (C, -)REL = (Eb, +) but (C, -)MED = (Ab, +)

# Common Confusions
- **Confusion**: Thinking REL changes both root and mode arbitrarily
  **Clarification**: REL preserves key signature; the root change and mode change are related by the relative major/minor relationship

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.1.1, page 177.

# Verification Notes
- Definition source: Direct quotation from 8.1.1
- Confidence rationale: Explicitly defined with comparisons to MED and SUBM
- Re-extraction notes: Re-extracted from v2 card; preserved: REL vs. MED/SUBM comparisons, involution property
