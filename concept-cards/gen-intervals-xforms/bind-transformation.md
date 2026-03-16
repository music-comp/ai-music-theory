---
concept: BIND Transformation
slug: bind-transformation

category: transformation-theory
subcategory: serial-transformations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.6.4"

extraction_confidence: high

aliases: []

prerequisites:
  - rich-transformation
extends: []
related:
  - tch-transformation
  - wagner-todesverkuendigung-analysis
  - fate-motive
  - input-node
contrasts_with:
  - rich-transformation

answers_questions:
  - "What is the BIND transformation?"
  - "How does BIND differ from RICH?"
---

# Quick Definition
BIND takes a pitch-class series and transforms it into that retrograde-inverted form which preserves the same first and last notes as the original, "binding" the series to its endpoints.

# Core Definition
BIND operates on a pitch-class series s: BIND(s) is that retrograde-inverted form of s which has the same first and last notes as s. If s begins on x and ends on y, then BIND(s) also begins on x and ends on y. BIND commutes with TCH, a property crucial for constructing the FATE motive network (Lewin, Section 9.6.4, pp. 239-240).

# Prerequisites
- **RICH transformation** — BIND is related but distinct (different endpoint constraints)

# Key Properties
1. Preserves first AND last notes of the series
2. The interior is retrograde-inverted
3. BIND commutes with TCH
4. Differs from RICH (which constrains the first two notes of the output, not last)
5. Used to connect melodic and bass forms sharing the same endpoints

# Construction / Recognition
## To Construct:
1. Given series s with first note x and last note y
2. Find the RI form of s that also begins on x and ends on y
## To Recognize:
1. Two series sharing first and last notes that are RI-related

# Context & Application
BIND appears in the Wagner Todesverkuendigung analysis (Figure 9.11), where diagonal arrows connect bass FATE forms to melodic FATE forms. The bass form A-C-B and a melodic form sharing endpoints A and B are related by BIND. The input node (A-C-B) at lower left has "special generative function."

# Examples
**Example 1** (Figure 9.11, pp. 239-240): BIND connects FATE motive forms across bass and melody chains. The horizontal arrows are RICH; the curved arrows are TCH; the diagonal arrows are BIND. BIND commutes with TCH, ensuring the network is well-formed.

# Relationships
## Builds Upon
- **RICH transformation** — related serial transformation
## Related
- **TCH transformation** — commutes with BIND
- **Wagner Todesverkuendigung analysis** — primary example
- **FATE motive** — the musical operand
- **Input node** — A-C-B as unique input demonstrates generative function
## Contrasts With
- **RICH transformation** — RICH constrains first two notes of output; BIND constrains first and last

# Common Errors
- **Error**: Confusing BIND with RICH
  **Correction**: BIND preserves first AND last notes; RICH preserves only the linking overlap

# Common Confusions
- **Confusion**: Thinking BIND always exists for any series
  **Clarification**: The specific RI form matching both endpoints must exist; this is not guaranteed for arbitrary series

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.6.4, pp. 239-240. See Figure 9.11.

# Verification Notes
- Definition source: direct from Section 9.6.4
- Confidence rationale: high -- explicitly defined in context
- Re-extracted from v2 card; preserved: FATE motive examples, commutativity with TCH, contrast with RICH
