---
concept: Ordinal-Pitch Pairs
slug: ordinal-pitch-pairs

category: generalized-set-theory
subcategory: injection-function
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: null

extraction_confidence: high

aliases:
  - "(n, p) pairs"

prerequisites:
  - inj-function
extends: []
related:
  - angst-hoffen-analysis
  - protocol-pairs
contrasts_with:
  - protocol-pairs

answers_questions:
  - "How can melodies be modeled as sets of ordinal-pitch pairs?"
---

# Quick Definition
The space of ordinal-pitch pairs S = {(n, p)} models melodies as sets where n is the ordinal position (1st, 2nd, ... note) and p is the pitch class. Transformations (k, OP) shift position by k and transform pitch by OP. This space is NOT a GIS, demonstrating INJ's generality.

# Core Definition
In the "Angst und Hoffen" melodic analysis (Lewin, pp. 161-163, Figure 6.4): elements are pairs (n, p) where n is a positive integer and p a pitch class. A melody is an unordered set of such pairs. Transformation (k, OP) maps (n, p) to (n+k, OP(p)). These transformations are NOT operations: no (n, p) with positive n satisfies (k, OP)(n, p) = (1, q) for k > 0.

# Prerequisites
- **INJ Function** — Required because transformations on this space are not operations

# Key Properties
1. S is NOT a GIS — transformations are not operations
2. (k, OP)(n, p) = (n+k, OP(p))
3. Even if OP is an operation on pitch classes, (k, OP) is not an operation on S
4. INJ handles this gracefully; IFUNC cannot apply here
5. Captures both "which note" and "which position" in the series

# Construction / Recognition
## To Construct:
1. List the melody as ordered notes p_1, p_2, ..., p_N
2. The set is {(1, p_1), (2, p_2), ..., (N, p_N)}

## To Recognize:
1. A set of (position, pitch-class) pairs modeling a serial melody

# Context & Application
This model enables analysis of serial melodic structure using INJ. In "Angst und Hoffen," internal transformations (1, I) and (2, w) bind the first tetrad; ordinal augmentation occurs in the second tetrad (I at distance 2, w at distance 3). T_6 is progressive between tetrads.

# Examples
**Example 1** (pp. 161-163, Figure 6.4): Vocal melody from "Angst und Hoffen." X_1^4 = {(1,D), (2,Gb), (3,Eb), (4,Fb)}. INJ(X_1^4, X_1^4)(1,I) = 2 (internal); INJ(X_1^4, X_5^8)(n,I) = 0 for all n (no I-arrows between tetrads); T_6 is progressive between tetrads.

# Relationships
## Builds Upon
- **INJ Function** — Required for analysis in this non-GIS space

## Enables
- **Angst und Hoffen Analysis** — Melodic portion uses this model

## Contrasts With
- **Protocol Pairs** — PROT captures ordering only; ordinal-pitch pairs capture position and pitch identity

# Common Errors
- **Error**: Treating (k, OP) as an operation
  **Correction**: These are transformations, not operations — they cannot map S onto itself

# Common Confusions
- **Confusion**: Thinking ordinal-pitch pairs require a GIS
  **Clarification**: This space has no GIS structure; INJ's independence from GIS is the point

# Source Reference
Chapter 6: Generalized Set Theory (2), melodic analysis in Figure 6.4, pp. 161-164.

# Verification Notes
- Definition source: Direct from melodic analysis discussion
- Confidence rationale: Detailed definition with analytical application
- Re-extraction notes: Re-extracted from v2 card; preserved: melody example, non-GIS emphasis, ordinal augmentation observation. Added v3.1 structure.
