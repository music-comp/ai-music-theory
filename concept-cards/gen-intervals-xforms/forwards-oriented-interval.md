---
concept: Forwards-Oriented Interval
slug: forwards-oriented-interval

category: generalized-set-theory
subcategory: time-span-set-theory
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.4"

extraction_confidence: high

aliases: []

prerequisites:
  - time-span-gis
  - attack-ordered-dyad
extends: []
related:
  - release-ordering
  - unrolling-interval-vector
contrasts_with: []

answers_questions:
  - "What is a forwards-oriented interval in the time-span GIS?"
  - "How do forwards-oriented intervals classify time-span dyads?"
---

# Quick Definition
A forwards-oriented interval (i, p) in the time-span GIS has i >= 0, and if i = 0 then p > 1. These intervals label the dyad classes of attack-ordered dyads, playing the role that interval classes play in pitch-class theory.

# Core Definition
An interval (i, p) is forwards-oriented if i > 0, or if i = 0 and p > 1 (Lewin, p. 113). Backwards-oriented intervals satisfy i < 0, or i = 0 and p < 1. The identity (0, 1) is neither. The inverse of a forwards-oriented interval is backwards-oriented and vice versa: (i, p)^{-1} = (-i/p, 1/p). The forwards-oriented intervals correspond 1-to-1 with 2-element set classes in the time-span GIS (using interval-preserving operations as CANON).

# Prerequisites
- **Time-Span GIS** — The interval group in which orientation is defined
- **Attack-Ordered Dyad** — Forwards-oriented intervals arise from AODs

# Key Properties
1. IVLS partitions into three categories: forwards-oriented, backwards-oriented, identity
2. Inverse of forwards = backwards, and vice versa
3. Label dyad classes exactly as Forte's interval classes label pitch-class dyads
4. EMB(D, X) = IFUNC(X, X)(i, p) for dyad D with forwards-oriented interval (i, p)

# Construction / Recognition
## To Determine Orientation:
1. Check the first component i of interval (i, p)
2. If i > 0: forwards-oriented
3. If i < 0: backwards-oriented
4. If i = 0: check p — forwards if p > 1, backwards if p < 1, identity if p = 1

## To Recognize:
1. The interval spanning an attack-ordered dyad is always forwards-oriented

# Context & Application
Forwards-oriented intervals enable a complete analog of Forte's interval-class analysis for rhythmic structures. The time-span interval vector lists EMB values for each forwards-oriented interval (i, p), capturing the rhythmic set's internal temporal relationships independent of tempo and absolute time.

# Examples
**Example 1** (p. 113): (1, 1) — next event begins 1 duration later, same length (successive equal notes). (2, 0.5) — event begins 2 durations later, half as long. (0, 2) — simultaneous event, twice as long.

**Example 2** (pp. 114-115, Figure 5.12): Chopin sonata motive (b) has interval vector showing (1,1) = 3, (2,1) = 2, (3,1) = 2 among other entries.

# Relationships
## Builds Upon
- **Attack-Ordered Dyad** — AOD intervals are always forwards-oriented

## Enables
- **Unrolling Interval Vector** — Uses forwards-oriented intervals as vector entries
- **M-Class Vector** — 2-class vector indexed by forwards-oriented intervals

# Common Errors
- **Error**: Confusing forwards-oriented with "ascending" pitch intervals
  **Correction**: Forwards-oriented refers to temporal direction (later onset), not pitch direction

# Common Confusions
- **Confusion**: Thinking orientation depends on musical context
  **Clarification**: Orientation is a purely formal property of the interval pair (i, p)

# Source Reference
Chapter 5: Generalized Set Theory (1), section 5.4, pp. 113-114.

# Verification Notes
- Definition source: Direct from section 5.4
- Confidence rationale: Explicit definition
- Re-extraction notes: Re-extracted from v2 card; preserved: examples, Forte analogy. Added Chopin reference, v3.1 structure.
