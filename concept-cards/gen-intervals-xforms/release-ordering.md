---
# === CORE IDENTIFICATION ===
concept: Release-Ordering
slug: release-ordering

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
section: "5.4.2"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - release ordering

# === TYPED RELATIONSHIPS ===
prerequisites:
  - time-span-gis
  - set-in-gis
extends: []
related:
  - attack-ordered-dyad
  - unrolling-interval-vector
  - brahms-rhapsody-emb
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is release-ordering and why is it needed?"
  - "How does release-ordering differ from attack-ordering?"
---

# Quick Definition
Release-ordering arranges time spans by when they end (release point), not when they begin, reflecting the order in which spans are fully perceived by a listener.

# Core Definition
"Given distinct spans s and t, s precedes t in the release-ordering if s ends before t ends, or if they end simultaneously and s is longer" (Lewin, p. 117). The rationale: "we cannot claim to have perceived [a span] as [a] span" until it ends, because "we do not yet know how long [it is] going to be" (p. 117). Release-ordering thus models the order of perceptual completion.

# Prerequisites
- **Time-Span GIS** — Release-ordering applies to sets of time spans
- **Set in a GIS** — The spans being ordered form a set

# Key Properties
1. For s = (a, x): release time = a + x
2. s precedes t in release-ordering if (a+x) < (b+y), or if equal and x > y
3. Release-ordering may differ from attack-ordering
4. Determines the stages for unrolling interval vectors and EMB
5. Models perceptual reality: a span is not fully perceived until it ends

# Construction / Recognition
## To Construct:
1. For each span (a, x), compute release time a + x
2. Order spans by ascending release time
3. Break ties by listing the longer span first (it began earlier)

## To Recognize:
1. Spans arranged so that each has ended before the next is listed

# Context & Application
Release-ordering is essential for "unrolling" interval vectors and EMB values. At time point 17, even though a viola may have attacked at time 16, we cannot count its span or any intervals involving it because we do not yet know its duration. Only spans that have released can participate in computed intervals.

# Examples
**Example 1** (p. 117, Figure 5.13): String trio passage. Attack-ordering: vn1, va1, vc1, vn2, ... Release-ordering: vn1, vn2, vc1, va1, ... At time 17 (attack of vn2), only vn1 has fully "happened" — no intervals are yet available.

# Relationships
## Builds Upon
- **Time-Span GIS** — Ordering applies to time spans

## Enables
- **Unrolling Interval Vector** — Uses release-ordering to determine stages
- **Brahms Rhapsody EMB Analysis** — Stage articulation based on release-ordering

## Related
- **Attack-Ordered Dyad** — Different ordering used for dyad classification

# Common Errors
- **Error**: Using attack-ordering for unrolling computations
  **Correction**: Unrolling must use release-ordering to properly model perception

# Common Confusions
- **Confusion**: Thinking release-ordering is simply the reverse of attack-ordering
  **Clarification**: A short note attacking late can release before a long note attacking early; the orderings can differ in complex ways

# Source Reference
Chapter 5: Generalized Set Theory (1), Example 5.4.2, pp. 116-118.

# Verification Notes
- Definition source: Direct from section 5.4.2
- Confidence rationale: Explicit definition with detailed rationale
- Re-extraction notes: Re-extracted from v2 card; preserved: string trio example, perceptual rationale. Added v3.1 structure.
