---
concept: GIS Condition A
slug: gis-condition-a

category: generalized-interval-systems
subcategory: core-definitions
tier: intermediate

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
section: "2.3.1(A)"

extraction_confidence: high

aliases:
  - path composition law
  - interval composition condition

prerequisites:
  - generalized-interval-system
  - group
extends: []
related:
  - gis-condition-b
  - gis-theorem-2-3-2
  - directed-interval
contrasts_with: []

answers_questions:
  - "How does the interval function int relate to the group IVLS?"
  - "What is a Generalized Interval System (GIS)?"
---

# Quick Definition

Condition (A) of the GIS definition states that intervals compose along paths: the interval from r to s, combined with the interval from s to t, equals the interval from r to t.

# Core Definition

"For all r, s, and t in S, int(r, s)int(s, t) = int(r, t)" (Lewin, Definition 2.3.1(A), p. 52). This condition ensures that the interval function respects path concatenation within the musical space. Combined with the group structure of IVLS, it implies int(s, s) = e and int(t, s) = int(s, t)^(-1) (Theorem 2.3.2).

# Prerequisites

- **Generalized Interval System** — Condition (A) is part of the GIS definition
- **Group** — the operation in IVLS is used to compose intervals

# Key Properties

1. int(r, s) * int(s, t) = int(r, t) for all r, s, t in S
2. Analogous to vector addition: displacement A->B plus B->C equals A->C
3. Implies int(s, s) = e (identity interval) via Theorem 2.3.2
4. Implies int(t, s) = int(s, t)^(-1) (direction reversal) via Theorem 2.3.2
5. Alone, Condition (A) does not define a GIS -- Condition (B) is also needed

# Construction / Recognition

## To Verify:
1. Pick any three elements r, s, t in S
2. Compute int(r, s), int(s, t), and int(r, t)
3. Check that int(r, s) * int(s, t) = int(r, t) in the group IVLS

# Context & Application

Condition (A) captures the basic intuition that intervals "add up" along a path. Going up a major third (4 semitones) then a minor third (3 semitones) equals going up a perfect fifth (7 semitones). Lewin emphasizes that this requires non-traditional interval numbering: in diatonic space, 2 + 2 = 4, not "3rd + 3rd = 5th."

# Examples

**Example 1** (p. 47): Diatonic pitch space: int(C4, E4) = 2, int(E4, G4) = 2, int(C4, G4) = 4, and 2 + 2 = 4.

**Example 2**: Chromatic pitch space: int(C4, E4) = 4, int(E4, G4) = 3, int(C4, G4) = 7, and 4 + 3 = 7.

**Example 3**: Pitch-class space mod 12: int(C, F#) = 6, int(F#, C) = 6, int(C, C) = 0, and 6 + 6 = 12 = 0 mod 12.

# Relationships

## Builds Upon
- **Group** — the group operation composes intervals

## Enables
- **GIS Theorem 2.3.2** — follows from Condition (A) and group structure

## Related
- **GIS Condition B** — the other GIS condition (space completeness)
- **Directed Interval** — Condition (A) governs how directed intervals compose

# Common Errors

- **Error**: Assuming Condition (A) alone defines a GIS.
  **Correction**: Both Condition (A) and Condition (B) are required.

# Common Confusions

- **Confusion**: Thinking int(s, s) = e must be stated as a separate axiom.
  **Clarification**: It follows from Condition (A) and the group structure (Theorem 2.3.2).

# Source Reference

Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1(A), pp. 52-53.

# Verification Notes

- Definition source: direct from Definition 2.3.1(A)
- Confidence rationale: explicit condition with clear statement
- Re-extracted from v2 card; preserved: diatonic "2+2=4" vs "3rd+3rd=5th" observation, chromatic and mod 12 examples
