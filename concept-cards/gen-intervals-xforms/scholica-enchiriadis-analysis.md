---
concept: Scholica Enchiriadis Analysis
category: analysis
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
An analysis of the opening phrase from the Scholica Enchiriadis example of Symphony of the Diatesseron, demonstrating homomorphisms, product networks, and networks-of-networks through medieval parallel organum.

# Formal Definition
The analysis (Figure 9.8) examines "Nos qui vivimus" in parallel organum:
- (a): Musical transcription showing Principalis and Organalis
- (b): Graph of the melody (step intervals 1, -1, 0, 0)
- (c): Disconnected network with separate Principalis and Organalis lines
- (d): Product network (not a homomorphic image of (b))
- (f): Network-of-networks with outer graph (b), inner graph (e)
- (g): Network-of-networks with outer graph (e), inner graph (b)

# Mathematical Formulation
Homomorphism result:
- Graph (b) IS a homomorphic image of graph (c)
- NODEMAP: collapses corresponding Principalis/Organalis nodes
- SGMAP: identity on interval group
- Graph (b) is NOT a homomorphic image of graph (d)
- Proof: No SGMAP can satisfy SGMAP(1) = 1 and SGMAP(3) = 0

Product structure:
- Graph (d) = "product" of graph (b) with graph (e)
- Graph (e): two nodes related by interval 3 (the diatesseron)

# Musical Context/Application
The analysis distinguishes multiple valid models for the same music:
- (c): Two separate melodic lines
- (d): Product of melody and diatesseron
- (f): "Singing the melody, singing diatessera as we go"
- (g): "Principalis sings the melody; I sing the melody in diatesseron relation"

The T_3 interval is "climb three rungs on the modal ladder" - not the same as RISE(4/3), the harmonic ratio 4:3. This distinction relates to performance problems of the style.

# Examples
From Figure 9.8:
- Melody intervals: 1, -1, 0, 0 (step motion in mode)
- Diatesseron interval: 3 (the Symphony)
- Product network has arrows labeled 1, -1, 0, 3

The non-homomorphism result:
- Any SGMAP preserving SGMAP(1) = 1 must have SGMAP(3) = 3
- But arrows on (d) show paths with interval 3 that collapse to interval 0 in (b)
- Therefore no valid SGMAP exists for (d) -> (b)

# Related Concepts
- Graph Homomorphism
- Network of Networks
- Product Networks
- Medieval Organum
- Diatesseron Interval

# Common Confusions
- T_3 (modal steps) differs from RISE(4/3) (frequency ratio)
- The failure of homomorphism (d) -> (b) is a theorem, not a technical flaw
- Multiple network models can be valid for the same music
- The "Symphony" refers to the diatesseron relationship, not a musical form

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.5.5, Figure 9.8
