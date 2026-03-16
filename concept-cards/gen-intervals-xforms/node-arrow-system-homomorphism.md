---
concept: Node/Arrow System Homomorphism
slug: node-arrow-system-homomorphism

category: transformation-theory
subcategory: graph-network-mappings
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.5.1"

extraction_confidence: high

aliases: []

prerequisites:
  - node-arrow-system
extends: []
related:
  - node-arrow-system-isomorphism
  - graph-homomorphism
contrasts_with:
  - node-arrow-system-isomorphism

answers_questions:
  - "What is a homomorphism of node/arrow systems?"
  - "How does a homomorphism differ from an isomorphism of node/arrow systems?"
---

# Quick Definition
A mapping between node/arrow systems that preserves arrows in one direction (if nodes are in the arrow relation, their images are too) but need not reflect them (the image system may have additional arrows).

# Core Definition
A mapping NODEMAP of NODES into NODES' is a homomorphism of (NODES, ARROW) into (NODES', ARROW') if (NODEMAP(N1), NODEMAP(N2)) is in the ARROW' relation whenever (N1, N2) is in ARROW. NODEMAP is "onto" if whenever N'1 and N'2 are in ARROW', there exist N1 and N2 in ARROW with N'1 = NODEMAP(N1) and N'2 = NODEMAP(N2). NODEMAP is "1-to-1 as a homomorphism" if it is injective (Lewin, Definition 9.5.1, pp. 233-234).

# Prerequisites
- **Node/arrow system** — the structures being mapped

# Key Properties
1. Preservation only: arrows in the source map to arrows in the target
2. The converse need NOT hold: the target may "have more arrows"
3. A 1-to-1 onto homomorphism = isomorphism (by Definition 9.4.1)
4. "Onto" has a special definition: every arrow in the target must be the image of some arrow in the source
5. A 1-to-1 surjective NODEMAP that is NOT "onto" in the special arrow sense is NOT an isomorphism

# Construction / Recognition
## To Construct:
1. Define a map NODEMAP: NODES -> NODES'
2. Verify that for all (N1, N2) in ARROW, (NODEMAP(N1), NODEMAP(N2)) is in ARROW'
## To Recognize:
1. Check arrow preservation
2. If also checking for "onto": verify the arrow-surjectivity condition

# Context & Application
Homomorphisms model many-to-one relationships between network structures: collapsing parallel voices into a single melodic line, abstracting away structural details, or showing one network as a simplification of another. They are weaker than isomorphisms, allowing more flexible structural comparisons.

# Examples
**Example 1** (Section 9.5.5, p. 236): The Scholica Enchiriadis analysis. NODEMAP collapses the first Principalis node and the first Organalis node of graph (c) both into the first node of graph (b), and so on. This is a homomorphism from (c) onto (b).

**Example 2** (Section 9.5.1, p. 234): It is possible for NODEMAP to be 1-to-1 as a set map and surjective, without being an isomorphism of systems, because the target may "have more arrows." Taking NODES' = NODES and adding arrows to ARROW gives ARROW'; the identity NODEMAP is then a homomorphism but not an isomorphism.

# Relationships
## Builds Upon
- **Node/arrow system** — the structures being mapped
## Enables
- **Graph homomorphism** — combines NODEMAP with SGMAP
## Contrasts With
- **Node/arrow system isomorphism** — isomorphism requires both preservation AND reflection of arrows

# Common Errors
- **Error**: Assuming a bijective homomorphism is an isomorphism
  **Correction**: The "onto" condition for homomorphisms has a special arrow-level meaning; bijective NODEMAP + arrow preservation is NOT sufficient

# Common Confusions
- **Confusion**: Thinking "homomorphism onto" just means surjective NODEMAP
  **Clarification**: It requires that every ARROW' pair comes from some ARROW pair; surjectivity of NODEMAP as a set map is a weaker condition

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.5.1, pp. 233-234.

# Verification Notes
- Definition source: direct from Definition 9.5.1
- Confidence rationale: explicit formal definition with careful distinctions
- Re-extracted from v2 card; preserved: special "onto" definition, Scholica Enchiriadis example
