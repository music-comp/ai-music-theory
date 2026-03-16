---
concept: SHADOW Function for Time Spans
slug: shadow-function

category: generalized-set-theory
subcategory: injection-function
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
section: "6.10"

extraction_confidence: high

aliases:
  - "SHADOW(b, y)"

prerequisites:
  - inj-measure-spaces
  - time-span-gis
extends: []
related:
  - inj-function
contrasts_with: []

answers_questions:
  - "What is the SHADOW function for time spans?"
---

# Quick Definition
SHADOW(b, y) is the set of all time spans (a, x) that "happen within" the time of span (b, y) — spans beginning at or after b and ending at or before b + y. It forms a triangular region in the time-span half-plane.

# Core Definition
Section 6.10 (Lewin, pp. 189-190): "Given a time span (b, y), we can construct the family SHADOW(b, y) of all time spans (a, x) that 'happen within' the time of (b, y)." Formally: SHADOW(b, y) = {(a, x) : b <= a and a + x <= b + y}. The shadow of the entire piece SHADOW(BEGIN, EXTENT) is a triangle containing all event spans. Event (a, x) is in SHADOW(b, y) iff event1 happens during event2.

# Prerequisites
- **INJ for Measure Spaces** — SHADOW sets are used in measure-theoretic INJ contexts
- **Time-Span GIS** — SHADOW is defined for the time-span half-plane

# Key Properties
1. SHADOW(b, y) = {(a, x) : b <= a, a + x <= b + y}
2. Forms a triangle in the half-plane with vertices at (b, 0), (b+y, 0), (b, y)
3. SHADOW(BEGIN, EXTENT) = all event spans within the piece
4. (a, x) in SHADOW(b, y) iff event at (a, x) happens during event at (b, y)
5. Can be used as X or Y in INJ computations

# Construction / Recognition
## To Construct:
1. Given a time span (b, y) representing an event or section
2. SHADOW = all spans (a, x) with b <= a and a + x <= b + y
3. Visualize as a triangle in the (onset, duration) half-plane

## To Recognize:
1. A triangular region in the time-span half-plane representing temporal containment

# Context & Application
SHADOW enables questions combining temporal containment with INJ analysis. For example: "How many string events above middle C in a given section have the property that doubling their duration places them within a brass event?" This is answered by choosing X = section shadow (filtered by strings above middle C), Y = SHADOW(BRASS), and f = duration-doubling transformation.

# Examples
**Example 1** (pp. 189-190): Take a section beginning at BEGSEC lasting DURSEC. X = SHADOW(BEGSEC, DURSEC). Let BRASS = set of brass-event time spans. Y = SHADOW(BRASS) (union over all brass spans). f = transposition by interval (4, 2) in the non-commutative GIS. "orange INJ(X, Y)(f)" counts string events above middle C in the section that satisfy the temporal relation.

# Relationships
## Builds Upon
- **Time-Span GIS** — Defined in the time-span half-plane
- **INJ for Measure Spaces** — Used in measure-theoretic INJ contexts

## Related
- **INJ Function** — SHADOW sets serve as arguments to INJ

# Common Errors
- **Error**: Confusing SHADOW with the event itself
  **Correction**: SHADOW(b, y) is the set of ALL spans fitting within (b, y), not just (b, y) itself

# Common Confusions
- **Confusion**: Thinking SHADOW only applies to pitch or harmonic analysis
  **Clarification**: SHADOW is about temporal containment; it interacts with pitch information through "colored" measures (like red-dot/yellow-dot measures in the Seurat analogy)

# Source Reference
Chapter 6: Generalized Set Theory (2), section 6.10 (optional), pp. 189-190.

# Verification Notes
- Definition source: Direct from section 6.10
- Confidence rationale: Explicit definition with geometric description
- Re-extraction notes: Re-extracted from v2 card; preserved: triangle geometry, piece-level SHADOW, brass/string example. Added v3.1 structure.
