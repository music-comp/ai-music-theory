---
# === CORE IDENTIFICATION ===
concept: Isography
slug: isography

# === CLASSIFICATION ===
category: transformation-theory
subcategory: non-intervallic-transformations
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.2.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "isographic relationship"
  - "network isography"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - tch-transformation
  - rich-transformation
extends: []
related:
  - graph-isomorphism
  - ipair-graph
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What does it mean for two networks to be isographic?"
  - "Why is TCH preferred over specific transposition labels?"
---

# Quick Definition
Two transformation networks are isographic when their underlying graphs are isomorphic -- they share the same structure of nodes, arrows, and transformation labels (up to semigroup isomorphism), even though the contents of their nodes may differ entirely.

# Core Definition
Lewin illustrates isography through the Wagner-Webern comparison: "If we excise any four consecutive nodes from figure 8.4(b), along with the arrows that connect them, we shall have essentially the same graph as that of figure 8.3. The same transformations are arranged and combined in the same structure of nodes and arrows, even though the contents of the nodes are Wagnerian in one case and Webernian in the other. We shall say that the two networks-of-series are isographic" (8.2.4, p. 182). Crucially: "The isography would not obtain if we wrote 'T_{10}' for TCH on figure 8.3 and 'T_3' for TCH on figure 8.4(b): T_{10} and T_3 are not the same transformation."

# Prerequisites
- **TCH transformation** — The isography depends on using TCH rather than specific T_i
- **RICH transformation** — The other transformation in the isographic networks

# Key Properties
1. Isography requires matching graph structure: same node/arrow configuration
2. Transformation labels must correspond under semigroup isomorphism
3. Node contents may differ entirely (different pitch classes, row forms, etc.)
4. Using TCH (series-dependent) rather than T_i (fixed) preserves isography
5. Isography reveals deep structural similarities across different musical domains

# Construction / Recognition
## To Construct:
1. Build transformation networks for two passages using series-dependent labels (RICH, TCH)
2. Compare graph structures
3. If the structures match (with transformation labels corresponding), the networks are isographic
## To Recognize:
1. Same pattern of nodes, arrows, and transformation types in different musical contexts
2. Transformation labels correspond (e.g., TCH in both, not T_{10} in one and T_3 in another)

# Context & Application
Isography enables hearing a way in which diverse musical passages "project the same overall transformational gesture" (p. 185). Using fixed transposition numbers would destroy this capacity. The concept becomes formally defined in Chapter 9 but is motivated here through examples.

# Examples
**Example 1** (Figures 8.3-8.4, p. 182): Wagner's Parsifal Zauber network (RICH and TCH with TCH-interval 10) is isographic to Webern's op. 27 network (RICH and TCH with TCH-interval 3). Both use RICH and TCH in identical graph arrangements.

**Example 2** (Figure 8.8, p. 186): The FATE' bass-line chain (4 TCH-sequences with TCH-interval 3) is isographic to the FATE bass-line chain of Figure 8.6 (4 TCH-sequences with TCH-interval 2). "Obviously we could not hear such a relation using T_2 in one case and T_3 in the other."

# Relationships
## Builds Upon
- **TCH transformation** — Series-dependent labeling enables isography
- **RICH transformation** — Part of the typical isographic structure
## Enables
- **IPAIR graph** — A specific isographic pattern
## Related
- **Graph isomorphism** — The formal underpinning of isography

# Common Errors
- **Error**: Using specific transposition numbers (T_{10}, T_3) instead of TCH
  **Correction**: This destroys isography; T_{10} and T_3 are different transformations

# Common Confusions
- **Confusion**: Thinking isographic networks must sound similar
  **Clarification**: Isography is structural, not perceptual; Wagnerian and Webernian content can be isographic
- **Confusion**: Believing isography requires identical transformations
  **Clarification**: It requires transformations that correspond under semigroup isomorphism, not identical transformations

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.2.4, page 182.

# Verification Notes
- Definition source: Direct quotation from 8.2.4
- Confidence rationale: Explicitly named and demonstrated with clear examples
- Re-extraction notes: Re-extracted from v2 card; preserved: Wagner-Webern comparison, T_{10} vs. T_3 argument
