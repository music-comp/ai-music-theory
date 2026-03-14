---
concept: CONTENTS Function
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
The function in a transformation network that assigns a musical object from the space S to each node, filling the abstract graph structure with concrete musical content.

# Formal Definition
In a transformation network (S, NODES, ARROW, SGP, TRANSIT, CONTENTS):
- CONTENTS: NODES -> S
- CONTENTS(N) is "the contents of node N"
- CONTENTS must be compatible with TRANSIT: for arrows (N_1, N_2), if f = TRANSIT(N_1, N_2), then f(CONTENTS(N_1)) = CONTENTS(N_2)

# Mathematical Formulation
Domain: NODES
Codomain: S (the family of musical objects)

Compatibility requirement:
- For all (N_1, N_2) in ARROW
- Let f = TRANSIT(N_1, N_2)
- Then CONTENTS(N_2) = f(CONTENTS(N_1))

This is the defining constraint that turns a graph-with-objects into a genuine network.

# Musical Context/Application
CONTENTS provides the musical interpretation of a network. The abstract graph structure (what connects to what) and transformation labels (how things transform) are meaningless without the actual musical objects occupying the nodes. Different CONTENTS on the same graph yield different analytical claims.

# Examples
From Figure 9.5:
- Same IPAIR graph underlies networks (a), (b), (c), (d), (e)
- Network (a): CONTENTS assigns Bb and G# to the two nodes
- Network (b): CONTENTS assigns A and A to the two nodes
- Network (c): CONTENTS assigns row forms to nodes
- All are valid networks (CONTENTS compatible with TRANSIT)

Theorem 9.3.3 application:
- In connected operation network, CONTENTS is determined by any single node's content
- Choose s_0 = CONTENTS(N_0)
- All other CONTENTS values follow from TRANSIT and the graph structure

# Related Concepts
- Transformation Network Definition
- TRANSIT Function
- Graph versus Network
- Isography
- Operation Network

# Common Confusions
- CONTENTS assigns objects to nodes, not to arrows (that's TRANSIT)
- CONTENTS is not arbitrary; it must satisfy the compatibility requirement
- Two networks can have same graph but different CONTENTS (they may be isographic)
- In operation networks, CONTENTS of one node determines all others

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.3.1
