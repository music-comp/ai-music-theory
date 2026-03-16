---
concept: Node/Arrow System
slug: node-arrow-system

category: transformation-theory
subcategory: graph-network-foundations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.1.1"

extraction_confidence: high

aliases:
  - "node-arrow system"
  - "(NODES, ARROW)"

prerequisites: []
extends: []
related:
  - communication-between-nodes
  - connected-system
  - arrow-chain
  - transformation-graph-definition
contrasts_with: []

answers_questions:
  - "What is a node/arrow system?"
  - "What is the foundational structure underlying transformation graphs?"
  - "What must I know before studying transformation networks?"
---

# Quick Definition
A node/arrow system is an ordered pair (NODES, ARROW) consisting of a set of nodes and a binary relation on those nodes, providing the combinatorial skeleton underlying all transformation graphs and networks.

# Core Definition
A node/arrow system is an ordered pair (NODES, ARROW) where NODES is a family (set) and ARROW is a subfamily of NODES x NODES (ordered pairs of nodes). Nodes N1 and N2 are "in the arrow relation" if (N1, N2) is in ARROW. By stipulation, every node is in the arrow relation with itself: (N, N) is in ARROW for all N in NODES (Lewin, Definition 9.1.1, p. 224).

# Prerequisites
This is a foundational concept for transformation graph/network theory. Prerequisites lie in earlier chapters:
- **Semigroup** — needed to label arrows in subsequent graph constructions
- **Transformation** — the node/arrow system serves as the skeleton that will carry transformation labels

# Key Properties
1. ARROW is a reflexive relation: (N, N) is in ARROW for every N in NODES
2. ARROW is not necessarily symmetric: (N1, N2) in ARROW does not imply (N2, N1) in ARROW
3. The system is purely abstract, carrying no information about musical content or transformation labels
4. Nodes may or may not be in the arrow relation; the relation defines the topology of the system

# Construction / Recognition
## To Construct:
1. Specify a set NODES
2. Specify which ordered pairs of distinct nodes are in the ARROW relation
3. Include all reflexive pairs (N, N) by stipulation
## To Recognize:
1. Check that a set of nodes is identified
2. Check that a binary relation (arrow relation) on nodes is specified
3. Verify reflexivity: every node has an arrow to itself

# Context & Application
Node/arrow systems provide the combinatorial skeleton for transformation graphs and networks. The nodes will eventually contain musical objects; the arrows will be labeled with transformations from a semigroup. The system itself is abstract, knowing nothing of musical content or transformation labels. It is the first layer of structure in the hierarchy: node/arrow system < transformation graph < transformation network.

# Examples
**Example 1** (Figure 9.1, p. 224): A system with nodes M1 through M6. M1 and M2 are NOT in the arrow relation; M1 and M3 ARE. Arrows from each node to itself are understood. The system is not connected (it decomposes into two communicating components).

**Example 2** (Figure 9.5, p. 231): An IPAIR system with two nodes where every pair is in the ARROW relation: NODES = {N1, N2}, ARROW = {(N1, N1), (N2, N2), (N1, N2), (N2, N1)}.

# Relationships
## Builds Upon
- No formal prerequisites within Chapter 9; this is the base structure
## Enables
- **Transformation graph** — built by adding SGP and TRANSIT to a node/arrow system
- **Arrow chain** — defined as a directed path through a node/arrow system
- **Communication between nodes** — defined as an equivalence relation on nodes
## Related
- **Connected system** — a node/arrow system where all nodes communicate
## Contrasts With
- **Transformation graph** — adds semigroup labeling to the node/arrow skeleton

# Common Errors
- **Error**: Forgetting the reflexive arrows (N, N) when constructing a system
  **Correction**: Reflexive arrows are stipulated for all nodes; they are always present even when not drawn

# Common Confusions
- **Confusion**: Thinking "arrow" requires a physical drawing
  **Clarification**: "Arrow" refers to membership in the ARROW relation, a formal binary relation, not necessarily a visual arrow on a diagram
- **Confusion**: Assuming the arrow relation is symmetric
  **Clarification**: (N1, N2) in ARROW does NOT imply (N2, N1) in ARROW; the relation is directed

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.1.1, pp. 224-225. See Figure 9.1 for a concrete example.

# Verification Notes
- Definition source: direct from Definition 9.1.1 (high confidence)
- The reflexivity stipulation is explicit in the source
- Re-extracted from v2 card; preserved: IPAIR example, reflexive arrow clarification in confusions
