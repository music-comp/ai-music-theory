---
# === CORE IDENTIFICATION ===
concept: Intervallic Augmentation Homomorphism
slug: intervallic-augmentation-homomorphism

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
section: "9.5.3"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "complementary gesture times 2"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - graph-homomorphism
extends:
  - graph-homomorphism
related:
  - brahms-horn-trio-analysis
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does intervallic augmentation work as a graph homomorphism?"
  - "What kind of homomorphism has isomorphic NODEMAP but non-isomorphic SGMAP?"
---

# Quick Definition
A graph homomorphism where NODEMAP is an isomorphism (identity on nodes) but SGMAP multiplies all intervals by a constant factor, transforming one intervallic structure into an augmented version while preserving the node/arrow system.

# Core Definition
An intervallic augmentation homomorphism from graph G to graph G' has: NODEMAP = identity on NODES (an isomorphism of node/arrow systems); SGMAP(i) = ki for some constant k (a semigroup homomorphism that is not 1-to-1). If interval i labels an arrow in G, then ki labels the corresponding arrow in G'. This satisfies Definition 9.5.2 because SGMAP preserves the group operation: SGMAP(i + j) = k(i + j) = ki + kj = SGMAP(i) + SGMAP(j) (Lewin, Section 9.5.3, pp. 235-236).

# Prerequisites
- **Graph homomorphism** — this is a specific type

# Key Properties
1. NODEMAP is an isomorphism (same nodes, same arrows)
2. SGMAP(i) = ki is a group homomorphism but not an isomorphism (not 1-to-1 in general)
3. SGMAP(0) = SGMAP(6) = 0 when k = 2, mod 12 -- so it collapses elements
4. The homomorphism is not "onto" unless the codomain is restricted to even intervals
5. Demonstrates that NODEMAP and SGMAP can have very different properties

# Construction / Recognition
## To Construct:
1. Take a transformation graph G
2. Keep the same node/arrow system
3. Multiply all TRANSIT labels by k to get G'
4. SGMAP(i) = ki; NODEMAP = identity
## To Recognize:
1. Same node/arrow system in both graphs
2. All interval labels in the second graph are k times those in the first

# Context & Application
In the Brahms Horn Trio analysis (Section 7.3), the "complementary gesture" has intervals 10, 10, 8 and the "complementary gesture times 2" has intervals 8, 8, 4 (all mod 12). This doubling is formalized as a graph homomorphism. The formalization shows it is a proper homomorphism (not an isomorphism), capturing the precise nature of the intervallic relationship.

# Examples
**Example 1** (Section 9.5.3, Figure 9.6, pp. 235-236): Brahms Horn Trio.
- Graph (a): intervals 10, 10, 8
- Graph (b): intervals 8, 8, 4
- SGMAP(10) = 20 = 8 (mod 12); SGMAP(8) = 16 = 4 (mod 12)
- Making it "onto" requires redefining SGPb as the group of even intervals

# Relationships
## Builds Upon
- **Graph homomorphism** — this is a specific application
## Related
- **Brahms Horn Trio analysis** — the musical context

# Common Errors
- **Error**: Assuming the augmentation creates an isomorphism
  **Correction**: SGMAP(i) = 2i is not 1-to-1 (e.g., SGMAP(0) = SGMAP(6) = 0), so it is a proper homomorphism

# Common Confusions
- **Confusion**: Confusing intervallic augmentation with rhythmic augmentation
  **Clarification**: "Times 2" applies to intervals here, not durations (though both may be augmented in the music)

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.5.3, pp. 235-236. See Figure 9.6.

# Verification Notes
- Definition source: direct from Section 9.5.3
- Confidence rationale: high -- explicitly worked example
- Re-extracted from v2 card; preserved: Brahms interval computations, non-isomorphism proof, "onto" redefinition
