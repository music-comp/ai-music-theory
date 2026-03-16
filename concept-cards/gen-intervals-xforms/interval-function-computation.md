---
# === CORE IDENTIFICATION ===
concept: Interval Function Computation (Time-Span GIS)
slug: interval-function-computation

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
pdf_page: 107
section: "4.1"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - "time-span interval formula"
  - "non-commutative interval computation"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - time-span
extends: []
related:
  - left-vs-right-group-operations
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How are intervals computed in the non-commutative time-span GIS?"
  - "Why does the interval formula divide by the first span's duration?"
  - "How does this formula differ from the commutative time-span GIS?"
---

# Quick Definition
Computing intervals in the non-commutative time-span GIS requires the formula int((a, x), (b, y)) = ((b - a)/x, y/x), where the first span's duration x serves as the measuring unit for temporal distance.

# Core Definition
**Theorem 4.1.3.2**: In the non-commutative time-span GIS (TMSPS, IVLS, int), the interval from time span (a, x) to time span (b, y) is:

$$\text{int}((a, x), (b, y)) = ((b - a)/x, \; y/x)$$

The first component (b - a)/x measures how many x-durations separate the beginnings; the second component y/x is the duration ratio. This formula differs from the commutative GIS 4.1.2 where int((a, x), (b, y)) = (b - a, y/x) -- the key difference being division of the temporal distance by x.

# Prerequisites
- **Time span** — The ordered pair (a, x) modeling a musical event beginning at time a with duration x

# Key Properties
1. The first component (b - a)/x normalizes temporal distance to x-units
2. The second component y/x is the duration ratio (same in both GIS versions)
3. The formula is unit-independent: changing time units preserves the interval
4. The group operation is non-commutative: (i, p)(j, q) = (i + pj, pq)
5. The identity interval is (0, 1)

# Construction / Recognition
## To Construct:
1. Given time spans (a, x) and (b, y), compute b - a (temporal distance)
2. Divide by x to get the first component: (b - a)/x
3. Compute the duration ratio: y/x
4. The interval is ((b - a)/x, y/x)
## To Recognize:
1. An interval computed using the first span's duration as measuring unit
2. A pair whose first component is normalized temporal distance

# Context & Application
The formula captures: "How many of span-1's durations until span-2 starts, and how do their durations compare?" This is musically meaningful when the first span's duration serves as the local time unit. The division by x is what makes the GIS non-commutative, since (b - a)/x depends on which span comes first.

# Examples
**Example 1** (p. 108): s = (0, 2), t = (6, 4). int(s, t) = ((6 - 0)/2, 4/2) = (3, 2). Meaning: t starts 3 s-durations after s and lasts twice as long.

**Example 2** (Figure 4.4, p. 108): s1 = (0, 1), t1 = (4, 2): int = (4, 2). s2 = (1, 0.5), t2 = (3, 1): int = ((3-1)/0.5, 1/0.5) = (4, 2). Same interval despite different absolute positions -- the formula is unit-independent.

**Example 3**: Unit independence: s = (0, 4), t = (12, 8) (doubling all times). int = ((12-0)/4, 8/4) = (3, 2) -- same as Example 1.

**Example 4**: Inverse calculation: if int((a, x), (b, y)) = (i, p), then b = a + ix and y = px.

# Relationships
## Builds Upon
- **Time span** — The objects whose intervals are computed
## Enables
- **Left vs. right group operations** — The non-commutative formula distinguishes T and P operations
## Related
- **Non-commutative GIS** — The time-span GIS is the primary example
## Contrasts With
- **Commutative time-span GIS (4.1.2)** — Uses int = (b - a, y/x) without dividing by x

# Common Errors
- **Error**: Forgetting to divide (b - a) by x in the first component
  **Correction**: The formula is (b - a)/x, not (b - a); this division is the key difference from GIS 4.1.2

# Common Confusions
- **Confusion**: Both components of the interval change when time units change
  **Clarification**: The formula is designed to be unit-independent; changing all times by a constant factor preserves the interval
- **Confusion**: The first component can only be positive
  **Clarification**: If b < a (second span starts before first), the first component is negative

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models, Theorem 4.1.3.2, pages 107-108.

# Verification Notes
- Definition source: Direct from Theorem 4.1.3.2
- Confidence rationale: High -- explicitly stated and demonstrated with examples
- Re-extraction notes: Re-extracted from v2 card; preserved: numerical examples, unit independence demonstration, contrast with GIS 4.1.2, inverse calculation
