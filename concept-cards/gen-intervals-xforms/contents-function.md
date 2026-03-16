---
concept: CONTENTS Function
slug: contents-function

category: transformation-theory
subcategory: graph-network-definitions
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.3.1"

extraction_confidence: high

aliases:
  - "CONTENTS"

prerequisites:
  - transformation-graph-definition
extends: []
related:
  - transformation-network-definition
  - operation-network
  - transit-function
contrasts_with:
  - transit-function

answers_questions:
  - "What is the CONTENTS function in a transformation network?"
  - "How do musical objects get assigned to network nodes?"
---

# Quick Definition
The CONTENTS function maps each node in a transformation network to a musical object from the family S, filling the abstract graph structure with concrete musical content while remaining compatible with the TRANSIT function.

# Core Definition
In a transformation network (S, NODES, ARROW, SGP, TRANSIT, CONTENTS), CONTENTS is a function mapping NODES into S. "CONTENTS(N)" reads "the contents of node N." CONTENTS must satisfy the compatibility requirement: for any arrow (N1, N2), if f = TRANSIT(N1, N2), then f(CONTENTS(N1)) = CONTENTS(N2) (Lewin, Definition 9.3.1(C)-(D), pp. 228-229).

# Prerequisites
- **Transformation graph** — CONTENTS is added to a graph to form a network

# Key Properties
1. Domain: NODES; Codomain: S (the family of musical objects)
2. CONTENTS is not arbitrary; it must satisfy compatibility with TRANSIT
3. In a connected operation network, CONTENTS is determined by any single node's value (Theorem 9.3.3)
4. Different CONTENTS on the same graph yield different networks (potentially isographic if graphs are isomorphic)
5. CONTENTS is what distinguishes a network from a graph

# Construction / Recognition
## To Construct:
1. For each node, assign a member of S
2. Verify: for all arrows (N1, N2), TRANSIT(N1, N2) applied to CONTENTS(N1) yields CONTENTS(N2)
## To Recognize:
1. Identify the objects assigned to each node
2. Check compatibility with TRANSIT at every arrow

# Context & Application
CONTENTS provides the musical interpretation of a network. The abstract graph structure (what connects to what) and transformation labels (how things transform) gain meaning only through the actual musical objects occupying the nodes. Different CONTENTS on the same graph yield different analytical claims about different passages or different aspects of the same passage.

# Examples
**Example 1** (Figure 9.5, p. 231): Same IPAIR graph underlies networks (a)-(e). Network (a): CONTENTS assigns pitch classes Bb and G# to two nodes. Network (c): CONTENTS assigns twelve-tone row forms. All satisfy the compatibility requirement.

**Example 2** (Figure 9.10, p. 238): In the CADENCE network, CONTENTS assigns Klangs (C,+), (G,+), (F,+), (C,+) to four nodes. The same Klang (C,+) occupies both the first (input) and last (output) node, but with different functional roles.

# Relationships
## Builds Upon
- **Transformation graph** — CONTENTS turns a graph into a network
## Enables
- **Transformation network** — defined by adding CONTENTS to a graph
## Related
- **Operation network** — in connected operation networks, CONTENTS is fully determined by one value
## Contrasts With
- **TRANSIT function** — TRANSIT labels arrows; CONTENTS fills nodes

# Common Errors
- **Error**: Assigning CONTENTS without checking compatibility with TRANSIT
  **Correction**: Every arrow must satisfy f(CONTENTS(N1)) = CONTENTS(N2)

# Common Confusions
- **Confusion**: Thinking CONTENTS is arbitrary
  **Clarification**: CONTENTS must be compatible with TRANSIT; not every assignment of objects to nodes is valid

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.3.1(C)-(D), pp. 228-229.

# Verification Notes
- Definition source: direct from Definition 9.3.1
- Confidence rationale: explicit component of formal definition
- Re-extracted from v2 card; preserved: Figure 9.5 example, CADENCE example, compatibility emphasis
