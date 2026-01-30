---
concept: Operation Graph
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A transformation graph in which SGP is a group (rather than just a semigroup), meaning all transformations are invertible operations.

# Formal Definition
An operation graph is a transformation graph (NODES, ARROW, SGP, TRANSIT) where SGP is a group.

Consequences:
- All transformations in SGP are invertible
- TRANSIT(N, N) must be the identity (the only idempotent in a group is identity)
- Networks built on operation graphs can "walk backwards" along arrows using inverse operations

# Mathematical Formulation
Since SGP is a group:
- Every element has an inverse in SGP
- The identity element e is the unique idempotent
- TRANSIT(N, N) = e for all nodes N
- Arrow chains can be traversed in reverse using inverse transformations

# Musical Context/Application
Operation graphs are the most common type in musical analysis because most musical transformations (transpositions, inversions, Klang transformations) are invertible. The group structure allows flexible navigation through the network, determining contents of any node from any other node in a connected system.

# Examples
Most graphs in chapters 7-8 are operation graphs:
- Klang transformation graphs (DOM, MED, PAR, etc. form a group)
- Pitch-class transformation graphs (transpositions and inversions form a group)
- RI-chain graphs (RICH is invertible)

From Theorem 9.3.3:
- Given a connected operation graph and contents for one node
- All other node contents are uniquely determined
- This follows from the invertibility of group operations

# Related Concepts
- Transformation Graph Definition
- Operation Network
- Group
- Simply Transitive Group
- TRANSIT Function

# Common Confusions
- Not all transformation graphs are operation graphs (some use non-invertible transformations)
- "Operation" here specifically means group element, not just any transformation
- The unique determination theorem (9.3.3) requires operation graphs, not just transformation graphs
- TRANSIT(N, N) = identity is forced by the group structure

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.2.3, Definition
