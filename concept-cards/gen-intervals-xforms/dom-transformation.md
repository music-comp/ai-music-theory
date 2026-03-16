---
# === CORE IDENTIFICATION ===
concept: DOM Transformation
slug: dom-transformation

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
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - klang-representation
extends: []
related:
  - subd-transformation
  - med-transformation
  - riemann-function-theory-critique
contrasts_with:
  - subd-transformation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I apply DOM to Klangs?"
  - "Why does Lewin define DOM as 'become the dominant of' rather than 'take the dominant'?"
---

# Quick Definition
A Klang transformation where the input Klang becomes the dominant of its output: (C, +)DOM = (F, +) means "C major becomes the dominant of F major." DOM transposes by the inverse of the dominant interval while preserving mode.

# Core Definition
"DOM is transposition by the inverse of the dominant interval. Thus (p, sign)DOM = (q, sign), where q is that pitch class of which p is the dominant. We can read this equation as telling us that (p, sign) becomes the dominant of (q, sign)" (Lewin, 8.1.1, p. 176). Right orthography means "being at (C, +) and following an arrow labeled DOM we arrive at (F, +); that is, (C, +)DOM = (F, +)" (p. 176).

# Prerequisites
- **Klang representation** — DOM operates on Klangs

# Key Properties
1. (p, sign)DOM = (p + 5, sign) mod 12
2. Preserves mode: major stays major, minor stays minor
3. DOM arrows point from dominant to tonic (showing functional dependency)
4. SUBD = DOM^{-1} (formal inverse)
5. DOM = MED^2 (two applications of MED equal one DOM)
6. Lewin's definition reverses the "usual" DOM' where (F, +)DOM' = (C, +)

# Construction / Recognition
## To Construct:
1. Take any Klang (p, sign)
2. Transpose pitch class down a fifth (p + 5 mod 12)
3. Preserve the sign
## To Recognize:
1. An arrow pointing from a dominant Klang to its tonic Klang
2. Mode is unchanged across the arrow

# Context & Application
Lewin's "unusual definition of DOM is what makes the graphs move naturally" (p. 177). DOM arrows point dependent Klangs at the local tonic they serve, matching harmonic intuition. This contrasts with Riemann's implicit DOM' (tonics generating dominants), where "the dominants just sit around, not going anywhere" (p. 177).

# Examples
**Example 1** (p. 176): (C, +)DOM = (F, +): C major becomes dominant of F major. (G, -)DOM = (C, -): G minor becomes dominant of C minor.

**Example 2** (Figure 8.1, p. 177): The normative network of Figure 7.9 rewritten with Klang transformations; DOM arrows drive the network "in a natural musical way."

# Relationships
## Builds Upon
- **Klang representation** — DOM is defined on Klangs
## Enables
- **Riemann function theory critique** — DOM vs. DOM' illuminates Riemann's conceptual flaw
## Related
- **MED transformation** — DOM = MED^2
## Contrasts With
- **SUBD transformation** — SUBD = DOM^{-1}

# Common Errors
- **Error**: Defining DOM as "take the dominant" (DOM')
  **Correction**: DOM means "become the dominant of"; (C, +)DOM = (F, +), not (G, +)

# Common Confusions
- **Confusion**: Thinking DOM and SUBD are functionally identical (just inverses)
  **Clarification**: While mathematically SUBD = DOM^{-1}, the graphic format distinguishes a forward SUBD arrow from a backward DOM arrow, encoding different analytical meanings

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.1.1, pages 176-177.

# Verification Notes
- Definition source: Direct quotation from 8.1.1
- Confidence rationale: Explicit formal definition with examples
- Re-extraction notes: Re-extracted from v2 card; preserved: DOM vs. DOM' distinction, MED^2 relationship, graph direction note
