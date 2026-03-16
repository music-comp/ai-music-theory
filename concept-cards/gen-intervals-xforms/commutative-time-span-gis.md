---
concept: Commutative Time-Span GIS
slug: commutative-time-span-gis

category: timbral-temporal-systems
subcategory: rhythmic-structures
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS; Some Timbral GIS Models"
chapter_number: 4
pdf_page: 91
section: "4.1"

extraction_confidence: high

aliases:
  - "GIS 4.1.2"

prerequisites:
  - time-span
  - direct-product-gis
  - generalized-interval-system
extends:
  - direct-product-gis
related:
  - referential-time-unit-problem
contrasts_with:
  - time-span-gis

answers_questions:
  - "What is the commutative time-span GIS?"
  - "How does the time-span GIS relate to simpler time-point and duration GIS structures?"
---

# Quick Definition
The commutative time-span GIS (Example 4.1.2) uses int((a,x), (b,y)) = (b-a, y/x), measuring temporal distance in absolute units and duration ratio. It is a direct-product GIS that depends on the choice of referential time-unit.

# Core Definition
Example 4.1.2: Take S = TMSPS. IVLS is the direct-product group R x R+ with (i, p)(j, q) = (i + j, pq). Define int((a, x), (b, y)) = (b - a, y/x). This is a commutative GIS. The interval measures absolute temporal distance (b - a) and duration ratio (y/x). Unlike GIS 4.1.3, this GIS depends on the choice of time-unit: changing units transforms (b - a) to (b - a)u (Lewin, Example 4.1.2, pp. 92-93).

# Prerequisites
- **Time Span** — Elements are time spans (a, x)
- **Direct-Product GIS** — This is literally a direct product of time-point and duration GIS
- **Generalized Interval System** — The GIS framework

# Key Properties
1. int((a, x), (b, y)) = (b - a, y/x)
2. IVLS = R x R+ with commutative operation (i, p)(j, q) = (i + j, pq)
3. Independent of zero time-point choice (Property A of 4.1.4)
4. NOT independent of time-unit choice (fails Property B of 4.1.4)
5. Is a direct-product GIS: time-point GIS x duration GIS
6. Duration ratio y/x is shared with GIS 4.1.3

# Construction / Recognition
## To Construct:
1. Take the time-point GIS (R, R, b - a) and the duration GIS (R+, R+, y/x)
2. Form their direct product

## To Recognize:
1. Temporal distance is measured in absolute units, not span-lengths
2. The two interval components are independent (direct product structure)

# Context & Application
This simpler GIS is appropriate when a fixed referential time-unit exists (e.g., the quarter-note in Classical-period music). It fails for music with multiple simultaneous tempi (Carter, Stockhausen, Nancarrow) where no single time-unit governs the texture. It was encountered earlier as Example 3.3.2.

# Examples
**Example 1** (p. 92): Basic interval:
- s = (0, 1), t = (5, 2): int(s, t) = (5, 2) — "5 units later, twice as long"

**Example 2** (p. 93): Unit-dependence:
- In beat units: int((0, 1), (5, 2)) = (5, 2)
- In half-beat units: int((0, 2), (10, 4)) = (10, 2) — first component changed!

# Relationships
## Builds Upon
- **Direct-Product GIS** — this is a direct product of two simpler GIS

## Enables
- Understanding of what the non-commutative GIS improves upon

## Contrasts With
- **Time-Span GIS** — GIS 4.1.3 divides by x to achieve reference-independence

# Common Errors
- **Error**: Using this GIS when the music has no fixed time-unit
  **Correction**: For multi-tempo music, use GIS 4.1.3 which is reference-independent

# Common Confusions
- **Confusion**: Thinking "simpler" means "better"
  **Clarification**: GIS 4.1.2 is algebraically simpler (commutative) but less powerful (unit-dependent). GIS 4.1.3 is more complex but captures more about rhythmic structure.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Example 4.1.2, pp. 92-93.

# Verification Notes
- Definition source: direct from Example 4.1.2
- Confidence rationale: high — explicit example
- Re-extraction notes: Re-extracted from v2 card; preserved: unit-dependence example, contrast with GIS 4.1.3, Carter/Stockhausen context
