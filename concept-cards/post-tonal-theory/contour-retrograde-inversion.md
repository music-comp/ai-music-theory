---
concept: Contour Retrograde-Inversion
slug: contour-retrograde-inversion
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
  - "CSEG retrograde-inversion"
  - "contour RI"
prerequisites:
  - contour-inversion
  - contour-retrograde
extends:
  - contour-inversion
  - contour-retrograde
related:
  - cseg-class
contrasts_with: []
answers_questions:
  - "What is contour retrograde-inversion?"
  - "How does RI work for contour segments?"
---

# Quick Definition
A contour transformation combining retrograde and inversion: the CSEG is both reversed in order and complemented in value, producing a shape that is the mirror image played backward.

# Core Definition
Contour retrograde-inversion (RI) applies both contour inversion and retrograde to a CSEG. The order of application does not matter: R(I(CSEG)) = I(R(CSEG)). For a CSEG <a, b, c, d> with n elements and max value m = n-1: RI = <m-d, m-c, m-b, m-a>. RI-related CSEGs share neither directional pattern (like I) nor temporal unfolding (like R), yet they belong to the same CSEG-class (Straus, pp. 144-145).

# Prerequisites
- **Contour inversion** -- one component operation
- **Contour retrograde** -- the other component operation

# Key Properties
1. RI = R composed with I = I composed with R (they commute)
2. RI-related CSEGs are the "most distant" relatives within a CSEG-class
3. Applied twice, returns to the original
4. RI is essential for determining complete CSEG-class membership

# Construction / Recognition
For CSEG <x1, x2, ..., xn> with max m = n-1:
- Method 1: Invert first -> then retrograde
- Method 2: Retrograde first -> then invert
- RI = <m-xn, m-x(n-1), ..., m-x2, m-x1>

Example: RI(<2013>) with m=3:
- Invert: <1320>; then Retrograde: <0231>
- Or: Retrograde: <3102>; then Invert: <0231>
- RI(<2013>) = <0231>

# Context & Application
RI appears in music with complex motivic transformations and is needed to identify complete CSEG-class membership. When a CSEG and its RI both appear, they represent the maximum transformation within the same contour equivalence class.

# Examples
**Example 1** (p. 144, Ex. 3-37): Crawford Seeger, String Quartet -- the four members of one CSEG-class:
- P: <2013>
- I: <1320>
- R: <3102>
- RI: <0231>

<0231> is the prime form (starts on 0). Both <2013> and <0231> appear in Crawford's melody, showing use of RI-related contours.

# Relationships
## Builds Upon
- **Contour inversion** and **Contour retrograde** -- the component operations

## Related
- **CSEG-class** -- RI completes the classification

# Common Errors
- Thinking RI is a different kind of operation (it is simply R and I combined)
- Getting the order wrong (but R and I commute, so it does not matter)

# Common Confusions
- Contour RI is analogous to but distinct from pitch-class RI
- RI-related CSEGs are the "most distant" within their class

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.10.2, pp. 144-145

# Verification Notes
Upgraded from old v2 card. Preserved Crawford Seeger example and commutativity proof. Added v3 template fields.
