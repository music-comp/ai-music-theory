---
# === CORE IDENTIFICATION ===
concept: IFUNC as Convolution
slug: convolution-interpretation

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: interval-functions
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: null

# === CONFIDENCE ===
extraction_confidence: medium

# === VARIANTS ===
aliases:
  - IFUNC convolution
  - characteristic function convolution

# === TYPED RELATIONSHIPS ===
prerequisites:
  - ifunc
  - generalized-interval-system
extends:
  - ifunc
related:
  - z-relation-generalized
  - ifunc-probability
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does IFUNC relate to convolution of characteristic functions?"
  - "What is the mathematical connection between set theory and harmonic analysis?"
---

# Quick Definition
In an advanced mathematical setting, IFUNC(X, Y)(i) can be interpreted as the convolution of the characteristic functions of sets X and Y on a locally compact group, connecting generalized set theory to harmonic analysis.

# Core Definition
Lewin notes (p. 135, marked as optional for graduate-level readers): Using LABEL to identify S with IVLS, we treat S = IVLS as a locally compact group under the discrete topology, with "sets" as compact subsets. If f and g are the characteristic functions of X and Y respectively, then IFUNC(X, Y)(i) = (f * g)(i), where * denotes convolution. Questions about IFUNC can thus be reformulated as questions about convolutions of characteristic functions of compact subsets in locally compact groups.

# Prerequisites
- **IFUNC** — The function being reinterpreted
- **Generalized Interval System** — Provides the group structure for convolution

# Key Properties
1. Characteristic function of X: f(s) = 1 if s in X, 0 otherwise
2. Convolution: (f * g)(i) = sum over s of f(s) * g(s + i) = IFUNC(X, Y)(i)
3. Self-convolution f * f corresponds to IFUNC(X, X), the "autocorrelation" of X
4. For continuous groups, summation is replaced by integration
5. Z-related sets are those with identical self-convolutions but not related by canonical operations

# Construction / Recognition
## To Compute:
1. Define characteristic functions f, g for sets X, Y
2. Compute the convolution (f * g)(i) = sum_{s in X} g(s + i)
3. The result equals IFUNC(X, Y)(i)

## To Recognize:
1. Any IFUNC computation can be viewed as a convolution

# Context & Application
This interpretation connects Lewin's music-theoretic constructions to harmonic analysis on groups, opening the possibility of applying Fourier analysis and representation theory to study IFUNC systematically. The study is "much simplified when the group is commutative" (Lewin). This mathematical perspective illuminates why certain questions about IFUNC (e.g., Z-relations) are deep and difficult.

# Examples
**Example 1** (derived from p. 135): In Z_12 (pitch classes), X = {0, 4, 7} (C major triad) with characteristic function f, and Y = {0, 3, 7} (C minor triad) with characteristic function g. IFUNC(X, Y) = f * g, computing how many ways each interval can be spanned from X to Y. IFUNC(X, X) = f * f is the autocorrelation, corresponding to Forte's interval vector.

# Relationships
## Builds Upon
- **IFUNC** — Convolution is an alternative characterization of IFUNC

## Enables
- **Z-Relation Generalized** — Z-sets share identical self-convolutions

## Related
- **IFUNC Probability** — Both provide alternative mathematical lenses on IFUNC

# Common Errors
- **Error**: Assuming convolution theory is needed to use IFUNC
  **Correction**: The convolution interpretation is optional and marked for advanced readers; IFUNC can be understood and applied without it

# Common Confusions
- **Confusion**: Thinking convolution adds information beyond what IFUNC already provides
  **Clarification**: Convolution is a different formulation of the same information, but it connects IFUNC to a rich mathematical literature with powerful analytical tools

# Source Reference
Chapter 5: Generalized Set Theory (1), paragraph following Theorem 5.1.8, p. 135 (marked as optional for graduate-level readers).

# Verification Notes
- Definition source: Synthesized from optional paragraph directed at mathematically advanced readers
- Confidence rationale: Medium — concept is presented briefly as an aside, not developed in detail
- Re-extraction notes: Re-extracted from v2 card; preserved: Z_12 example, autocorrelation interpretation, emphasis on optional nature. Added v3.1 structure.
