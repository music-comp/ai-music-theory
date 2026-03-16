---
concept: CSEG-Class
slug: cseg-class
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
  - "contour class"
  - "CSEG equivalence class"
prerequisites:
  - contour-segment
  - contour-inversion
  - contour-retrograde
extends:
  - contour-segment
related:
  - contour-retrograde-inversion
  - set-class
contrasts_with: []
answers_questions:
  - "What is a CSEG-class?"
  - "How are CSEGs grouped into equivalence classes?"
---

# Quick Definition
A group of CSEGs related by contour inversion (I), retrograde (R), or retrograde-inversion (RI); analogous to set classes, CSEG-classes group contours that are transformationally equivalent.

# Core Definition
A CSEG-class is the collection of all CSEGs related by contour inversion, retrograde, or retrograde-inversion. Just as pitch-class sets related by Tn or In belong to the same set class, CSEGs related by contour I, R, or RI belong to the same CSEG-class. Each class contains up to four members. The prime form is the member beginning on the lowest note (0). Corresponding elements in inversionally related CSEGs sum to (n-1) (Straus, pp. 144-146).

# Prerequisites
- **Contour segment** -- the objects being classified
- **Contour inversion** -- one of the grouping operations
- **Contour retrograde** -- another grouping operation

# Key Properties
1. Up to 4 members per class (P, I, R, RI)
2. Some classes have fewer members due to symmetry
3. Prime form starts on 0
4. CSEG-classes for n=3: 2 classes; n=4: 8 classes
5. Numbers grow rapidly with segment length
6. Analogous to set classes but for contour

# Construction / Recognition
Contour operations for CSEG <a, b, c, d> with max value m = n-1:
- Inversion (I): <m-a, m-b, m-c, m-d>
- Retrograde (R): <d, c, b, a>
- Retrograde-Inversion (RI): <m-d, m-c, m-b, m-a>

Prime form: select the member starting on 0.

Complete list for 3 and 4 notes (Ex. 3-38):
| Name | Prime form |
|------|------------|
| 3-1 | <012> |
| 3-2 | <021> |
| 4-1 | <0123> |
| 4-2 | <0132> |
| 4-3 | <0213> |
| 4-4 | <0231> |
| 4-5 | <0312> |
| 4-6 | <0321> |
| 4-7 | <1032> |
| 4-8 | <1302> |

# Context & Application
CSEG-classes allow grouping related melodic shapes regardless of inversion or direction. They are useful for analyzing variations where melodic shape is preserved under transformation, and for identifying coherence in music that uses diverse pitch materials.

# Examples
**Example 1** (p. 144, Ex. 3-37): Crawford Seeger, String Quartet -- the four members of one CSEG-class:
- <2013> (original, appears 3 times)
- <1320> (inversion)
- <0231> (retrograde-inversion)
- <3102> (retrograde)
- Prime form: <0231> (starts on 0)

Crawford's melody reshapes this basic contour through multiple transformations.

# Relationships
## Builds Upon
- **Contour segment** -- the objects being classified

## Related
- **Set class** -- analogous concept for pitch-class sets
- **Contour retrograde-inversion** -- one of the classifying operations

# Common Errors
- Confusing contour inversion with pitch-class inversion (different domains)
- Thinking all CSEG-classes have exactly 4 members (some have fewer due to symmetry)

# Common Confusions
- CSEG-class prime form is selected differently from pc set prime form (starts on 0, not smallest span)
- Retrograde reverses order; inversion complements values -- they are independent operations

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.10.2, pp. 144-146

# Verification Notes
Upgraded from old v2 card. Preserved Crawford Seeger example, complete 3/4-note CSEG-class table, and operation formulas. Added v3 template fields.
