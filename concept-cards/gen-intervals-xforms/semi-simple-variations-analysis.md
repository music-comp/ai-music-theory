---
concept: Semi-Simple Variations INJ Analysis
slug: semi-simple-variations-analysis

category: analytical-applications
subcategory: serial-analysis
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.2.4"

extraction_confidence: high

aliases: []

prerequisites:
  - inj-function
  - protocol-pairs
  - partial-ordering
extends: []
related:
  - signature-motive
  - if-only-adjustment
contrasts_with: []

answers_questions:
  - "How does INJ on PROT reveal structural differences in a homogeneous-sounding piece?"
---

# Quick Definition
Lewin's analysis of Babbitt's Semi-Simple Variations uses INJ on protocol-pair space PROT to measure how SATB aggregates relate to row forms, revealing that Variation 3 is maximally compatible with the row and that Variations 4 and 2 share a unique "pivot aggregate" relationship.

# Core Definition
The analysis (Lewin, pp. 169-172, Figures 6.7-6.9) uses L_2 = the row and X_2 = SATB aggregate from Variation 3 (12 pairs in PROT). INJ(L_2, X_2)(f) = 11 for f = T_1, RT_1, J, RT_7J (near maximum of 12). Other variations' aggregates achieve at most 10. Cross-variation INJ(V_m, V_n)(T_0) is typically <= 2, except INJ(V_4, V_2)(T_0) = 5 (unique maximum), revealing a "pivot aggregate" shared between Variations 4 and 2.

# Prerequisites
- **INJ Function** — Primary analytical tool
- **Protocol Pairs** — Space in which the analysis operates
- **Partial Ordering** — Aggregates are partial orderings in PROT

# Key Properties
1. Variation 3 aggregates are maximally compatible with row forms (11/12)
2. Other variations' aggregates fit at most 10/12
3. Cross-variation "ordering cross-talk" is low (INJ <= 2) with two exceptions
4. INJ(V_4, V_2)(T_0) = 5 reveals unique connection via "pivot aggregate"
5. The pivot aggregate {D#-B-E, Ab-C-G, C#-F#-D, Bb-F-A} controls specific voices

# Construction / Recognition
## To Apply This Method:
1. Model SATB aggregates as partial orderings in PROT
2. Compute INJ(L, X)(f) for row forms f of interest
3. Compare INJ values across variations to find structural distinctions
4. Look for unique cross-variation connections via high INJ(V_m, V_n)(T_0)

## To Recognize:
1. INJ-based structural differentiation in an otherwise homogeneous-sounding piece

# Context & Application
This analysis demonstrates INJ's power to make "very useful" structural discriminations "within a composition that sounds at first extremely homogeneous in texture throughout" (Lewin, p. 171). The if-only adjustments (11/12 instead of 12) and the pivot aggregate provide concrete structural insights.

# Examples
**Example 1** (pp. 169-170, Figures 6.7-6.8): X_2 fits "11/12" in T_1(L_2) and J(L_2). In T_1(L_2), only (C#, F#) is reversed. "If only" the tenor went E-F#-C# instead of E-C#-F#, embedding would be perfect.

**Example 2** (pp. 171-172, Figure 6.9): The pivot aggregate controls tenor/bass of Variation 2 and soprano/alto of Variation 4, explaining their unique INJ = 5 connection. Variations share 3-note linear segments from the pivot aggregate.

# Relationships
## Builds Upon
- **INJ Function** on **Protocol Pairs** — The analytical framework

## Enables
- Understanding of structural differentiation in serial music

## Related
- **If-Only Adjustment** — 11/12 embeddings invite if-only analysis

# Common Errors
- **Error**: Concluding that 11/12 means "almost the same as a row"
  **Correction**: The specific pair that fails reveals compositional structure

# Common Confusions
- **Confusion**: Thinking INJ analysis requires hearing all 132 PROT pairs
  **Clarification**: INJ values summarize aggregate-level relationships; the analyst need not enumerate all pairs

# Source Reference
Chapter 6: Generalized Set Theory (2), Example 6.2.4, Figures 6.7-6.9, pp. 167-172.

# Verification Notes
- Definition source: Synthesized from extended analytical discussion
- Confidence rationale: Detailed analysis with figures and specific INJ values
- Re-extraction notes: Re-extracted from v2 card; preserved: all key INJ values, pivot aggregate, if-only analysis. Added v3.1 structure.
