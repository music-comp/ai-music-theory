---
# === CORE IDENTIFICATION ===
concept: Multiplicative Transposition
slug: multiplicative-transposition

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
  - "rhythmic augmentation"
  - "rhythmic diminution"
  - "T_k (durational)"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - durational-motive
extends: []
related:
  - multiplicative-inversion
  - mozart-k550-development-analysis
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is multiplicative transposition of a durational series?"
  - "How are augmentation and diminution formalized?"
---

# Quick Definition
Multiplicative transposition scales all duration values in a rhythmic series by a constant factor k, formalizing augmentation (k > 1) and diminution (k < 1) as transformational operations on durational motives.

# Core Definition
Given a durational series S = (d1, d2, ..., dn), multiplicative transposition by factor k produces Tk(S) = (k*d1, k*d2, ..., k*dn). The operation preserves proportional relationships while scaling absolute values. Tk is an isomorphism: Tk(Tm(S)) = T_{km}(S); T1 is the identity; T_{1/k} is the inverse of Tk (Lewin, Section 10.1, pp. 253-256).

# Prerequisites
- **Durational motive** — the operand of multiplicative transposition

# Key Properties
1. Augmentation: k > 1 (e.g., T2 doubles all durations)
2. Diminution: k < 1 (e.g., T_{1/2} halves all durations)
3. Preserves proportional relationships between durations
4. Differs from additive pitch transposition (which adds a constant)
5. Forms a group: Tk * Tm = T_{km}

# Construction / Recognition
## To Construct:
1. Multiply each duration in the series by factor k
## To Recognize:
1. Check if all durations in one series are a constant multiple of another

# Context & Application
Multiplicative transposition formalizes traditional augmentation and diminution. In the Mozart K.550 analysis, T2 transforms DM = 1+2+2 into series 3 = 2+4+4. Later, T_{1/2} transforms series 6 into series 7, "undoing the effect of the earlier augmentation."

# Examples
**Example 1** (Section 10.1, pp. 253-256): DM = 1+2+2; T2(DM) = 2+4+4 (augmentation). Series 7 = 2+1+1 = T_{1/2}(series 6 = 4+2+2) (diminution undoing the augmentation).

# Relationships
## Builds Upon
- **Durational motive** — the operand
## Related
- **Multiplicative inversion** — the other durational transformation
- **Mozart K.550 development analysis** — primary example

# Common Errors
- **Error**: Confusing multiplicative transposition (rhythm) with additive transposition (pitch)
  **Correction**: Pitch Tn adds n to each value; durational Tk multiplies each value by k

# Common Confusions
- **Confusion**: Thinking augmentation/diminution is always by factor 2
  **Clarification**: Any factor k is valid; the Mozart analysis uses T2 but other values are possible

# Source Reference
Chapter 10: Transformation Graphs and Networks (4): Some Further Analyses, Section 10.1, pp. 253-256. See Figures 10.2-10.3.

# Verification Notes
- Definition source: direct from Section 10.1
- Confidence rationale: high -- explicitly defined with worked examples
- Re-extracted from v2 card; preserved: DM transformation chain, inverse relationship
