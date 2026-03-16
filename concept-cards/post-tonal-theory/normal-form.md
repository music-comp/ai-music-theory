---
concept: Normal Form
slug: normal-form
category: set-theory
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.2 Normal Form"
extraction_confidence: high
aliases:
  - normal order
prerequisites:
  - pitch-class-set
extends:
  - pitch-class-set
related:
  - normal-form-algorithm
  - prime-form
  - pitch-class-clockface
  - transpositional-equivalence
contrasts_with:
  - prime-form
answers_questions:
  - "What is normal form?"
  - "How do I put a pitch-class set into normal form?"
  - "How does normal form help compare sets?"
---

# Quick Definition
Normal form is the most compact way of writing a pitch-class set, arranged ascending within an octave, making it easy to visualize and compare sets.

# Core Definition
Normal form is "a simple, compact, easily grasped form" (Straus, Ch. 2) for representing a pitch-class set. It is the most compressed arrangement of the pitch classes ascending within an octave. Sets in normal form are written in square brackets. When two sets are transpositionally related, their normal forms have the same succession of intervals. When inversionally related, their normal forms have mirror-image interval successions.

# Prerequisites
- **Pitch-class set** -- normal form is a way of writing a pitch-class set

# Key Properties
1. Written in square brackets: [C, E, G] or [0, 4, 7]
2. Elements arranged ascending within an octave
3. Most compact span from first to last element
4. Transpositionally related sets share the same interval succession in normal form
5. If a set is in normal form, its transposition is also in normal form

# Construction / Recognition
See the companion card **normal-form-algorithm** for the full step-by-step procedure. In brief:
1. List all rotations of the pitch classes ascending within an octave
2. **Rule 1**: Choose the rotation with the smallest span (first to last)
3. **Rule 2** (tiebreaker): Choose the rotation most packed to one end
4. **Rule 3** (symmetrical sets): Prefer packing to the bottom (larger intervals at top)

Alternatively, display the set on a pitch-class clockface and read off the normal form by inspection.

# Context & Application
Normal form enables systematic comparison of pitch-class sets regardless of how they appear in the music. It is the first step toward identifying set-class membership (via prime form). Analysts routinely put sets into normal form when comparing collections found in a score.

# Examples
**Example 2-3** (p. 61): Three worked examples:
- {A, Bb, F}: rotations have spans 8, 11, 5. Smallest span = 5 gives normal form [F, A, Bb].
- {F, Ab, A, C#}: tie at span 8; [C#, F, Ab, A] is more packed to the top, so it is the normal form.
- {C, E, G#, A, B}: symmetrical set with two equally valid arrangements; [G#, A, B, C, E] preferred (packed to bottom).

**Example 2-4** (p. 62, Ruggles, *Lilacs*): Four chords displayed on pitch-class clockfaces with normal forms determined by visual inspection.

# Relationships
## Builds Upon
- **Pitch-class set** -- normal form is a representation of a pitch-class set
## Enables
- **Prime form** -- prime form is derived from normal form
- **Transpositional equivalence** -- recognized by identical interval successions in normal form
- **Inversional equivalence** -- recognized by mirror-image interval successions in normal form
## Related
- **Pitch-class clockface** -- visual shortcut for determining normal form
## Contrasts With
- **Prime form** -- normal form preserves specific pitch-class content; prime form always starts on 0 and identifies the set class

# Common Errors
- **Error**: Confusing normal form with prime form. **Correction**: Normal form preserves the actual pitch classes; prime form transposes to start on 0 and may invert.
- **Error**: Forgetting to check all rotations. **Correction**: There are as many rotations as pitch classes in the set; check the span of each.

# Common Confusions
- **Confusion**: This algorithm vs. Forte's original. **Clarification**: Straus adopts Brinkman's formulation, which differs slightly from Forte's (always packed to the bottom) but leads more smoothly to prime form.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.2, pages 61--62.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: clearly defined with explicit algorithm and examples
- Re-extraction notes: preserved old card's note about the algorithm differing from Forte's original; upgraded to v3 template
