---
# === CORE IDENTIFICATION ===
concept: Webern Op. 7 No. 3 IFUNC Analysis
slug: webern-op7-analysis

# === CLASSIFICATION ===
category: analytical-applications
subcategory: webern-analysis
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: null

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - Webern Four Pieces op. 7 analysis
  - Webern violin piece IFUNC analysis

# === TYPED RELATIONSHIPS ===
prerequisites:
  - ifunc
  - set-in-gis
  - transposition-operation
extends: []
related:
  - ifunc-probability
  - inj-function
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does IFUNC reveal structural relationships in atonal music?"
  - "How can a single set be articulated in multiple valid ways?"
---

# Quick Definition
Lewin's analysis of Webern's op. 7 no. 3 demonstrates how IFUNC reveals multiple structural relationships between melodic phrases X, Y and ostinato figures Z_0, Z_3, showing that a single set can be validly articulated in several ways depending on intervallic context.

# Core Definition
The analysis examines the Four Pieces for Violin and Piano, op. 7 no. 3 (Figures 5.2-5.7), focusing on X = {Ab, Bb, Eb} (melodic phrase 1, mm. 3-4), Y = 7-note set (melodic phrase 2, mm. 5-8), and Z_0, Z_3 = violin ostinato forms. IFUNC values from Figure 5.3(a) reveal T_3 relationships between sets and multiple valid articulations of Y. The analysis was initiated by Michael Bushnell at Stony Brook.

# Prerequisites
- **IFUNC** — The primary analytical tool used
- **Set in a GIS** — X, Y, Z_0, Z_3 are sets in the pitch-class GIS
- **Transposition Operation** — T_3 and T_8 relationships are central

# Key Properties
1. IFUNC(Z_0, Z_3)(3) = 4 (maximum): Z_3 = T_3(Z_0)
2. IFUNC(X, Y)(3) = 3 (maximum): T_3(X) embeds in Y as every-third-note backbone
3. IFUNC(X, Y)(8) = 3 (maximum): T_8(X) = {E, F#, B} bounds Y registrally/temporally
4. IFUNC(X, Z_0) and IFUNC(Z_3, Y) share maxima at i = 0, 5, 6, 11: structural proportion X:Z_0 :: Z_3:Y
5. Multiple valid articulations of Y arise from different intervallic contexts

# Construction / Recognition
## To Apply This Analytical Method:
1. Identify sets of interest (melodic phrases, ostinato figures)
2. Compute IFUNC tables between all relevant pairs
3. Find maximal IFUNC values — these indicate embedding and transposition relationships
4. Explore musical significance of discovered relationships
5. Consider multiple contexts for the same set

## To Recognize:
1. IFUNC maximum = card(X) indicates complete embedding of a transposed form
2. Multiple maxima suggest multiple valid structural readings

# Context & Application
The analysis is paradigmatic for Lewin's approach: IFUNC serves both as a "precision tool" and as an exploratory device, leading the analyst to discover relationships "our ears might not otherwise pick up quickly." Crucially, Lewin argues that multiple valid articulations of Y are not contradictory but reflect different listening environments. The pianist shapes the melody differently when attending to interval-3 versus interval-8 relationships.

# Examples
**Example 1** (pp. 121-123, Figure 5.2c): T_3(X) embedded in Y — ordered T_3(X) = {B, C#, F#} appears as every third note of ordered Y, with C# receiving an agogic accent analogous to Bb in X.

**Example 2** (p. 126, Figure 5.4): T_8(X) = {E, F#, B} as boundary frame — B is first/lowest note, E is highest, F# is last note of Y.

**Example 3** (pp. 127-128, Figures 5.5-5.6): Proportion at intervals 0, 5, 6, 11 — X's fourths map to Z_0's fourths, Z_3's 3-5 trichords map to Y's boundary trichords.

# Relationships
## Builds Upon
- **IFUNC** — Primary analytical tool

## Enables
- **INJ Function** — Lewin notes that IFUNC "cannot suggest" the inversional relation at Figure 5.7; the injection function is needed

## Related
- **IFUNC Probability** — "Scarce" versus "common" intervals provide analytical backdrop

# Common Errors
- **Error**: Assuming one articulation of Y must be "correct" and others "wrong"
  **Correction**: Each articulation is valid in its specific intervallic/contextual environment

# Common Confusions
- **Confusion**: Thinking maximum IFUNC values always indicate the most important relationship
  **Clarification**: Sub-maximal but still prominent values (like the proportion at i=0,5,6,11) can reveal equally significant structures

# Source Reference
Chapter 5: Generalized Set Theory (1), Figures 5.2-5.7, pp. 121-131.

# Verification Notes
- Definition source: Synthesized from extended analytical discussion
- Confidence rationale: High — detailed analysis with musical examples and figures
- Re-extraction notes: Re-extracted from v2 card; preserved: IFUNC table values, three articulations of Y, Bushnell credit, proportion structure. Added v3.1 structure.
