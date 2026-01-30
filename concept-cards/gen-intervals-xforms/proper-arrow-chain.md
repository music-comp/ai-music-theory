---
concept: Proper Arrow Chain
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
unit: null
authors: David Lewin
---

# Quick Definition
An arrow chain that contains at least one "one-way arrow" - an arrow (N_{j-1}, N_j) where the reverse (N_j, N_{j-1}) is NOT in the ARROW relation, making the chain impossible to walk backwards.

# Formal Definition
An arrow chain N_0, N_1, ..., N_J is proper if:
- There exists at least one j (between 1 and J) such that (N_j, N_{j-1}) is NOT in ARROW
- The chain contains at least one genuinely directed step
- The chain "cannot be walked backwards" at some point

# Mathematical Formulation
For arrow chain N_0 -> N_1 -> ... -> N_J:
- Each step: (N_{j-1}, N_j) in ARROW
- Proper if: exists j with (N_j, N_{j-1}) not in ARROW

The implicit arrow (N, N) always has its reverse (N, N) in ARROW, so:
- (N, N) alone is never a "one-way arrow"
- Trivial chains (J = 0) are not proper

# Musical Context/Application
Proper arrow chains define the precedence relation. They represent genuine directed transformational paths that cannot be reversed, capturing the asymmetric "before/after" or "cause/effect" relationships in a network. Proper chains distinguish transformations that are intrinsically directed from those that are merely relational.

# Examples
From Figure 9.12:
- M_1 -> M_2 -> M_3: If (M_2, M_3) is one-way (M_3 -> M_2 not in ARROW), this is proper
- M_1 -> M_2: If both directions are in ARROW, this by itself is not proper
- M_1 precedes M_3 because of the proper chain through M_2

Testing properness:
- Chain from N to N': Check each arrow (N_{j-1}, N_j)
- If any reverse (N_j, N_{j-1}) is absent from ARROW, the chain is proper
- If ALL reverses are present, the chain is not proper

# Related Concepts
- Arrow Chain
- Precedence Ordering
- Precedence-Ordered System
- One-Way Arrow
- Node/Arrow System

# Common Confusions
- "Proper" does not mean "correct" - it means "contains one-way arrows"
- A chain may be proper due to just one one-way arrow
- Proper chains define precedence; non-proper chains only establish communication
- The trivial chain (node to itself) is never proper

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.1, Definition
