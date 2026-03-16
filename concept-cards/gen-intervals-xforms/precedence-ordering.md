---
# === CORE IDENTIFICATION ===
concept: Precedence Ordering
slug: precedence-ordering

# === CLASSIFICATION ===
category: transformation-theory
subcategory: graph-network-structure
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.7.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "PRECEDENCE relation"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - proper-arrow-chain
extends: []
related:
  - precedence-ordered-system
  - partial-ordering
  - input-node
  - output-node
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is the precedence relation in a node/arrow system?"
  - "How does precedence differ from being in the arrow relation?"
---

# Quick Definition
In a node/arrow system, node N precedes node N' if there exists a proper arrow chain from N to N', capturing a formal "before/after" relationship that is stricter than merely being in the arrow relation and need not match musical chronology.

# Core Definition
In a node/arrow system, node N precedes node N', and N' follows N, if there exists some proper arrow chain from N to N'. One must carefully distinguish "N precedes N'" from "N is in the ARROW relation to N'" (Lewin, Definition 9.7.2, p. 241).

# Prerequisites
- **Proper arrow chain** — precedence is defined via proper arrow chains

# Key Properties
1. Precedence requires a proper arrow chain (with at least one one-way arrow)
2. Being in the ARROW relation does NOT imply precedence (if both directions exist)
3. Precedence does NOT imply being in the ARROW relation (may require intermediate nodes)
4. In a precedence-ordered system, PRECEDENCE is a strict partial ordering (Theorem 9.7.4)
5. Precedence is a formal property; it may differ from musical chronology

# Construction / Recognition
## To Construct:
1. Find a proper arrow chain from N to N'
2. If one exists, N precedes N'
## To Recognize:
1. Check for proper arrow chains (chains with at least one one-way arrow) from N to N'
2. Note: N being in ARROW to N' does not suffice; the chain must be proper

# Context & Application
Precedence captures the inherent directionality in a network's arrow structure. When precedence-ordered, a system is "potentially compatible with naive chronology" -- nodes can be arranged so that precedence agrees with temporal order. However, actual music may violate this arrangement, leading to "carriage return" moments.

# Examples
**Example 1** (Figure 9.12, p. 241): M1 precedes M3 (via M1->M2->M3 with one-way arrow M2->M3). But M1 does NOT precede M2 (all arrows between them are two-way). And M1 is in the ARROW relation to M2 but does not precede M2.

**Example 2** (Figure 9.13, p. 243): Two left-hand nodes each precede each right-hand node. Neither left-hand node precedes the other; neither right-hand node precedes the other. Multiple linear chronologies are compatible.

# Relationships
## Builds Upon
- **Proper arrow chain** — precedence is defined by the existence of proper chains
## Enables
- **Precedence-ordered system** — a system where precedence has no cycles
## Related
- **Partial ordering** — in precedence-ordered systems, PRECEDENCE is a strict partial ordering
- **Input node** — input nodes precede all nodes they communicate with
- **Output node** — output nodes follow all nodes they communicate with

# Common Errors
- **Error**: Equating "in the arrow relation" with "precedes"
  **Correction**: Precedence requires proper (one-way) arrows; two-way arrow relations do not establish precedence

# Common Confusions
- **Confusion**: Thinking precedence must match musical chronology
  **Clarification**: Precedence is formal; the music may present objects in an order that violates precedence ordering

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.7.2, p. 241. See Figures 9.12-9.13.

# Verification Notes
- Definition source: direct from Definition 9.7.2
- Confidence rationale: explicit definition with careful distinction from ARROW relation
- Re-extracted from v2 card; preserved: Figure 9.12 examples showing ARROW vs precedence distinction
