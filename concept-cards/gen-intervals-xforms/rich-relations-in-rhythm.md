---
# === CORE IDENTIFICATION ===
concept: RICH-Relations in Rhythm
slug: rich-relations-in-rhythm

# === CLASSIFICATION ===
category: analytical-applications
subcategory: rhythmic-analysis
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
  - "rhythmic RICH transformation"
  - "durational RI-chaining"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - rich-transformation
  - durational-motive
extends:
  - rich-transformation
related:
  - mozart-k550-development-analysis
  - multiplicative-inversion
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does RICH apply to durational patterns?"
  - "How can pitch and rhythm share the same transformational logic?"
---

# Quick Definition
RICH-relations in rhythm apply the retrograde-inversion chaining operation to durational series, revealing how rhythmic motives transform through the same RI-chain processes used for pitch motives.

# Core Definition
The RICH transformation applied to a durational series produces a retrograde-inversion that preserves the linking (chaining) property: the end of the original series shares elements with the beginning of the RICH-transformed series. In the durational domain, RICH can be defined multiplicatively (about a product) or additively (about a sum). The "musical effect is bewildering on first hearing, because both the durational series involved in the RICH relation... are heard as alternative rhythmic settings for one and the same pitch motive" (Lewin, Section 10.1, p. 254).

# Prerequisites
- **RICH transformation** — the pitch-domain operation being applied to rhythm
- **Durational motive** — the rhythmic operand

# Key Properties
1. RICH operates on durational series just as on pitch series
2. The linking property is preserved: end of source shares elements with start of RICH-transform
3. Can be computed multiplicatively or additively
4. Demonstrates that pitch and rhythmic domains can share the same transformational logic
5. Creates "bewildering" aural effects when dual RI-chaining occurs

# Construction / Recognition
## To Construct:
1. Take a durational series
2. Invert multiplicatively (about a product) or additively (about a sum)
3. Retrograde the result
4. The output is the RICH-transform
## To Recognize:
1. Look for durational series that are retrograde-inversions of preceding series
2. Check for the linking (shared elements) property between adjacent series

# Context & Application
RICH-relations in rhythm demonstrate that the transformational logic governing pitch structure can simultaneously govern rhythmic structure. This reveals a deeper level of compositional coherence where pitch and rhythm are unified through the same transformational processes, as demonstrated in the Mozart K.550 analysis.

# Examples
**Example 1** (Figures 10.2-10.3, pp. 255-256): In Mozart K.550:
- DM = 1 + 2 + 2; RICH(DM) = 2 + 2 + 4 (series 5)
- Series 4a = 4 + 4 + 2; RICH(series 4a) = 4 + 2 + 2 (series 4b)
- The RICH-arrow between DM and series 5 can be heard by focusing on rhythmic identity of measure 127 with measure 132

# Relationships
## Builds Upon
- **RICH transformation** — the pitch-domain operation being extended
- **Durational motive** — the rhythmic operand
## Related
- **Mozart K.550 development analysis** — the analytical context
- **Multiplicative inversion** — the inversion component of rhythmic RICH

# Common Errors
- **Error**: Applying retrograde and inversion separately without recognizing the chaining property
  **Correction**: RICH specifically requires the linking (overlapping) property between adjacent forms

# Common Confusions
- **Confusion**: Thinking RICH in rhythm works identically to RICH in pitch
  **Clarification**: The principle is the same (retrograde-inversion with linking), but the inversion can be multiplicative or additive in the durational domain, whereas pitch-class inversion is always additive mod 12

# Source Reference
Chapter 10: Transformation Graphs and Networks (4): Some Further Analyses, Section 10.1, pp. 254-256. See Figures 10.2-10.3.

# Verification Notes
- Definition source: direct from Section 10.1 discussion
- Confidence rationale: high -- explicitly analyzed with network figures
- Re-extracted from v2 card; preserved: "bewildering" quote, DM/series 5 RICH example, linking property
