---
concept: Transposition (Tn)
slug: transposition-tn
category: analysis
subcategory: post-tonal-analysis
tier: advanced
source: "Music Theory for the 21st-Century Classroom"
source_slug: 21st-century-classroom
authors: "Robert Hutchinson"
chapter: "Set Theory"
chapter_number: 33
pdf_page: 488
section: "33.7 Transposition (Tn)"
extraction_confidence: high
aliases:
  - "Tn"
  - "set transposition"
prerequisites:
  - pitch-class-set
  - integer-notation-pitches
extends: []
related:
  - inversion-tni
  - transposition-numbers-rows
contrasts_with:
  - inversion-tni
answers_questions:
  - "How does transposition work in set theory?"
  - "What is modulo 12 arithmetic?"
---

# Quick Definition
Transposition (Tn) moves every pitch class in a set up by n semitones, using modulo 12 arithmetic.

# Core Definition
Transposition is notated Tn, where n is the number of semitones up a set is transposed. Calculations use modulo 12: numbers larger than 11 are reduced by subtracting 12. Example: [1, 2, 4, 6] at T4 = [5, 6, 8, 10]. Two sets related by Tn share the same prime form and interval vector (Hutchinson, Ch. 33, pp. 488-489).

# Prerequisites
- **Pitch-class set** -- Transposition operates on sets
- **Integer notation** -- Uses integer arithmetic

# Key Properties
1. Tn = transpose up by n semitones
2. Modulo 12: reduce results > 11 by subtracting 12
3. Preserves prime form and interval vector
4. T0 = identity (no change)

# Construction / Recognition
Add n to each pitch class; if result > 11, subtract 12.

Example: [6, 8, 10, 11] at T9 -> [15, 17, 19, 20] -> subtract 12 -> [3, 5, 7, 8]

# Examples
- [1, 2, 4, 6] at T4 = [5, 6, 8, 10] (p. 489)
- [6, 8, 10, 11] at T9 = [3, 5, 7, 8] (Table 33.7.1, p. 489)

# Relationships
## Related
- **Inversion (TnI)** -- Compound operation combining inversion and transposition
- **Transposition numbers (rows)** -- Related concept in serialism
## Contrasts With
- **Inversion (TnI)** -- Transposition preserves interval direction; inversion flips it

# Common Errors
- **Error**: Forgetting to reduce results to 0-11
  **Correction**: Always apply modulo 12

# Common Confusions
- **Confusion**: Confusing Tn in set theory with transposition of twelve-tone rows
  **Clarification**: Same concept, but row transposition applies to ordered sequences

# Source Reference
Chapter 33, Section 33.7, PDF pages 488-489. Table 33.7.1.

# Verification Notes
- Definition and examples from source, pp. 488-489
- Re-extracted from v2 card; preserved: modulo 12 examples, T0 identity
- Confidence: HIGH -- source provides explicit procedure
