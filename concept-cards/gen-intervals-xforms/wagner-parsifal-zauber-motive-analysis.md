---
# === CORE IDENTIFICATION ===
concept: Wagner Parsifal Zauber Motive Analysis
slug: wagner-parsifal-zauber-motive-analysis

# === CLASSIFICATION ===
category: analytical-applications
subcategory: interval-transposition-analysis
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (1): Intervals and Transpositions"
chapter_number: 7
pdf_page: 188
section: "7.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "Parsifal transformation music analysis"
  - "Zauber motive analysis"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - intervals-as-transpositions
  - ri-chaining
  - rich-transformation
extends: []
related:
  - structural-sequencing
  - tch-transformation
  - dispersive-interval
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does the Zauber motive's intervallic structure govern large-scale tonal organization in Parsifal?"
  - "How does RI-chaining create structural sequencing?"
---

# Quick Definition
An extended analysis showing how the Zauber (Magic) motive's intervallic structure governs the plan of modulations in Wagner's Parsifal Act 1 transformation music, demonstrating RI-chaining, structural sequencing, and the unity of intervallic and transpositional phenomena.

# Core Definition
The Zauber motive (Figure 7.2) is a four-note pitch-class series appearing during the Act 2 kiss. Its intervallic structure -- an overall progression of 7, subarticulated into two subprogressions of 3 (Figure 7.5) -- governs the modulatory plan of the Act 1 transformation music (Figure 7.4). Four forms of Zauber (Z_1 through Z_4) are linked by RI-chaining, where each form is the retrograde inversion beginning with the final two notes of the previous form. The TCH interval of 10 creates structural sequencing: Z_3 = T_{10}(Z_1) and Z_4 = T_{10}(Z_2) (Lewin, 7.2, pp. 161-165).

# Prerequisites
- **Intervals as transpositions** — The analysis demonstrates this equivalence
- **RI-chaining** — The technique linking successive Z-forms
- **RICH transformation** — The specific operation connecting Z-forms

# Key Properties
1. Zauber series: Ab-Cb-Ebb-Eb (as appearing at Act 2 kiss)
2. Intervallic structure: overall interval 7, subarticulated into 3 + 3
3. RI-chaining links Z_1, Z_2, Z_3, Z_4 via RICH
4. TCH interval = 10 (dispersive: INJ(Z, Z)(T_{10}) = 0)
5. Open noteheads through m. 1140 form a non-repeating ten-note series (F# and B missing)
6. Bell motive has complementary structure: overall -3, subarticulated into 7 + 7

# Construction / Recognition
## To Construct:
1. Identify the Zauber series and its intervallic network
2. Apply RICH repeatedly to generate Z_1 through Z_4
3. Compute TCH interval: int(s_1, s_N) + int(s_2, s_{N-1})
4. Map Z-forms onto principal local tonics in the transformation music
## To Recognize:
1. The same intervallic relationships appear in the motive's structure and the modulatory scheme
2. Alternate Z-forms are related by transposition at the TCH interval

# Context & Application
The analysis shows that the Motive of Faith introduction (Figure 7.3) is already governed by Z's intervallic structure "long before Z has appeared in the foreground" (p. 161). The modulatory scheme is neither prolongational nor Schenkerian; Lewin prefers "modulations" precisely because Schenker rejected the term. After m. 1140, the Bell motive takes over Z's organizing function, its interval structure complementary to Zauber's.

# Examples
**Example 1** (Figure 7.3, p. 161): The phrase introducing the Motive of Faith in the Act 1 Prelude modulates through keys Ab-Cb-Ebb-Eb, matching Z's pitch classes.

**Example 2** (Figure 7.4, pp. 162-163): The transformation music organizes principal local tonics by Z-forms. Measures 1096-1140 structurally sequence measures 1074-1100 at T_{10}.

**Example 3** (Figure 7.5, p. 164): Bell motive (overall -3, sub-intervals 7, 7) vs. Zauber (overall 7, sub-intervals 3, 3) -- complementary intervallic structures.

# Relationships
## Builds Upon
- **RI-chaining** — The technique generating successive Z-forms
- **RICH transformation** — Links each Z-form to its successor
## Enables
- **Structural sequencing** — Demonstrated by the T_{10} relationship between Z-form pairs
## Related
- **TCH transformation** — TCH(Z_1) = Z_3 = T_{10}(Z_1)
- **Dispersive interval** — The TCH interval 10 is dispersive for Z as unordered set

# Common Errors
- **Error**: Interpreting the Z-form tonics as Schenkerian prolongation
  **Correction**: The relationships are intervallic/transpositional, not prolongational; Lewin explicitly distinguishes this from Schenker

# Common Confusions
- **Confusion**: Assuming the foreground music of sequenced passages must be related
  **Clarification**: "The musical foregrounds of the two passages are not related sequentially" (p. 164); the sequence is structural, not superficial

# Source Reference
Chapter 7: Transformation Graphs and Networks (1): Intervals and Transpositions, Section 7.2, Figures 7.2-7.5, pages 161-165.

# Verification Notes
- Definition source: Direct from Lewin's analysis in 7.2
- Confidence rationale: Detailed analytical example with figures
- Re-extraction notes: Re-extracted from v2 card; preserved: Zauber pitch classes, Bell motive comparison, dispersive interval note
