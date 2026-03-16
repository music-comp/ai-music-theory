---
# === CORE IDENTIFICATION ===
concept: Non-Commutative GIS
slug: non-commutative-gis

# === CLASSIFICATION ===
category: timbral-temporal-systems
subcategory: rhythmic-structures
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models"
chapter_number: 4
pdf_page: 91
section: "4.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "non-abelian GIS"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - generalized-interval-system
  - commutative-vs-noncommutative-gis
  - time-span
extends:
  - generalized-interval-system
related:
  - time-span-gis
  - time-span-interval-group
contrasts_with:
  - commutative-time-span-gis

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is a non-commutative GIS?"
  - "What must I understand before non-commutative GIS structures?"
  - "How do commutative and non-commutative GIS structures differ?"
---

# Quick Definition
A non-commutative GIS is a Generalized Interval System whose interval group IVLS is non-abelian, meaning the order of interval composition matters. Chapter 4 provides the time-span GIS as the primary musically significant non-commutative example.

# Core Definition
A GIS (S, IVLS, int) is non-commutative when there exist intervals i, j in IVLS such that ij differs from ji. Chapter 4 opens by explaining why the careful separation of commutative and non-commutative cases in Chapter 3 was necessary: the time-span GIS provides a musically significant non-commutative specimen (Lewin, p. 91). In such a GIS, transpositions do not preserve intervals, inversions are not self-inverse, and no interval-reversing transformations exist.

# Prerequisites
- **Generalized Interval System** — The GIS framework
- **Commutative vs. Non-Commutative GIS** — Understanding the distinction
- **Time Span** — The elements of the primary non-commutative example

# Key Properties
1. IVLS is non-abelian: there exist i, j with ij != ji
2. Transposition T_i does not preserve intervals (unless i is central)
3. Interval-preserving P_i differs from T_i
4. I_u^v is not necessarily equal to I_v^u
5. No interval-reversing transformations exist
6. Every T commutes with every P (Theorem 3.4.10 still holds)

# Construction / Recognition
## To Recognize:
1. Find two intervals i, j such that ij != ji
2. Equivalently: find a transposition that does not preserve intervals

# Context & Application
The time-span GIS (4.1.3) is the primary non-commutative example, modeling music with multiple local tempi. Chapter 4 was written to justify the non-commutative theory developed abstractly in Chapter 3, showing that non-commutative GIS are not merely theoretical curiosities but arise naturally from rhythmic analysis of Carter, Stockhausen, Nancarrow, and Ligeti.

# Examples
**Example 1** (p. 91): Time-span interval group:
- (1, 2)(0, 3) = (1, 6) but (0, 3)(1, 2) = (3, 6)

**Example 2**: Non-commutative octatonic GIS structures (Appendix B) provide additional examples using dihedral groups.

# Relationships
## Builds Upon
- **Generalized Interval System** — non-commutative GIS is a GIS with non-abelian IVLS

## Enables
- **Time-Span GIS** — the principal non-commutative example
- Analysis of multi-tempo music

## Contrasts With
- **Commutative Time-Span GIS** — the simpler alternative that assumes a fixed time-unit

# Common Errors
- **Error**: Assuming all GIS results from pitch-class theory generalize
  **Correction**: Many familiar results (IT = T^{-1}I, inversions self-inverse) require commutativity

# Common Confusions
- **Confusion**: Thinking non-commutativity means intervals are ambiguous
  **Clarification**: Non-commutative GIS have perfectly well-defined intervals. It is only the composition of intervals that is order-dependent.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, pp. 91-114.

# Verification Notes
- Definition source: synthesized from Chapter 4 introduction and Notes 4.1.7
- Confidence rationale: high — central concept of the chapter with explicit examples
- Re-extraction notes: Re-extracted from v2 card; old card focused on Appendix B octatonic examples; this version focuses on Chapter 4's time-span GIS as the primary example per the task scope
