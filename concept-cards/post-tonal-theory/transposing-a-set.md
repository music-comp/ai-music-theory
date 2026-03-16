---
concept: Transposing a Pitch-Class Set
slug: transposing-a-set
category: operations
subcategory: null
tier: intermediate
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Pitch-Class Sets"
chapter_number: 2
pdf_page: 59
section: "2.3.5 Transposing a pitch-class set"
extraction_confidence: high
aliases:
  - set transposition procedure
prerequisites:
  - transposition
  - normal-form
extends:
  - transposition
related:
  - inverting-a-set
  - mapping
contrasts_with:
  - inverting-a-set
answers_questions:
  - "How do I transpose a pitch-class set?"
  - "Does transposition preserve normal form?"
---

# Quick Definition
To transpose a set, add the transposition interval n to each member (mod 12). If the original is in normal form, the result will also be in normal form.

# Core Definition
"To transpose a set of pitch classes, simply add a single pitch-class interval to each member of the set" (Straus, Ch. 2). The result is a new set at a different pitch level. A key property: "If the first set was in normal form, its transposition will also be in normal form." We say either "[Y] is Tn of [X]" or "Tn maps [X] onto [Y]."

# Prerequisites
- **Transposition (Tn)** -- the operation being performed
- **Normal form** -- input and output are typically in normal form

# Key Properties
1. Add n to each element (mod 12)
2. Normal form is preserved: if input is in normal form, output is too
3. Each element has exactly one image (one-to-one mapping)
4. Interval succession is preserved

# Construction / Recognition
**Procedure:**
1. Start with a set (preferably in normal form)
2. Add n to each element, reducing mod 12
3. The result is the transposed set (already in normal form if the original was)

**Example**: T8[5, 7, 8, 11]
- 5 + 8 = 13 = 1
- 7 + 8 = 15 = 3
- 8 + 8 = 16 = 4
- 11 + 8 = 19 = 7
- Result: [1, 3, 4, 7]

# Context & Application
This is the mechanical procedure for transposition. While simple, it is used constantly in analysis -- computing transpositions to verify relationships, generating all members of a Tn-type, and finding the specific Tn connecting two sets.

# Examples
**Example 2-9** (p. 66): T8[5, 7, 8, 11] = [1, 3, 4, 7]. "We would say either '[1, 3, 4, 7] is T8 of [5, 7, 8, 11]' or 'T8 maps [5, 7, 8, 11] onto [1, 3, 4, 7].'"

# Relationships
## Builds Upon
- **Transposition (Tn)** -- this is the mechanical application
- **Normal form** -- preserved under transposition
## Enables
- **Transpositional equivalence** -- verified by performing the transposition
## Related
- **Mapping** -- transposition creates element-to-element correspondences
## Contrasts With
- **Inverting a set** -- inversion subtracts from n rather than adding n

# Common Errors
- **Error**: Forgetting mod 12. **Correction**: All results must be reduced mod 12. E.g., 11 + 8 = 19 = 7 (mod 12).

# Common Confusions
- **Confusion**: Whether the result needs reordering. **Clarification**: If the input was in normal form, the output is automatically in normal form -- no reordering needed.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.3.5, pages 65--66.

# Verification Notes
- Definition source: direct from source
- Confidence rationale: explicit procedure with worked example
- Re-extraction notes: new card; extracted from section 2.3.5
