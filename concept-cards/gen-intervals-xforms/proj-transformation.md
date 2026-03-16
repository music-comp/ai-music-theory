---
# === CORE IDENTIFICATION ===
concept: PROJ Transformation
slug: proj-transformation

# === CLASSIFICATION ===
category: transformation-theory
subcategory: graph-network-extensions
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
  - "PROJ+"
  - "PROJ-"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - schenkerian-network
extends: []
related:
  - beethoven-appassionata-analysis
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What are the PROJ transformations in a Schenkerian network?"
  - "How do structural levels connect in Lewin's network formalism?"
---

# Quick Definition
PROJ+ and PROJ- are transformations that move between structural levels in a Schenkerian network: PROJ+ increments the level (toward foreground), PROJ- decrements it (toward background), both preserving Klang and degree.

# Core Definition
PROJ+ operates on triples (Kng, deg, lev) by transforming to (Kng, deg, lev+1), projecting content one level closer to the foreground. PROJ- is its inverse, transforming to (Kng, deg, lev-1). Both are context-free operations, well-defined even when the target level has no analytic pertinence, ensuring they are proper invertible operations (Lewin, Section 9.7.6, pp. 249-250).

# Prerequisites
- **Schenkerian network** — PROJ operates within this network type

# Key Properties
1. PROJ+ and PROJ- are inverses of each other
2. Both preserve Klang and degree; only level changes
3. Context-free: always increment/decrement by exactly 1
4. Typically drawn as two-way arrows for conciseness
5. Connect corresponding events across Schenkerian levels

# Construction / Recognition
## To Construct:
1. Identify events at adjacent structural levels sharing Klang and degree
2. Connect with PROJ+/PROJ- arrows
## To Recognize:
1. Look for arrows between levels that preserve Klang and degree

# Context & Application
PROJ arrows connect different structural levels in Schenkerian networks, allowing input/output terminology to be qualified by level. This refinement enables statements like "input at level 3" rather than just "input."

# Examples
**Example 1** (Figure 9.16, pp. 249-250): PROJ+ connects (Db, 5, 1) at level 1 to (Db, 5, 2) at level 2. Within-level transformations use pairs like (DOM, SUST) for "dominant, degree sustains."

# Relationships
## Builds Upon
- **Schenkerian network** — the framework for PROJ operations
## Related
- **Beethoven Appassionata analysis** — primary demonstration

# Common Errors
- **Error**: Thinking PROJ changes Klang or degree
  **Correction**: PROJ only changes level; Klang and degree are preserved

# Common Confusions
- **Confusion**: Thinking PROJ arrows are directional constraints
  **Clarification**: They are typically two-way (PROJ+ and PROJ- are inverses)

# Source Reference
Chapter 9: Transformation Graphs and Networks (3): Formalities, Section 9.7.6, pp. 249-250. See Figure 9.16.

# Verification Notes
- Definition source: direct from Section 9.7.6
- Confidence rationale: high -- explicitly defined with examples
- Re-extracted from v2 card; preserved: context-free property, within-level transformation notation
