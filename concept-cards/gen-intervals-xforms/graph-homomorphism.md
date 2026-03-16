---
# === CORE IDENTIFICATION ===
concept: Graph Homomorphism
slug: graph-homomorphism

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
section: "9.5.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases: []

# === TYPED RELATIONSHIPS ===
prerequisites:
  - node-arrow-system-homomorphism
  - homomorphism
extends: []
related:
  - graph-isomorphism
  - intervallic-augmentation-homomorphism
  - scholica-enchiriadis-analysis
contrasts_with:
  - graph-isomorphism

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a homomorphism of transformation graphs?"
  - "How do graph homomorphisms generalize graph isomorphisms?"
---

# Quick Definition
A structure-preserving map between transformation graphs consisting of a pair (NODEMAP, SGMAP) that need not be bijective, generalizing isomorphism to allow many-to-one node maps and non-injective semigroup maps.

# Core Definition
A homomorphism of graph (NODES, ARROW, SGP, TRANSIT) into/onto graph (NODES', ARROW', SGP', TRANSIT') is a pair (NODEMAP, SGMAP) having: (A) NODEMAP is a homomorphism of node/arrow systems; (B) SGMAP is a homomorphism of semigroups SGP into/onto SGP'; (C) for every (N1, N2) in ARROW, TRANSIT'(NODEMAP(N1), NODEMAP(N2)) = SGMAP(TRANSIT(N1, N2)). The homomorphism is 1-to-1 if both NODEMAP and SGMAP are injective. A 1-to-1 homomorphism onto = isomorphism (Lewin, Definition 9.5.2, pp. 234-235).

# Prerequisites
- **Node/arrow system homomorphism** — provides the NODEMAP component
- **Homomorphism** (of semigroups) — provides the SGMAP component

# Key Properties
1. NODEMAP may collapse multiple nodes (not 1-to-1)
2. SGMAP may collapse multiple transformations (not 1-to-1)
3. Criterion (C) ensures TRANSIT labels correspond under the maps
4. "Onto" requires both NODEMAP and SGMAP to be onto in their respective senses
5. "1-to-1" requires both to be injective
6. Examples 9.5.3-9.5.5 show great variety: NODEMAP can be iso while SGMAP is not, or vice versa

# Construction / Recognition
## To Construct:
1. Define NODEMAP between node/arrow systems (preserving arrows)
2. Define SGMAP between semigroups (preserving the operation)
3. Verify criterion (C): TRANSIT labels correspond
## To Recognize:
1. Identify the two graph components (NODEMAP, SGMAP)
2. Verify arrow preservation and semigroup homomorphism
3. Check TRANSIT compatibility

# Context & Application
Graph homomorphisms model diverse relationships: intervallic augmentation (doubling all intervals), voice collapsing (parallel voices into one line), and structural abstraction (complex structure mapped to simpler underlying pattern). The three examples in the source (9.5.3-9.5.5) show strikingly different forms of homomorphism.

# Examples
**Example 1** (Section 9.5.3, p. 235): Brahms Horn Trio "complementary gesture times 2." NODEMAP = identity (isomorphism of systems); SGMAP(i) = 2i (not 1-to-1, maps interval to its double). This is intervallic augmentation as graph homomorphism.

**Example 2** (Section 9.5.4, pp. 236-237): Tritone GIS example. NODEMAP is onto but not 1-to-1 (two top nodes map to one); SGMAP(i) = i-or-(i+6) (onto but not 1-to-1).

**Example 3** (Section 9.5.5, pp. 237-239): Scholica Enchiriadis "Nos qui vivimus." NODEMAP collapses Principalis and Organalis nodes (not 1-to-1); SGMAP = identity (isomorphism). Graph (b) is a homomorphic image of (c), but NOT of (d) — no SGMAP can satisfy both SGMAP(1) = 1 and SGMAP(3) = 0.

# Relationships
## Builds Upon
- **Node/arrow system homomorphism** — provides structural component
## Enables
- **Intervallic augmentation homomorphism** — a specific graph homomorphism application
## Related
- **Scholica Enchiriadis analysis** — Example 9.5.5 demonstrates voice-collapsing homomorphism
## Contrasts With
- **Graph isomorphism** — isomorphism requires bijective NODEMAP and SGMAP

# Common Errors
- **Error**: Assuming any SGMAP works for a given NODEMAP
  **Correction**: SGMAP must be a semigroup homomorphism AND satisfy the TRANSIT compatibility criterion

# Common Confusions
- **Confusion**: Thinking there is only one type of graph homomorphism
  **Clarification**: Examples 9.5.3-9.5.5 show three qualitatively different types: iso NODEMAP with proper SGMAP, proper NODEMAP with proper SGMAP, and proper NODEMAP with iso SGMAP

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Definition 9.5.2, pp. 234-235. Examples 9.5.3-9.5.5, pp. 235-239.

# Verification Notes
- Definition source: direct from Definition 9.5.2
- Confidence rationale: explicit definition with three worked examples
- Re-extracted from v2 card; preserved: all three example summaries, variety of homomorphism types
