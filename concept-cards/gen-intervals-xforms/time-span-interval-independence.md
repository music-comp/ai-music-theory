---
concept: Time-Span Interval Independence
slug: time-span-interval-independence

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
  - "Theorem 4.1.4"
  - "reference independence"
  - "Properties (A) and (B)"

prerequisites:
  - time-span-gis
extends:
  - time-span-gis
related:
  - time-span-gis-uniqueness
  - referential-time-unit-problem
  - referential-zero-time-point
contrasts_with:
  - commutative-time-span-gis

answers_questions:
  - "Why is the non-commutative time-span GIS independent of referential choices?"
  - "How does the time-span GIS relate to simpler time-point and duration GIS structures?"
---

# Quick Definition
The time-span GIS 4.1.3 has two independence properties: its interval function is invariant under shifts of the referential zero time-point (Property A) and under changes of the referential time-unit (Property B).

# Core Definition
Theorem 4.1.4: GIS 4.1.3 has properties: (A) For any real h, int((a+h, x), (b+h, y)) = int((a, x), (b, y)). (B) For any positive u, int((au, xu), (bu, yu)) = int((a, x), (b, y)). Together these ensure that int(s, t) always delivers the same pair of numbers regardless of when you play the piece or at what tempo (Lewin, Theorem 4.1.4, pp. 108-110).

# Prerequisites
- **Time-Span GIS** — The GIS whose independence is being established

# Key Properties
1. Property (A): int invariant under time-point translation (a -> a+h)
2. Property (B): int invariant under unit rescaling ((a,x) -> (au,xu))
3. The commutative GIS 4.1.2 has Property (A) but NOT Property (B)
4. These two properties together force non-commutativity
5. GIS 4.1.3 is essentially the only time-span GIS with both properties (Theorem 4.1.5)

# Construction / Recognition
## To Verify Property (A):
1. Replace a with a+h and b with b+h: ((b+h)-(a+h))/x = (b-a)/x — unchanged

## To Verify Property (B):
1. Replace (a,x) with (au,xu) and (b,y) with (bu,yu): (bu-au)/(xu) = (b-a)/x — unchanged

# Context & Application
These independence properties give GIS 4.1.3 a privileged theoretical status. The function int delivers the same interval no matter when or at what tempo the music is performed. This makes the GIS suitable for analyzing music that exists independently of any particular performance.

# Examples
**Example 1** (p. 109): Property (A):
- int((3, 1), (7, 2)) = (4, 2)
- Shift by h = -3: int((0, 1), (4, 2)) = (4, 2) — same

**Example 2** (p. 109): Property (B):
- In beat units: int((3, 1), (7, 2)) = (4, 2)
- In half-beat units: int((6, 2), (14, 4)) = (4, 2) — same
- Compare GIS 4.1.2: int changes from (4, 2) to (8, 2) — different!

# Relationships
## Builds Upon
- **Time-Span GIS** — the GIS whose properties are established

## Enables
- **Time-Span GIS Uniqueness** — these properties characterize GIS 4.1.3

## Related
- **Referential Time-Unit Problem** — Property (B) resolves the unit problem
- **Referential Zero Time-Point** — Property (A) resolves the zero-point problem

## Contrasts With
- **Commutative Time-Span GIS** — has Property (A) only, not (B)

# Common Errors
- **Error**: Thinking all time-span GIS have both properties
  **Correction**: GIS 4.1.2 fails Property (B). Only GIS 4.1.3 (and isomorphs) has both.

# Common Confusions
- **Confusion**: Thinking independence means the GIS contains no temporal information
  **Clarification**: The GIS captures relative temporal relationships. It simply doesn't depend on which absolute unit was used.

- **Confusion**: Thinking Property (A) is the important one
  **Clarification**: Property (A) is shared by both GIS. Property (B) is what distinguishes the non-commutative GIS and motivates its construction.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Theorem 4.1.4, pp. 108-110.

# Verification Notes
- Definition source: direct from Theorem 4.1.4
- Confidence rationale: high — explicit theorem with proof
- Re-extraction notes: Re-extracted from v2 card; preserved: both property proofs, contrast with GIS 4.1.2, numerical examples
