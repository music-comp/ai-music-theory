---
concept: Transformation Network Definition
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A transformation network is a transformation graph together with a CONTENTS function that assigns a musical object to each node, such that following an arrow applies the corresponding transformation to the contents.

# Formal Definition
A transformation network is an ordered sextuple (S, NODES, ARROW, SGP, TRANSIT, CONTENTS) where:
- (A): S is a family of objects (to be transformed)
- (B): (NODES, ARROW, SGP, TRANSIT) is a transformation graph with SGP acting on S
- (C): CONTENTS is a function mapping NODES into S
- (D): For any arrow (N_1, N_2), if f = TRANSIT(N_1, N_2), then f(CONTENTS(N_1)) = CONTENTS(N_2)

# Mathematical Formulation
The key requirement (D):
- Let s_1 = CONTENTS(N_1) and s_2 = CONTENTS(N_2)
- Let f = TRANSIT(N_1, N_2)
- Then: f(s_1) = s_2

This ensures that the arrow labels correctly describe transformations between node contents.

# Musical Context/Application
Transformation networks are the analytical tool proper - they combine:
1. Abstract graph structure (nodes and arrows)
2. Transformation labels (TRANSIT function)
3. Musical content (CONTENTS function)

The network represents a complete analytical claim about how musical objects relate through transformations.

# Examples
From Figure 9.3:
- S = pitch classes (or Klangs, or row forms, etc.)
- Nodes N_1 and N_2 connected by arrow labeled f
- s_1 at N_1, s_2 at N_2
- The network asserts: f(s_1) = s_2

Klang network example:
- S = 24 Klangs
- CONTENTS(N) = (C, +), CONTENTS(N') = (F, +)
- TRANSIT(N, N') = DOM
- Verification: (C, +)DOM = (F, +) check

# Related Concepts
- Transformation Graph Definition
- CONTENTS Function
- Operation Network
- Isography
- Graph versus Network

# Common Confusions
- A graph has no contents; a network has both graph structure AND contents
- The CONTENTS function must be compatible with TRANSIT (requirement D)
- Not every assignment of objects to nodes forms a valid network
- Networks on the same graph may be isographic without having identical contents

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.3.1, Definition
