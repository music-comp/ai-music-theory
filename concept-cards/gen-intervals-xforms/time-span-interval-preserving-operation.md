---
# === CORE IDENTIFICATION ===
concept: Time-Span Interval-Preserving Operation
slug: time-span-interval-preserving-operation

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
  - "P_{(h,u)}"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - time-span-gis
  - interval-preserving-operation
extends:
  - interval-preserving-operation
related:
  - time-span-transposition
contrasts_with:
  - time-span-transposition

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How do interval-preserving operations work in the time-span GIS?"
---

# Quick Definition
The time-span interval-preserving operation P_{(h,u)}(a, x) = (h + ua, ux) first scales a time span by u, then shifts it by h units. Unlike transpositions, these operations always preserve all intervallic relationships.

# Core Definition
Notes 4.1.7(D): P_{(h,u)}(a, x) = (h, u)(a, x) = (h + ua, ux). This is left-multiplication in IVLS. The operation scales all durations by u (tempo change) and shifts all events by h (position change). Interval preservation is verified: int(P(a,x), P(b,y)) = ((u(b-a))/(ux), uy/(ux)) = ((b-a)/x, y/x) = int((a,x), (b,y)). These are NOT transpositions in this non-commutative GIS (Note 4.1.7F) (Lewin, Notes 4.1.7(D), (F), p. 113).

# Prerequisites
- **Time-Span GIS** — The non-commutative GIS context
- **Interval-Preserving Operation** — The general theory from Chapter 3

# Key Properties
1. P_{(h,u)}(a, x) = (h + ua, ux) (left-multiplication)
2. Always preserves intervals: int(P(s), P(t)) = int(s, t)
3. Not a transposition (except for identity P_{(0,1)} = T_{(0,1)})
4. Physical interpretation: play at tempo u starting at time h

# Construction / Recognition
## To Construct:
1. Choose interval (h, u): h = time shift, u = tempo scaling factor
2. For time span (a, x): compute (h + ua, ux)

# Context & Application
P_{(h,u)} models uniform temporal transformation: changing the tempo by factor u and the start time by h. This is like "play the piece twice as fast and start 10 seconds later" — the internal rhythmic structure is preserved.

# Examples
**Example 1** (p. 113): P_{(5,2)}(3, 1) = (5 + 2*3, 2*1) = (11, 2)

**Example 2**: Contrast with transposition:
- T_{(2,3)}(3, 1) = (3 + 2*1, 3*1) = (5, 3)
- P_{(2,3)}(3, 1) = (2 + 3*3, 3*1) = (11, 3) — different result

# Relationships
## Builds Upon
- **Interval-Preserving Operation** — general P_i theory
- **Time-Span GIS** — the specific context

## Contrasts With
- **Time-Span Transposition** — T right-multiplies labels and does NOT preserve intervals; P left-multiplies and DOES

# Common Errors
- **Error**: Confusing P_{(h,u)} with T_{(h,u)}
  **Correction**: P uses left-multiplication (h + ua, ux); T uses right-multiplication (a + hx, ux). Same second component, different first.

# Common Confusions
- **Confusion**: Thinking interval-preserving operations are "more natural" than transpositions
  **Clarification**: Both are valid operations with different musical interpretations. Transpositions measure from the original span; interval-preserving operations apply uniform scaling.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Notes 4.1.7(D), p. 113.

# Verification Notes
- Definition source: direct from Notes 4.1.7(D)
- Confidence rationale: high — explicit formula with verification
- Re-extraction notes: Re-extracted from v2 card; preserved: interval preservation proof, contrast with transposition, physical interpretation
