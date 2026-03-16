---
concept: Normal Form Algorithm
slug: normal-form-algorithm
category: set-theory
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.2.1 Putting a set into normal form"
extraction_confidence: high
aliases:
  - normal form procedure
prerequisites:
  - pitch-class-set
extends:
  - normal-form
related:
  - prime-form-algorithm
  - pitch-class-clockface
contrasts_with: []
answers_questions:
  - "How do I find the normal form of a pitch-class set?"
  - "What is the step-by-step procedure for normal form?"
---

# Quick Definition
The normal form algorithm is a step-by-step procedure for arranging a pitch-class set in its most compact ascending form within an octave.

# Core Definition
The normal form algorithm determines the unique most-compact ascending representation of a pitch-class set. This standardized procedure enables consistent comparison between sets. The algorithm presented in Straus (following Brinkman) differs slightly from older formulations (Forte, Rahn) but leads more directly to prime form.

# Prerequisites
- **Pitch-class set** -- the algorithm operates on pitch-class sets

# Key Properties
1. Produces a unique representation for each set (except certain symmetrical sets)
2. Result is written in square brackets, ascending within an octave
3. Transpositionally related sets yield normal forms with the same interval succession

# Construction / Recognition
**Step-by-step procedure (Example 2-3, p. 61):**

**Step 1**: Write all rotations. Excluding doublings, write the pitch classes ascending within an octave starting on each pitch class in turn. Calculate the span (interval from first to last) for each rotation.

**Step 2 -- Rule 1**: Choose the rotation with the smallest span from first to last. If unique, this is the normal form.

**Step 3 -- Rule 2** (tiebreaker): If two or more rotations tie for smallest span, choose the one most packed to one end -- the one with a relatively large concentration of big intervals at the top or at the bottom.

**Step 4 -- Rule 3** (symmetrical sets): If there is still a tie (as happens with inversionally symmetrical sets), prefer the rotation packed to the bottom (bigger intervals at the top).

**Clockface shortcut** (Section 2.2.2, p. 62): Display the set on a pitch-class clockface. Find the largest gap between adjacent pitch classes. The normal form begins with the pitch class after the gap, reading clockwise.

# Context & Application
This algorithm is the essential first step in any set-class analysis. Mastering it allows rapid comparison of pitch-class collections found in a score. The clockface method makes this nearly instantaneous for experienced analysts.

# Examples
**Example 2-3** (p. 61):

*Example 1*: {A, Bb, F}
- A--Bb--F: span = 8; Bb--F--A: span = 11; F--A--Bb: span = 5
- Smallest span = 5. Normal form: **[F, A, Bb]**

*Example 2*: {F, Ab, A, C#}
- F--Ab--A--C#: span = 8; C#--F--Ab--A: span = 8 (tie)
- [C#, F, Ab, A] has intervals 4-3-1 (packed to top); its competitor has intervals 3-1-4
- Normal form: **[C#, F, Ab, A]**

*Example 3*: {C, E, G#, A, B} (inversionally symmetrical)
- [E, G#, A, B, C] and [G#, A, B, C, E] both have span 8 and are identically packed
- Prefer packed to bottom: **[G#, A, B, C, E]** (bigger intervals at top)

# Relationships
## Builds Upon
- **Pitch-class set** -- the input to the algorithm
## Enables
- **Normal form** -- the output of the algorithm
- **Prime form algorithm** -- prime form procedure begins with normal form
## Related
- **Pitch-class clockface** -- provides visual shortcut for the algorithm

# Common Errors
- **Error**: Forgetting to check all rotations. **Correction**: There are exactly as many rotations as pitch classes in the set.
- **Error**: Applying only Rule 1 when there is a tie. **Correction**: Ties require Rule 2 (packing) and sometimes Rule 3 (prefer bottom-packed).

# Common Confusions
- **Confusion**: Straus's procedure vs. Forte's. **Clarification**: Forte's formulation always packs to the bottom; Straus/Brinkman's procedure packs to either end and then prefers bottom for symmetrical sets. This changes the normal form for only a few sets.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.2.1, pages 61--62.

# Verification Notes
- Definition source: direct from source (Example 2-3 table)
- Confidence rationale: algorithm given explicitly with worked examples
- Re-extraction notes: preserved old card's detailed worked examples and note about Forte/Brinkman difference; upgraded to v3 template
