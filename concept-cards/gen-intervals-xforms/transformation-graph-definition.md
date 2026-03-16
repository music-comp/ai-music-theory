---
# === CORE IDENTIFICATION ===
concept: Transformation Graph
slug: transformation-graph-definition

# === CLASSIFICATION ===
category: transformation-theory
subcategory: graph-network-definitions
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.2.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "(NODES, ARROW, SGP, TRANSIT)"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - node-arrow-system
  - semigroup
  - arrow-chain
extends:
  - node-arrow-system
related:
  - transit-function
  - idempotent-transit-value
contrasts_with:
  - transformation-network-definition
  - operation-graph

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a transformation graph?"
  - "How do transformation graphs relate to transformation networks?"
  - "What distinguishes a transformation graph from a transformation network?"
  - "How do I determine if a transformation graph is well-formed?"
---

# Quick Definition
A transformation graph is an ordered quadruple (NODES, ARROW, SGP, TRANSIT) that labels the arrows of a node/arrow system with elements of a semigroup, subject to a consistency requirement ensuring path-independent transformation products.

# Core Definition
A transformation graph is an ordered quadruple (NODES, ARROW, SGP, TRANSIT) satisfying: (A) (NODES, ARROW) is a node/arrow system; (B) SGP is a semigroup; (C) TRANSIT is a function mapping ARROW into SGP; (D) given nodes N and N', for any two arrow chains from N to N', the semigroup products of TRANSIT values along those chains must be equal (Lewin, Definition 9.2.1, pp. 226-227).

Criterion (D) ensures that the net transformation from N to N' is well-defined regardless of which path is taken through the graph.

# Prerequisites
- **Node/arrow system** — provides the combinatorial skeleton (NODES, ARROW)
- **Semigroup** — provides the algebraic structure SGP for labeling arrows
- **Arrow chain** — paths along which the consistency criterion (D) is evaluated

# Key Properties
1. TRANSIT maps each arrow (ordered pair in ARROW) to a member of SGP
2. Criterion (D) guarantees path-independence of transformation products
3. TRANSIT(N, N) must be an idempotent member of SGP (proved in 9.2.2)
4. When SGP is a group, TRANSIT(N, N) must be the identity element
5. The graph carries no musical content; it is an abstract template that can be filled with various contents to form networks
6. Transformation products combine in left orthography: rightmost transformation applies first

# Construction / Recognition
## To Construct:
1. Define a node/arrow system (NODES, ARROW)
2. Choose a semigroup SGP
3. Assign TRANSIT values from SGP to each arrow in ARROW
4. Verify criterion (D): for every pair of arrow chains between the same endpoints, the products of TRANSIT values are equal
## To Recognize:
1. Identify the node/arrow system
2. Identify the semigroup of transformations
3. Identify the TRANSIT function labeling each arrow
4. Check that all arrow-chain products between common endpoints agree

# Context & Application
Transformation graphs provide the formal skeleton for analyzing musical transformations. The graph is abstract, knowing nothing about what musical objects will fill its nodes. Multiple networks (with different contents) can share the same underlying graph, which is the basis for isography. The consistency requirement ensures that the graph represents a coherent transformational structure.

# Examples
**Example 1** (Figure 9.4, pp. 228-229): Operations A, B, C, D on arrows of a four-node graph. Two arrow chains from N2 to N3 yield the consistency requirement B = CD.

**Example 2**: A simple graph with NODES = {N1, N2, N3}, SGP = group of transpositions, TRANSIT assigning T5 to (N1, N2), T3 to (N2, N3), T8 to (N1, N3). Consistency: T3 * T5 = T8 (required).

# Relationships
## Builds Upon
- **Node/arrow system** — the graph adds SGP and TRANSIT to a node/arrow system
- **Semigroup** — provides the algebraic structure for arrow labels
## Enables
- **Transformation network** — a graph plus a CONTENTS function on nodes
- **Graph isomorphism** — comparing abstract graph structures
- **Graph homomorphism** — relating graphs via structure-preserving maps
## Related
- **TRANSIT function** — the labeling function within the graph
- **Idempotent TRANSIT value** — TRANSIT(N, N) must be idempotent
## Contrasts With
- **Transformation network** — a network adds contents; a graph has no contents
- **Operation graph** — a transformation graph where SGP is specifically a group

# Common Errors
- **Error**: Assigning TRANSIT values that violate criterion (D)
  **Correction**: Always verify that all arrow-chain products between common endpoints agree before declaring a valid graph

# Common Confusions
- **Confusion**: Thinking a transformation graph contains musical objects
  **Clarification**: Graphs are abstract templates; musical objects are assigned when constructing a network
- **Confusion**: Assuming SGP must be a group
  **Clarification**: SGP can be any semigroup; only operation graphs require SGP to be a group

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.2.1, pp. 226-227. See Figures 9.2 and 9.4.

# Verification Notes
- Definition source: direct from Definition 9.2.1
- Confidence rationale: explicit four-part formal definition in source
- Re-extracted from v2 card; preserved: Figure 9.4 example, left orthography note, distinction from network
