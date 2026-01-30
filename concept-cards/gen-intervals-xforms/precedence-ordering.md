---
concept: Precedence Ordering
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
A partial ordering on nodes derived from the arrow structure, where N precedes N' if there is a proper arrow chain from N to N' - capturing a formal "before/after" relationship without requiring it to match musical chronology.

# Formal Definition
In a node/arrow system:
- N precedes N' if there exists a proper arrow chain from N to N'
- A proper arrow chain is one with at least one "one-way arrow" (an arrow (N_i, N_j) where (N_j, N_i) is NOT in ARROW)

The system is precedence-ordered if no node both precedes and follows another (no cycles through one-way arrows).

# Mathematical Formulation
PRECEDENCE = {(N, N') : N precedes N'}

Theorem 9.7.4: In a precedence-ordered system, PRECEDENCE is a strict partial ordering:
- (PO1): No (N, N') has both (N, N') and (N', N) in PRECEDENCE
- (PO2): If (N_1, N_2) and (N_2, N_3) in PRECEDENCE, then (N_1, N_3) in PRECEDENCE

# Musical Context/Application
Precedence ordering captures the inherent directionality in a network's arrow structure. Precedence-ordered systems are "potentially compatible with naive chronology" - nodes can be arranged so that precedence agrees with temporal order. However, precedence is a formal property; it need not match actual musical chronology.

# Examples
From Figure 9.12:
- M_1 precedes M_3 (via M_1 -> M_2 -> M_3 with one-way arrow M_2 -> M_3)
- M_1 does NOT precede M_2 (all arrows between them are two-way)
- M_1 is in ARROW relation to M_2, but M_1 does not precede M_2

From Figure 9.13:
- Two left nodes precede two right nodes
- Neither left node precedes the other
- Neither right node precedes the other
- Multiple linear orderings are compatible

# Related Concepts
- Proper Arrow Chain
- Precedence-Ordered System
- Linear Ordering
- Input Node
- Output Node

# Common Confusions
- Precedence is stricter than being in ARROW relation (requires one-way arrows)
- Precedence differs from musical chronology (formal vs. temporal)
- Not all systems are precedence-ordered (some have cycles)
- A node can be in ARROW relation to another without preceding it

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Sections 9.7.2-9.7.4
