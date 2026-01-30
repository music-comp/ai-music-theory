---
concept: Network of Networks
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A transformation network where each node contains not a simple musical object but an entire network, allowing hierarchical or layered analytical structures.

# Formal Definition
A network-of-networks has:
1. An outer network structure (NODES, ARROW, SGP, TRANSIT)
2. Each CONTENTS(N) is itself a transformation network
3. The outer transformations operate on entire inner networks

Two varieties shown in Section 9.5.5:
- Type (f): Outer graph is (b), each node contains an (e)-network
- Type (g): Outer graph is (e), each node contains a (b)-network

# Mathematical Formulation
For networks-of-networks to be well-formed:
- The transformations in outer SGP must be operations (invertible)
- The inner network transformations must commute with outer transformations
- Example: T_3 (outer) commutes with T_1, T_{-1}, T_0 (inner)

Product networks (type (d) in Figure 9.8) are related but formally distinct:
- Product network: NODES is a Cartesian product
- Network-of-networks: NODES contains entire networks as elements

# Musical Context/Application
Networks-of-networks model analytical situations where:
- Each structural element is itself structured (e.g., each chord is a melodic pattern)
- Transformations operate at multiple levels (transposing entire sub-structures)
- Hierarchical relationships exist between levels of structure

The Scholica Enchiriadis example models parallel organum: "We are singing the melody, singing diatessera as we go."

# Examples
From Figure 9.8 (Scholica Enchiriadis):
- Network (f): Graph (b) as outer structure, graph (e) at each node
  - Each node contains a diatesseron (two voices a fourth apart)
  - Outer arrows transpose entire diatessera along the melody
  - Models: "singing the melody, singing diatessera as we go"

- Network (g): Graph (e) as outer structure, graph (b) at each node
  - Each node contains the entire melody "Nos qui vivimus"
  - Outer arrow relates Principalis melody to Organalis melody
  - Models: "I am singing the melody, in diatesseron relation to Principalis"

# Related Concepts
- Transformation Network Definition
- Product Networks
- Commuting Transformations
- Hierarchical Structure
- Scholica Enchiriadis Analysis

# Common Confusions
- Network-of-networks differs from product networks (though related)
- The outer/inner distinction is a matter of perspective (f) vs (g)
- Commutativity of transformations is required for well-formedness
- Not all analytical situations suit this structure

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.5.5, Figure 9.8
