---
concept: Graph Homomorphism
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A structure-preserving map between transformation graphs that need not be bijective - a generalization of isomorphism allowing many-to-one mappings and non-surjective mappings.

# Formal Definition
A homomorphism from graph (NODES, ARROW, SGP, TRANSIT) into/onto graph (NODES', ARROW', SGP', TRANSIT') is a pair (NODEMAP, SGMAP) with:
- (A): NODEMAP is a homomorphism of node/arrow systems
- (B): SGMAP is a homomorphism of semigroups SGP into/onto SGP'
- (C): For all (N_1, N_2) in ARROW: TRANSIT'(NODEMAP(N_1), NODEMAP(N_2)) = SGMAP(TRANSIT(N_1, N_2))

A homomorphism is "onto" if both NODEMAP and SGMAP are onto.
A homomorphism is "1-to-1" if both NODEMAP and SGMAP are 1-to-1.

# Mathematical Formulation
A 1-to-1 homomorphism onto is an isomorphism.

Types of homomorphisms:
- NODEMAP may collapse multiple nodes (not 1-to-1)
- SGMAP may collapse multiple transformations (not 1-to-1)
- The target graph may have "more structure" than the image (homomorphism not onto)

# Musical Context/Application
Graph homomorphisms model relationships where one structure is a simplification, augmentation, or abstraction of another. Examples include:
- Intervallic augmentation (doubling all intervals)
- Collapsing parallel voices into one line
- Relating complex structures to simpler underlying patterns

# Examples
From Section 9.5.3 (Brahms Horn Trio):
- Graph (a): complementary gesture (8 = 10 + 10)
- Graph (b): complementary gesture times 2 (4 = 8 + 8)
- NODEMAP = identity on NODES (an isomorphism)
- SGMAP(i) = 2i (maps interval i to interval 2i)
- SGMAP is a homomorphism but not an isomorphism (not 1-to-1)

From Section 9.5.5 (Scholica Enchiriadis):
- Graph (b) is a homomorphic image of graph (c)
- NODEMAP collapses Principalis and Organalis nodes to single nodes
- SGMAP = identity (same interval group)

# Related Concepts
- Graph Isomorphism
- Semigroup Homomorphism
- Node/Arrow System Homomorphism
- Intervallic Augmentation
- Network Relationships

# Common Confusions
- Homomorphism is weaker than isomorphism (allows non-bijective maps)
- "Onto" for a graph homomorphism requires both components to be onto
- A homomorphism that is 1-to-1 and onto is automatically an isomorphism
- The three examples (9.5.3, 9.5.4, 9.5.5) show very different homomorphism types

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.5.1-9.5.2
