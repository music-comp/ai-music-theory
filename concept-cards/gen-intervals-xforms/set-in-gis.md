---
concept: Set in a GIS
slug: set-in-gis

category: generalized-set-theory
subcategory: foundational-definitions
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
section: "5.1.1"

extraction_confidence: high

aliases:
  - finite subset of S
  - set (generalized)

prerequisites:
  - generalized-interval-system
  - musical-space-s
extends: []
related:
  - ifunc
  - canonical-equivalence
  - set-class
contrasts_with: []

answers_questions:
  - "What is a set in the context of generalized set theory?"
  - "How does Lewin's definition of 'set' differ from general mathematical usage?"
---

# Quick Definition
In generalized set theory, a "set" is a finite unordered subfamily of the space S of musical elements within a GIS. This restricts the standard mathematical notion to finite collections.

# Core Definition
Definition 5.1.1: "Given a GIS (S, IVLS, int), we shall mean by a set in the present chapter a finite unordered subfamily of S" (Lewin, p. 119). This definition restricts attention to finite collections of elements, though S itself may be infinite. Definition 5.1.2 further specifies that for any mapping f of S into itself and any set X, f(X) denotes the set of elements f(s) as s varies over X; if f is 1-to-1, f(X) has the same cardinality as X, otherwise f(X) may have smaller cardinality.

# Prerequisites
- **Generalized Interval System** — Sets are defined within the context of a GIS (S, IVLS, int)
- **Musical Space S** — The space from which finite subsets are drawn

# Key Properties
1. Must be a finite collection of elements from S
2. Is unordered — listing order is immaterial
3. The cardinality card(X) counts the number of distinct elements
4. Under a 1-to-1 mapping f, card(f(X)) = card(X)
5. Under a non-1-to-1 mapping f, card(f(X)) may be less than card(X)
6. S itself may be infinite even though sets must be finite

# Construction / Recognition
## To Construct:
1. Begin with a GIS (S, IVLS, int)
2. Select a finite number of distinct elements from S
3. The resulting collection, ignoring order, is a set

## To Recognize:
1. Verify the elements belong to the space S of some GIS
2. Confirm the collection is finite
3. Confirm no element is repeated

# Context & Application
Sets in GIS theory generalize pitch-class sets from traditional atonal theory. While Forte's sets are subsets of 12 chromatic pitch classes, sets in a GIS can be collections of any musical objects: pitches, time points, time spans, durations, or elements of any space S admitting a GIS structure. This definition is the foundation for all of Chapter 5's generalized set theory, including IFUNC, canonical equivalence, and EMB.

# Examples
**Example 1** (p. 120): In the standard pitch-class GIS, X1 = {E, Bb} and Y1 = {F, A, C#} are sets of cardinalities 2 and 3 respectively, used to compute IFUNC(X1, Y1).

**Example 2** (p. 121): In the Webern op. 7 analysis, X = {Ab, Bb, Eb} models pitch classes of a melodic phrase, and Y is a 7-note set modeling the second phrase.

**Example 3** (pp. 111-112): In the non-commutative GIS of time spans, a set Y models temporal aspects of a string trio passage, where each member is a time span (a, x).

# Relationships
## Builds Upon
- **Generalized Interval System** — Sets are defined as subsets of the GIS's space S

## Enables
- **IFUNC** — Requires sets as arguments for the interval function
- **Canonical Equivalence** — Defines equivalence between sets
- **EMB Function** — Counts embeddings of one set class in another set
- **INJ Function** — Counts elements of a set mapping into another

## Related
- **Set Class** — The equivalence class of a set under canonical operations

# Common Errors
- **Error**: Including infinite subsets as "sets" in this theory
  **Correction**: Only finite subsets of S qualify as sets in Lewin's generalized set theory

- **Error**: Treating the listing order of a set's elements as significant
  **Correction**: Sets are unordered; {C, E, G} and {G, E, C} are the same set

# Common Confusions
- **Confusion**: Assuming "set" here means the same as in general mathematics
  **Clarification**: Lewin explicitly restricts "set" to mean finite unordered subfamily; in section 6.10 he relaxes this restriction for measure-theoretic generalizations

- **Confusion**: Thinking sets must be pitch-class sets
  **Clarification**: Sets can be drawn from any musical space S — pitches, time spans, durations, protocol pairs, etc.

# Source Reference
Chapter 5: Generalized Set Theory (1), Definitions 5.1.1 and 5.1.2, pp. 119-120.

# Verification Notes
- Definition source: Direct from Definition 5.1.1 and 5.1.2
- Confidence rationale: Explicit definitions provided in source
- Re-extraction notes: Re-extracted from v2 card; preserved: time-span example reference, basic definition structure. Added typed relationships, prerequisites, v3.1 sections.
