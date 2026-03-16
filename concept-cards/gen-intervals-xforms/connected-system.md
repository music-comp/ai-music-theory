---
# === CORE IDENTIFICATION ===
concept: Connected System
slug: connected-system

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
section: "9.1.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "connected node/arrow system"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - node-arrow-system
  - communication-between-nodes
extends: []
related:
  - operation-network
  - equivalence-class
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a connected node/arrow system?"
  - "How does connectivity relate to operation networks?"
---

# Quick Definition
A node/arrow system is connected if any two nodes communicate, meaning there is always a path (forwards or backwards along arrows) between any pair of nodes.

# Core Definition
A node/arrow system (NODES, ARROW) is connected if for any two nodes N and N' in NODES, N communicates with N'. Equivalently, the communication equivalence relation has only one equivalence class (Lewin, Definition 9.1.3, p. 226).

# Prerequisites
- **Node/arrow system** — the structure being classified
- **Communication between nodes** — the relation that defines connectivity

# Key Properties
1. All nodes lie in a single equivalence class under communication
2. Any disconnected system decomposes into connected subsystems (components)
3. Each component is (NODES_i, ARROW_i) where NODES_i is an equivalence class and ARROW_i restricts ARROW to pairs within NODES_i
4. Connectivity is essential for Theorem 9.3.3 (unique determination of operation network contents)

# Construction / Recognition
## To Construct:
1. Ensure every pair of nodes can be linked by a path of forwards-or-backwards arrows
## To Recognize:
1. Pick any node; follow all possible arrow paths (forward and backward)
2. If all nodes are reachable, the system is connected
3. If some nodes are unreachable, the system is disconnected

# Context & Application
Connected networks represent unified analytical structures where any musical object can be related to any other through transformations. The key consequence is Theorem 9.3.3: in a connected operation network, specifying the contents of any single node uniquely determines the contents of all others.

# Examples
**Example 1** (Figure 9.1, p. 225): This system is NOT connected; it decomposes into two components, each internally connected, with no communication between them.

**Example 2**: The CADENCE graph (from Section 7.4) is connected: all four Klang nodes communicate through the arrow structure.

# Relationships
## Builds Upon
- **Communication between nodes** — connectivity = single equivalence class under communication
## Enables
- **Operation network** — Theorem 9.3.3 requires connectivity for unique content determination
## Related
- **Equivalence class** — connected components are equivalence classes under communication

# Common Errors
- **Error**: Assuming connectivity means every pair has a direct arrow
  **Correction**: Connectivity only requires the existence of some path (possibly through intermediaries)

# Common Confusions
- **Confusion**: Thinking disconnected systems are malformed
  **Clarification**: Disconnected systems are perfectly valid; they represent multiple independent analytical structures (e.g., separate voice analyses)

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.1.3, p. 226.

# Verification Notes
- Definition source: direct from Definition 9.1.3
- Confidence rationale: explicit definition with clear consequences
- Re-extracted from v2 card; preserved: CADENCE graph example, clarification about disconnected systems
