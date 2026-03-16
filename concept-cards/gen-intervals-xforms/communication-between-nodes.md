---
# === CORE IDENTIFICATION ===
concept: Communication Between Nodes
slug: communication-between-nodes

# === CLASSIFICATION ===
category: transformation-theory
subcategory: graph-network-foundations
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.1.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "communication relation"
  - "communicating nodes"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - node-arrow-system
extends: []
related:
  - connected-system
  - equivalence-relation
  - arrow-chain
contrasts_with:
  - arrow-chain

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "When do two nodes in a node/arrow system communicate?"
  - "What kind of relation is communication among nodes?"
---

# Quick Definition
Two nodes N and N' in a node/arrow system communicate if there exists a finite path of forwards-or-backwards arrows connecting them, forming an equivalence relation that partitions nodes into connected components.

# Core Definition
Nodes N and N' in a node/arrow system communicate if there exist nodes N0, N1, ..., NJ satisfying: (A) N0 = N; (B) for each j between 1 and J inclusive, either (N_{j-1}, N_j) or (N_j, N_{j-1}) is in ARROW; (C) NJ = N'. The criteria demand a finite unbroken path of forwards-or-backwards arrows starting at N and ending at N' (Lewin, Definition 9.1.2, p. 225).

# Prerequisites
- **Node/arrow system** — communication is defined on the nodes of a node/arrow system

# Key Properties
1. Communication is reflexive: N communicates with N (trivial path)
2. Communication is symmetric: if N communicates with N', then N' communicates with N (reverse the path)
3. Communication is transitive: if N communicates with N' and N' with N'', then N communicates with N'' (concatenate paths)
4. Communication is therefore an equivalence relation on NODES
5. The equivalence classes partition NODES into connected components

# Construction / Recognition
## To Construct:
1. Start at node N
2. Follow any arrow forwards or backwards to reach adjacent nodes
3. Continue until reaching N' or exhausting reachable nodes
## To Recognize:
1. Given nodes N and N', find any path (forwards or backwards along arrows) connecting them
2. If such a path exists, they communicate; otherwise they do not

# Context & Application
Communication determines which parts of a network can influence each other through chains of transformations. Non-communicating nodes belong to separate, independent components. The equivalence classes under communication are the connected components of the system.

# Examples
**Example 1** (Figure 9.1, p. 225): Nodes M1 and M3 communicate (direct arrow). M3 and M4 communicate. M1 and M4 communicate (via M3). But neither M1 nor M3 communicates with M5, which belongs to a different component.

# Relationships
## Builds Upon
- **Node/arrow system** — the structure on which communication is defined
## Enables
- **Connected system** — defined as a system where all nodes communicate
## Related
- **Equivalence relation** — communication is an equivalence relation
## Contrasts With
- **Arrow chain** — arrow chains follow arrows forwards only; communication paths allow backwards traversal

# Common Errors
- **Error**: Requiring arrows to be traversed forwards only
  **Correction**: Communication allows backwards traversal; only arrow chains require forward direction

# Common Confusions
- **Confusion**: Thinking non-communication means no direct arrow
  **Clarification**: Non-communication means complete separation; two nodes can lack a direct arrow yet still communicate via intermediate nodes

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.1.2, pp. 225-226. See Figure 9.1.

# Verification Notes
- Definition source: direct from Definition 9.1.2
- Confidence rationale: explicit formal definition in source
- Re-extracted from v2 card; preserved: example from Figure 9.1, equivalence relation properties
