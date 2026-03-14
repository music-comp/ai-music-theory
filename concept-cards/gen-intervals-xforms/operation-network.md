---
concept: Operation Network
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A transformation network for which SGP is a group of operations on S, meaning all transformations are invertible and the network structure allows unique determination of all contents from any single node.

# Formal Definition
An operation network is a transformation network (S, NODES, ARROW, SGP, TRANSIT, CONTENTS) where SGP is a group of operations on S.

By Theorem 9.3.3: In a connected operation network, specifying CONTENTS for any one node uniquely determines CONTENTS for all nodes.

# Mathematical Formulation
Key theorem (9.3.3):
- Let (NODES, ARROW, GP, TRANSIT) be a connected operation graph
- Let N_0 be any node, s_0 any member of S
- Then there exists a unique operation network with this graph such that s_0 = CONTENTS(N_0)

Proof sketch:
- From N_0, follow any path to node N
- Compose TRANSIT values (and inverses for backwards arrows) along path
- The result gives CONTENTS(N)
- Path independence follows from consistency criterion 9.2.1(D)

# Musical Context/Application
Operation networks are the standard analytical tool. The unique determination theorem means that:
1. An analyst can "seed" one node with a musical object
2. All other contents follow automatically from the graph structure
3. Alternatively, given a musical passage, one can check if it fits a proposed graph

# Examples
From Figure 9.4:
- Given s_0 at N_0, and operations A, B, C, D on arrows
- CONTENTS of N_1 = A(s_0) [forward along A-arrow]
- CONTENTS of N_2 = B^(-1)(s_1) = B^(-1)(A(s_0)) [backward along B-arrow]
- CONTENTS of N_3 = D(s_2) [forward along D-arrow]
- Consistency: s_3 should also equal C^(-1)(s_1), verified by B = CD

# Related Concepts
- Operation Graph
- Transformation Network Definition
- CONTENTS Function
- Connected System
- Group of Operations

# Common Confusions
- Operation networks require SGP to be a group, not just a semigroup
- "Unique determination" requires the graph to be connected
- The theorem guarantees existence and uniqueness given one node's contents
- Walking backwards along arrows uses inverse operations (possible because SGP is a group)

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.3.2 and Theorem 9.3.3
