---
# === CORE IDENTIFICATION ===
concept: Durational Motive
slug: durational-motive

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
  - "DM"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - transformation-network-definition
extends: []
related:
  - pitch-motive
  - rich-relations-in-rhythm
  - multiplicative-transposition
  - multiplicative-inversion
  - mozart-k550-development-analysis
contrasts_with:
  - pitch-motive

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a durational motive?"
  - "How can rhythmic patterns undergo serial transformations?"
---

# Quick Definition
A durational motive (DM) is a specific series of time-interval durations between successive attacks that can undergo serial transformations (transposition, inversion, retrograde, RICH) parallel to those applied to pitch motives.

# Core Definition
A durational motive is a series of numerical durations representing time intervals (in beat units) between successive note attacks. Like pitch motives, durational motives can be subjected to multiplicative transposition (scaling all values), multiplicative or additive inversion, retrograde, and retrograde-inversion (RICH). The DM in the Mozart K.550 analysis is 1 + 2 + 2 (quarter-note beats), the rhythmic setting for the pitch motive PM (Lewin, Section 10.1, pp. 253-254).

# Prerequisites
- **Transformation network** — durational motives are analyzed within network structures

# Key Properties
1. Durations measured as time intervals between attacks (not note durations)
2. Multiplicative transposition Tn: multiply all values by n
3. Multiplicative inversion about product p: divide p by each value
4. Additive inversion about sum s: subtract each value from s
5. Both types of inversion can yield the same result in specific cases
6. RICH applies to durational series just as to pitch series

# Construction / Recognition
## To Construct:
1. Identify attack points in a passage
2. Measure time intervals between successive attacks (in beat units)
3. Record the resulting series as the durational motive
## To Recognize:
1. Look for recurring rhythmic patterns expressible as series of durations
2. Check for transformed forms (augmented, inverted, retrograded)

# Context & Application
Durational motives enable applying the same transformational apparatus used for pitch to the rhythmic domain. This reveals how composers create rhythmic coherence through transformational processes that parallel pitch manipulation, as demonstrated in the Mozart K.550 analysis.

# Examples
**Example 1** (Figure 10.1, pp. 253-254): DM = 1 + 2 + 2 in Mozart K.550. Transformations:
- T2(DM) = 2 + 4 + 4 (augmentation by factor 2)
- RICH(DM) = 2 + 2 + 4 (multiplicative retrograde-inversion)
- I(2 + 4 + 4) = 4 + 2 + 2 (inversion: either 8/2, 8/4, 8/4 or 6-2, 6-4, 6-4)

# Relationships
## Builds Upon
- **Transformation network** — DM is analyzed within networks
## Related
- **Pitch motive** — DM is the rhythmic counterpart to PM
- **RICH-relations in rhythm** — RICH operates on DM
- **Multiplicative transposition** — scaling all durations
- **Multiplicative inversion** — inverting durations about a product
## Contrasts With
- **Pitch motive** — PM operates on pitch intervals; DM operates on durations

# Common Errors
- **Error**: Confusing note durations with inter-attack durations
  **Correction**: DM measures time between attack points, not the lengths of individual notes

# Common Confusions
- **Confusion**: Thinking durational inversion is always additive
  **Clarification**: It can be multiplicative (division about a product) or additive (subtraction from a sum); both yield the same result in certain cases

# Source Reference
Chapter 10: Transformation Graphs and Networks (4): Some Further Analyses, Section 10.1, pp. 253-254. See Figures 10.1-10.3.

# Verification Notes
- Definition source: direct from Section 10.1 discussion
- Confidence rationale: high -- explicitly named concept with detailed transformational analysis
- Re-extracted from v2 card; preserved: dual inversion computation, DM = 1+2+2 example
