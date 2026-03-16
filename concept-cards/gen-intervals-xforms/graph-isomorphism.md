---
# === CORE IDENTIFICATION ===
concept: Graph Isomorphism
slug: graph-isomorphism

# === CLASSIFICATION ===
category: transformation-theory
subcategory: graph-network-mappings
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.4.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "(NODEMAP, SGMAP) isomorphism"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - node-arrow-system-isomorphism
  - isomorphism
extends:
  - node-arrow-system-isomorphism
related:
  - isography
  - graph-homomorphism
contrasts_with:
  - graph-homomorphism

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "When are two transformation graphs isomorphic?"
  - "What is the formal basis for isography?"
---

# Quick Definition
Two transformation graphs are isomorphic if there exists a pair (NODEMAP, SGMAP) where NODEMAP is a node/arrow isomorphism and SGMAP is a semigroup isomorphism, and the TRANSIT functions correspond under both maps.

# Core Definition
Given two transformation graphs (NODES, ARROW, SGP, TRANSIT) and (NODES', ARROW', SGP', TRANSIT'), they are isomorphic if there exists a pair (NODEMAP, SGMAP) with: (A) NODEMAP is an isomorphism of (NODES, ARROW) with (NODES', ARROW'); (B) SGMAP is an isomorphism of SGP with SGP'; (C) for every pair (N1, N2) in ARROW, TRANSIT'(NODEMAP(N1), NODEMAP(N2)) = SGMAP(TRANSIT(N1, N2)). The pair (NODEMAP, SGMAP) is called an isomorphism of the first graph with the second (Lewin, Definition 9.4.2, pp. 231-232).

# Prerequisites
- **Node/arrow system isomorphism** — NODEMAP must be such an isomorphism
- **Isomorphism** (of semigroups) — SGMAP must preserve semigroup structure

# Key Properties
1. Requires both NODEMAP (bijection on nodes) and SGMAP (bijection on semigroups)
2. Criterion (C) ensures TRANSIT labels correspond under the two maps
3. Graph isomorphism is the foundation for defining isography of networks
4. A 1-to-1 homomorphism onto is precisely an isomorphism (by definition)

# Construction / Recognition
## To Construct:
1. Find a bijection NODEMAP between node sets preserving arrows both ways
2. Find a semigroup isomorphism SGMAP between SGP and SGP'
3. Verify that TRANSIT labels correspond: TRANSIT'(NODEMAP(N1), NODEMAP(N2)) = SGMAP(TRANSIT(N1, N2))
## To Recognize:
1. Check for bijections between nodes and between semigroups
2. Verify the TRANSIT compatibility criterion (C)

# Context & Application
Graph isomorphism captures structural equivalence. Two graphs may use different semigroups acting on different families of objects but share identical transformational structure. This is the foundation for isography: two networks are isographic precisely when their graphs are isomorphic.

# Examples
**Example 1** (Section 9.4.4, p. 231): Graphs (a) and (c) on Figure 9.5. Graph (a) has SGP = {E, I} as pitch-class operations; graph (c) has SGP' = {E, I} as row operations. SGMAP maps pitch-class-E to row-E and pitch-class-I to row-I. NODEMAP is the identity on NODES. The graphs are isomorphic.

**Example 2** (Section 9.4.4, p. 232): Graphs (a) and (d) where SGP = {E, I_A} and SGP' = {E, J = I_Bb}. SGMAP(E) = E, SGMAP(I) = J. The graphs are isomorphic despite using different inversion operations.

# Relationships
## Builds Upon
- **Node/arrow system isomorphism** — provides the NODEMAP component
## Enables
- **Isography** — two networks are isographic iff their graphs are isomorphic
## Related
- **Graph homomorphism** — generalization allowing non-bijective maps
## Contrasts With
- **Graph homomorphism** — homomorphisms need not be bijective

# Common Errors
- **Error**: Checking only NODEMAP without verifying SGMAP
  **Correction**: Both components must be isomorphisms, and criterion (C) must hold

# Common Confusions
- **Confusion**: Thinking isomorphic graphs must use the "same" semigroup
  **Clarification**: The semigroups may act on entirely different objects; they need only be isomorphic as abstract semigroups

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.4.2, pp. 231-232. See Example 9.4.4 and Figure 9.5.

# Verification Notes
- Definition source: direct from Definition 9.4.2
- Confidence rationale: explicit three-part formal definition
- Re-extracted from v2 card; preserved: Figure 9.5 examples, semigroup isomorphism clarification
