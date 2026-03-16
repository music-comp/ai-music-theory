---
concept: Time-Span Transposition
slug: time-span-transposition

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
  - "T_{(i,p)}"

prerequisites:
  - time-span-gis
  - transposition-operation
extends:
  - transposition-operation
related:
  - time-span-interval-preserving-operation
  - time-span-inversion
contrasts_with:
  - time-span-interval-preserving-operation

answers_questions:
  - "How does transposition work in the non-commutative time-span GIS?"
---

# Quick Definition
Time-span transposition T_{(i,p)}(a, x) = (a + ix, px) shifts the attack point i span-lengths later and scales the duration by factor p. Due to non-commutativity, it does not preserve intervals (except for the identity) and may even reverse chronological order.

# Core Definition
Notes 4.1.7(A): Given interval (i, p) and time span (a, x), transposition yields T_{(i,p)}(a, x) = (a + ix, px). The transposed span begins i x-lengths later than a and lasts p times as long. Equivalently, T_{(i,p)}(a, x) = (a, x)(i, p) in IVLS (right-multiplication). Since no non-identity interval is central in this GIS (Note 4.1.7E), no non-identity transposition preserves intervals (Note 4.1.7F) (Lewin, Notes 4.1.7, pp. 112-114).

# Prerequisites
- **Time-Span GIS** — The GIS in which transposition operates
- **Transposition Operation** — The general theory from Chapter 3

# Key Properties
1. T_{(i,p)}(a, x) = (a + ix, px)
2. The shift is ix (not i) — measured in x-units
3. No non-identity transposition preserves intervals (Note 4.1.7F)
4. Transposition may reverse chronological order of events (Figure 4.4)
5. T_{(i,p)}(a, x) = (a, x)(i, p) (right-multiplication in IVLS)

# Construction / Recognition
## To Construct:
1. Choose interval (i, p)
2. For time span (a, x): shift attack by ix, multiply duration by p
3. Result: (a + ix, px)

# Context & Application
Unlike familiar pitch transposition, time-span transposition can distort intervals and reverse chronology. On Figure 4.4, two spans s_1 and s_2 with s_1 preceding s_2 can be transposed by (4, 2) so that t_1 = T(s_1) follows t_2 = T(s_2). This counterintuitive behavior is inherent to non-commutative structure.

# Examples
**Example 1** (p. 113): T_{(2,3)}(1, 4) = (1 + 2*4, 3*4) = (9, 12)

**Example 2** (p. 113): Non-preservation of intervals:
- s = (0, 1), t = (2, 1): int(s, t) = (2, 1)
- T_{(1,2)}(s) = (1, 2), T_{(1,2)}(t) = (3, 2)
- int((1, 2), (3, 2)) = (1, 1) — different from (2, 1)

**Example 3** (Figure 4.4): Chronology reversal when transposing two spans at different local tempi.

# Relationships
## Builds Upon
- **Transposition Operation** — general T_i theory from Chapter 3
- **Time-Span GIS** — the specific GIS context

## Enables
- Analysis of rhythmic transformations in multi-tempo music

## Contrasts With
- **Time-Span Interval-Preserving Operation** — P_{(h,u)} preserves intervals; T_{(i,p)} does not

# Common Errors
- **Error**: Expecting transposition to preserve intervals as in pitch-class GIS
  **Correction**: Only T_{(0,1)} = identity preserves intervals in this non-commutative GIS

# Common Confusions
- **Confusion**: Thinking the shift is i absolute units
  **Clarification**: The shift is ix — measured in x-units (the duration of the original span)

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Notes 4.1.7(A)-(C), (F), pp. 112-114.

# Verification Notes
- Definition source: direct from Notes 4.1.7(A)
- Confidence rationale: high — explicit formula with examples
- Re-extraction notes: Re-extracted from v2 card; preserved: non-preservation example, chronology reversal, right-multiplication interpretation
