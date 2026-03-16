---
concept: FLIPSTART Transformation
slug: flipstart-transformation

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
  - flipend-transformation
contrasts_with:
  - flipend-transformation

answers_questions:
  - "What is the FLIPSTART transformation?"
---

# Quick Definition
A transformation on three-element series that inverts the first element about the second, preserving the last two: FLIPSTART(s_1-s_2-s_3) = a-s_2-s_3, where a = I^{s_2}(s_1).

# Core Definition
"Dually, FLIPSTART transforms s_1-s_2-s_3 into a-s_2-s_3, where a is the inversion-about-s_2 of s_1; then FLIPSTART^{-1} transforms t_1-t_2-t_3 into t_1-b-t_3, where b is the inversion-about-t_1 of t_3" (Lewin, 8.3.2, p. 189).

# Prerequisites
- (none specific within this source)

# Key Properties
1. Operates only on 3-element series
2. Preserves the last two elements (s_2 and s_3)
3. "Flips" the first element about the second
4. FLIPSTART^{-1} preserves first and third, flips the second about the first
5. Dual to FLIPEND (acting on opposite end of series)

# Construction / Recognition
## To Construct:
1. Keep s_2 and s_3
2. Compute a = 2*s_2 - s_1 (inversion of s_1 about s_2)
3. Result: a-s_2-s_3
## To Recognize:
1. Last two elements unchanged; first reflected about the second

# Context & Application
When alternated with FLIPEND, creates chains of three-pitch series modeling registral expansion, contraction, and displacement in Varese's music (Bernard's analysis formalized by Lewin).

# Examples
**Example 1** (Figure 8.11, p. 190): Two starting series produce different chains when FLIPEND and FLIPSTART^{-1} alternate.

# Relationships
## Related
- **FLIPEND transformation** — Dual operation acting on the series ending
## Contrasts With
- **FLIPEND transformation** — FLIPSTART flips the start; FLIPEND flips the end

# Common Errors
- **Error**: Thinking FLIPSTART and FLIPEND are inverse operations
  **Correction**: They are "dual" (acting on opposite ends), not inverses

# Common Confusions
- **Confusion**: Assuming these only apply to atonal music
  **Clarification**: While motivated by Varese analysis, the operations are defined abstractly for any three-element series

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.3.2, Figure 8.11, page 189.

# Verification Notes
- Definition source: Direct from 8.3.2
- Confidence rationale: Explicitly defined
- Re-extraction notes: Re-extracted from v2 card; preserved: duality with FLIPEND, inverse definition
