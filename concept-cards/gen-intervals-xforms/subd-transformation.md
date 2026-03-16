---
concept: SUBD Transformation
slug: subd-transformation

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

aliases: []

prerequisites:
  - klang-representation
  - dom-transformation
extends: []
related:
  - subm-transformation
contrasts_with:
  - dom-transformation

answers_questions:
  - "How do I apply SUBD to Klangs?"
---

# Quick Definition
The formal inverse of DOM: SUBD transforms a Klang so that it becomes the subdominant of the result, preserving mode. (F, +)SUBD = (C, +) means "F major becomes the subdominant of C major."

# Core Definition
"We may continue to explore other transformations on the family of Klangs, following or modifying Riemann. We can define SUBD, the formal inverse of DOM. '(F, +)SUBD = (C, +)' means that F major becomes the subdominant of C major" (Lewin, 8.1.1, p. 177). Even though SUBD = DOM^{-1} in the group, the graphic format distinguishes a forward SUBD arrow from a backward DOM arrow, encoding different analytical meanings.

# Prerequisites
- **Klang representation** — SUBD operates on Klangs
- **DOM transformation** — SUBD is defined as DOM's inverse

# Key Properties
1. (p, sign)SUBD = (p + 7, sign) mod 12
2. SUBD = DOM^{-1}
3. Preserves mode
4. A forward SUBD arrow encodes plagal motion (IV to I)
5. Analytically distinct from a backward DOM arrow despite identical mathematical content

# Construction / Recognition
## To Construct:
1. Take any Klang (p, sign)
2. Transpose pitch class up a fifth (p + 7 mod 12)
3. Preserve the sign
## To Recognize:
1. An arrow pointing from subdominant Klang to tonic Klang
2. Mode unchanged; root moves up a fifth

# Context & Application
Analyzing a plagal cadence in C major: draw a SUBD arrow from (F, +) on the left to (C, +) on the right. For a half cadence in F major: put (F, +) on left, (C, +) on right, but draw a DOM arrow pointing leftward from (C, +) to (F, +). The arrows distinguish functional interpretation (p. 177).

# Examples
**Example 1** (p. 177): (F, +)SUBD = (C, +): F major becomes subdominant of C major. (G, -)SUBD = (D, -): G minor becomes subdominant of D minor.

# Relationships
## Builds Upon
- **DOM transformation** — SUBD is its inverse
## Related
- **SUBM transformation** — Similarly an inverse (of MED)
## Contrasts With
- **DOM transformation** — SUBD = DOM^{-1}; forward SUBD differs analytically from backward DOM

# Common Errors
- **Error**: Treating forward SUBD and backward DOM as analytically identical
  **Correction**: The arrow direction encodes analytical meaning about functional direction

# Common Confusions
- **Confusion**: Thinking SUBD changes mode like SUBM
  **Clarification**: SUBD preserves mode (like DOM); SUBM changes mode (like MED)

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.1.1, page 177.

# Verification Notes
- Definition source: Direct quotation from 8.1.1
- Confidence rationale: Explicitly defined as inverse of DOM
- Re-extraction notes: Re-extracted from v2 card; preserved: plagal cadence example, analytical vs. mathematical distinction
