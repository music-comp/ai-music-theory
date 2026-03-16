---
concept: Precedence-Ordered System
slug: precedence-ordered-system

category: transformation-theory
subcategory: graph-network-structure
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.7.3"

extraction_confidence: high

aliases:
  - "precedence-ordered node/arrow system"

prerequisites:
  - precedence-ordering
extends: []
related:
  - partial-ordering
  - formal-melody
  - carriage-return-function
contrasts_with: []

answers_questions:
  - "What is a precedence-ordered system?"
  - "When is a node/arrow system compatible with chronology?"
---

# Quick Definition
A node/arrow system is precedence-ordered if no node both precedes and follows another, meaning PRECEDENCE forms a strict partial ordering with no cycles through one-way arrows.

# Core Definition
A node/arrow system is precedence-ordered if there is no pair of nodes (N, N') such that N both precedes and follows N'. Equivalently, PRECEDENCE has no cycles (Lewin, Definition 9.7.3, p. 242).

**Theorem 9.7.4**: In a precedence-ordered system, PRECEDENCE is a strict partial ordering satisfying (PO1) antisymmetry and (PO2) transitivity (p. 242).

**Section 9.7.6 Summary**: When a finite system is precedence-ordered, its J nodes can be labeled 1 through J so that when j < k, it is possible for N_j to precede N_k but impossible for N_k to precede N_j. All one-way arrows can be drawn left to right.

# Prerequisites
- **Precedence ordering** — the relation whose acyclicity is required

# Key Properties
1. No pair of nodes (N, N') has N preceding N' AND N' preceding N
2. PRECEDENCE is a strict partial ordering (Theorem 9.7.4)
3. Finite systems can be linearized: nodes labeled so one-way arrows go "left to right" (Section 9.7.6)
4. Multiple linear orderings may be compatible with one partial ordering
5. The system is "potentially compatible with naive chronology" (Section 9.7.5)

# Construction / Recognition
## To Construct:
1. Build a node/arrow system ensuring no cycles of one-way arrows
## To Recognize:
1. Check that no node both precedes and follows another
2. Equivalently, verify no cycle of proper arrow chains exists

# Context & Application
Precedence-ordered systems model situations where the arrow structure has a consistent direction. In musical analysis, they enable left-to-right layout where all one-way arrows point forward. When the listening chronology violates this layout, Lewin identifies "carriage return" moments -- points where the listener shoots back from right to left, violating precedence direction.

# Examples
**Example 1** (Figure 9.13, p. 243): A precedence-ordered system with two left-hand nodes and two right-hand nodes. Each left-hand node precedes each right-hand node. Neither left-hand node precedes the other; neither right-hand node precedes the other. Multiple linear chronologies are compatible.

**Example 2** (Figure 9.14, pp. 244-247): The Beethoven Appassionata network is precedence-ordered, but the musical chronology violates precedence. The Gb input nodes formally precede the Db nodes, but Db is heard first. This discrepancy produces "carriage return" moments.

# Relationships
## Builds Upon
- **Precedence ordering** — the relation that must be acyclic
## Enables
- **Formal melody** — requires a linearly ordered precedence-ordered system
- **Carriage return function** — identified at moments where chronology violates precedence
## Related
- **Partial ordering** — PRECEDENCE is a strict partial ordering in these systems

# Common Errors
- **Error**: Assuming precedence-ordered implies linearly ordered
  **Correction**: Precedence ordering produces a partial order; some nodes may be incomparable

# Common Confusions
- **Confusion**: Thinking precedence-ordered means events occur in precedence order
  **Clarification**: "Potentially compatible" means a compatible chronology EXISTS; the actual music need not follow it

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.7.3, Theorem 9.7.4, Sections 9.7.5-9.7.6, pp. 242-244.

# Verification Notes
- Definition source: direct from Definition 9.7.3 and Theorem 9.7.4
- Confidence rationale: explicit definition with theorem and discussion
- Re-extracted from v2 card; preserved: linearization discussion, Figure 9.13 example, compatible chronology clarification
