---
# === CORE IDENTIFICATION ===
concept: Carriage Return Function
slug: carriage-return-function

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
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "carriage-return moment"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - precedence-ordered-system
extends: []
related:
  - input-node
  - beethoven-appassionata-analysis
  - start-node
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a carriage-return moment in a transformation network?"
  - "What happens when musical chronology violates precedence ordering?"
---

# Quick Definition
A carriage-return moment occurs when the listening chronology of a musical passage violates the precedence ordering of its network, marking the precise moments when the listener must "shoot back from right to left" on a precedence-compatible graph layout.

# Core Definition
In a precedence-ordered system laid out so one-way arrows go left to right, carriage-return moments are precisely those moments in the listening chronology at which that chronology violates precedence-ordering. At all other moments, listening chronology is compatible with precedence-ordering. The expression implicitly supposes that the node/arrow system is precedence-ordered (Lewin, Section 9.7.6, pp. 246-247).

# Prerequisites
- **Precedence-ordered system** — carriage returns require precedence-ordered systems

# Key Properties
1. Precisely defined: moments where chronology violates precedence
2. At all other moments, chronology is compatible with precedence
3. The concept requires both a precedence-ordered graph AND a musical chronology
4. Carriage returns are analytically informative, not flaws
5. Lewin notes this concept is hard to express precisely in any other theoretical vocabulary

# Construction / Recognition
## To Construct:
1. Lay out the precedence-ordered graph with one-way arrows left to right
2. Trace the musical chronology through the nodes
3. Identify moments where chronology moves from right to left
## To Recognize:
1. Find nodes where the musical event occurs after an event at a node further right in the precedence layout

# Context & Application
Carriage-return moments often correspond to structural "returns" or "resets" in the music. In the Beethoven Appassionata analysis, the Gb harmonies function as carriage returns, interacting effectively with the phrasing. The concept pinpoints where the temporal and structural orderings diverge, giving precise vocabulary for a phenomenon otherwise difficult to articulate.

# Examples
**Example 1** (Figure 9.14, pp. 244-247): Beethoven Appassionata slow movement. The Gb nodes are formal inputs in the precedence-compatible layout (b). But in the music, Db is heard first. The moments when Gb is heard are carriage-return moments -- the listener "shoots back from right to left," and this interacts with the phrasing of the passage. The second Gb carriage-return is felt especially strongly.

# Relationships
## Builds Upon
- **Precedence-ordered system** — carriage returns occur within such systems
## Enables
- (Analytical insight into temporal vs. structural ordering)
## Related
- **Input node** — formal inputs that produce carriage returns when not heard first
- **START node** — a device to establish structural priority over formal input nodes
- **Beethoven Appassionata analysis** — primary example of carriage returns

# Common Errors
- **Error**: Treating carriage returns as defects in the analysis
  **Correction**: Carriage returns are analytically informative features, not problems

# Common Confusions
- **Confusion**: Thinking carriage returns mean arrows point backward
  **Clarification**: The arrows are correctly oriented; it is the musical chronology that moves against the arrow direction
- **Confusion**: Thinking carriage returns apply to all networks
  **Clarification**: They require precedence-ordered systems; non-precedence-ordered systems lack the necessary framework

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.6, pp. 244-247. See Figure 9.14.

# Verification Notes
- Definition source: synthesized from Section 9.7.6 discussion
- Confidence rationale: high -- concept explicitly named and defined in context
- Re-extracted from v2 card; preserved: Appassionata example, "shoot back from right to left" quote
