---
concept: Inversional Equivalence
slug: inversional-equivalence
category: set-theory
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.4.4 Set of pitch classes"
extraction_confidence: high
aliases:
  - In-equivalence
prerequisites:
  - inversion
  - normal-form
extends:
  - inversion
related:
  - transpositional-equivalence
  - set-class
  - tn-type
contrasts_with:
  - transpositional-equivalence
answers_questions:
  - "What does it mean for two sets to be inversionally equivalent?"
  - "How do I test whether two sets are related by inversion?"
---

# Quick Definition
Two pitch-class sets are inversionally equivalent when related by some In operation, sharing the same interval-class content with mirror-image interval successions in normal form.

# Core Definition
Two pitch-class sets are inversionally equivalent if one can be transformed into the other by inversion (In). Inversionally equivalent sets share the same interval-class vector but have mirror-image interval successions when placed in normal form. The first element of one set's normal form corresponds to the last element of the other's, the second to the second-to-last, and so on. The sum of each corresponding pair equals n (the index number). Sets related by inversion belong to the same set class but different Tn-types.

# Prerequisites
- **Inversion (In)** -- the operation establishing equivalence
- **Normal form** -- needed to compare interval successions

# Key Properties
1. Mirror-image interval successions in normal form
2. Same interval-class vector
3. Same cardinality and set-class membership
4. Different Tn-type membership
5. Corresponding elements sum to a constant (the index number)
6. First <-> last, second <-> second-to-last correspondence in normal form

# Construction / Recognition
**Test for inversional equivalence:**
1. Put both sets in normal form
2. Extract interval successions
3. Check if they are mirror images (one reads bottom-up as the other reads top-down)
4. If yes, verify by adding corresponding elements (first + last, second + second-to-last)
5. All pairs should yield the same sum n

# Context & Application
Inversionally equivalent sets share interval-class content (same "sound quality") but present intervals in reversed direction. Together with transpositional equivalence, inversional equivalence defines set-class membership. A passage may use both transposition and inversion to develop material within a single set class.

# Examples
**Example 2-21** (p. 73, Schoenberg, *Three Piano Pieces*, op. 11, no. 1):
- Set 1: [G, G#, B] -- intervals: 1-3
- Set 2: [Db, E, F] -- intervals: 3-1 (mirror image)
- Related at I0: G+F = 7+5 = 0, G#+E = 8+4 = 0, B+Db = 11+1 = 0 (all mod 12)

**Example 2-22** (p. 73): Sets 1 and 3: [G, G#, B] and [G, Bb, B] related at I6. G+B = 7+11 = 6, G#+Bb = 8+10 = 6.

# Relationships
## Builds Upon
- **Inversion (In)** -- the defining operation
- **Normal form** -- the diagnostic tool
## Enables
- **Set class** -- inversional equivalence is one component of set-class equivalence
## Related
- **Tn-type** -- inversionally equivalent sets belong to different Tn-types
## Contrasts With
- **Transpositional equivalence** -- transpositionally equivalent sets have *identical* interval successions in normal form; inversionally equivalent sets have *mirror-image* successions

# Common Errors
- **Error**: Comparing first-to-first in normal form. **Correction**: Under inversion, the correspondence is first-to-last, second-to-second-to-last.

# Common Confusions
- **Confusion**: Transpositional vs. inversional equivalence. **Clarification**: Check interval successions -- identical means transposition; mirror-image means inversion.

# Source Reference
Chapter 2: Pitch-Class Sets, Sections 2.4.4, pages 72--74.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: clearly defined with explicit tests and examples
- Re-extraction notes: preserved old card's Schoenberg examples and distinction from transpositional equivalence; upgraded to v3 template
