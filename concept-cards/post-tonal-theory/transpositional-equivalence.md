---
concept: Transpositional Equivalence
slug: transpositional-equivalence
category: set-theory
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.3.3 Set of pitch classes"
extraction_confidence: high
aliases:
  - Tn-equivalence
prerequisites:
  - transposition
  - normal-form
extends:
  - transposition
related:
  - tn-type
  - set-class
  - inversional-equivalence
  - interval-class-content-preservation
contrasts_with:
  - inversional-equivalence
answers_questions:
  - "What does it mean for two sets to be transpositionally equivalent?"
  - "How do I test whether two sets are related by transposition?"
---

# Quick Definition
Two pitch-class sets are transpositionally equivalent when they are related by some Tn operation, sharing the same interval-class content and the same interval succession in normal form.

# Core Definition
Two pitch-class sets are transpositionally equivalent if one can be transformed into the other by adding a constant to all pitch classes (mod 12). Transpositionally equivalent sets share identical interval-class vectors and, when placed in normal form, have the same succession of intervals between adjacent elements. They belong to the same Tn-type and, by extension, the same set class.

# Prerequisites
- **Transposition (Tn)** -- the operation establishing equivalence
- **Normal form** -- needed to compare interval successions

# Key Properties
1. Sets in normal form have the same interval succession
2. Share the same interval-class vector
3. Same cardinality
4. Belong to the same Tn-type and set class
5. Specific pitch-class content differs

# Construction / Recognition
**Test for transpositional equivalence:**
1. Put both sets in normal form
2. Extract the interval succession of each
3. If identical, the sets are transpositionally equivalent
4. Find n by subtracting corresponding elements: n = (y - x) mod 12

# Context & Application
Transpositionally equivalent sets have a similar sound quality due to their shared interval content. Recognizing transpositional equivalence reveals structural relationships that might not be apparent from note names alone. This is one of the primary means of creating coherence in post-tonal music.

# Examples
**Example 2-7** (p. 64, Webern, *Concerto for Nine Instruments*, op. 24): Four transpositionally equivalent sets:
- [D#, E, G] --T11--> [D, D#, F#] --T6--> [Ab, A, C] --T3--> [B, C, Eb]
- All have interval succession 1-3
- All contain ic1, ic3, ic4 and no others

**Example 2-10** (p. 66): [1, 3, 4, 7] and [5, 7, 8, 11] both have interval succession 2-1-3, confirming transpositional equivalence at T4.

# Relationships
## Builds Upon
- **Transposition (Tn)** -- the defining operation
- **Normal form** -- the diagnostic tool
## Enables
- **Tn-type** -- a collection of transpositionally equivalent sets
- **Set class** -- transpositional equivalence is one component of set-class equivalence
## Related
- **Interval-class content preservation** -- transpositional equivalence guarantees shared interval content
## Contrasts With
- **Inversional equivalence** -- inversionally equivalent sets have mirror-image interval successions in normal form, not identical ones

# Common Errors
- **Error**: Assuming same prime form means same Tn-type. **Correction**: Sets with the same prime form belong to the same set class but may be in different Tn-types (related by inversion rather than transposition).

# Common Confusions
- **Confusion**: Transpositional equivalence vs. set-class equivalence. **Clarification**: Transpositional equivalence is more restrictive -- sets must be related by Tn only. Set-class equivalence allows Tn or In.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.3.3, pages 64--66.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: clearly defined with explicit tests and examples
- Re-extraction notes: preserved old card's distinction from set-class equivalence; upgraded to v3 template
