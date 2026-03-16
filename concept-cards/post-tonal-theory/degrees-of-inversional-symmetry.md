---
concept: Degrees of Inversional Symmetry
slug: degrees-of-inversional-symmetry
category: set-theory
subcategory: symmetry
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 126
section: "3.4.3 Degrees of inversional symmetry"
extraction_confidence: high
aliases:
  - "degree of In-symmetry"
prerequisites:
  - inversional-symmetry
  - addition-table
extends:
  - inversional-symmetry
related:
  - degrees-of-transpositional-symmetry
  - symmetry-and-set-class-size
contrasts_with: []
answers_questions:
  - "How many inversion levels map a set onto itself?"
  - "How do I read the degree of inversional symmetry from the List of Set Classes?"
---

# Quick Definition
The count of distinct inversion levels at which a set class maps onto itself; listed as the second number in the (n, m) degree-of-symmetry notation in the List of Set Classes.

# Core Definition
The degree of inversional symmetry counts the distinct index numbers at which any member of a set class maps entirely onto itself under inversion. This appears as the second value in the ordered pair (n, m) in the List of Set Classes. Many set classes have degree 0 (not inversionally symmetrical). Some map onto themselves at one level (degree 1), and a small number at multiple levels. Only 11 set classes have degree greater than 1 (Straus, pp. 126-128).

# Prerequisites
- **Inversional symmetry** -- the property being measured
- **Addition table** -- used to determine the degree

# Key Properties
1. Possible degrees: 0, 1, 2, 3, 4, or 6
2. Degree 0: not inversionally symmetrical (most common)
3. Degree 1: inversionally symmetrical at one level (reasonably common -- 79 set classes)
4. Degree > 1: rare (only 11 set classes)
5. Virtually all sets with high Tn-symmetry also have high In-symmetry

# Construction / Recognition
The 11 set classes with degree > 1 (from Ex. 3-18):

| Set Class | Name | In-levels (re: prime form) | Degree |
|-----------|------|----------------------------|--------|
| 3-12 (048) | Augmented triad | I0, I4, I8 | 3 |
| 4-9 (0167) | -- | I1, I7 | 2 |
| 4-25 (0268) | French aug. 6th | I2, I8 | 2 |
| 4-28 (0369) | Dim. 7th chord | I0, I3, I6, I9 | 4 |
| 6-7 (012678) | -- | I2, I8 | 2 |
| 6-20 (014589) | Hexatonic scale | I1, I5, I9 | 3 |
| 6-35 (02468T) | Whole-tone scale | I0, I2, I4, I6, I8, I10 | 6 |
| 8-9 (01236789) | -- | I3, I9 | 2 |
| 8-25 (0124678T) | -- | I2, I8 | 2 |
| 8-28 (0134679T) | Octatonic scale | I1, I4, I7, I10 | 4 |
| 9-12 (01245689T) | Enneatonic | I2, I6, I10 | 3 |

# Context & Application
Sc(024), for example, has degree (1, 1): it maps onto itself at one transpositional level (T0) and one inversional level (I4). The whole-tone scale has the highest degree of symmetry of all: (6, 6), mapping onto itself at six transpositional and six inversional levels.

The comparison between Ex. 3-7 and Ex. 3-18 reveals that virtually all Tn-symmetrical sets are also highly In-symmetrical. The sole exception is (013679), which has degree (2, 0) -- transpositionally symmetrical at T6 but not inversionally symmetrical at more than one level.

# Examples
**Example 1** (p. 127, Ex. 3-18): The whole-tone scale (02468T) with degree (6, 6) is the most symmetrical set of all.

**Example 2**: The diminished seventh chord (0369) with degree (4, 4) maps onto itself at I0, I3, I6, I9 -- four inversional axes.

# Relationships
## Builds Upon
- **Inversional symmetry** -- this quantifies that property

## Enables
- **Symmetry and set class size** -- degree determines class size along with Tn-symmetry

## Related
- **Degrees of transpositional symmetry** -- the complementary measure in the (n, m) pair

# Common Errors
- Confusing degree 0 (not In-symmetrical) with degree 1 (In-symmetrical at one level)

# Common Confusions
- (013679) is the only set class that is Tn-symmetrical but not In-symmetrical at multiple levels
- Every set class with In-degree > 1 is also Tn-symmetrical

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.4.3, pp. 126-128

# Verification Notes
Upgraded from old v2 card. Preserved complete table of 11 set classes, whole-tone/diminished examples, and (013679) exception. Added v3 template fields.
