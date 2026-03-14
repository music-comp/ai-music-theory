---
concept: Communication Between Nodes
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
Two nodes N and N' in a node/arrow system communicate if there exists a finite path of forwards-or-backwards arrows connecting them.

# Formal Definition
Nodes N and N' communicate if there exist nodes N_0, N_1, ..., N_J satisfying:
- (A): N_0 = N
- (B): For each j between 1 and J inclusive, either (N_{j-1}, N_j) or (N_j, N_{j-1}) is in ARROW
- (C): N_J = N'

The path need not follow arrows in their designated direction; traversing backwards is allowed.

# Mathematical Formulation
Communication defines an equivalence relation on NODES:
- Reflexive: N communicates with N (via the trivial path N_0 = N)
- Symmetric: If N communicates with N', then N' communicates with N (reverse the path)
- Transitive: If N communicates with N' and N' communicates with N'', then N communicates with N'' (concatenate paths)

The equivalence classes partition NODES into "connected components."

# Musical Context/Application
Communication determines which parts of a network can influence each other through chains of transformations. Non-communicating nodes belong to separate, independent components of the analytical structure. Connected networks (where all nodes communicate) represent unified analytical structures.

# Examples
From Figure 9.1:
- M_1 and M_3 communicate (direct arrow)
- M_3 and M_4 communicate (direct arrow)
- M_1 and M_4 communicate (via M_3)
- M_5 does not communicate with M_1, M_3, or M_4 (separate component)

The system has two equivalence classes (connected components):
- {M_1, M_2, M_3, M_4}
- {M_5, M_6} (assuming these form a separate component)

# Related Concepts
- Node/Arrow System
- Connected System
- Arrow Chain
- Equivalence Relation
- Connected Component

# Common Confusions
- Communication allows backwards traversal; arrow direction doesn't restrict it
- Communication is symmetric: if A communicates with B, B communicates with A
- Non-communication means complete separation, not just missing direct arrow
- Every node communicates with itself (reflexivity)

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.1.2, Definition
