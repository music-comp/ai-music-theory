---
concept: Transformation Network
slug: transformation-network-definition

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
  - "(S, NODES, ARROW, SGP, TRANSIT, CONTENTS)"

prerequisites:
  - transformation-graph-definition
  - contents-function
extends:
  - transformation-graph-definition
related:
  - operation-network
  - isography
contrasts_with:
  - transformation-graph-definition

answers_questions:
  - "What is a transformation network?"
  - "How do transformation graphs relate to transformation networks?"
  - "What distinguishes a transformation graph from a transformation network?"
---

# Quick Definition
A transformation network is a transformation graph together with a family of objects S and a CONTENTS function that assigns an object from S to each node, such that applying the TRANSIT transformation along any arrow yields the contents of the destination node.

# Core Definition
A transformation network is an ordered sextuple (S, NODES, ARROW, SGP, TRANSIT, CONTENTS) having the features: (A) S is a family of objects to be transformed; (B) (NODES, ARROW, SGP, TRANSIT) is a transformation graph with SGP being a semigroup of transformations on S; (C) CONTENTS is a function mapping NODES into S, where CONTENTS(N) means "the contents of node N"; (D) given nodes N1 and N2 in the ARROW relation, if f = TRANSIT(N1, N2), then f(CONTENTS(N1)) = CONTENTS(N2) (Lewin, Definition 9.3.1, pp. 228-229).

# Prerequisites
- **Transformation graph** — provides the abstract template (NODES, ARROW, SGP, TRANSIT)
- **CONTENTS function** — fills the graph's nodes with musical objects

# Key Properties
1. Combines abstract graph structure, transformation labels, and musical content
2. Criterion (D) ensures compatibility: TRANSIT transformations actually map contents correctly
3. Not every assignment of objects to nodes forms a valid network; criterion (D) must hold
4. Different networks can share the same graph (basis for isography)
5. The network represents a complete analytical claim about transformational relationships

# Construction / Recognition
## To Construct:
1. Choose a transformation graph (NODES, ARROW, SGP, TRANSIT)
2. Choose a family of objects S on which SGP acts
3. Assign CONTENTS: NODES -> S
4. Verify criterion (D): for each arrow, TRANSIT applied to source contents yields destination contents
## To Recognize:
1. Identify the underlying graph
2. Identify the objects at each node
3. Verify that TRANSIT transformations correctly relate the contents

# Context & Application
Transformation networks are the primary analytical tool in Lewin's transformational theory. They represent complete analytical claims: the objects S are the musical entities under study (pitch classes, Klangs, row forms, etc.), and the network asserts specific transformational relationships among them. The same graph can underlie many different networks, revealing structural similarities across diverse musical domains.

# Examples
**Example 1** (Figure 9.3, p. 228): Nodes N1 and N2 with s1 = CONTENTS(N1), s2 = CONTENTS(N2), and f = TRANSIT(N1, N2). The network asserts f(s1) = s2.

**Example 2** (Figure 9.5, p. 231): Five networks (a)-(e) all share the same two-node IPAIR graph. Networks (a) and (b) contain pitch classes; network (c) contains row forms. All are valid networks on isomorphic graphs.

# Relationships
## Builds Upon
- **Transformation graph** — the network adds S and CONTENTS to a graph
## Enables
- **Isography** — networks are isographic when their graphs are isomorphic
- **Operation network** — a network where SGP is a group
## Related
- **CONTENTS function** — assigns objects to nodes within the network
## Contrasts With
- **Transformation graph** — a graph has no contents; a network has both graph and contents

# Common Errors
- **Error**: Assigning arbitrary objects to nodes without checking criterion (D)
  **Correction**: CONTENTS must be compatible with TRANSIT; verify f(CONTENTS(N1)) = CONTENTS(N2) for all arrows

# Common Confusions
- **Confusion**: Thinking a network IS a graph
  **Clarification**: A network is a graph PLUS objects and their assignment to nodes; the graph is the abstract template
- **Confusion**: Thinking isographic networks have the same contents
  **Clarification**: Isography concerns graph structure only; contents may differ entirely

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.3.1, pp. 228-229. See Figure 9.3.

# Verification Notes
- Definition source: direct from Definition 9.3.1
- Confidence rationale: explicit six-part formal definition
- Re-extracted from v2 card; preserved: Klang network example concept, graph vs network distinction
