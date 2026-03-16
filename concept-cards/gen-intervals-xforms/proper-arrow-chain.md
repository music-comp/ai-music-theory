---
# === CORE IDENTIFICATION ===
concept: Proper Arrow Chain
slug: proper-arrow-chain

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
section: "9.7.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - arrow-chain
extends:
  - arrow-chain
related:
  - precedence-ordering
  - precedence-ordered-system
contrasts_with:
  - arrow-chain

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What makes an arrow chain 'proper'?"
  - "How do proper arrow chains define precedence?"
---

# Quick Definition
A proper arrow chain is an arrow chain containing at least one "one-way arrow" -- an arrow (N_{j-1}, N_j) whose reverse (N_j, N_{j-1}) is NOT in the ARROW relation -- making the chain impossible to walk completely backwards.

# Core Definition
An arrow chain N0, N1, ..., NJ is proper if there is at least one j between 1 and J such that (N_j, N_{j-1}) is not in the ARROW relation. Intuitively, the chain contains at least one genuinely directed step and "cannot be walked backwards" (Lewin, Definition 9.7.1, p. 241).

# Prerequisites
- **Arrow chain** — a proper arrow chain is a special type of arrow chain

# Key Properties
1. Requires at least one one-way (irreversible) arrow along the chain
2. The trivial chain (J = 0) from N to N is never proper
3. A chain where all arrows are two-way is not proper
4. Proper arrow chains define the precedence relation (9.7.2)
5. The reflexive arrow (N, N) is always two-way, so it never makes a chain proper

# Construction / Recognition
## To Construct:
1. Form an arrow chain from N to N'
2. Include at least one step where the reverse arrow does not exist
## To Recognize:
1. For each step (N_{j-1}, N_j) in the chain, check if (N_j, N_{j-1}) is also in ARROW
2. If at least one step has no reverse, the chain is proper

# Context & Application
Proper arrow chains capture genuine directionality in a node/arrow system. They distinguish truly directed transformational paths from paths that could equally well run in reverse. This distinction is the basis for the precedence relation and the formal notion of "before/after" in networks.

# Examples
**Example 1** (Figure 9.12, p. 241): M1 -> M2 -> M3. If the arrow from M2 to M3 is one-way (M3 -> M2 not in ARROW), then this is a proper arrow chain from M1 to M3, establishing that M1 precedes M3.

**Example 2** (Figure 9.12): M1 -> M2. If both directions are in ARROW, this chain is NOT proper, and M1 does not precede M2 despite being in the arrow relation.

# Relationships
## Builds Upon
- **Arrow chain** — a proper arrow chain is an arrow chain with additional constraint
## Enables
- **Precedence ordering** — defined via proper arrow chains
- **Precedence-ordered system** — systems where precedence forms a partial ordering
## Contrasts With
- **Arrow chain** — general arrow chains may lack one-way arrows

# Common Errors
- **Error**: Thinking any arrow chain establishes precedence
  **Correction**: Only proper arrow chains (with at least one one-way arrow) establish precedence

# Common Confusions
- **Confusion**: Thinking "proper" means "correct" or "well-formed"
  **Clarification**: "Proper" here means "containing at least one one-way arrow"

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.7.1, p. 241. See Figure 9.12.

# Verification Notes
- Definition source: direct from Definition 9.7.1
- Confidence rationale: explicit formal definition
- Re-extracted from v2 card; preserved: Figure 9.12 examples, distinction from general arrow chains
