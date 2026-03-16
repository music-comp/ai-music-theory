---
# === CORE IDENTIFICATION ===
concept: "Attack-Ordered Dyad (AOD)"
slug: attack-ordered-dyad

# === CLASSIFICATION ===
category: generalized-set-theory
subcategory: time-span-set-theory
tier: advanced

# === PROVENANCE ===
source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.4"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - AOD

# === TYPED RELATIONSHIPS ===
prerequisites:
  - set-in-gis
  - time-span-gis
extends: []
related:
  - forwards-oriented-interval
  - release-ordering
  - canonical-group
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is an attack-ordered dyad in the time-span GIS?"
  - "How do dyad classes correspond to forwards-oriented intervals?"
---

# Quick Definition
An attack-ordered dyad (AOD) is a pair of distinct time spans ordered by onset time (earlier first), with duration as tiebreaker for simultaneous onsets (shorter first). This ordering enables a 1-to-1 correspondence between dyad classes and forwards-oriented intervals.

# Core Definition
A "dyad" is a set containing two distinct members s and t. An AOD orders them as follows: "If s begins before t, the order is (s, t); if t begins before s, the order is (t, s); if both time spans begin at the same time, the shorter of the two spans is listed first" (Lewin, p. 113). Since s and t are distinct, these criteria suffice to order any dyad. Given AOD D = (s, t), int(s, t) = (i, p) is always forwards-oriented: i >= 0, and if i = 0 then p > 1.

# Prerequisites
- **Set in a GIS** — Dyads are 2-element sets
- **Time-Span GIS** — AODs are defined for the time-span GIS specifically

# Key Properties
1. The interval int(s, t) of an AOD is always forwards-oriented
2. Conversely, if int(s, t) is forwards-oriented, then (s, t) is an AOD
3. Crucial theorem (proved in Appendix 5.6): D1 and D2 are canonically equivalent iff int(s1, t1) = int(s2, t2)
4. 2-element set classes correspond 1-to-1 with forwards-oriented intervals
5. This gives an analog of Forte's interval classes for the time-span GIS

# Construction / Recognition
## To Construct:
1. Given two distinct time spans s = (a, x) and t = (b, y)
2. If a < b, order is (s, t); if a > b, order is (t, s); if a = b, shorter first

## To Recognize:
1. A pair of time spans where the first has equal or earlier onset, and if simultaneous, shorter duration

# Context & Application
The AOD construction is essential for developing an interval vector in the time-span GIS. The forwards-oriented intervals label dyad classes exactly as Forte's interval classes label pitch-class dyad classes. This enables applying EMB and IFUNC techniques to rhythmic analysis.

# Examples
**Example 1** (p. 113): Two successive eighth notes: s = (0, 0.5), t = (0.5, 0.5). AOD = (s, t). int(s, t) = (1, 1).

**Example 2** (p. 113): Simultaneous quarter and half note: s = (0, 1), t = (0, 2). AOD = (s, t) since s is shorter. int(s, t) = (0, 2).

# Relationships
## Builds Upon
- **Time-Span GIS** — AODs are dyads in this specific GIS

## Enables
- **Forwards-Oriented Interval** — Labels AOD classes
- **Unrolling Interval Vector** — Uses AOD structure for time-span interval vectors

## Related
- **Canonical Group** — AOD equivalence uses interval-preserving operations as CANON

# Common Errors
- **Error**: Using attack-ordering for spans when computing unrolling interval vectors
  **Correction**: Unrolling uses release-ordering, not attack-ordering (see section 5.4.2)

# Common Confusions
- **Confusion**: Thinking attack-ordering captures perceptual order
  **Clarification**: Attack-ordering captures onset order; perceptual order (when spans are fully "heard") requires release-ordering

# Source Reference
Chapter 5: Generalized Set Theory (1), section 5.4 and Appendix 5.6, pp. 113-114, 121-122.

# Verification Notes
- Definition source: Direct from section 5.4
- Confidence rationale: Explicit definition with crucial theorem (proved in appendix)
- Re-extraction notes: Re-extracted from v2 card; preserved: examples, perceptual distinction. Added v3.1 structure.
