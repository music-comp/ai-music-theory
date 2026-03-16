---
concept: Inversion (TnI)
slug: inversion-tni
category: analysis
subcategory: post-tonal-analysis
tier: advanced
source: "Music Theory for the 21st-Century Classroom"
source_slug: 21st-century-classroom
authors: "Robert Hutchinson"
chapter: "Set Theory"
chapter_number: 33
pdf_page: 489
section: "33.8 Inversion (TnI)"
extraction_confidence: high
aliases:
  - "TnI"
  - "In"
  - "set inversion"
prerequisites:
  - transposition-tn
extends:
  - transposition-tn
related:
  - prime-form
contrasts_with:
  - transposition-tn
answers_questions:
  - "How does inversion work in set theory?"
  - "How do you identify TnI between two sets?"
---

# Quick Definition
Inversion (TnI) first inverts each pitch class around C (12 minus x), then transposes by n semitones.

# Core Definition
Inversion (TnI) is a compound operation. Step 1: invert each note below C using C as an axis (formula: 12 - x). Step 2: transpose by n semitones. Straus simplifies the notation to In. To identify n between two inversionally-related sets: write the second set backward, add corresponding elements; each sum equals n (Hutchinson, Ch. 33, pp. 489-490).

# Prerequisites
- **Transposition (Tn)** -- Inversion includes a transposition step

# Key Properties
1. Compound operation: invert then transpose
2. T0I: invert around C (12 - x for each element)
3. Then apply Tn
4. Sets related by TnI share the same prime form
5. To find n: second set backward + first set = n for all pairs

# Construction / Recognition
**Example:** [2, 4, 5] at T7I
1. Invert (T0I): [10, 8, 7]
2. Ascending: [7, 8, 10]
3. Transpose T7: [2, 3, 5]

**To identify n:** [2, 4, 5] and [2, 3, 5]
- Second backward: 5, 3, 2
- Add: 2+5=7, 4+3=7, 5+2=7 -> T7I confirmed

# Examples
- E (4) at T0I = Ab (8): 12-4 = 8 (p. 489)
- [2, 4, 5] at T7I = [2, 3, 5] (p. 489)

# Relationships
## Builds Upon
- **Transposition (Tn)** -- Inversion includes transposition
## Related
- **Prime form** -- Sets related by TnI share the same prime form
## Contrasts With
- **Transposition (Tn)** -- Tn preserves interval direction; TnI reverses it

# Common Errors
- **Error**: Inverting then forgetting to transpose
  **Correction**: TnI is a compound operation -- both steps are required

# Common Confusions
- **Confusion**: Confusing TnI with Tn
  **Clarification**: Tn only transposes; TnI inverts first, then transposes

# Source Reference
Chapter 33, Section 33.8, PDF pages 489-490.

# Verification Notes
- Definition and examples from source, pp. 489-490
- Identification method (backward addition) from Section 33.8.1
- Re-extracted from v2 card; preserved: backward addition method, Straus In notation
- Confidence: HIGH -- source provides explicit procedure with examples
