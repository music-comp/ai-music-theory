---
# === CORE IDENTIFICATION ===
concept: Output Node
slug: output-node

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
  - "OUT node"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - node-arrow-system
extends: []
related:
  - input-node
  - tonic-as-input-output-center
  - precedence-ordering
contrasts_with:
  - input-node

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an output node in a node/arrow system?"
  - "How do output nodes relate to musical function?"
---

# Quick Definition
An output node is a node in a node/arrow system from which no proper arrows issue -- if (OUT, N) is in ARROW, then N = OUT -- giving it a terminal or goal function in the network.

# Core Definition
An output node OUT in a node/arrow system satisfies: if (OUT, N) is in the ARROW relation, we must have N = OUT. No proper arrows go FROM the output node; only the reflexive self-arrow (OUT, OUT) issues from it (Lewin, Definition 9.6.1, p. 238).

# Prerequisites
- **Node/arrow system** — output nodes are defined within node/arrow systems

# Key Properties
1. No proper arrows issue FROM the output node; only the reflexive (OUT, OUT)
2. Zero out-degree (except reflexive); possibly positive in-degree
3. Output nodes have a formal "goal" or "terminal" function
4. A system may have multiple output nodes, one, or none
5. The same musical object can be input in one position and output in another

# Construction / Recognition
## To Construct:
1. Identify all nodes N such that the only arrow from N is (N, N)
## To Recognize:
1. For a candidate node OUT, check all arrows (OUT, N) in ARROW
2. If the only such N is OUT itself, it is an output node

# Context & Application
Output nodes model endpoints or goals of transformational processes. In the CADENCE graph, the tonic Klang as output reflects "tonic-as-goal." Output nodes often represent cadential arrivals, structural goals, or points of repose.

# Examples
**Example 1** (Figure 9.9, p. 238): Brahms complementary gesture. The right node is output. Gb is "generated" by the network from the input Bb.

**Example 2** (Figure 9.10, p. 238): CADENCE network. The (C,+) node on the right is output, reflecting "tonic-as-goal." The same Klang (C,+) on the left is input. The visual balance reflects "tonic-as-center."

**Example 3** (Section 9.6.3, p. 239): Lewin notes that we do not want to assert (G,+) or (F,+) as "tonics" even though one is input and the other output. Input/output formalities are suggestive but not sufficient for tonicity.

# Relationships
## Builds Upon
- **Node/arrow system** — output is defined by arrow structure
## Enables
- **Tonic-as-input-output-center** — output function reflects tonic-as-goal
## Related
- **Precedence ordering** — output nodes "happen after" others they communicate with
## Contrasts With
- **Input node** — input nodes emit arrows; output nodes receive them

# Common Errors
- **Error**: Equating output with "last heard chronologically"
  **Correction**: Output is a graph-structural property, not a temporal claim

# Common Confusions
- **Confusion**: Thinking output nodes are always tonic
  **Clarification**: Output status is necessary but not sufficient for tonic function; non-tonic Klangs can occupy output positions

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.6.1, p. 238. Examples 9.6.2-9.6.3, pp. 238-239.

# Verification Notes
- Definition source: direct from Definition 9.6.1
- Confidence rationale: explicit definition
- Re-extracted from v2 card; preserved: CADENCE tonic-as-goal example, caution about tonicity attribution
