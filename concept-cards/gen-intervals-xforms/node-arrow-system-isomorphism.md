---
# === CORE IDENTIFICATION ===
concept: Node/Arrow System Isomorphism
slug: node-arrow-system-isomorphism

# === CLASSIFICATION ===
category: transformation-theory
subcategory: graph-network-mappings
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (3): Formalities"
chapter_number: 9
pdf_page: 224
section: "9.4.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "NODEMAP isomorphism"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - node-arrow-system
extends: []
related:
  - graph-isomorphism
  - node-arrow-system-homomorphism
  - isography
contrasts_with:
  - node-arrow-system-homomorphism

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "When are two node/arrow systems isomorphic?"
  - "What is a NODEMAP?"
---

# Quick Definition
Two node/arrow systems are isomorphic if there exists a bijection (NODEMAP) between their node sets that preserves the arrow relation in both directions: (N1, N2) is in ARROW if and only if (NODEMAP(N1), NODEMAP(N2)) is in ARROW'.

# Core Definition
Two node/arrow systems (NODES, ARROW) and (NODES', ARROW') are isomorphic if there exists a 1-to-1 map NODEMAP of NODES onto NODES' such that for every pair (N1, N2) of NODES, (N1, N2) is in the ARROW relation if and only if (NODEMAP(N1), NODEMAP(N2)) is in the ARROW' relation. Such a NODEMAP is called an isomorphism of the two systems (Lewin, Definition 9.4.1, p. 231).

# Prerequisites
- **Node/arrow system** — the structures being compared

# Key Properties
1. NODEMAP must be bijective (1-to-1 and onto)
2. Arrow preservation: (N1, N2) in ARROW implies (NODEMAP(N1), NODEMAP(N2)) in ARROW'
3. Arrow reflection: (NODEMAP(N1), NODEMAP(N2)) in ARROW' implies (N1, N2) in ARROW
4. Both directions are required; preservation alone gives only a homomorphism
5. Isomorphic systems have identical combinatorial structure

# Construction / Recognition
## To Construct:
1. Find a bijection between NODES and NODES'
2. Verify arrow preservation in both directions
## To Recognize:
1. Check that a bijection exists between node sets
2. Check that arrows correspond exactly under the bijection

# Context & Application
Node/arrow system isomorphism is the foundation for comparing transformation graphs (Definition 9.4.2) and for defining isography of networks (Definition 9.4.3). If two systems are isomorphic, they have identical structure regardless of what the nodes are named or what objects they contain.

# Examples
**Example 1** (Section 9.4.4, p. 231): The IPAIR system with two nodes where every pair is in ARROW. Any two such systems (regardless of node names) are isomorphic under any bijection between their two-element node sets.

# Relationships
## Builds Upon
- **Node/arrow system** — the structures being compared
## Enables
- **Graph isomorphism** — combines NODEMAP with SGMAP
- **Isography** — defined via graph isomorphism, which uses node/arrow isomorphism
## Contrasts With
- **Node/arrow system homomorphism** — homomorphism preserves arrows in one direction only; isomorphism requires both

# Common Errors
- **Error**: Checking arrow preservation in only one direction
  **Correction**: Both preservation AND reflection of arrows are required for isomorphism

# Common Confusions
- **Confusion**: Thinking a bijective homomorphism is automatically an isomorphism
  **Clarification**: A bijective NODEMAP that preserves arrows but does not reflect them is NOT an isomorphism; the target system may "have more arrows"

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.4.1, p. 231.

# Verification Notes
- Definition source: direct from Definition 9.4.1
- Confidence rationale: explicit formal definition
- Re-extracted from v2 card; preserved: bidirectionality emphasis, IPAIR example
