---
# === CORE IDENTIFICATION ===
concept: TCH Interval
slug: tch-interval
# === CLASSIFICATION ===
category: analytical-applications
subcategory: motivic-analysis
tier: advanced
# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (4): Some Further Analyses"
chapter_number: 10
pdf_page: 251
section: "10.1"
# === CONFIDENCE ===
extraction_confidence: high
# === VARIANTS ===
aliases:
  - "transposition chain interval"
# === TYPED RELATIONSHIPS ===
prerequisites:
  - tch-transformation
  - rich-transformation
extends: []
related:
  - mozart-k550-development-analysis
  - bartok-syncopation-analysis
contrasts_with: []
# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a TCH interval in the context of RI-chaining?"
---

# Quick Definition
The TCH interval is the transposition distance between successive forms in an RI-chain, measuring how far each RICH-transformed motive is transposed from the previous one.

# Core Definition
In an RI-chain of motive forms linked by RICH, the TCH interval measures the transposition between successive forms. In the Mozart K.550 analysis, the TCH interval is a falling fourth (the pitch-class distance between successive PM-forms). In the Bartok analysis, the TCH interval is 5 (spanning the trichords). The TCH interval depends on the specific motive and its intervallic content (Lewin, Sections 10.1-10.2).

# Prerequisites
- **TCH transformation** — the transposition chain operation
- **RICH transformation** — the RI-chain operation that generates successive forms

# Key Properties
1. Measures transposition distance between consecutive RI-chain forms
2. Determined by the intervallic content of the motive
3. A falling fourth in the Mozart K.550 PM chain
4. Equal to 5 in the Bartok Syncopation trichord chains
5. The TCH interval is fixed for a given motive within a chain

# Construction / Recognition
## To Construct:
1. Identify consecutive forms in an RI-chain
2. Measure the transposition between them
## To Recognize:
1. Look for a constant transposition distance between successive chain members

# Context & Application
The TCH interval characterizes the "speed" of an RI-chain through pitch space. Different motives produce different TCH intervals, and the specific TCH interval shapes the harmonic trajectory of the chain.

# Examples
**Example 1** (Section 10.1, p. 252): Mozart K.550 PM chain: (E-Ab-B), (Ab-B-Eb), (B-Eb-F#), ... TCH interval = falling fourth.

**Example 2** (Section 10.2, p. 262): Bartok "Syncopation" trichord chains. TCH interval = 5.

# Relationships
## Builds Upon
- **TCH transformation** — the operation whose interval is measured
## Related
- **Mozart K.550 development analysis** — TCH interval = falling fourth
- **Bartok Syncopation analysis** — TCH interval = 5

# Common Errors
- **Error**: Assuming TCH interval is the same for all motives
  **Correction**: It depends on the specific motive's intervallic content

# Common Confusions
- **Confusion**: Confusing TCH interval with RICH interval
  **Clarification**: RICH links adjacent forms; TCH measures the net transposition between them

# Source Reference
Chapter 10, Sections 10.1-10.2, pp. 252, 262.

# Verification Notes
- Definition source: synthesized from Sections 10.1-10.2
- Confidence rationale: high -- explicitly used in multiple analyses
- Re-extracted from v2 card; preserved: falling fourth and 5 examples
