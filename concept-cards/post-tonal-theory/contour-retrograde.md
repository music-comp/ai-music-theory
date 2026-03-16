---
concept: Contour Retrograde
slug: contour-retrograde
category: analysis
subcategory: contour
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 144
section: "3.10.2 CSEG-class"
extraction_confidence: high
aliases:
  - "CSEG retrograde"
  - "contour R"
prerequisites:
  - contour-segment
extends:
  - contour-segment
related:
  - contour-inversion
  - contour-retrograde-inversion
  - cseg-class
contrasts_with: []
answers_questions:
  - "What is contour retrograde?"
  - "How does reversing a CSEG work?"
---

# Quick Definition
A contour transformation reversing the temporal order of elements in a CSEG; the last element becomes first and the first becomes last, while the contour values remain unchanged.

# Core Definition
Contour retrograde transforms a CSEG by reversing the order of its elements. If the original is <a, b, c, d>, the retrograde is <d, c, b, a>. The actual contour values are unchanged; only their temporal sequence reverses. Retrograde-related CSEGs describe the same relative heights played in opposite temporal directions (Straus, pp. 144-145).

# Prerequisites
- **Contour segment** -- the object being transformed

# Key Properties
1. Formula: RET(<x1, x2, ..., xn>) = <xn, ..., x2, x1>
2. Values unchanged; only order reverses
3. Applied twice, returns to the original
4. A palindromic CSEG equals its own retrograde
5. Independent of contour inversion

# Construction / Recognition
Simply reverse the order of elements:
- RET(<2013>) = <3102>
- Verification: first of original = last of retrograde, and vice versa

# Context & Application
Contour retrograde identifies melodies played "backward" in shape. It is useful for analyzing palindromic structures and comparing beginnings and endings that mirror each other temporally. It is a more general concept than pitch retrograde, capturing shape relationships even when specific pitches differ.

# Examples
**Example 1** (p. 144, Ex. 3-37): Crawford Seeger -- <2013> and <3102> are retrograde-related. <0231> and <1320> are also retrogrades of each other.

# Relationships
## Builds Upon
- **Contour segment** -- the objects being transformed

## Related
- **Contour inversion** -- independent operation (changes values, not order)
- **Contour retrograde-inversion** -- combination of R and I
- **CSEG-class** -- R is one of the classifying operations

# Common Errors
- Confusing retrograde (order reversal) with inversion (value complementation)
- Thinking retrograde changes the contour values (it does not)

# Common Confusions
- R and I are independent operations that can be combined (RI)
- Contour retrograde applies to ordered segments, not unordered sets

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.10.2, pp. 144-145

# Verification Notes
Upgraded from old v2 card. Preserved Crawford Seeger example. Tightened definition and added v3 template fields.
