---
# === CORE IDENTIFICATION ===
concept: Time-Span Inversion
slug: time-span-inversion

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
  - "I_{(c,z)}^{(d,w)}"
  - "time-span I_u^v"

# === TYPED RELATIONSHIPS ===
prerequisites:
  - time-span-gis
  - inversion-operation
extends:
  - inversion-operation
related:
  - time-span-transposition
  - time-span-interval-preserving-operation
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How does inversion work in the non-commutative time-span GIS?"
---

# Quick Definition
Time-span inversion I_{(c,z)}^{(d,w)}(a, x) = (d + (c-a)w/x, zw/x) is uniquely rigid: two inversions are equal only when their parameters are identical, since only the identity interval is central.

# Core Definition
Notes 4.1.7(G): The (c,z)/(d,w) inversion applied to time span (a, x) yields I_{(c,z)}^{(d,w)}(a, x) = (d + (c-a)w/x, zw/x) = (d, w)(a, x)^{-1}(c, z). Notes 4.1.7(H): I_{s'}^{t'} = I_s^t iff s' = s and t' = t, since only (0, 1) is central. There are no interval-reversing operations (Note 4.1.7I) (Lewin, Notes 4.1.7(G)-(I), pp. 113-114).

# Prerequisites
- **Time-Span GIS** — The non-commutative GIS context
- **Inversion Operation** — The general theory from Chapter 3

# Key Properties
1. I_{(c,z)}^{(d,w)}(a, x) = (d + (c-a)w/x, zw/x)
2. Equivalently: (d, w)(a, x)^{-1}(c, z) in IVLS
3. Rigid: I_{s'}^{t'} = I_s^t iff s' = s and t' = t
4. No interval-reversing operations exist (Theorem 3.6.4)
5. (I_{(c,z)}^{(d,w)})^{-1} = I_{(d,w)}^{(c,z)}, which differs from the original

# Construction / Recognition
## To Construct:
1. Choose parameter time spans (c, z) and (d, w)
2. Apply formula: I(a, x) = (d + (c-a)w/x, zw/x)

# Context & Application
Time-span inversion is much more rigid than pitch-class inversion. In pitch-class GIS, many pairs of parameters yield the same inversion (e.g., I_C^C = I_{F#}^{F#}). In the time-span GIS, every distinct pair of time spans defines a distinct inversion operation. This rigidity follows from the fact that only the identity is central in the time-span interval group.

# Examples
**Example 1** (from old card): I_{(2,3)}^{(4,5)}(1, 6) = (4 + (2-1)*5/6, 3*5/6) = (29/6, 5/2)

**Example 2** (Figure 4.4): If int(s_2, t_2) = int(s_1, t_1), then t_2 = I_{t_1}^{s_2}(s_1).

# Relationships
## Builds Upon
- **Inversion Operation** — general I_u^v theory from Chapter 3
- **Time-Span GIS** — the specific non-commutative context

## Enables
- Analysis of time-span inversion relationships in multi-tempo music

## Related
- **Time-Span Transposition** — T does not preserve intervals; I does not reverse them

# Common Errors
- **Error**: Assuming some inversions are equivalent as in pitch-class GIS
  **Correction**: Every distinct pair of time-span parameters gives a distinct inversion

# Common Confusions
- **Confusion**: Expecting inversion to reverse intervals
  **Clarification**: There are NO interval-reversing operations in the time-span GIS (Theorem 3.6.4)

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Notes 4.1.7(G)-(I), pp. 113-114.

# Verification Notes
- Definition source: direct from Notes 4.1.7(G)
- Confidence rationale: high — explicit formula with derivation
- Re-extraction notes: Re-extracted from v2 card; preserved: worked calculation, rigidity examples, LABEL formula verification
