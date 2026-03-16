---
concept: Serial Transformations Generalization
slug: serial-transformations-generalization

category: transformation-theory
subcategory: serial-operations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.4"

extraction_confidence: high

aliases: []

prerequisites:
  - rich-transformation
  - non-commutative-gis
extends:
  - rich-transformation
related:
  - tch-transformation
  - much-transformation
contrasts_with: []

answers_questions:
  - "How do serial transformations generalize to abstract GIS settings?"
  - "What complications arise in non-commutative GIS?"
---

# Quick Definition
The extension of serial transformations (RICH, TCH, MUCH, TLAST, TFIRST, FLIPEND, FLIPSTART) to series in abstract commutative GIS contexts, with complications in non-commutative cases where multiple plausible definitions exist for operations like RICH.

# Core Definition
"The serial transformations just studied... are all easily generalized to operate on series whose elements are members of an abstract commutative GIS. In the non-commutative case, it is not clear just how some of the operations are to be defined; different possibilities are equally plausible" (Lewin, 8.4, p. 189). For RICH in a non-commutative GIS, three candidates t, u, v may all differ: t and u are retrogrades of different inversions, while v preserves the reversed serial intervals. "If the GIS is commutative, t, u, and v will all be the same series. If the GIS is not commutative, t, u, and v may be three distinct series" (p. 190).

# Prerequisites
- **RICH transformation** — The primary operation being generalized
- **Non-commutative GIS** — The context where complications arise

# Key Properties
1. All serial transformations generalize straightforwardly to commutative GIS
2. Non-commutative GIS creates ambiguity: three candidates for RICH(s)
3. Candidate t: retrograde of (a/b)-inversion of s
4. Candidate u: retrograde of (b/a)-inversion of s
5. Candidate v: series starting with a, using reversed serial intervals
6. All three share the same first two elements (a, b)
7. In commutative GIS: t = u = v

# Construction / Recognition
## To Construct:
1. In commutative GIS: apply standard definitions directly
2. In non-commutative GIS: choose among t, u, v based on analytical context
## To Recognize:
1. Ambiguity in RICH definition signals a non-commutative GIS context

# Context & Application
This generalization shows that serial transformations depend on the underlying intervallic structure. The non-commutative case reveals interpretive choices even for "standard" operations.

# Examples
**Example 1** (p. 189): For s = s_1, s_2, ..., a, b with serial intervals i_1, ..., i_{n-1}: candidate v starts with a and proceeds by intervals i_{n-1}, i_{n-2}, ..., i_1 (reversed order).

# Relationships
## Builds Upon
- **RICH transformation** — The primary operation generalized
## Related
- **Non-commutative GIS** — The context where complications arise

# Common Errors
- **Error**: Assuming RICH is unambiguous in all GIS contexts
  **Correction**: In non-commutative GIS, three plausible candidates may differ

# Common Confusions
- **Confusion**: Thinking this affects standard pitch-class applications
  **Clarification**: Z_12 is commutative, so all candidates collapse to one; only non-commutative GIS (e.g., time-span GIS) creates ambiguity

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.4, pages 189-190.

# Verification Notes
- Definition source: Direct from 8.4
- Confidence rationale: Explicitly discussed
- Re-extraction notes: Re-extracted from v2 card; preserved: three candidates, commutative collapse
