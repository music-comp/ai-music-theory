---
concept: Inverting a Pitch-Class Set
slug: inverting-a-set
category: operations
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
  - set inversion procedure
prerequisites:
  - inversion
  - normal-form
  - index-number-sum
extends:
  - inversion
related:
  - transposing-a-set
  - inversional-equivalence
  - mapping
contrasts_with:
  - transposing-a-set
answers_questions:
  - "How do I invert a pitch-class set?"
  - "What does the result look like in normal form?"
---

# Quick Definition
To invert a set at In, subtract each element from the index number n (mod 12), then put the result in normal form. The resulting normal form will be a mirror image of the original.

# Core Definition
"To invert a set, simply invert each member of the set in turn" (Straus, Ch. 2). For In[a, b, c, ...], compute (n - a), (n - b), (n - c), ... all mod 12, then arrange the result in normal form. A key visual property: "the normal forms of the inversionally related sets are mirror images of each other: the first note of one set corresponds to the last note of the other, the second note of one set corresponds to the second-to-last note of the other, and so on."

# Prerequisites
- **Inversion (In)** -- the operation being performed
- **Normal form** -- result must be put in normal form
- **Index number (sum)** -- the n from which each element is subtracted

# Key Properties
1. Subtract each element from n (mod 12)
2. Result must be put in normal form (unlike transposition, where normal form is automatically preserved)
3. Normal forms of original and inverted sets are mirror images
4. First <-> last, second <-> second-to-last correspondence
5. All corresponding pairs sum to n

# Construction / Recognition
**Procedure:**
1. Start with a set (in normal form)
2. Subtract each element from the index number n (mod 12)
3. Put the result in normal form

**Example**: I5[1, 3, 4, 7]
- 5 - 1 = 4
- 5 - 3 = 2
- 5 - 4 = 1
- 5 - 7 = -2 = 10 (mod 12)
- Raw result: {4, 2, 1, 10}
- Normal form: **[10, 1, 2, 4]**

**Verification**: Normal forms are mirror images:
- Original [1, 3, 4, 7]: intervals 2-1-3
- Result [10, 1, 2, 4]: intervals 3-1-2 (mirror)
- Correspondences: 1+4=5, 3+2=5, 4+1=5, 7+10=5 (mod 12). All sum to 5. Confirmed I5.

# Context & Application
This procedure is used constantly in analysis to compute inversions and verify inversional relationships. The mirror-image property provides a quick visual check: if normal forms are not mirror images, the sets are not inversionally related.

# Examples
**Example 2-23** (p. 74):
- I5[1, 3, 4, 7] = [10, 1, 2, 4]
- I4[11, 1, 2, 6] = [10, 2, 3, 5]

For the first: 5-1=4, 5-3=2, 5-4=1, 5-7=10. Normal form: [10, 1, 2, 4].
For the second: 4-11=5, 4-1=3, 4-2=2, 4-6=10. Normal form: [10, 2, 3, 5].

# Relationships
## Builds Upon
- **Inversion (In)** -- the operation being applied
- **Normal form** -- the result format
- **Index number (sum)** -- the parameter n
## Enables
- **Inversional equivalence** -- verified by performing the inversion
## Related
- **Mapping** -- inversion creates specific element-to-element correspondences
## Contrasts With
- **Transposing a set** -- transposition adds n; inversion subtracts from n. Transposition preserves normal form automatically; inversion may require reordering.

# Common Errors
- **Error**: Adding n instead of subtracting from n. **Correction**: In(x) = n - x, not n + x.
- **Error**: Forgetting to put the result in normal form. **Correction**: Unlike transposition, inversion does not automatically preserve normal form; you must reorder.

# Common Confusions
- **Confusion**: Why does inversion not preserve normal form automatically? **Clarification**: Transposition shifts all elements by the same amount, preserving their relative spacing. Inversion reverses the spacing, so the elements may need reordering.

# Source Reference
Chapter 2: Pitch-Class Sets, Section 2.4.4, pages 74--75.

# Verification Notes
- Definition source: direct from source (Example 2-23)
- Confidence rationale: explicit procedure with two worked examples
- Re-extraction notes: new card; extracted from section 2.4.4 with Example 2-23
