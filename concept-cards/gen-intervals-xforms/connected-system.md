---
concept: Connected System
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A node/arrow system is connected if any two nodes communicate - that is, there is always a path (forwards or backwards) between any pair of nodes.

# Formal Definition
A node/arrow system (NODES, ARROW) is connected if for any two nodes N and N' in NODES, N communicates with N'.

Equivalently: The communication equivalence relation has only one equivalence class (all nodes are in the same class).

# Mathematical Formulation
(NODES, ARROW) is connected iff:
- For all N, N' in NODES: there exists a path N_0, N_1, ..., N_J with N_0 = N, N_J = N', and each consecutive pair in some arrow relation

Disconnected systems decompose:
- Any disconnected system can be analyzed into connected subsystems
- Each subsystem is (NODES_i, ARROW_i) where NODES_i is an equivalence class under communication
- ARROW_i is the restriction of ARROW to pairs within NODES_i

# Musical Context/Application
Connected networks represent unified analytical structures where any musical object can be related to any other through a chain of transformations. Disconnected networks represent analytically independent components - useful when analyzing music with genuinely separate structural elements.

# Examples
Figure 9.1 is not connected:
- It decomposes into two connected components
- Within each component, all nodes communicate
- Between components, no nodes communicate

A typical connected network:
- The CADENCE graph is connected (all four nodes communicate)
- A linear melodic network (each note connected to next) is connected

# Related Concepts
- Communication Between Nodes
- Node/Arrow System
- Connected Component
- Transformation Graph
- Equivalence Class

# Common Confusions
- Connected does not mean every pair has a direct arrow (only that paths exist)
- Disconnected systems are not malformed; they represent multiple independent structures
- Connected refers to the abstract system, not to musical continuity
- A single disconnected system can be analytically useful (parallel voice analysis, etc.)

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.1.3, Definition
