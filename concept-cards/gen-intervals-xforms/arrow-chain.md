---
# === CORE IDENTIFICATION ===
concept: Arrow Chain
slug: arrow-chain

# === CLASSIFICATION ===
category: transformation-theory
subcategory: graph-network-foundations
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.1.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - node-arrow-system
extends: []
related:
  - transformation-graph-definition
  - transit-function
  - proper-arrow-chain
contrasts_with:
  - communication-between-nodes

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an arrow chain in a node/arrow system?"
  - "How do arrow chains relate to transformation consistency?"
---

# Quick Definition
An arrow chain from node N to node N' is a finite series of nodes where each consecutive pair is connected by a forwards-oriented arrow, representing a directed path through a node/arrow system.

# Core Definition
An arrow chain from node N to node N' in a node/arrow system is a finite series of nodes N0, N1, ..., NJ satisfying: (A) N0 = N; (B) for each j between 1 and J inclusive, (N_{j-1}, N_j) is in the ARROW relation; (C) NJ = N'. The criteria demand a finite unbroken path of forwards-oriented arrows, starting at N and ending at N' (Lewin, Definition 9.1.4, p. 226).

# Prerequisites
- **Node/arrow system** — arrow chains are paths within a node/arrow system

# Key Properties
1. All arrows must be traversed in the forward direction (unlike communication paths)
2. J >= 0 is allowed (trivial chain from N to N with J = 0)
3. Arrow chains are used to define the consistency criterion (9.2.1(D)) for transformation graphs
4. The product of TRANSIT values along an arrow chain gives the total transformation from start to end
5. Multiple arrow chains between the same endpoints must yield equal transformation products (consistency)

# Construction / Recognition
## To Construct:
1. Start at node N = N0
2. Follow a forwards-oriented arrow to the next node N1
3. Continue following forward arrows until reaching the destination N' = NJ
## To Recognize:
1. Check that consecutive pairs are in the ARROW relation (forward direction)
2. Verify the chain starts at N and ends at N'

# Context & Application
Arrow chains are the paths along which transformations compose. The consistency criterion 9.2.1(D) for well-formed transformation graphs requires that all arrow chains between the same pair of nodes yield the same semigroup product of TRANSIT values, ensuring that the net transformation between any two nodes is path-independent.

# Examples
**Example 1** (Figure 9.2, p. 226): Two arrow chains from N to N'. Chain 1 gives TRANSIT values x1, x2, ..., xJ; Chain 2 gives y1, y2, ..., yK. Consistency requires xJ...x2x1 = yK...y2y1 (in left orthography).

**Example 2** (Figure 9.4, p. 228): Two arrow chains from N2 to N3: one direct (with TRANSIT = C), another through N0 and N1 (with TRANSIT = BD). Consistency requires B = CD.

# Relationships
## Builds Upon
- **Node/arrow system** — arrow chains are paths within such systems
## Enables
- **Transformation graph** — consistency of arrow chains is the key criterion
- **Proper arrow chain** — an arrow chain with at least one one-way arrow
## Related
- **TRANSIT function** — labels arrows; products along chains must be consistent
## Contrasts With
- **Communication between nodes** — communication allows backwards traversal; arrow chains do not

# Common Errors
- **Error**: Following arrows backwards in an arrow chain
  **Correction**: Arrow chains require all arrows to be traversed forward; backwards traversal is only for communication paths

# Common Confusions
- **Confusion**: Thinking the trivial chain (J = 0) from N to N is not an arrow chain
  **Clarification**: It is valid; the reflexive arrow (N, N) is always in ARROW

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.1.4, p. 226. See Figures 9.2 and 9.4.

# Verification Notes
- Definition source: direct from Definition 9.1.4
- Confidence rationale: explicit formal definition with clear examples
- Re-extracted from v2 card; preserved: Figure 9.2 example, consistency requirement discussion
