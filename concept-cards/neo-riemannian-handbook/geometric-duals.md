---
concept: Geometric Duals
slug: geometric-duals

category: pitch-space
subcategory: spatial representations of pitch
tier: advanced

source: "The Oxford Handbook of Neo-Riemannian Music Theories"
source_slug: neo-riemannian-handbook
authors: "Richard Cohn"
chapter: "Tonal Pitch Space and the (Neo-)Riemannian Tonnetz"
chapter_number: 11
pdf_page: null
section: null

extraction_confidence: high

aliases:
  - "graph duality"
  - "dual graphs"
  - "chicken-wire torus and Tonnetz duality"

prerequisites:
  - tonnetz
  - toroidal-tonnetz
extends: []
related:
  - hexatonic-systems
  - common-tone-relationships
  - regional-space
contrasts_with: []

answers_questions:
  - "How are the pitch-class Tonnetz and the triadic Tonnetz related?"
  - "What are geometric duals in the context of the Tonnetz?"
---

# Quick Definition

The mathematical relationship between two Tonnetz representations where the nodes of one graph become the faces of the other and vice versa: the pitch-class Tonnetz (nodes = pitches, triangles = triads) and the triadic Tonnetz (nodes = triads, edges = common-tone connections) are geometric duals of each other.

# Core Definition

Cohn (Ch. 11) identifies a fundamental **geometric duality** between two Tonnetz representations. In the **pitch-class Tonnetz**, nodes represent pitch classes, edges represent consonant dyadic intervals, and triangular faces represent triads. In the **triadic Tonnetz** (also called the "chicken-wire torus"), nodes represent triads and edges represent common-tone or PLR connections between triads. These two graphs are **geometric duals**: the labels (faces) of one become the nodes of the other. This duality is central to Cohn's argument that the Tonnetz naturally represents multiple levels of tonal organization simultaneously, contra Lerdahl's claim that pitch, chord, and key spaces require separate representations. The mutual implication of these levels through geometric duality demonstrates that knowing any two levels determines the third.

# Prerequisites

- **Tonnetz**: The pitch-class lattice from which the dual is derived
- **Toroidal Tonnetz**: The duality holds specifically on the conforming (equal-tempered) torus

# Key Properties

1. **Node-face exchange**: Nodes in one graph become faces in the dual, and vice versa
2. **Edge correspondence**: Edges connecting nodes in one graph correspond to edges separating faces in the dual
3. **Topological preservation**: Both graphs live on the same underlying surface (torus)
4. **Mutual implication**: Pitch-class information determines triadic information and vice versa
5. **Regional emergence**: Diatonic regions (parallelograms) in the pitch-class Tonnetz become nodes of a third dual graph at the key level

# Construction / Recognition

To construct the triadic dual of the pitch-class Tonnetz:
1. Start with the pitch-class Tonnetz (nodes = 12 pitch classes, triangular faces = 24 triads)
2. Place a new node at the center of each triangular face (representing each triad)
3. Connect two new nodes if their original triangles share an edge (common-tone connection)
4. The resulting graph is the triadic Tonnetz (chicken-wire torus)
5. Conversely, starting from the triadic graph and placing nodes at face centers recovers the pitch-class Tonnetz

# Context & Application

Geometric duality is Cohn's key technical argument against Lerdahl's separation of tonal levels. If the pitch-class Tonnetz and triadic Tonnetz are duals of the same underlying structure, then pitch-class space and chord space are not independent — they are two views of the same object. Extending to a third level, diatonic regions (grouped triads) can be seen as faces of the triadic graph, producing a regional dual. This three-level mutual implication makes Lerdahl's three separate spaces redundant, according to Cohn.

# Examples

On the conforming (toroidal) Tonnetz (Ch. 11):
- **Pitch-class Tonnetz**: 12 nodes (pitch classes), 36 edges (consonant intervals), 24 triangular faces (triads)
- **Triadic Tonnetz (chicken-wire torus)**: 24 nodes (triads), 36 edges (P, L, R connections), 12 hexagonal faces (pitch classes)
- The 36 edges are shared: each edge in the pitch-class graph separating two triads becomes the edge in the triadic graph connecting those same two triads

# Relationships

## Builds Upon
- tonnetz (the pitch-class graph from which the dual is derived)
- toroidal-tonnetz (duality applies on the torus)

## Enables
- Cohn's argument for unified multi-level tonal representation
- regional-space (regions emerge as a third level of the duality chain)

## Related
- hexatonic-systems (appear as specific substructures on both dual graphs)
- common-tone-relationships (edges in the triadic dual represent common-tone connections)

## Contrasts With
(none specific)

# Common Errors

- **Error**: Treating the pitch-class and triadic Tonnetze as entirely separate structures
  **Correction**: They are geometric duals of the same underlying topological surface; information in one completely determines the other

# Common Confusions

- **Confusion**: Geometric duality means the two graphs look the same
  **Clarification**: Duality means they have a precise mathematical relationship (node-face exchange), not that they are visually identical; the pitch-class Tonnetz has triangular faces while the triadic Tonnetz has hexagonal faces

# Source Reference

Cohn, Richard. "Tonal Pitch Space and the (Neo-)Riemannian Tonnetz." In *The Oxford Handbook of Neo-Riemannian Music Theories*, edited by Edward Gollin and Alexander Rehding. Oxford University Press, 2011. Chapter 11.

# Verification Notes

New card (no previous version; the concept was briefly mentioned in the old tonnetz card but not given its own treatment). High confidence: Cohn explicitly discusses geometric duality as a key structural property of the Tonnetz in Ch. 11.
