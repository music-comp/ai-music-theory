---
concept: FLIPEND Transformation
slug: flipend-transformation

category: transformation-theory
subcategory: serial-operations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.3.2"

extraction_confidence: high

aliases: []

prerequisites: []
extends: []
related:
  - flipstart-transformation
contrasts_with:
  - flipstart-transformation

answers_questions:
  - "What is the FLIPEND transformation?"
---

# Quick Definition
A transformation on three-element series that inverts the last element about the second, preserving the first two: FLIPEND(s_1-s_2-s_3) = s_1-s_2-a, where a = I^{s_2}(s_3), the inversion of s_3 about s_2.

# Core Definition
"FLIPEND transforms the series s_1-s_2-s_3 into the series s_1-s_2-a, where a is the inversion-about-s_2 of s_3. (a = I^{s_2}(s_3); int(s_3,a) = int(s_2,s_3).)" Its inverse "FLIPEND^{-1} then transforms the series t_1-t_2-t_3 into the series t_1-b-t_3, where b is the inversion-about-t_3 of t_1" (Lewin, 8.3.2, p. 189). Names are Lewin's own, for transformations identified by Jonathan W. Bernard in studying Varese.

# Prerequisites
- (none specific within this source)

# Key Properties
1. Operates only on 3-element series
2. Preserves the first two elements (s_1 and s_2)
3. "Flips" the third element about the second
4. FLIPEND^{-1} preserves first and third, flips the second about the third
5. Dual to FLIPSTART

# Construction / Recognition
## To Construct:
1. Keep s_1 and s_2
2. Compute a = 2*s_2 - s_3 (inversion of s_3 about s_2)
3. Result: s_1-s_2-a
## To Recognize:
1. First two elements unchanged; third reflected about the second

# Context & Application
Bernard used these transformations to study how Varese's music "expands, contracts, and displaces registral space" (p. 189). When FLIPEND and FLIPSTART^{-1} are alternated, they create chains showing systematic registral manipulation (Figure 8.11).

# Examples
**Example 1** (Figure 8.11, p. 190): Arrows above staff show FLIPEND; arrows below show FLIPSTART^{-1}. Alternating creates chains of three-pitch series.

# Relationships
## Related
- **FLIPSTART transformation** — Dual operation acting on the series beginning
## Contrasts With
- **FLIPSTART transformation** — FLIPEND flips the end; FLIPSTART flips the start

# Common Errors
- **Error**: Applying FLIPEND to series longer than 3 elements
  **Correction**: FLIPEND is defined only for three-element series

# Common Confusions
- **Confusion**: Thinking FLIPEND and FLIPEND^{-1} are the same
  **Clarification**: FLIPEND^{-1} flips the middle element about the third, preserving first and third

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.3.2, Figure 8.11, pages 189-190.

# Verification Notes
- Definition source: Direct quotation from 8.3.2
- Confidence rationale: Explicitly defined
- Re-extraction notes: Re-extracted from v2 card; preserved: Bernard/Varese connection, inverse definition
