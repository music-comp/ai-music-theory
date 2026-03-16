---
concept: TCH Transformation
slug: tch-transformation

category: transformation-theory
subcategory: serial-operations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.2.1"

extraction_confidence: high

aliases:
  - "transposition chain"

prerequisites:
  - rich-transformation
extends: []
related:
  - ri-chaining
  - structural-sequencing
  - isography
contrasts_with:
  - rich-transformation

answers_questions:
  - "What distinguishes RICH from TCH?"
  - "Why use TCH instead of a specific transposition label?"
---

# Quick Definition
The composition of RICH with itself: TCH = (RICH)(RICH). TCH always produces a transposed form of the input series, but which transposition depends on the series' internal structure: TCH(s) = T_i(s) where i = int(s_1, s_N) + int(s_2, s_{N-1}).

# Core Definition
"When we define the operation TCH as (RICH)(RICH), then TCH(s) is always some transposed form of s, but just which transposed form depends on the internal structure of any given argument s upon which TCH is operating. Specifically, if i = int(s_1, s_N) + int(s_2, s_{N-1}), then TCH(s) = T_i(s). We shall call i here 'the TCH-interval for s'" (Lewin, 8.2.1, p. 181).

# Prerequisites
- **RICH transformation** — TCH is defined as RICH composed with RICH

# Key Properties
1. TCH = (RICH)(RICH)
2. TCH(s) = T_i(s) where i = int(s_1, s_N) + int(s_2, s_{N-1})
3. The TCH interval for a retrograde or inverted form of s is -i (negative/group inverse)
4. The TCH interval for a retrograde-inverted form of s is the same i
5. TCH is always some transposition, but the specific transposition varies by series
6. Writing "TCH" instead of "T_i" preserves isographic relationships

# Construction / Recognition
## To Construct:
1. Compute i = int(s_1, s_N) + int(s_2, s_{N-1})
2. TCH(s) = T_i(s)
## To Recognize:
1. Alternate forms in an RI-chain are TCH-related
2. The transposition level depends on the series' structure

# Context & Application
TCH governs "structural sequencing" in RI-chaining contexts. Using "TCH" rather than specific transposition numbers is essential for preserving isographic relationships: "The isography would not obtain if we wrote 'T_{10}' for TCH on figure 8.3 and 'T_3' for TCH on figure 8.4(b): T_{10} and T_3 are not the same transformation" (8.2.4, p. 182).

# Examples
**Example 1** (Figure 8.3, p. 182): Wagner's Zauber forms: Z_3 = TCH(Z_1) = T_{10}(Z_1). TCH interval = 10.

**Example 2** (Figure 8.4b, p. 182): Webern's op. 27: TCH produces T_3. Using "TCH" on both figures preserves the isography between the Wagner and Webern networks.

# Relationships
## Builds Upon
- **RICH transformation** — TCH = (RICH)(RICH)
## Enables
- **Structural sequencing** — TCH governs the transposition level of structural sequences
- **Isography** — Using TCH (not T_i) preserves isographic relationships
## Related
- **RI-chaining** — TCH relates alternate forms in a chain
## Contrasts With
- **RICH transformation** — RICH produces an RI form; TCH produces a transposition

# Common Errors
- **Error**: Writing T_{10} or T_3 instead of TCH on network diagrams
  **Correction**: This destroys isographic relationships between networks with different TCH intervals

# Common Confusions
- **Confusion**: Thinking TCH is a fixed transposition
  **Clarification**: TCH varies by series; it is "the same transformation" only in the sense that it always means "RICH composed with RICH"
- **Confusion**: Conflating the TCH interval with the interval between a series' endpoints
  **Clarification**: The TCH interval is int(s_1, s_N) + int(s_2, s_{N-1}), involving both the outer and inner pairs

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.2.1, page 181.

# Verification Notes
- Definition source: Direct quotation from 8.2.1
- Confidence rationale: Explicit formal definition with formula
- Re-extraction notes: Re-extracted from v2 card; preserved: isography argument, both examples, TCH interval formula
