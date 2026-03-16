---
concept: Time-Span GIS Properties Summary
slug: time-span-properties-summary

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
  - "Notes 4.1.7"
  - "time-span GIS summary"

prerequisites:
  - time-span-gis
  - time-span-transposition
  - time-span-inversion
  - time-span-interval-preserving-operation
extends:
  - time-span-gis
related:
  - commutative-vs-noncommutative-gis
contrasts_with: []

answers_questions:
  - "What are the key properties of the time-span GIS?"
  - "How do the abstract results from Chapter 3 apply to the time-span GIS?"
---

# Quick Definition
Notes 4.1.7 summarizes how the abstract GIS theory from Chapter 3 specializes to the time-span GIS: T_{(i,p)}(a,x) = (a+ix, px), P_{(h,u)}(a,x) = (h+ua, ux), only (0,1) is central, no non-identity transposition preserves intervals, and every distinct pair of time spans defines a distinct inversion.

# Core Definition
Notes 4.1.7 collects the key formulas and facts for GIS 4.1.3: (A) T_{(i,p)}(a,x) = (a+ix, px); (B) LABEL(a,x) = (a,x) with ref = (0,1); (C) T_{(i,p)}(a,x) = (a,x)(i,p) (right-multiplication); (D) P_{(h,u)}(a,x) = (h+ua, ux) (left-multiplication); (E) only (0,1) is central; (F) no transposition preserves intervals except identity; (G) I_{(c,z)}^{(d,w)}(a,x) = (d+(c-a)w/x, zw/x); (H) I_s^t = I_{s'}^{t'} iff s=s' and t=t'; (I) no interval-reversing operations exist (Lewin, Notes 4.1.7, pp. 112-114).

# Prerequisites
- **Time-Span GIS** — The GIS whose properties are summarized
- **Time-Span Transposition** — Property (A)
- **Time-Span Inversion** — Property (G)-(H)
- **Time-Span Interval-Preserving Operation** — Property (D)

# Key Properties
1. (A) T_{(i,p)}(a,x) = (a+ix, px)
2. (B) LABEL(a,x) = (a,x) with ref = (0,1)
3. (C) T_{(i,p)}(a,x) = (a,x)(i,p)
4. (D) P_{(h,u)}(a,x) = (h+ua, ux)
5. (E) Only (0,1) is central in IVLS
6. (F) No transposition preserves intervals (except identity)
7. (G) I_{(c,z)}^{(d,w)}(a,x) = (d+(c-a)w/x, zw/x)
8. (H) Rigid inversions: I_s^t = I_{s'}^{t'} iff s=s' and t=t'
9. (I) No interval-reversing operations

# Context & Application
This summary serves as a reference for how all the abstract machinery from Chapter 3 — transpositions, interval-preserving operations, inversions, interval-reversing operations — manifests in the concrete non-commutative time-span GIS. It concretely illustrates the theoretical distinctions between commutative and non-commutative GIS behavior.

# Examples
See individual cards for time-span-transposition, time-span-inversion, and time-span-interval-preserving-operation.

# Relationships
## Builds Upon
- **Time-Span GIS** — all properties are specializations of GIS 4.1.3

## Related
- **Commutative vs. Non-Commutative GIS** — this summary concretely illustrates the abstract distinctions

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Notes 4.1.7(A)-(I), pp. 112-114.

# Verification Notes
- Definition source: direct from Notes 4.1.7
- Confidence rationale: high — explicit enumerated properties
- Re-extraction notes: Re-extracted from v2 card; preserved: complete enumeration of all nine properties
