---
concept: Isography
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
unit: null
authors: David Lewin
---

# Quick Definition
Two transformation networks are isographic if their underlying transformation graphs are isomorphic - they share the same structure of nodes, arrows, and transformation labels (up to semigroup isomorphism), even though the contents of their nodes may differ.

# Formal Definition
Networks (S, NODES, ARROW, SGP, TRANSIT, CONTENTS) and (S', NODES', ARROW', SGP', TRANSIT', CONTENTS') are isographic if:
- Their graphs (NODES, ARROW, SGP, TRANSIT) and (NODES', ARROW', SGP', TRANSIT') are isomorphic

Graph isomorphism requires:
1. NODEMAP: isomorphism of node/arrow systems
2. SGMAP: isomorphism of semigroups
3. Compatibility: TRANSIT'(NODEMAP(N_1), NODEMAP(N_2)) = SGMAP(TRANSIT(N_1, N_2))

# Mathematical Formulation
Formally, if (NODEMAP, SGMAP) is an isomorphism of graph 1 with graph 2, then it is an isography of network 1 with network 2.

The key requirement is that transformations correspond under SGMAP:
- If arrow (N_1, N_2) has label f in network 1
- Then arrow (NODEMAP(N_1), NODEMAP(N_2)) has label SGMAP(f) in network 2

# Musical Context/Application
Isography allows comparison of structural relationships across different musical domains. Two networks may be isographic even when:
- S and S' are different object families (pitch classes vs. row forms)
- The specific transformations differ (T_10 vs. T_3)
- The musical content appears unrelated

This reveals deep structural similarities that transcend surface differences.

# Examples
From Sections 8.2.4 and 8.5:
- Wagner's Parsifal network (Figure 8.3) is isographic to portions of Webern's op. 27 (Figure 8.4b)
- Both use RICH and TCH in the same graph structure
- The isography would NOT obtain if we wrote "T_10" and "T_3" instead of TCH

From Figure 9.5:
- Networks (a), (b), (c), (d), (e) are all isographic
- They involve different pitch classes but the same IPAIR graph structure
- The I operation differs ((I_A^A) vs. (I_Bb^Bb)) but is isomorphic

# Related Concepts
- Graph Isomorphism
- Transformation Graph
- Transformation Network
- SGMAP
- NODEMAP

# Common Confusions
- Isography is about graph structure, not about musical similarity
- The contents (CONTENTS function) do not affect isography
- Using specific transposition numbers (T_10, T_3) instead of TCH would destroy isography
- Isography requires semigroup isomorphism, not just node/arrow correspondence

# Source Reference
Chapter 8, Section 8.2.4, and Chapter 9, Sections 9.4.1-9.4.4
