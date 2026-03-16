---
# === CORE IDENTIFICATION ===
concept: Input Node
slug: input-node

# === CLASSIFICATION ===
category: transformation-theory
subcategory: graph-network-structure
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.6.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "IN node"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - node-arrow-system
extends: []
related:
  - output-node
  - tonic-as-input-output-center
  - start-node
  - precedence-ordering
contrasts_with:
  - output-node

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an input node in a node/arrow system?"
  - "How do input nodes relate to musical function?"
---

# Quick Definition
An input node is a node in a node/arrow system to which no proper arrows point -- if (N, IN) is in ARROW, then N = IN -- giving it a generative or initiating function in the network.

# Core Definition
An input node for a node/arrow system is a node IN to which no proper arrows point. That is, if (N, IN) is in the ARROW relation, we must have N = IN. Only the reflexive self-arrow points to an input node (Lewin, Definition 9.6.1, p. 238).

# Prerequisites
- **Node/arrow system** — input nodes are defined within node/arrow systems

# Key Properties
1. No proper arrows point TO the input node; only the reflexive (IN, IN) does
2. Zero in-degree (except reflexive); possibly positive out-degree
3. Input nodes have a formal "generative" or "initiating" function
4. A system may have multiple input nodes, one, or none
5. Input function is a property of graph structure, not of musical chronology

# Construction / Recognition
## To Construct:
1. Identify all nodes N such that the only arrow pointing to N is (N, N)
## To Recognize:
1. For a candidate node IN, check all arrows (N, IN) in ARROW
2. If the only such N is IN itself, it is an input node

# Context & Application
Input nodes model points where transformational processes begin. In the CADENCE graph, the tonic Klang as input reflects "tonic-as-generator." In the FATE motive network, the input node containing A-C-B has "special generative function" corresponding to its musical priority. However, input function is formal, not necessarily temporal.

# Examples
**Example 1** (Figure 9.9, p. 238): Brahms complementary gesture. The left node is input (only arrows go out); the right node is output. Bb "goes into" the network and Gb "comes out."

**Example 2** (Figure 9.11, p. 239): Die Walkure FATE network. The lower-left node (containing A-C-B) is the unique input, reflecting its "special generative function."

**Example 3** (Figure 9.10, p. 238): CADENCE network. The (C,+) node on the left is input, reflecting "tonic-as-generator."

# Relationships
## Builds Upon
- **Node/arrow system** — input is defined by arrow structure
## Enables
- **Tonic-as-input-output-center** — input function reflects tonic-as-generator
## Related
- **Start node** — a formal device to override other input nodes
- **Precedence ordering** — input nodes "happen before" others they communicate with
## Contrasts With
- **Output node** — output nodes receive arrows; input nodes emit them

# Common Errors
- **Error**: Equating input with "first heard chronologically"
  **Correction**: Input is a graph-structural property; it may or may not align with temporal order

# Common Confusions
- **Confusion**: Thinking every network must have an input node
  **Clarification**: A network may have no input nodes (if every node receives at least one proper arrow)

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.6.1, p. 238. Examples 9.6.2-9.6.4, pp. 238-240.

# Verification Notes
- Definition source: direct from Definition 9.6.1
- Confidence rationale: explicit definition with multiple examples
- Re-extracted from v2 card; preserved: Brahms, FATE, CADENCE examples, formal vs temporal distinction
