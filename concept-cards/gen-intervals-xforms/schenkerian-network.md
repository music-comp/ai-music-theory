---
concept: Schenkerian Network
slug: schenkerian-network

category: transformation-theory
subcategory: graph-network-extensions
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.7.6"

extraction_confidence: medium

aliases: []

prerequisites:
  - transformation-network-definition
  - proj-transformation
extends:
  - transformation-network-definition
related:
  - beethoven-appassionata-analysis
  - input-node
  - output-node
contrasts_with: []

answers_questions:
  - "How can Schenkerian analysis be incorporated into transformation networks?"
  - "What is a Schenkerian network?"
---

# Quick Definition
A transformation network incorporating Schenkerian structural levels by having nodes contain ordered triples (Klang, degree, level), with PROJ operations moving between levels and within-level transformations combining Klang and degree changes.

# Core Definition
In a Schenkerian network, node contents are ordered triples (Klang, degree, level). Klang is a harmonic entity, degree is a scale degree in the structural voice, and level indicates structural depth (1 = deepest background, higher = closer to foreground). PROJ+ increments the level (projects toward foreground); PROJ- decrements it (projects toward background). Within each level, transformations are specified by pairs (Klangtrans, degtrans) (Lewin, Section 9.7.6, pp. 249-250).

# Prerequisites
- **Transformation network** — the base formalism
- **PROJ transformation** — level-changing operations

# Key Properties
1. Nodes contain (Klang, degree, level) triples
2. PROJ+ and PROJ- are context-free operations (always increment/decrement by 1)
3. Within-level transformations combine Klang and degree changes: e.g., (DOM, SUST) or (SUBD, N+)
4. Input/output status can be qualified by level: "input at level 3"
5. Not equivalent to a Schenkerian reading, which would attend more to voice leading and bass

# Construction / Recognition
## To Construct:
1. Identify structural levels (1 = background, higher = foreground)
2. At each level, assign (Klang, degree, level) to nodes
3. Connect across levels with PROJ arrows
4. Connect within levels with (Klangtrans, degtrans) arrows
## To Recognize:
1. Look for networks with level-differentiated contents and PROJ transformations

# Context & Application
Schenkerian networks integrate hierarchical tonal analysis with transformational methodology. They allow distinguishing "input at level 3" from "input at level 2," refining the blunt input/output analysis. Lewin suggests actual Schenkerian graphs "could be represented in network formats of the sort under present consideration, when suitably extended" (p. 250), though he acknowledges the representation is not complete.

# Examples
**Example 1** (Figure 9.16, pp. 249-250): Beethoven Appassionata slow movement. Level 1: (Db, 5, 1). Level 2: Db expands to (Db, 5, 2) -- (Ab, 5, 2) -- (Db, 5, 2) with (DOM, SUST) and (SUBD, N+) arrows. Level 3: further foreground elaboration with Gb Klangs. The Gb nodes are "input at level 3" but not at lower levels.

# Relationships
## Builds Upon
- **Transformation network** — the base formalism
- **PROJ transformation** — enables level connections
## Related
- **Input/output nodes** — refined by level qualification
- **Beethoven Appassionata analysis** — primary example

# Common Errors
- **Error**: Equating Schenkerian networks with Schenkerian graphs
  **Correction**: They are different formalisms; Schenkerian networks are Lewin's adaptation, not Schenker's notation

# Common Confusions
- **Confusion**: Thinking this captures all of Schenkerian analysis
  **Clarification**: Lewin notes the representation "fails to model the middleground progression" and voice-leading details; it captures harmonic hierarchy but not full Schenkerian content

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.6, pp. 249-250. See Figure 9.16.

# Verification Notes
- Definition source: synthesized from Section 9.7.6 discussion
- Confidence rationale: medium -- described in context as an exploratory model, not formally defined
- Re-extracted from v2 card; preserved: Figure 9.16 example, (DOM, SUST) notation, level qualification of input/output
