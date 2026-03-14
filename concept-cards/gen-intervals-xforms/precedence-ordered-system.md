---
concept: Precedence-Ordered System
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A node/arrow system where no node both precedes and follows another - equivalently, where PRECEDENCE forms a strict partial ordering with no cycles.

# Formal Definition
A node/arrow system is precedence-ordered if:
- There is no pair of nodes (N, N') such that N both precedes and follows N'
- Equivalently: PRECEDENCE satisfies (PO1) - no (N, N') has both (N, N') and (N', N) in PRECEDENCE
- There are no cycles through one-way arrows

# Mathematical Formulation
In a precedence-ordered system:
- PRECEDENCE is a strict partial ordering (Theorem 9.7.4)
- Nodes can be labeled 1 through J so that N_j precedes N_k implies j < k (Section 9.7.6)
- All one-way arrows can be drawn pointing "left to right"

Linear extension (Section 9.7.5):
- Every partial ordering can be extended to a linear ordering
- Multiple linear orderings may be compatible with one partial ordering

# Musical Context/Application
Precedence-ordered systems are "potentially compatible with naive chronology." When analyzing music with such a system, one can arrange nodes temporally so that formal precedence never contradicts musical chronology. However, the system does NOT require this - precedence is formal, chronology is musical.

# Examples
Most analytical networks are precedence-ordered:
- A chain of transpositions (A -> B -> C -> D) is precedence-ordered
- A network with multiple paths but no cycles is precedence-ordered
- The CADENCE network is precedence-ordered

Non-precedence-ordered (cyclic):
- If A precedes B, B precedes C, and C precedes A, the system is not precedence-ordered
- Such cycles are rare in musical analysis but possible

From Section 9.7.5:
- A precedence-ordered finite system can be "linearized" in possibly multiple ways
- The grouping of segments in linear orderings reflects the partial ordering structure

# Related Concepts
- Precedence Ordering
- Proper Arrow Chain
- Partial Ordering
- Linear Ordering
- Compatible Chronology

# Common Confusions
- Precedence-ordered does not mean linearly ordered (some nodes may be incomparable)
- Being precedence-ordered is about formal structure, not musical time
- A system can be precedence-ordered while musical events occur in different temporal order
- "Potentially compatible" means a compatible chronology exists, not that one is imposed

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.3, Definition
