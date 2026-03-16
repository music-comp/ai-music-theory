---
# === CORE IDENTIFICATION ===
concept: Operation Network
slug: operation-network

# === CLASSIFICATION ===
category: transformation-theory
subcategory: graph-network-definitions
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.3.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - transformation-network-definition
  - operation-graph
  - group
extends:
  - transformation-network-definition
related:
  - connected-system
  - contents-function
contrasts_with:
  - transformation-network-definition

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do operation networks differ from transformation networks?"
  - "What is an operation network?"
  - "Why does knowing one node's contents determine all others?"
---

# Quick Definition
An operation network is a transformation network whose SGP is a group of operations on S, enabling unique determination of all node contents from any single node in a connected system.

# Core Definition
An operation network is a transformation network (S, NODES, ARROW, SGP, TRANSIT, CONTENTS) where SGP is a group of operations on S (Lewin, Definition 9.3.2, p. 229).

**Theorem 9.3.3**: Let S be a family of objects, GP a group of operations on S, and (NODES, ARROW, GP, TRANSIT) a connected operation graph. Let N0 be any node and s0 any member of S. Then there exists a unique operation network having S for its objects and the given graph, such that s0 = CONTENTS(N0) (p. 229).

# Prerequisites
- **Transformation network** — an operation network is a specialization
- **Operation graph** — the underlying graph must have group SGP
- **Group** — the algebraic structure ensuring invertibility

# Key Properties
1. SGP is a group of operations (1-to-1, onto transformations) on S
2. In a connected operation network, CONTENTS of any one node determines all others (Theorem 9.3.3)
3. Contents are computed by following arrows forward (applying TRANSIT) or backward (applying inverses)
4. Path-independence of contents follows from criterion 9.2.1(D) of the underlying graph
5. This is the standard analytical tool: most musical transformations form groups

# Construction / Recognition
## To Construct:
1. Choose a connected operation graph (NODES, ARROW, GP, TRANSIT)
2. Choose any node N0 and assign it contents s0 in S
3. For each other node N, follow any path from N0 to N
4. Apply TRANSIT operations (or inverses for backward arrows) along the path
5. The result is uniquely CONTENTS(N)
## To Recognize:
1. Verify SGP is a group of operations on S
2. Verify CONTENTS satisfies f(CONTENTS(N1)) = CONTENTS(N2) for all arrows

# Context & Application
Operation networks are the standard analytical tool in transformational theory. The unique determination theorem means an analyst can "seed" one node with a musical object and derive all other contents, or propose a graph and check whether a musical passage fits it. Most Klang, pitch-class, and row-form networks in Chapters 7-10 are operation networks.

# Examples
**Example 1** (Figure 9.4, pp. 228-229): Given s0 at N0 with operations A, B, C, D: CONTENTS(N1) = A(s0) (forward along A-arrow); CONTENTS(N2) = B^{-1}(A(s0)) (backward along B-arrow); CONTENTS(N3) = D(B^{-1}(A(s0))) (forward along D-arrow). Consistency: also C^{-1}(A(s0)), verified by B = CD.

**Example 2** (Figure 9.5, p. 231): Networks (a)-(e) are all operation networks on the same two-node IPAIR graph with group {E, I}. Each is determined by a single node's content.

# Relationships
## Builds Upon
- **Transformation network** — operation network is a specialization
- **Operation graph** — the underlying graph type
## Enables
- **Isography** — comparing operation networks on isomorphic graphs
## Related
- **Connected system** — unique determination requires connectivity
- **CONTENTS function** — determined by one node's value in connected systems
## Contrasts With
- **Transformation network** — general case allows non-invertible transformations

# Common Errors
- **Error**: Attempting unique determination on a disconnected graph
  **Correction**: Theorem 9.3.3 requires connectivity; disconnected components are independently determined

# Common Confusions
- **Confusion**: Thinking unique determination works for transformation networks in general
  **Clarification**: It requires operation networks (group SGP), not just transformation networks (semigroup SGP)

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.3.2 and Theorem 9.3.3, pp. 229-230.

# Verification Notes
- Definition source: direct from Definition 9.3.2 and Theorem 9.3.3
- Confidence rationale: explicit definition and theorem with proof sketch
- Re-extracted from v2 card; preserved: Figure 9.4 worked example, unique determination emphasis
