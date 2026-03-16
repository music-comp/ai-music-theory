---
# === CORE IDENTIFICATION ===
concept: TRANSIT Function
slug: transit-function

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
section: "9.2.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "TRANSIT"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - node-arrow-system
  - semigroup
extends: []
related:
  - transformation-graph-definition
  - contents-function
  - idempotent-transit-value
contrasts_with:
  - contents-function

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the TRANSIT function in a transformation graph?"
  - "How are arrows labeled in a transformation graph?"
---

# Quick Definition
The TRANSIT function maps each arrow in a transformation graph to a member of the semigroup SGP, labeling arrows with the specific transformations they represent.

# Core Definition
In a transformation graph (NODES, ARROW, SGP, TRANSIT), TRANSIT is a function from ARROW into SGP. For each pair (N1, N2) in ARROW, TRANSIT(N1, N2) is a member of SGP representing the transformation associated with that arrow. TRANSIT must satisfy the consistency criterion 9.2.1(D): products of TRANSIT values along all arrow chains between the same endpoints must be equal (Lewin, Definition 9.2.1(C), p. 226).

# Prerequisites
- **Node/arrow system** — TRANSIT's domain is the ARROW relation
- **Semigroup** — TRANSIT's codomain is SGP

# Key Properties
1. Domain: ARROW (all ordered pairs in the arrow relation)
2. Codomain: SGP (the semigroup of transformations)
3. TRANSIT(N, N) must be idempotent in SGP (proved in 9.2.2)
4. When SGP is a group, TRANSIT(N, N) = identity for all N
5. In a network, TRANSIT relates to CONTENTS: if f = TRANSIT(N1, N2), then f(CONTENTS(N1)) = CONTENTS(N2)
6. Products along chains follow left orthography: rightmost applies first

# Construction / Recognition
## To Construct:
1. For each arrow (N1, N2) in ARROW, assign a member of SGP
2. Verify consistency: all arrow-chain products between common endpoints agree
## To Recognize:
1. Identify the labeling of arrows with semigroup elements
2. Check the consistency criterion on all path pairs

# Context & Application
TRANSIT provides the "labeling" that makes arrows meaningful. When analyzing music, TRANSIT encodes which transformation (transposition, inversion, mode change, RICH, etc.) corresponds to moving from one musical object to another. The TRANSIT function is what distinguishes a transformation graph from a mere node/arrow system.

# Examples
**Example 1** (Figure 9.3, p. 228): f = TRANSIT(N1, N2); s1 = CONTENTS(N1), s2 = CONTENTS(N2). The network requirement ensures f(s1) = s2.

**Example 2** (Figure 9.4, pp. 228-229): TRANSIT assigns operations A, B, C, D to various arrows. Consistency requires B = CD (products along different paths from N2 to N3 must match).

**Example 3** (Figure 9.5, p. 231): TRANSIT(N1, N1) = TRANSIT(N2, N2) = E (identity); TRANSIT(N1, N2) = TRANSIT(N2, N1) = I (inversion about A).

# Relationships
## Builds Upon
- **Semigroup** — TRANSIT takes values in SGP
## Enables
- **Transformation graph** — TRANSIT is a defining component
- **Transformation network** — TRANSIT constrains valid CONTENTS assignments
## Related
- **Idempotent TRANSIT value** — TRANSIT(N, N) must be idempotent
## Contrasts With
- **CONTENTS function** — TRANSIT labels arrows; CONTENTS fills nodes

# Common Errors
- **Error**: Assigning TRANSIT values to arrows without checking consistency
  **Correction**: Always verify criterion 9.2.1(D) for all pairs of arrow chains

# Common Confusions
- **Confusion**: Thinking TRANSIT assigns values to nodes
  **Clarification**: TRANSIT assigns values to arrows (ordered pairs of nodes), not to individual nodes
- **Confusion**: Assuming TRANSIT(N, N) is always the identity
  **Clarification**: TRANSIT(N, N) must be idempotent; it equals the identity only when SGP is a group

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.2.1(C), pp. 226-228.

# Verification Notes
- Definition source: direct from Definition 9.2.1(C)
- Confidence rationale: explicit component of formal definition
- Re-extracted from v2 card; preserved: Figure 9.3 example, left orthography note
