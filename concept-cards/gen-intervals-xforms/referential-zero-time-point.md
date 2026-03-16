---
concept: Referential Zero Time-Point
slug: referential-zero-time-point

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
  - "time-point zero"
  - "zero time-point problem"

prerequisites:
  - time-span
extends: []
related:
  - referential-time-unit-problem
  - time-span-interval-independence
contrasts_with: []

answers_questions:
  - "What is the referential zero time-point problem?"
---

# Quick Definition
The referential zero time-point problem asks: what is the abstractly privileged moment from which we measure the attack time a of a time span (a, x)? Both GIS 4.1.2 and GIS 4.1.3 are independent of this choice (Property A of Theorem 4.1.4).

# Core Definition
The number a in a time span (a, x) is measured relative to some "time-point zero." Lewin asks what this privileged moment is: the Big Bang? A notational convention (first barline)? Some contextual downbeat? Unlike the time-unit problem, the zero-point problem affects both commutative and non-commutative GIS equally — both have Property (A): shifting zero by h units transforms (a, x) to (a+h, x) without changing intervals (Lewin, pp. 95-96).

# Prerequisites
- **Time Span** — The concept whose measurement depends on the choice of zero

# Key Properties
1. Both GIS 4.1.2 and GIS 4.1.3 are independent of zero-point choice (Property A)
2. Shifting zero by h: (a, x) becomes (a+h, x); intervals unchanged
3. Less problematic than the time-unit problem (which only GIS 4.1.3 resolves)

# Examples
**Example 1** (p. 96): int((a,x),(b,y)) in GIS 4.1.3: shifting zero by h gives int((a+h,x),(b+h,y)) = ((b-a)/x, y/x) — unchanged.

# Relationships
## Related
- **Referential Time-Unit Problem** — the companion problem; more difficult
- **Time-Span Interval Independence** — Property (A) resolves the zero-point problem

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, pp. 95-96.

# Verification Notes
- Definition source: direct from Section 4.1 discussion
- Confidence rationale: high — explicit discussion
- Re-extraction notes: Re-extracted from v2 card; preserved: less-problematic-than-unit-problem observation
