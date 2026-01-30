---
concept: Node/Arrow System
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
The foundational structure underlying transformation graphs: an ordered pair (NODES, ARROW) where NODES is a set and ARROW is a collection of ordered pairs of nodes indicating which nodes are connected by arrows.

# Formal Definition
A node/arrow system is an ordered pair (NODES, ARROW) where:
- NODES is a family (set)
- ARROW is a subfamily of NODES x NODES (ordered pairs of nodes)
- Nodes N_1 and N_2 are "in the arrow relation" if (N_1, N_2) is in ARROW
- By stipulation: Every node is in the arrow relation with itself ((N, N) in ARROW for all N)

# Mathematical Formulation
(NODES, ARROW) where:
- NODES = {N_1, N_2, ..., N_k}
- ARROW subset of NODES x NODES
- Reflexive: (N, N) in ARROW for all N in NODES
- Not necessarily symmetric: (N_1, N_2) in ARROW does not imply (N_2, N_1) in ARROW

# Musical Context/Application
Node/arrow systems provide the combinatorial skeleton for transformation graphs and networks. The nodes will eventually contain musical objects; the arrows will be labeled with transformations. The system itself is abstract, knowing nothing of musical content or transformation labels.

# Examples
From Figure 9.1:
- M_1 and M_2 are NOT in the arrow relation (no arrow between them)
- M_1 and M_3 ARE in the arrow relation
- M_4 and M_3 ARE in the arrow relation
- Arrows from each node to itself are understood

A simple IPAIR system:
- NODES = {N_1, N_2}
- ARROW = {(N_1, N_1), (N_2, N_2), (N_1, N_2), (N_2, N_1)}
- Every pair of nodes is in the arrow relation

# Related Concepts
- Transformation Graph
- Transformation Network
- Communication (between nodes)
- Connected System
- Arrow Chain

# Common Confusions
- The reflexive arrows (N to N) are always assumed but often not drawn
- Being "in the arrow relation" is ordered: (N_1, N_2) differs from (N_2, N_1)
- The system is abstract - no musical content or transformation labels yet
- "Arrow" refers to the relation, not a physical drawing (though drawings represent it)

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.1.1, Definition
