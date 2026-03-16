---
concept: TLAST Transformation
slug: tlast-transformation

category: transformation-theory
subcategory: serial-operations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.3.1"

extraction_confidence: high

aliases: []

prerequisites:
  - rich-transformation
extends: []
related:
  - tfirst-transformation
contrasts_with:
  - tfirst-transformation

answers_questions:
  - "What is the TLAST transformation?"
---

# Quick Definition
A serial transformation that transposes a series by its last interval, making the last note of the original become the next-to-last note of the transformed series. TLAST is "dual" to TFIRST^{-1}.

# Core Definition
"The operation TLAST transposes a series by its last interval... TLAST makes the last note of a given series the next-to-last note of the transformed series; TFIRST^{-1} has a sort of 'dual' effect, in that it makes the first note of a given series the second note of the transformed series" (Lewin, 8.3.1, p. 188).

# Prerequisites
- **RICH transformation** — Context for serial transformations

# Key Properties
1. TLAST(s) = T_i(s) where i = int(s_{N-1}, s_N)
2. TLAST makes last note become next-to-last of result
3. TLAST is "dual" to TFIRST^{-1}
4. Series-dependent transposition

# Construction / Recognition
## To Construct:
1. Compute i = int(s_{N-1}, s_N), the last interval of s
2. TLAST(s) = T_i(s)
## To Recognize:
1. The next-to-last note of the result equals the last note of the original

# Context & Application
In Webern's op. 5, no. 4, TLAST and TFIRST^{-1} connect forms of the FLYAWAY motive in a network where the Ab form is "central." The visual centrality portrays the Ab form's cadential function (it ends the piece), "balanced" between the other two forms. "While it would be perfectly possible to label the arrows of figure 8.10 as 'T_5' and 'T_2' rather than TFIRST^{-1} and TLAST, the transpositional labels would conceal, not reveal, the balancing centrality" (p. 188).

# Examples
**Example 1** (Figure 8.10, p. 188): FLYAWAY forms C-E-F#-B-C#-G-Bb, Ab-C-D-G-A-Eb-F#, F-A-B-E-F#-C-Eb connected by TFIRST^{-1} and TLAST.

# Relationships
## Related
- **TFIRST transformation** — Dual operation
## Contrasts With
- **TFIRST transformation** — TLAST acts on ending; TFIRST on beginning

# Common Errors
- **Error**: Using T_5 and T_2 instead of TFIRST^{-1} and TLAST
  **Correction**: Specific transposition labels hide the balancing centrality

# Common Confusions
- **Confusion**: Thinking the visual centrality of the Ab form is arbitrary
  **Clarification**: It captures the cadential (piece-ending) function transformationally

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.3.1, Figure 8.10, page 188.

# Verification Notes
- Definition source: Direct from 8.3.1
- Confidence rationale: Explicitly defined
- Re-extraction notes: Re-extracted from v2 card; preserved: FLYAWAY series, balancing centrality argument
