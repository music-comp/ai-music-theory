---
concept: Musical Space S
slug: musical-space-s

category: generalized-interval-systems
subcategory: foundational-definitions
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "Definition 2.3.1, Section 2.4"

extraction_confidence: high

aliases:
  - "space of a GIS"
  - "GIS space"

prerequisites:
  - generalized-interval-system
extends: []
related:
  - interval-group-ivls
  - interval-function-int
  - theoretical-versus-practical-space
  - simply-transitive-action
contrasts_with: []

answers_questions:
  - "What is the space S in a GIS?"
  - "What constraints does Condition (B) place on S?"
  - "Can S be finite or must it be infinite?"
---

# Quick Definition
The musical space S in a GIS is the family of elements -- pitches, pitch classes, time points, durations, or other musical objects -- between which intervals are measured. S must be theoretically complete: for every element s and interval i, there must exist a unique element t with int(s, t) = i.

# Core Definition
In a GIS (S, IVLS, int), "S, the space of the GIS, is a family of elements" (Lewin, Definition 2.3.1, p. 47). Condition (B) requires: "For every s in S and every i in IVLS, there is a unique t in S which lies the interval i from s." This means S must contain all elements reachable by any interval from any starting point.

# Prerequisites
- **Generalized Interval System** — S is the first component of the GIS triple (S, IVLS, int)

# Key Properties
1. S is a non-empty set of musical elements
2. Condition (B) implies: for fixed s, the map t -> int(s, t) is a bijection from S to IVLS
3. |S| = |IVLS| (same cardinality, from simple transitivity)
4. S is the domain of the interval function int: S x S -> IVLS
5. S may be finite (e.g., 12 pitch classes) or infinite (e.g., chromatic pitches)

# Construction / Recognition
## To Construct:
1. Identify the musical elements of interest (pitches, durations, etc.)
2. Extend the collection as needed to satisfy Condition (B)
3. Verify that for every element and every interval, a unique target exists
## To Recognize:
1. A set paired with a group IVLS and function int satisfying both Conditions (A) and (B)

# Context & Application
"We must conceive the formal space of a GIS as a space of theoretical potentialities, rather than as a compendium of musical practicalities" (Lewin, p. 47). Condition (B) may require extending practical spaces: supersonic/subsonic "pitches" in chromatic space, or infinitely remote keys in harmonic space. A composition uses only a finite region of S, but the theoretical framework requires the full space.

# Examples
**Example 1** (Section 2.4): Diatonic pitch space (2.1.1) -- S = diatonic scale degrees extended indefinitely up and down.

**Example 2** (Section 2.4): Chromatic pitch space (2.1.2) -- S = all chromatic pitches extended indefinitely.

**Example 3** (Section 2.4): Pitch-class space (2.1.3) -- S = the twelve pitch classes. Here practical = theoretical: "Every one of its twelve pitch-classes is easily referenced by any pertinent music."

**Example 4** (Section 2.4): Harmonic space (Figure 2.2) -- S = the infinite two-dimensional game board of subscripted pitch classes.

**Example 5** (Section 2.4): Beat-class space (2.2.2) -- S = N beat classes on an N-hour clock.

# Relationships
## Builds Upon
- **Generalized Interval System** — S is one of the three defining components
## Enables
- **Interval function int** — int is defined on S x S
- **Transposition** — Operations on S defined via intervals
## Related
- **Interval group IVLS** — Measures intervals between elements of S; |S| = |IVLS|
- **Theoretical versus practical space** — S is the theoretical extension; practical use is a subset
- **Simply transitive action** — IVLS acts simply transitively on S

# Common Errors
- **Error**: Restricting S to only practically occurring musical elements
  **Correction**: S must be theoretically complete per Condition (B), even if this requires supersonic pitches or infinitely remote time points

# Common Confusions
- **Confusion**: Confusing S (the set of elements) with IVLS (the set of intervals)
  **Clarification**: S contains musical objects (pitches, durations, etc.); IVLS contains the directed measurements between them

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1, Section 2.4, pages 47-49.

# Verification Notes
- Definition source: Direct quotation from Definition 2.3.1
- Confidence rationale: Core GIS component, extensively illustrated
- Re-extraction notes: Re-extracted from v2 card; preserved: all five space examples, theoretical completeness requirement, Lewin quotation
