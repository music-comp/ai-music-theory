---
# === CORE IDENTIFICATION ===
concept: Brahms Rhapsody Unrolling EMB Analysis
slug: brahms-rhapsody-emb

# === CLASSIFICATION ===
category: analytical-applications
subcategory: rhythmic-analysis
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.4.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - emb-function
  - release-ordering
  - set-class
extends:
  - unrolling-interval-vector
related:
  - attack-ordered-dyad
  - forwards-oriented-interval
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can EMB be unrolled over time to model evolving perception?"
---

# Quick Definition
The unrolling EMB analysis of Brahms's G-Minor Rhapsody tracks how embedding numbers for competing set classes /X1/ and /X2/ develop over rhythmic stages, revealing that /X2/ "comes on late and strong."

# Core Definition
Example 5.4.3 (Lewin, pp. 119-120, Figures 5.14-5.15): The set Y models temporal aspects of the Rhapsody's opening. Y is articulated into four stages based on release points. At each stage, EMB(/X1/, Y) and EMB(/X2/, Y) are computed, tracking how the listener's perception of rhythmic set-class dominance evolves.

# Prerequisites
- **EMB Function** — The embedding numbers being tracked
- **Release-Ordering** — Stages are determined by when spans end, not when they begin
- **Set Class** — /X1/ and /X2/ are competing rhythmic set classes

# Key Properties
1. Stage 1: EMB(/X1/) = 1, EMB(/X2/) = 1 (tied)
2. Stage 2: EMB(/X1/) = 2, EMB(/X2/) = 2 (still tied)
3. Stage 3: EMB(/X1/) = 2, EMB(/X2/) = 3 (/X2/ pulls ahead)
4. Stage 4: EMB(/X1/) = 3, EMB(/X2/) = 5 (/X2/ decisively ahead)
5. /X2/ dominance is reinforced at the closing group, where the theme liquidates to /X2/-forms

# Construction / Recognition
## To Construct an Unrolling EMB Analysis:
1. Model the passage as a set Y of time spans
2. Articulate Y into stages at release points
3. At each stage, compute EMB for the set classes of interest
4. Track how values evolve across stages

## To Recognize:
1. A time-series of EMB values showing set-class "competition"

# Context & Application
The technique models real-time perception: we cannot know a time span's duration until it ends, so EMB values update at release points. The analysis shows that augmented (canonical) forms X1' and X2' only "count" once the dotted half note releases at Stage 4. The rhythmic reading is not exclusive of other interpretations.

# Examples
**Example 1** (Figures 5.14-5.15): Y models the opening of Brahms's G-Minor Rhapsody. X1' and X2' are augmented canonical forms found within Y. The dotted half note releasing at Stage 4 adds two forms of X2 but only one of X1, creating the decisive /X2/ advantage. This is "reinforced by the end of the closing group in the music, where the closing theme is liquidated rhythmically down to a succession of X2-forms alternating on the tonic and dominant of D minor."

# Relationships
## Builds Upon
- **EMB Function** — The values being tracked
- **Release-Ordering** — Determines stage articulation

## Enables
- Understanding of how rhythmic structure can be analyzed through set-theoretic methods

## Related
- **Unrolling Interval Vector** — Analogous technique for interval vectors

# Common Errors
- **Error**: Computing EMB for the complete set without tracking temporal development
  **Correction**: The unrolling reveals which set class dominates and when, information lost in a static EMB computation

# Common Confusions
- **Confusion**: Thinking the unrolling reveals "the" rhythmic structure
  **Clarification**: It reveals one reading; Lewin notes that other rhythmic interpretations (e.g., reading triplet eighths for triplet rests) are also valid

# Source Reference
Chapter 5: Generalized Set Theory (1), Example 5.4.3, Figures 5.14-5.15, pp. 119-120.

# Verification Notes
- Definition source: Direct from Example 5.4.3
- Confidence rationale: Detailed worked example with figures
- Re-extraction notes: Re-extracted from v2 card; preserved: four-stage EMB values, closing group connection, alternative interpretations note. Added v3.1 structure.
