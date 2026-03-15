---
# === CORE IDENTIFICATION ===
concept: Scalar Inversion
slug: scalar-inversion

# === CLASSIFICATION ===
category: scales-modes
subcategory: operations
tier: intermediate-advanced

# === PROVENANCE ===
source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "Scales"
chapter_number: 4
pdf_page: 138
section: "4.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "diatonic inversion"
  - "inversion within a scale"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - scalar-transposition
  - scale-degree-arithmetic
extends: []
related:
  - scale-as-ruler
contrasts_with:
  - chromatic-inversion

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is scalar inversion?"
  - "How does it differ from chromatic inversion?"
---

# Quick Definition
Scalar inversion reflects a pattern around a fixed point within a scale, sending each note x steps above the fixed point to x steps below it. It preserves scalar distances while potentially altering chromatic distances.

# Core Definition
Scalar inversion is the scale-based analogue of chromatic inversion. To invert relative to a scale, choose a fixed point; any note x scale steps above the fixed point gets sent to the note x steps below, and vice versa. Algebraically, scalar inversion is subtraction from a constant value (twice the fixed-point scale degree). In C major, inverting (C, D, F) around E (scale degree 3) means subtracting each degree from 6 (=2x3): degrees 1, 2, 4 become 5, 4, 2, yielding (G, F, D). Like scalar transposition, scalar inversion can act on out-of-scale notes via fractional scale degrees. Two chords related by scalar transposition or scalar inversion belong to the same scalar "set class."

# Prerequisites
- Scalar transposition
- Scale degree arithmetic

# Key Properties
1. Reflects around a fixed point using scalar (not chromatic) distances
2. Algebraically: subtract each scale degree from 2 times the fixed point's degree
3. Preserves scalar intervals; may change chromatic intervals
4. Can act on out-of-scale notes via fractional scale degrees
5. Scalar inversion around a point midway between two scale degrees sends scale tones to scale tones

# Construction / Recognition
## To Apply:
1. Choose a fixed point (scale degree x)
2. For each note with scale degree d, compute the inverted degree: 2x - d
3. Use scale degree arithmetic (wrap at scale size)
4. Convert back to pitch names

# Context & Application
Scalar inversion is essential for defining scalar set classes and for understanding the symmetry properties of scales. Together with scalar transposition, it generates the scalar equivalence classes that generalize the OPTIC symmetries to scalar contexts.

# Examples
**Example 1** (p. 138): In C major, inverting (C, D, F) around E: degrees (1, 2, 4) become (5, 4, 2) = (G, F, D) (Figure 4.2.2b).
**Example 2** (p. 138): {C, D, F} and {D, F, G} belong to the same scalar set class in C harmonic minor because they are related by scalar inversion around Eb.

# Relationships
## Builds Upon
- **scalar-transposition** — The other scalar distance-preserving operation
- **scale-degree-arithmetic** — The mathematical framework
## Related
- **scale-as-ruler** — Scalar inversion uses the ruler's metric
## Contrasts With
- Chromatic inversion — which uses semitone distances

# Common Errors
- **Error**: Confusing the fixed point of scalar inversion with a tonic
  **Correction**: The fixed point is arbitrary and has no tonal significance

# Common Confusions
- **Confusion**: Does scalar inversion always produce scale tones from scale tones?
  **Clarification**: Only when the fixed point is a scale degree or exactly halfway between two scale degrees

# Source Reference
Chapter 4: Scales, Section 4.2, pages 138-139.

# Verification Notes
- Definition source: Directly from Section 4.2
- Confidence rationale: High — formally defined with algebraic notation
- Cross-reference status: Verified against the examples and footnote 8
