---
concept: Tritone GIS Example
category: analysis
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A GIS on the six tritones (unordered pairs of pitch classes spanning tritone interval), demonstrating how a simply transitive group of six formal "transposition operations" creates an interval system for these set-classes.

# Formal Definition
The Tritone GIS:
- S = the six tritones: (C, F#), (C#, G), (D, Ab), (Eb, A), (E, Bb), (F, B)
- Transposition by interval i and by interval i+6 have the same effect on tritones
- Six formal transposition operations: 0-or-6, 1-or-7, 2-or-8, 3-or-9, 4-or-10, 5-or-11
- These form a simply transitive group on the six tritones

# Mathematical Formulation
The six operations form a GIS:
- int((C, F#), (F, B)) = 5-or-11 (transposing (C, F#) by 5 or by 11 yields (F, B))
- Group structure: cyclic group of order 6
- Each operation is a coset of {0, 6} in Z_12

The homomorphism SGMAP:
- Domain: pitch-class intervals (Z_12)
- Codomain: tritone intervals (Z_6)
- SGMAP(i) = i-or-(i+6)
- This is a group homomorphism (not isomorphism)

# Musical Context/Application
This example demonstrates graph homomorphism (Section 9.5.4). A network on pitch classes can map homomorphically to a network on tritones, collapsing interval information in a systematic way. The tritone GIS captures relationships between tritone set-classes using interval-like language.

# Examples
From Figure 9.7:
- Network (a): pitch-class network with intervals 6 and 5
- Network (b): tritone network with interval 5-or-11
- NODEMAP collapses pairs of pc-nodes into single tritone nodes
- SGMAP collapses pairs of pc-intervals into single tritone intervals

The homomorphism:
- Two top nodes of (a) both map to top node of (b)
- Two bottom nodes of (a) both map to bottom node of (b)
- The six intervals between them collapse to one tritone interval

# Related Concepts
- Simply Transitive Group
- GIS from Simply Transitive Group
- Graph Homomorphism
- SGMAP
- NODEMAP

# Common Confusions
- Tritone here means unordered pair, not the interval itself
- The six formal intervals (like "5-or-11") are equivalence classes of pitch-class intervals
- SGMAP is a homomorphism, not an isomorphism (not 1-to-1)
- This GIS exists because the transposition group acts simply transitively on tritones

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.5.4, Figure 9.7
