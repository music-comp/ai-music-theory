---
concept: "Transposition (Tn)"
slug: transposition
category: operations
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.3 Transposition (Tn)"
extraction_confidence: high
aliases:
  - Tn
  - pitch-class transposition
prerequisites:
  - pitch-class-set
  - pitch-class
extends:
  - pitch-class
related:
  - transposition-number
  - transpositional-equivalence
  - inverse-transposition
  - mapping
  - inversion
contrasts_with:
  - inversion
answers_questions:
  - "How do I transpose a pitch-class set?"
  - "What does Tn mean?"
  - "What is preserved under transposition?"
---

# Quick Definition
Transposition (Tn) is an operation that adds a fixed interval n to every pitch class in a set or line, producing a new set or line at a different pitch level.

# Core Definition
Transposition is an operation represented as Tn, where T stands for transposition and n is the transposition number (the pitch-class interval added to each element). Transposition can operate on lines of pitches (preserving order and contour), lines of pitch classes (preserving ordered pitch-class intervals), or sets of pitch classes (preserving interval-class content but not order or contour). The formula is: Tn(x) = (x + n) mod 12.

# Prerequisites
- **Pitch-class set** -- the object being transposed
- **Pitch class** -- transposition operates on pitch classes via mod 12 arithmetic

# Key Properties
1. Tn(x) = (x + n) mod 12, for n in {0, 1, ..., 11}
2. Transposition of a line of pitches preserves ordered pitch intervals and contour
3. Transposition of a line of pitch classes preserves ordered pitch-class intervals
4. Transposition of a set of pitch classes preserves interval-class content
5. If a set is in normal form, its transposition is also in normal form
6. T0 is the identity operation

# Construction / Recognition
**To transpose a set**: Add n to each element (mod 12).
- Example: T8[5, 7, 8, 11] = [1, 3, 4, 7]

**To recognize transpositional relationship**:
1. Put both sets in normal form
2. Check if they have the same interval succession
3. If yes, calculate n: for corresponding elements x and y, n = (y - x) mod 12

# Context & Application
Transposition is a fundamental means of developing material in post-tonal music. Sets related by transposition share the same interval-class content, giving them a similar sound quality. The choice of transposition level is often compositionally meaningful, frequently replicating intervals found within the set being transposed.

# Examples
**Example 2-5** (p. 63, Schoenberg, *String Quartet No. 4*): Two melodies related by T6. Despite different contours, their ordered pitch-class intervals are identical: 11-8-1-7-10-1-8-8-11-11-5.

**Example 2-7** (p. 64, Webern, *Concerto for Nine Instruments*, op. 24): Four pitch-class sets related by T11, T6, and T3. All share ic1, ic3, ic4 and no others.

**Example 2-9** (p. 66): T8[5, 7, 8, 11] = [1, 3, 4, 7]. Each element has 8 added to it (mod 12).

# Relationships
## Builds Upon
- **Pitch-class set** -- the object being transposed
- **Mod 12 arithmetic** -- the mathematical framework
## Enables
- **Transpositional equivalence** -- sets related by Tn are transpositionally equivalent
- **Tn-type** -- a family of sets all related by transposition
- **Set class** -- defined by transposition and inversion equivalence
## Related
- **Mapping** -- transposition creates a one-to-one correspondence between elements
- **Inverse transposition** -- Tn and T(12-n) undo each other
- **Nodes and arrows** -- networks represent transpositional relationships
## Contrasts With
- **Inversion (In)** -- inversion subtracts from an index number; transposition adds a constant

# Common Errors
- **Error**: Expecting transposition to preserve order for sets. **Correction**: Sets are unordered; transposition preserves interval-class content but not order or contour.
- **Error**: Forgetting mod 12. **Correction**: All arithmetic is mod 12. E.g., 8 + 8 = 16 = 4 (mod 12).

# Common Confusions
- **Confusion**: Pitch transposition vs. pitch-class transposition. **Clarification**: Pitch transposition preserves contour (each ascending interval stays ascending). Pitch-class transposition preserves ordered pitch-class intervals but may change contour, since pitch-class intervals do not specify direction.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.3, pages 62--69.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: core operation defined explicitly with formula and multiple examples
- Re-extraction notes: preserved old card's distinction between pitch, pc-line, and pc-set transposition; added Example 2-9; upgraded to v3 template
