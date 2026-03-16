---
# === CORE IDENTIFICATION ===
concept: Operation Graph
slug: operation-graph

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
section: "9.2.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - transformation-graph-definition
  - group
extends:
  - transformation-graph-definition
related:
  - operation-network
  - idempotent-transit-value
contrasts_with:
  - transformation-graph-definition

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What distinguishes an operation graph from a transformation graph?"
  - "What is an operation graph?"
---

# Quick Definition
An operation graph is a transformation graph in which SGP is a group (rather than merely a semigroup), ensuring all transformations are invertible and TRANSIT(N, N) equals the identity.

# Core Definition
An operation graph is a transformation graph (NODES, ARROW, SGP, TRANSIT) where SGP is a group. Since the only idempotent in a group is the identity element, TRANSIT(N, N) must equal the identity for all nodes N. The group structure allows arrows to be traversed backwards using inverse operations (Lewin, Definition 9.2.3, p. 227).

# Prerequisites
- **Transformation graph** — an operation graph is a special case
- **Group** — SGP must be a group, not merely a semigroup

# Key Properties
1. All transformations in SGP are invertible (have inverses in SGP)
2. TRANSIT(N, N) = identity for all nodes N (the only idempotent in a group)
3. Arrow chains can be traversed in reverse using inverse transformations
4. Most graphs in Lewin's musical analyses are operation graphs
5. Operation graphs support the unique determination theorem (9.3.3) for networks

# Construction / Recognition
## To Construct:
1. Build a transformation graph (NODES, ARROW, SGP, TRANSIT)
2. Ensure SGP is a group (closure, associativity, identity, inverses)
3. TRANSIT(N, N) = identity follows automatically
## To Recognize:
1. Verify that SGP forms a group
2. All other transformation graph criteria automatically apply

# Context & Application
Operation graphs are the most common type in musical analysis because most musical transformations (transpositions, inversions, Klang transformations like DOM, MED, PAR, SLIDE) are invertible. The group structure allows flexible navigation through associated networks, enabling the unique determination of contents from any single node (Theorem 9.3.3).

# Examples
**Example 1** (Section 9.2.3, p. 227): Most graphs in Chapters 7-8 are operation graphs: Klang transformation graphs (DOM, MED, PAR form a group), pitch-class transformation graphs (transpositions and inversions form a group).

**Example 2** (Theorem 9.3.3, p. 228): Given a connected operation graph and contents for any one node, all other node contents are uniquely determined by following arrows forward (applying TRANSIT) or backward (applying inverses).

# Relationships
## Builds Upon
- **Transformation graph** — operation graph is a specialization
- **Group** — the key additional requirement
## Enables
- **Operation network** — a network whose graph is an operation graph
## Related
- **Idempotent TRANSIT value** — in operation graphs, always the identity
## Contrasts With
- **Transformation graph** — the general case allows non-invertible semigroup transformations

# Common Errors
- **Error**: Assuming all transformation graphs are operation graphs
  **Correction**: Only those with group SGP qualify; semigroup-based graphs are also valid

# Common Confusions
- **Confusion**: Thinking "operation" means "any transformation"
  **Clarification**: In Lewin's usage, "operation" specifically means a transformation that is 1-to-1 and onto (a member of a group)

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.2.3, p. 227.

# Verification Notes
- Definition source: direct from Definition 9.2.3
- Confidence rationale: concise explicit definition
- Re-extracted from v2 card; preserved: examples from Chapters 7-8, unique determination reference
