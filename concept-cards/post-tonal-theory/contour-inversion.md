---
concept: Contour Inversion
slug: contour-inversion
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
  - "CSEG inversion"
  - "contour I"
prerequisites:
  - contour-segment
extends:
  - contour-segment
related:
  - contour-retrograde
  - contour-retrograde-inversion
  - cseg-class
  - inversion
contrasts_with:
  - inversion
answers_questions:
  - "What is contour inversion?"
  - "How does contour inversion differ from pitch-class inversion?"
---

# Quick Definition
A contour transformation replacing each element x with (n-1) - x, where n is the number of distinct pitch levels; the highest note becomes the lowest and vice versa, creating a mirror image of the up-down pattern.

# Core Definition
Contour inversion transforms a CSEG by replacing each integer x with (n-1) - x, where n is the number of distinct pitch levels. This exchanges highest and lowest positions: what was (n-1) becomes 0, and vice versa. Contour-inverted CSEGs are mirror images of each other in terms of up-down motion. Corresponding elements always sum to (n-1). Unlike pitch-class inversion (which maps around a fixed axis mod 12), contour inversion simply flips the up-down pattern (Straus, pp. 144-145).

# Prerequisites
- **Contour segment** -- the object being transformed

# Key Properties
1. Formula: INV(x) = (n-1) - x for each element
2. Order is preserved; only values change
3. Corresponding elements sum to (n-1)
4. Where the original goes up, the inversion goes down
5. Applied twice, returns to the original

# Construction / Recognition
For CSEG <a, b, c, d> with max value m = n-1:
- INV = <m-a, m-b, m-c, m-d>
- Verification: add corresponding elements; all sums = m

Example: INV(<2013>) with n=4, m=3:
- <3-2, 3-0, 3-1, 3-3> = <1320>
- Check: 2+1=3, 0+3=3, 1+2=3, 3+0=3

# Context & Application
Contour inversion identifies melodic mirror images. When an ascending gesture is answered by a descending one of the same shape, contour inversion captures this relationship without requiring specific pitch matching.

# Examples
**Example 1** (p. 144, Ex. 3-37): Crawford Seeger -- <2013> inverts to <1320>. The original moves down-up-up; the inversion moves up-down-down.

**Example 2** (p. 146, Ex. 3-39): Stockhausen, Klavierstuck II -- upper melody <3201> and lower melody <0132> are contour inversions. The dynamics of the upper melody also follow <3201>.

# Relationships
## Builds Upon
- **Contour segment** -- the objects being transformed

## Related
- **Contour retrograde** -- independent operation (reverses order, not values)
- **CSEG-class** -- contour I is one of the classifying operations

## Contrasts With
- **Inversion (In)** -- pitch-class inversion operates mod 12; contour inversion operates relative to (n-1)

# Common Errors
- Thinking contour inversion changes the order (only values change)
- Using 12 or some other base instead of (n-1)

# Common Confusions
- Contour inversion is domain-specific: it flips relative heights, not pitch classes
- Inversion and retrograde are independent and commute: R(I(x)) = I(R(x))

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.10.2, pp. 144-145

# Verification Notes
Upgraded from old v2 card. Preserved Crawford Seeger and Stockhausen examples. Added v3 template fields.
