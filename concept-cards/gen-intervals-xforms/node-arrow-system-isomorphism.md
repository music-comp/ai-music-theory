---
concept: Node/Arrow System Isomorphism
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
Two node/arrow systems are isomorphic if there exists a bijection between their node sets that preserves the arrow relation in both directions - nodes are in the arrow relation if and only if their images are.

# Formal Definition
Node/arrow systems (NODES, ARROW) and (NODES', ARROW') are isomorphic if there exists NODEMAP: NODES -> NODES' such that:
1. NODEMAP is 1-to-1 (injective)
2. NODEMAP is onto (surjective)
3. For every pair (N_1, N_2) of NODES: (N_1, N_2) is in ARROW if and only if (NODEMAP(N_1), NODEMAP(N_2)) is in ARROW'

Such a NODEMAP is called an isomorphism of the systems.

# Mathematical Formulation
The bijection NODEMAP preserves and reflects arrows:
- Preservation: (N_1, N_2) in ARROW implies (NODEMAP(N_1), NODEMAP(N_2)) in ARROW'
- Reflection: (NODEMAP(N_1), NODEMAP(N_2)) in ARROW' implies (N_1, N_2) in ARROW

Both directions are required; preservation alone only gives a homomorphism.

# Musical Context/Application
Node/arrow system isomorphism is the foundation for comparing transformation graphs. If two systems are isomorphic, they have identical structure - same number of nodes, same pattern of arrows. The musical content may differ, but the combinatorial skeleton is the same.

# Examples
Two IPAIR systems are isomorphic:
- System 1: NODES = {A, B}, all pairs in ARROW
- System 2: NODES' = {X, Y}, all pairs in ARROW'
- NODEMAP: A -> X, B -> Y
- Arrow preservation: (A, B) in ARROW iff (X, Y) in ARROW' [both true]

The CADENCE system (four nodes, specific arrows) is isomorphic to any other four-node system with the same arrow pattern.

# Related Concepts
- Node/Arrow System
- Graph Isomorphism
- Node/Arrow System Homomorphism
- Transformation Graph Definition
- NODEMAP

# Common Confusions
- Isomorphism is bidirectional (preservation AND reflection of arrows)
- Homomorphism is only one direction (preservation only)
- Isomorphism requires bijection; homomorphism allows non-bijective maps
- Node/arrow isomorphism ignores transformation labels (that's graph isomorphism)

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.4.1, Definition
