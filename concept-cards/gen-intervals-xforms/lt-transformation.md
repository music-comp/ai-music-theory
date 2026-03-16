---
# === CORE IDENTIFICATION ===
concept: LT Transformation
slug: lt-transformation

# === CLASSIFICATION ===
category: transformation-theory
subcategory: klang-operations
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.1.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "leading-tone exchange"
  - "Leittonwechsel"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - klang-representation
extends: []
related:
  - par-transformation
  - rel-transformation
  - slide-transformation
contrasts_with:
  - slide-transformation

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do I apply LT to Klangs?"
  - "What is the leading-tone exchange?"
---

# Quick Definition
Riemann's "leading-tone exchange" as a Klang transformation: (C, +)LT = (E, -) and (E, -)LT = (C, +). LT exchanges a major triad with the minor triad whose root is a major third above, sharing two common tones. LT is an involution.

# Core Definition
"We can define Riemann's 'leading-tone exchange' as an operation LT: (C, +)LT = (E, -); (E, -)LT = (C, +)" (Lewin, 8.1.1, p. 177). The name derives from the fact that C major {C, E, G} and E minor {E, G, B} share two common tones (E and G), differing only in the leading-tone relationship C vs. B.

# Prerequisites
- **Klang representation** — LT operates on Klangs

# Key Properties
1. (p, +)LT = (p + 4, -) mod 12
2. (p, -)LT = (p + 8, +) mod 12
3. LT^2 = identity (involution)
4. Two common tones between input and output triads
5. The differing tones are related by semitone (leading-tone motion)
6. LT is the "L" in the PLR group of Neo-Riemannian theory

# Construction / Recognition
## To Construct:
1. For major Klang (p, +): move root up a major third, change to minor
2. For minor Klang (p, -): move root down a major third, change to major
## To Recognize:
1. Two triads sharing two common tones with a semitone voice-leading difference
2. Mode change accompanying root motion by major third

# Context & Application
LT captures parsimonious voice leading between triads: only one voice moves, and by semitone. This property makes LT central to Neo-Riemannian theory (though Lewin's work predates that terminology). Together with PAR and REL, LT forms the PLR group that generates all 24 triadic transformations.

# Examples
**Example 1** (p. 177): (C, +)LT = (E, -): C major {C, E, G} to E minor {E, G, B}; common tones {E, G}. (F, +)LT = (A, -): F major {F, A, C} to A minor {A, C, E}; common tones {A, C}.

# Relationships
## Builds Upon
- **Klang representation** — LT is defined on Klangs
## Related
- **PAR transformation** — Fellow involutory Klang operation in PLR group
- **REL transformation** — Fellow member of PLR group
## Contrasts With
- **SLIDE transformation** — Both change mode, but LT preserves two common tones (root and fifth of one = third and fifth of other), while SLIDE preserves only the third

# Common Errors
- **Error**: Confusing LT with MED or any power of MED
  **Correction**: LT is not expressible as a power of MED

# Common Confusions
- **Confusion**: Thinking "leading tone" refers to scale degree 7 specifically
  **Clarification**: The name refers to the voice-leading character (semitone motion), not to a specific scale degree

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.1.1, page 177.

# Verification Notes
- Definition source: Direct quotation from 8.1.1
- Confidence rationale: Explicitly defined
- Re-extraction notes: Re-extracted from v2 card; preserved: common-tone analysis, PLR group reference
