---
# === CORE IDENTIFICATION ===
concept: Left vs. Right Group Operations
slug: left-vs-right-group-operations

# === CLASSIFICATION ===
category: timbral-temporal-systems
subcategory: time-span-gis
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models"
chapter_number: 4
pdf_page: 112
section: "4.1 (Notes 4.1.7)"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "left-multiplication vs. right-multiplication"
  - "left action vs. right action"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - transposition-operation
  - interval-preserving-operation
  - label-function
extends: []
related:
  - interval-function-computation
  - central-interval
  - transposition-and-interval-preservation
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "Why do transpositions and interval-preserving operations differ in non-commutative GIS?"
  - "What is the algebraic distinction between left and right multiplication in a GIS?"
  - "How do T and P operations act differently on time spans?"
---

# Quick Definition
In a non-commutative group, left multiplication (g * h) and right multiplication (h * g) generally differ. Interval-preserving operations use left multiplication of labels (LABEL(P_i(s)) = i * LABEL(s)), while transpositions use right multiplication (LABEL(T_i(s)) = LABEL(s) * i), explaining why T_i and P_i differ in non-commutative GIS.

# Core Definition
In any GIS, the interval-preserving operation P_i is defined by left-multiplying labels: LABEL(P_i(s)) = i * LABEL(s) (Theorem 3.4.4). The transposition T_i is defined by right-multiplying labels: LABEL(T_i(s)) = LABEL(s) * i (Theorem 3.4.3). In a commutative group these coincide; in a non-commutative group they generally do not. Notes 4.1.7 (pp. 112-114) explores the consequences for the time-span GIS.

# Prerequisites
- **Transposition operation (T_i)** — Defined via right multiplication
- **Interval-preserving operation (P_i)** — Defined via left multiplication
- **LABEL function** — The coordinate system in which left/right multiplication acts

# Key Properties
1. P_i: LABEL(P_i(s)) = i * LABEL(s) (left multiply)
2. T_i: LABEL(T_i(s)) = LABEL(s) * i (right multiply)
3. In commutative GIS: P_i = T_i for all i
4. In non-commutative GIS: P_i = T_i only when i is central
5. P preserves intervals uniformly; T may distort intervals

# Construction / Recognition
## To Construct:
1. For P_{(h,u)} on time span (a, x): compute (h, u) * (a, x) = (h + ua, ux)
2. For T_{(i,p)} on time span (a, x): compute (a, x) * (i, p) = (a + xi, xp)
3. Compare the results to see the difference
## To Recognize:
1. P scales everything uniformly then shifts (preserves proportions)
2. T shifts each span by a multiple of its own duration (context-dependent)

# Context & Application
The left/right distinction explains why P operations preserve intervals while T operations generally do not in non-commutative GIS. In the time-span GIS: P_{(h,u)} uniformly rescales all durations by u and shifts all beginnings by h + ua (a uniform transformation); T_{(i,p)} shifts each span's beginning by xi (proportional to its own duration x), creating a context-dependent transformation that distorts intervallic relationships.

# Examples
**Example 1** (Notes 4.1.7, pp. 112-114): Time-span calculations:
- T_{(0,3)}(1, 2) = (1, 2) * (0, 3) = (1 + 2*0, 2*3) = (1, 6)
- P_{(0,3)}(1, 2) = (0, 3) * (1, 2) = (0 + 3*1, 3*2) = (3, 6)
- Different results! T and P are genuinely distinct operations.

**Example 2**: Another computation:
- (1, 2) * (0, 3) = (1, 6) [right multiply -- T]
- (0, 3) * (1, 2) = (3, 6) [left multiply -- P]
The difference: T_{(0,3)} triples the duration but keeps the starting point; P_{(0,3)} triples both duration and starting-point displacement.

# Relationships
## Builds Upon
- **Transposition operation** — Right-multiplication action
- **Interval-preserving operation** — Left-multiplication action
- **LABEL function** — The labels on which left/right multiplication acts
## Enables
- **Understanding non-commutative GIS** — The left/right distinction is the algebraic source of non-commutativity's consequences
## Related
- **Central interval** — Central intervals are precisely those where left and right multiplication agree
- **Transposition and interval preservation** — T preserves intervals iff T = P, iff the interval is central

# Common Errors
- **Error**: Assuming T_{(i,p)} and P_{(i,p)} are the same operation in the time-span GIS
  **Correction**: They coincide only for the identity interval (0, 1); otherwise they differ

# Common Confusions
- **Confusion**: "Left" and "right" refer to spatial position rather than algebraic multiplication order
  **Clarification**: They refer to whether the interval multiplies the label from the left (P) or right (T) in the group operation
- **Confusion**: The distinction is an artifact of notation
  **Clarification**: It reflects a genuine algebraic and musical difference: P transforms uniformly while T transforms context-dependently

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models, Notes 4.1.7, pages 112-114.

# Verification Notes
- Definition source: Direct from Theorems 3.4.3-3.4.4 applied in Notes 4.1.7
- Confidence rationale: High -- explicitly discussed with computational examples
- Re-extraction notes: Re-extracted from v2 card; preserved: time-span computation examples, uniform vs. context-dependent interpretation, central interval connection
