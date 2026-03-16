---
concept: Network of Networks
slug: network-of-networks

category: transformation-theory
subcategory: graph-network-extensions
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.5.5"

extraction_confidence: medium

aliases: []

prerequisites:
  - transformation-network-definition
  - operation
  - commutativity
extends:
  - transformation-network-definition
related:
  - product-networks
  - scholica-enchiriadis-analysis
contrasts_with:
  - product-networks

answers_questions:
  - "What is a network of networks?"
  - "How can networks be nested hierarchically?"
---

# Quick Definition
A transformation network where each node contains not a simple musical object but an entire sub-network, allowing hierarchical or layered analytical structures with outer transformations operating on complete inner networks.

# Core Definition
A network-of-networks has an outer network structure whose CONTENTS at each node is itself a transformation network. The outer transformations operate on entire inner networks. Two varieties are illustrated in Section 9.5.5: type (f) where the outer graph is the melody and each node contains a diatesseron interval-network, and type (g) where the outer graph is the diatesseron and each node contains the full melody-network (Lewin, Section 9.5.5, pp. 238-239).

Well-formedness requires: the outer transformations must be operations (invertible), the inner transformations must commute with the outer transformations.

# Prerequisites
- **Transformation network** — both outer and inner structures are networks
- **Operation** — outer transformations must be operations
- **Commutativity** — inner and outer transformations must commute

# Key Properties
1. Each node of the outer network contains an entire inner network
2. Outer transformations transpose or otherwise transform entire sub-networks
3. Inner/outer commutativity is required for well-formedness
4. Two perspectives (f) and (g) model the same musical situation differently
5. Formally distinct from product networks, though related

# Construction / Recognition
## To Construct:
1. Define an outer network (nodes, arrows, semigroup, TRANSIT)
2. At each outer node, place a complete inner network as CONTENTS
3. Ensure outer transformations commute with inner transformations
## To Recognize:
1. Look for networks whose node contents are themselves networks
2. Verify the commutativity requirement

# Context & Application
Networks-of-networks model analytical situations with multiple structural levels: chords that are themselves patterned, voices that each carry melodic content, etc. The Scholica Enchiriadis example models parallel organum: (f) represents "singing the melody, singing diatessera as we go," while (g) represents "I am singing the same melody as Principalis, in diatesseron relation."

# Examples
**Example 1** (Figure 9.8, pp. 238-239): Scholica Enchiriadis "Nos qui vivimus."
- Network (f): outer graph = melody (b), inner graph = diatesseron (e). Models: "We are singing (the graph of) 'Nos qui vivimus,' singing diatessera as we go."
- Network (g): outer graph = diatesseron (e), inner graph = melody (b). Models Organalis thinking: "Principalis is singing the melody; I am singing the same melody at the diatesseron."

# Relationships
## Builds Upon
- **Transformation network** — both levels are networks
## Related
- **Product networks** — related but formally distinct; product networks use Cartesian products of nodes
- **Scholica Enchiriadis analysis** — primary example
## Contrasts With
- **Product networks** — product networks are not hierarchically nested

# Common Errors
- **Error**: Omitting the commutativity requirement
  **Correction**: Well-formedness requires inner and outer transformations to commute

# Common Confusions
- **Confusion**: Thinking (f) and (g) are the same structure
  **Clarification**: They model different perspectives on the same music and are formally distinct

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.5.5, pp. 238-239. See Figure 9.8(f) and (g).

# Verification Notes
- Definition source: synthesized from Section 9.5.5 discussion
- Confidence rationale: medium -- concept described through examples, not given a numbered definition
- Re-extracted from v2 card; preserved: Scholica Enchiriadis examples, commutativity requirement, (f) vs (g) distinction
