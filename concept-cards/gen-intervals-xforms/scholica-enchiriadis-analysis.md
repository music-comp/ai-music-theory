---
# === CORE IDENTIFICATION ===
concept: Scholica Enchiriadis Analysis
slug: scholica-enchiriadis-analysis

# === CLASSIFICATION ===
category: analytical-applications
subcategory: medieval-analysis
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.5.5"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Nos qui vivimus analysis"
  - "Symphony of the Diatesseron analysis"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - graph-homomorphism
  - network-of-networks
extends: []
related:
  - product-networks
  - formal-melody
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does Lewin analyze medieval parallel organum using transformation networks?"
  - "How do homomorphisms, product networks, and networks-of-networks differ?"
---

# Quick Definition
An analysis of "Nos qui vivimus" from the Scholica Enchiriadis (Symphony of the Diatesseron), demonstrating graph homomorphisms, product networks, and two types of networks-of-networks applied to medieval parallel organum.

# Core Definition
The analysis (Figure 9.8, Section 9.5.5) examines parallel organum at the fourth using multiple network models: (b) the melody graph; (c) a disconnected network with separate Principalis and Organalis lines; (d) a product network; (f) a network-of-networks with melody as outer graph and diatesseron as inner; (g) a network-of-networks with diatesseron as outer and melody as inner. Graph (b) is a homomorphic image of (c) but NOT of (d) -- no SGMAP can satisfy both SGMAP(1) = 1 and SGMAP(3) = 0 (Lewin, pp. 236-239).

# Prerequisites
- **Graph homomorphism** — the relationship between (c) and (b)
- **Network of networks** — models (f) and (g)

# Key Properties
1. Multiple valid network models for the same music
2. Homomorphism from (c) onto (b): NODEMAP collapses voice pairs, SGMAP = identity
3. Non-homomorphism from (d) to (b): algebraically impossible
4. Model (f): "singing the melody, singing diatessera as we go"
5. Model (g): "Principalis sings the melody; I sing it in diatesseron relation"
6. T3 (modal steps) differs from RISE(4/3) (harmonic ratio) -- a salient problem of the style

# Construction / Recognition
## To Construct:
1. Transcribe the organum
2. Build separate network models (melody, voices, product, networks-of-networks)
3. Test homomorphism relationships between them
## To Recognize:
1. Identify parallel motion at a fixed interval
2. Test whether collapsing voices preserves graph structure

# Context & Application
This analysis demonstrates the variety of network models applicable to a single passage and the formal relationships between them. The distinction between models (f) and (g) reflects genuinely different ways of hearing the organum. The T3/RISE(4/3) distinction relates to intonation problems of medieval style.

# Examples
**Example 1** (Figure 9.8, pp. 236-239): Melody intervals: 1, -1, 0, 0 (step motion). Diatesseron interval: 3. Product network has arrows labeled 1, -1, 0, 3. The non-homomorphism proof: SGMAP(1) = 1 forces SGMAP(3) = 3, but graph (b) would require SGMAP(3) = 0. Contradiction.

# Relationships
## Builds Upon
- **Graph homomorphism** — tested between various models
- **Network of networks** — models (f) and (g)
## Related
- **Product networks** — model (d)
- **Formal melody** — graph (b) models a series as a network

# Common Errors
- **Error**: Assuming all network models of the same music are homomorphically related
  **Correction**: The non-homomorphism (d) -> (b) demonstrates that distinct models need not be structurally reducible to each other

# Common Confusions
- **Confusion**: Thinking (f) and (g) model the same thing
  **Clarification**: They represent genuinely different analytical perspectives on the same music

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.5.5, pp. 236-239. See Figure 9.8.

# Verification Notes
- Definition source: direct from Section 9.5.5 discussion
- Confidence rationale: high -- extended worked example with formal proofs
- Re-extracted from v2 card; preserved: non-homomorphism proof, T3/RISE(4/3) distinction, multiple model perspectives
