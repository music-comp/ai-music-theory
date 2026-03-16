---
# === CORE IDENTIFICATION ===
concept: "K and Kh Relations (Generalized)"
slug: k-relation-generalized

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: injection-function
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.8"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - generalized K relation
  - generalized Kh relation

# === TYPED RELATIONSHIPS ===
prerequisites:
  - inj-function
  - canonical-group
  - progressive-transformation
  - dispersive-transformation
extends:
  - emb-function
related:
  - rgnpf-partition-function
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How are Forte's K and Kh relations generalized using INJ?"
---

# Quick Definition
The generalized K and Kh relations reformulate Forte's inclusion relations using INJ: K holds when INJ(X, Y)(A) attains either its maximum (card X) or minimum (0) for some canonical operation A; Kh holds when both extremes are attained.

# Core Definition
Section 6.8 (Lewin, pp. 183-185): With CANON and cardinality restrictions (card X <= card complement(X), card X <= card Y): K_1: some A in CANON satisfies INJ(X, Y)(A) = card X (maximally progressive); K_2: some B in CANON satisfies INJ(X, Y)(B) = 0 (maximally dispersive). K = K_1 or K_2; Kh = K_1 and K_2. Multiplicities count how many operations achieve maximum/minimum.

# Prerequisites
- **INJ Function** — K/Kh expressed through INJ values
- **Canonical Group** — Determines which operations are considered
- **Progressive/Dispersive Transformation** — K_1 = maximally progressive; K_2 = maximally dispersive

# Key Properties
1. K_1 (embedding): INJ(X, Y)(A) = card X for some A in CANON
2. K_2 (disjointness): INJ(X, Y)(B) = 0 for some B in CANON
3. K = K_1 or K_2; Kh = K_1 and K_2
4. When CANON is finite, multiplicities give refined information
5. Different set pairs (same classes) may have different multiplicities

# Construction / Recognition
## To Determine K/Kh:
1. Compute INJ(X, Y)(A) for all A in CANON
2. Check if maximum = card X (K_1) and/or minimum = 0 (K_2)
3. K = either extreme achieved; Kh = both

## To Recognize:
1. Set pairs where canonical forms can be either fully embedded or fully disjoint

# Context & Application
The generalized K/Kh uses only S, CANON, and INJ — no GIS needed. Multiplicities (how many operations achieve extremes) give finer discrimination than the binary K/Kh relation.

# Examples
**Example 1** (p. 184): Y = black-note pentatonic, X = F#-major triad, CANON = T_i and I_j. Kh with multiplicity (2, 6): 2 operations embed X in Y, 6 make X disjoint from Y. For X' = {Ab, Bb, Db} (same class): Kh with multiplicity (4, 8).

# Relationships
## Builds Upon
- **INJ Function** — Formal framework for K/Kh
- **Progressive/Dispersive Transformation** — K_1 = maximally progressive; K_2 = maximally dispersive

## Enables
- **RGNPF Partition Function** — Full distribution of INJ values

# Common Errors
- **Error**: Assuming K/Kh multiplicities depend only on set classes
  **Correction**: Multiplicities depend on specific sets X, Y, not just their classes

# Common Confusions
- **Confusion**: Thinking K/Kh requires pitch-class space
  **Clarification**: The generalization works for any S with a canonical group

# Source Reference
Chapter 6: Generalized Set Theory (2), section 6.8, pp. 183-185.

# Verification Notes
- Definition source: Direct from section 6.8
- Confidence rationale: Explicit formulation with examples
- Re-extraction notes: Re-extracted from v2 card; preserved: pentatonic/triad example with multiplicities. Added v3.1 structure.
