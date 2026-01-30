---
concept: Node/Arrow System Homomorphism
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A mapping between node/arrow systems that preserves arrows (but not necessarily reflects them) - if nodes are in the arrow relation, their images are too, but the image system may have "more arrows."

# Formal Definition
A homomorphism from (NODES, ARROW) into (NODES', ARROW') is a map NODEMAP: NODES -> NODES' such that:
- (NODEMAP(N_1), NODEMAP(N_2)) is in ARROW' whenever (N_1, N_2) is in ARROW

A homomorphism is "onto" if: whenever N'_1 and N'_2 are in ARROW', there exist N_1, N_2 in ARROW with NODEMAP(N_1) = N'_1 and NODEMAP(N_2) = N'_2.

A homomorphism is "1-to-1" if NODEMAP is injective.

# Mathematical Formulation
Homomorphism NODEMAP satisfies:
- (N_1, N_2) in ARROW implies (NODEMAP(N_1), NODEMAP(N_2)) in ARROW'

The converse need not hold:
- It is possible for (NODEMAP(N_1), NODEMAP(N_2)) to be in ARROW' even when (N_1, N_2) is not in ARROW
- This happens when the target system "has more arrows"

A 1-to-1 onto homomorphism is an isomorphism (by Definition 9.4.1).

# Musical Context/Application
Homomorphisms model relationships where one network structure is a simplification or abstraction of another. Collapsing parallel voices, abstracting away structural details, or showing one structure as the image of another all involve homomorphisms.

# Examples
From Section 9.5.5 (Scholica Enchiriadis):
- Graph (c) has separate Principalis and Organalis voice networks
- Graph (b) is the single melody
- NODEMAP collapses corresponding voice nodes into melody nodes
- This is a homomorphism from (c) onto (b)

Contrast with isomorphism:
- An onto 1-to-1 homomorphism is automatically an isomorphism
- But a 1-to-1 homomorphism that is not onto, or an onto homomorphism that is not 1-to-1, is NOT an isomorphism

# Related Concepts
- Node/Arrow System Isomorphism
- Graph Homomorphism
- Node/Arrow System
- NODEMAP

# Common Confusions
- "Homomorphism onto" has a special definition (stronger than just surjective NODEMAP)
- 1-to-1 homomorphism is not automatically an isomorphism (must also be onto in the special sense)
- The target system may have arrows not in the image of ARROW
- Preservation of arrows is one-way; reflection is not required

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.5.1, Definition
