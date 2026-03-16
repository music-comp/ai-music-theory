---
# === CORE IDENTIFICATION ===
concept: TFIRST Transformation
slug: tfirst-transformation

# === CLASSIFICATION ===
category: transformation-theory
subcategory: serial-operations
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.3.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - rich-transformation
extends: []
related:
  - tlast-transformation
contrasts_with:
  - tlast-transformation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the TFIRST transformation?"
---

# Quick Definition
A serial transformation that transposes a series by its first interval. Its inverse TFIRST^{-1} transposes by the complement of the first interval, making the first note of the original become the second note of the transformed series.

# Core Definition
"The operation TFIRST transposes a series by its first interval... TFIRST^{-1} transposes a series by the complement of its first interval. TFIRST^{-1} has a sort of 'dual' effect [to TLAST], in that it makes the first note of a given series the second note of the transformed series" (Lewin, 8.3.1, p. 188).

# Prerequisites
- **RICH transformation** — Context for understanding serial transformations

# Key Properties
1. TFIRST(s) = T_i(s) where i = int(s_1, s_2)
2. TFIRST^{-1}(s) = T_{-i}(s), the complement of the first interval
3. TFIRST^{-1} makes the first note become the second note of the result
4. TFIRST^{-1} is "dual" to TLAST (acting on opposite ends of the series)
5. Series-dependent: the transposition varies by series

# Construction / Recognition
## To Construct:
1. Compute i = int(s_1, s_2), the first interval of series s
2. TFIRST(s) = T_i(s)
## To Recognize:
1. Transposition where the second note of the result equals the first note of the original (for TFIRST^{-1})

# Context & Application
TFIRST and TLAST appear in Webern's op. 5, no. 4, where they reveal the "balancing centrality" of the Ab form of the FLYAWAY motive, which ends the piece.

# Examples
**Example 1** (Figure 8.10, p. 188): Three FLYAWAY forms connected by TFIRST^{-1} and TLAST. The Ab form is "central," balanced between the others.

# Relationships
## Related
- **TLAST transformation** — Dual operation acting on the series ending
## Contrasts With
- **TLAST transformation** — TFIRST acts on the beginning; TLAST on the ending

# Common Errors
- **Error**: Using specific transposition numbers instead of TFIRST/TLAST
  **Correction**: "The transpositional labels would conceal, not reveal, the balancing centrality" (p. 188)

# Common Confusions
- **Confusion**: Thinking TFIRST and TFIRST^{-1} are the same
  **Clarification**: TFIRST transposes by the first interval; TFIRST^{-1} by its complement

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.3.1, Figure 8.10, page 188.

# Verification Notes
- Definition source: Direct from 8.3.1
- Confidence rationale: Explicitly defined
- Re-extraction notes: Re-extracted from v2 card; preserved: duality with TLAST, FLYAWAY example
