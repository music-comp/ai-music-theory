---
# === CORE IDENTIFICATION ===
concept: START Node
slug: start-node

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
section: "9.7.6"

# === CONFIDENCE ===
extraction_confidence: medium

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - input-node
  - precedence-ordered-system
extends: []
related:
  - carriage-return-function
  - beethoven-appassionata-analysis
contrasts_with:
  - input-node

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How can structural priority be established when formal input nodes don't match musical intuition?"
---

# Quick Definition
A START node is a formal device -- a node containing "START" adjoined to a network with an arrow to a designated structurally prior node -- declared by convention to supersede all other input nodes, establishing structural priority independently of the graph's intrinsic arrow structure.

# Core Definition
The START node is adjoined to the node/arrow system along with an arrow from START to a designated node. A formal convention declares it supersedes all other input nodes in function. When starting at the START node, reaching certain other input nodes requires traversing arrows backwards, formally capturing their subordinate status. The analytic criterion for placing the START arrow may be diachronic (the music starts there) or synchronic (that node begins a higher-level tonic function) (Lewin, Section 9.7.6, pp. 247-248).

# Prerequisites
- **Input node** — START supersedes intrinsic input nodes
- **Precedence-ordered system** — the context in which START addresses structural priority

# Key Properties
1. START is formally adjoined to the existing system (new node + new arrow)
2. Declared by convention to supersede other input nodes
3. Reaching other input nodes from START may require traversing arrows backwards
4. The analytic criterion for START placement may be diachronic or synchronic
5. START itself has no musical contents (or contains only the marker "START")

# Construction / Recognition
## To Construct:
1. Identify the node that should have structural priority
2. Adjoin a new START node to NODES
3. Add an arrow from START to the designated node
4. Declare START supersedes all other input nodes
## To Recognize:
1. Look for an adjoined node labeled START with an arrow to a specific node

# Context & Application
START nodes address situations where formal input nodes don't match analytical intuitions about structural priority. In the Beethoven Appassionata, the Gb nodes are formal inputs but Db has tonal priority. A START node pointing to Db captures this priority. Nodes reachable from START only by backward arrow-traversal are thereby formally subordinated.

# Examples
**Example 1** (Figure 9.14(b), pp. 247-248): Beethoven Appassionata slow movement. The Gb nodes are formal inputs, but Db has structural priority as tonic. A box labeled "START" with an arrow to the Db node is adjoined. From START, reaching Gb requires traversing arrows backwards, formally capturing Gb's subordinate status despite its input position.

# Relationships
## Builds Upon
- **Input node** — START addresses limitations of intrinsic input nodes
## Related
- **Carriage return function** — both address discrepancies between graph structure and musical priority
- **Beethoven Appassionata analysis** — primary example
## Contrasts With
- **Input node** — intrinsic graph property vs. externally adjoined convention

# Common Errors
- **Error**: Treating START as musically significant content
  **Correction**: START is a formal analytical device, not a musical event

# Common Confusions
- **Confusion**: Thinking START changes the intrinsic graph structure
  **Clarification**: It is adjoined to the system; the original graph's properties remain
- **Confusion**: Thinking there is only one valid START placement
  **Clarification**: The criterion may be diachronic or synchronic, potentially yielding different placements

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.6, pp. 247-248. See Figure 9.14(b).

# Verification Notes
- Definition source: synthesized from Section 9.7.6 discussion (not a numbered definition)
- Confidence rationale: medium -- concept is described in context but not given a formal numbered definition
- Re-extracted from v2 card; preserved: diachronic/synchronic criteria, Appassionata example
