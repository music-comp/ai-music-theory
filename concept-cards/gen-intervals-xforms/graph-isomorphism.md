---
concept: Graph Isomorphism
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
Two transformation graphs are isomorphic if there exists a pair of bijections (NODEMAP, SGMAP) that preserve all structure: node/arrow relationships and transformation labels.

# Formal Definition
Graphs (NODES, ARROW, SGP, TRANSIT) and (NODES', ARROW', SGP', TRANSIT') are isomorphic if there exists (NODEMAP, SGMAP) with:
- (A): NODEMAP is an isomorphism of node/arrow systems
- (B): SGMAP is an isomorphism of semigroups SGP -> SGP'
- (C): For all (N_1, N_2) in ARROW: TRANSIT'(NODEMAP(N_1), NODEMAP(N_2)) = SGMAP(TRANSIT(N_1, N_2))

The pair (NODEMAP, SGMAP) is called an isomorphism of the first graph with the second.

# Mathematical Formulation
NODEMAP requirements:
- Bijection: NODES -> NODES'
- Preserves arrows: (N_1, N_2) in ARROW iff (NODEMAP(N_1), NODEMAP(N_2)) in ARROW'

SGMAP requirements:
- Bijection: SGP -> SGP'
- Preserves semigroup operation: SGMAP(fg) = SGMAP(f)SGMAP(g)

Compatibility (C):
- Arrow labels correspond under SGMAP after nodes are mapped by NODEMAP

# Musical Context/Application
Graph isomorphism captures structural equivalence between transformation graphs. Two graphs may use different semigroups and different node sets but have identical structure. This is the foundation for isography of networks - the key concept for comparing analytical claims across different musical domains.

# Examples
From Section 9.4.4:
- Graph (a) has SGP = {E, I} where I = I^A_A (pitch-class operations)
- Graph (c) has SGP' = {E, I} where I = inversion of rows about A
- SGMAP: pitch-class-E -> row-E, pitch-class-I -> row-I
- NODEMAP: identity on NODES
- The graphs are isomorphic (same structure, different object domains)

# Related Concepts
- Isography
- Node/Arrow System Isomorphism
- Semigroup Isomorphism
- Transformation Graph Definition
- Graph Homomorphism

# Common Confusions
- Isomorphism requires both NODEMAP and SGMAP to be bijections
- The semigroups may be "different" (acting on different sets) but must be isomorphic as abstract semigroups
- Graph isomorphism ignores CONTENTS (that's what networks add)
- Isomorphic graphs can underlie isographic networks with very different musical content

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.4.2, Definition
