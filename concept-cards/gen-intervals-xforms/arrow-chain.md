---
concept: Arrow Chain
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A finite series of nodes where each consecutive pair is connected by a forwards-oriented arrow, representing a directed path through a node/arrow system.

# Formal Definition
An arrow chain from node N to node N' is a finite series of nodes N_0, N_1, ..., N_J satisfying:
- (A): N_0 = N
- (B): For each j between 1 and J inclusive, (N_{j-1}, N_j) is in the ARROW relation
- (C): N_J = N'

The chain must follow arrows in their designated forward direction (unlike communication paths).

# Mathematical Formulation
Arrow chain N_0 -> N_1 -> ... -> N_J:
- Each (N_{j-1}, N_j) must be in ARROW
- The chain has J "steps" (arrows traversed)
- J >= 0 is allowed (trivial chain from N to N with J = 0)

Arrow chains are used to define:
- Transformation products along paths
- Precedence relations
- Proper arrow chains (with at least one one-way arrow)

# Musical Context/Application
Arrow chains represent directed transformational paths through a network. When a graph is labeled with transformations, following an arrow chain means composing the transformations in sequence. The chain from input to output in a network represents the total transformation applied.

# Examples
From Figure 9.2:
- Arrow chain from N to N': N = N_0 -> N_1 -> ... -> N_J = N'
- TRANSIT labels: x_1 = TRANSIT(N_0, N_1), ..., x_J = TRANSIT(N_{J-1}, N_J)
- The product x_J ... x_2 x_1 (left orthography) gives the total transformation

Multiple chains between same endpoints:
- Figure 9.2 shows two chains from N to N'
- Criterion 9.2.1(D) requires their transformation products to be equal
- This consistency requirement is essential for well-formed graphs

# Related Concepts
- Node/Arrow System
- Communication Between Nodes
- Proper Arrow Chain
- Transformation Graph
- TRANSIT Function

# Common Confusions
- Arrow chains follow arrows forwards only (unlike communication paths)
- The trivial chain (J = 0) from N to N is an arrow chain
- Multiple chains between the same endpoints must yield equal transformation products
- "Chain" is ordered; the same nodes in different order form different chains

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.1.4, Definition
