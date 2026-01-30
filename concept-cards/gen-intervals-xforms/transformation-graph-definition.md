---
concept: Transformation Graph Definition
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A transformation graph is an ordered quadruple (NODES, ARROW, SGP, TRANSIT) consisting of a node/arrow system, a semigroup of transformations, and a function assigning a transformation to each arrow, with a consistency requirement on arrow chains.

# Formal Definition
A transformation graph is (NODES, ARROW, SGP, TRANSIT) satisfying:
- (A): (NODES, ARROW) is a node/arrow system
- (B): SGP is a semigroup
- (C): TRANSIT is a function mapping ARROW into SGP
- (D): For any two arrow chains from N to N', the products of TRANSIT values along the chains must be equal

Criterion (D) ensures that the transformation from N to N' is well-defined regardless of which path is taken.

# Mathematical Formulation
The consistency condition (D) formally:
- Let N_0, N_1, ..., N_J be one arrow chain from N to N'
- Let M_0, M_1, ..., M_K be another arrow chain from N to N'
- Let x_j = TRANSIT(N_{j-1}, N_j) for j = 1 to J
- Let y_k = TRANSIT(M_{k-1}, M_k) for k = 1 to K
- Then x_J ... x_2 x_1 = y_K ... y_2 y_1 (in left orthography)

Consequence: TRANSIT(N, N) must be idempotent in SGP.

# Musical Context/Application
Transformation graphs provide the formal skeleton for analyzing musical transformations. The TRANSIT function labels arrows with specific transformations; the consistency requirement ensures that the graph represents a coherent transformational structure where different analytical paths yield compatible results.

# Examples
Simple transformation graph:
- NODES = {N_1, N_2, N_3}
- ARROW = all pairs plus reflexive
- SGP = group of transpositions
- TRANSIT assigns T_5 to (N_1, N_2), T_3 to (N_2, N_3), T_8 to (N_1, N_3)
- Consistency: T_3 * T_5 = T_8 (required)

From Figure 9.4:
- Operations A, B, C, D on arrows
- Arrow chains must satisfy B = CD (consistency along two paths to N_3)

# Related Concepts
- Node/Arrow System
- TRANSIT Function
- Semigroup
- Operation Graph
- Transformation Network

# Common Confusions
- The graph does not yet contain musical objects (that's what networks add)
- SGP being a semigroup (not necessarily a group) allows non-invertible transformations
- Criterion (D) is a constraint on valid graphs, not something to verify after construction
- TRANSIT values combine in left orthography: rightmost transformation applies first

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.2.1, Definition
