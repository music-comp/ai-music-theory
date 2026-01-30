---
concept: START Node
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A formal node adjoined to a network with an arrow pointing to a designated "starting" node, providing a mechanism to establish structural priority that supersedes formal input nodes.

# Formal Definition
A START node is a node added to the node/arrow system such that:
1. It contains the symbol "START" (or similar marker)
2. An arrow issues from START to a designated node N
3. START is declared to supersede other input nodes in function
4. The START arrow establishes structural priority independently of the graph's intrinsic input nodes

# Mathematical Formulation
Formally adjoining START:
- New NODES' = NODES union {START}
- New ARROW' = ARROW union {(START, N)} for designated N
- The START node becomes the unique input in NODES'
- All former input nodes can now be reached from START only by traversing arrows

# Musical Context/Application
START nodes address situations where formal input nodes do not match structural priority. When a graph has multiple inputs or when the formal input does not represent the musical "beginning," a START node can designate the analytically prior element. This is particularly useful for:
- Tonal centers that are not graph-theoretic inputs
- Temporal beginnings that differ from structural origins
- Analytical readings that prioritize specific nodes

# Examples
From Figure 9.14(b) (Beethoven Appassionata):
- The Gb nodes are formal input nodes (arrows only go out)
- But Db has structural priority as tonic
- A START node pointing to Db establishes this priority
- From START, reaching Gb nodes requires walking arrows backwards
- This asymmetry formally captures Gb's subordinate status

Criteria for placing START arrow:
- Diachronic: Point to the first event heard in music
- Synchronic: Point to the structurally prior element (e.g., tonic)
- Either criterion is valid depending on analytical purpose

# Related Concepts
- Input Node
- Output Node
- Precedence Ordering
- Structural Priority
- Beethoven Appassionata Analysis

# Common Confusions
- START is a formal device, not a musical event
- Multiple analytical approaches may place START differently
- START supersedes intrinsic input nodes by convention, not by graph structure
- The START node itself has no musical contents (or contains only "START")

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.6
