---
# === CORE IDENTIFICATION ===
concept: Unrolling Interval Vector
slug: unrolling-interval-vector

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: time-span-set-theory
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.4.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - unrolling interval-vector
  - stage-wise interval vector

# === TYPED RELATIONSHIPS ===
prerequisites:
  - release-ordering
  - forwards-oriented-interval
  - emb-function
extends:
  - m-class-vector
related:
  - brahms-rhapsody-emb
  - attack-ordered-dyad
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does an interval vector develop over time in a time-span GIS?"
  - "What is the unrolling technique for interval vectors?"
---

# Quick Definition
An unrolling interval vector tracks how a set's interval vector develops over time, stage by stage, as time spans release and become fully perceived. Stages are articulated at release points, modeling the listener's evolving perception of intervallic structure.

# Core Definition
Given a set Y of time spans, the unrolling procedure (Lewin, pp. 117-119) is: (1) list Y in release-ordering; (2) identify stages at distinct release points; (3) at each stage, compute the interval vector for the subset of Y whose spans have fully released; (4) track how the vector grows. This models "how our sense of intervallic structure evolves as we listen to the musical passage" (p. 118). Lewin also suggests a computer visualization using colored dots on a half-plane grid.

# Prerequisites
- **Release-Ordering** — Stages are determined by release points
- **Forwards-Oriented Interval** — The interval vector entries
- **EMB Function** — The interval vector entries are EMB values for dyad classes

# Key Properties
1. Stages articulate at release points of successive spans
2. Multiple simultaneous releases may be grouped into one stage
3. Y_1 subset Y_2 subset ... subset Y = Y_final
4. Each new stage adds intervals involving newly-released spans
5. Care must be taken when release-ordering differs from attack-ordering
6. The technique extends to EMB for set classes of any cardinality (Example 5.4.3)

# Construction / Recognition
## To Construct:
1. List set Y = (s_1, ..., s_N) in release-ordering
2. Identify stages at distinct release points, grouping simultaneous releases
3. At each stage k, compute interval vector for Y_k
4. Display the progressive development

## To Recognize:
1. A sequence of growing interval vectors indexed by temporal stages

# Context & Application
The technique models real-time perception and can be applied to IFUNC(X, Y) as well (Note 5.4.4). Lewin suggests a color-monitor visualization where dots at (i, log p) change color as intervals accumulate, following either serial stage rhythm or "perceptual rhythm" of release points.

# Examples
**Example 1** (pp. 117-118, Figure 5.13): String trio passage with 4 stages: Stage 1 at time 18 (vn1, vn2 released), Stage 2 at 18.5 (vc1), Stage 3 at 18.75 (va1), Stage 4 at time 20 (all remaining).

# Relationships
## Builds Upon
- **Release-Ordering** — Determines stage articulation
- **M-Class Vector** — Each stage produces an interval vector (2-class vector)

## Enables
- **Brahms Rhapsody EMB Analysis** — Extended to unrolling EMB for higher set classes

# Common Errors
- **Error**: Adding intervals at attack points rather than release points
  **Correction**: A span's intervals can only be counted after it releases

# Common Confusions
- **Confusion**: Thinking the final interval vector captures all the information
  **Clarification**: The temporal trajectory of vector development is analytically significant — which intervals appear early vs. late matters

# Source Reference
Chapter 5: Generalized Set Theory (1), Example 5.4.2, Note 5.4.4, pp. 116-120.

# Verification Notes
- Definition source: Direct from section 5.4.2
- Confidence rationale: Detailed procedural description with example
- Re-extraction notes: Re-extracted from v2 card; preserved: string trio stages, computer visualization suggestion. Added v3.1 structure.
