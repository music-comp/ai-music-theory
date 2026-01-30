---
concept: TRANSIT Function
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
The function in a transformation graph that assigns a transformation (from the semigroup SGP) to each arrow in the node/arrow system, labeling the arrows with specific operations.

# Formal Definition
In a transformation graph (NODES, ARROW, SGP, TRANSIT):
- TRANSIT: ARROW -> SGP
- For each pair (N_1, N_2) in ARROW, TRANSIT(N_1, N_2) is a member of SGP
- TRANSIT assigns the transformation that takes contents of N_1 to contents of N_2 (when network is filled)

# Mathematical Formulation
Properties:
- Domain: ARROW (all ordered pairs in the arrow relation)
- Codomain: SGP (the semigroup of transformations)
- TRANSIT(N, N) must be idempotent (when SGP is a group, must be identity)

In networks:
- If f = TRANSIT(N_1, N_2), then f(CONTENTS(N_1)) = CONTENTS(N_2)
- Following arrows means applying TRANSIT transformations in sequence

# Musical Context/Application
TRANSIT provides the "labeling" that makes arrows meaningful. When analyzing music, TRANSIT encodes which transformation (transposition, inversion, mode change, etc.) corresponds to moving from one musical object to another.

# Examples
From Figure 9.3:
- f = TRANSIT(N_1, N_2)
- s_1 = CONTENTS(N_1), s_2 = CONTENTS(N_2)
- The network requirement: f(s_1) = s_2

From Figure 9.4:
- TRANSIT assigns A, B, C, D to various arrows
- Consistency requires B = CD (transformation products along different paths must match)

# Related Concepts
- Transformation Graph Definition
- Arrow Chain
- CONTENTS Function
- Semigroup
- Operation Graph

# Common Confusions
- TRANSIT assigns transformations to arrows, not to nodes
- TRANSIT(N, N) is not necessarily the identity (only idempotent) unless SGP is a group
- The function labels arrows in the abstract graph before any musical content is assigned
- Following an arrow "applies" the TRANSIT transformation to node contents

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.2.1
