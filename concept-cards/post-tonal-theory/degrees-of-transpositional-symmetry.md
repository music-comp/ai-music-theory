---
concept: Degrees of Transpositional Symmetry
slug: degrees-of-transpositional-symmetry
category: set-theory
subcategory: symmetry
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 119
section: "3.2.4 Degrees of transpositional symmetry"
extraction_confidence: high
aliases:
  - "degree of Tn-symmetry"
prerequisites:
  - transpositional-symmetry
  - interval-class-vector
extends:
  - transpositional-symmetry
related:
  - degrees-of-inversional-symmetry
  - symmetry-and-set-class-size
contrasts_with: []
answers_questions:
  - "How many transposition levels map a set onto itself?"
  - "How do I read the degree of transpositional symmetry from the List of Set Classes?"
---

# Quick Definition
The count of distinct transposition levels (including T0) at which a set class maps onto itself; listed as the first number in the (n, m) degree-of-symmetry notation in the List of Set Classes.

# Core Definition
The degree of transpositional symmetry measures how many distinct values of n allow Tn to map a set onto itself. In the List of Set Classes, this appears as the first number in the ordered pair (n, m), where n = degree of Tn-symmetry and m = degree of In-symmetry. Every set class has degree at least 1 (T0). The 14 set classes with degree greater than 1 are the truly transpositionally symmetrical ones (Straus, p. 119).

# Prerequisites
- **Transpositional symmetry** -- the property being measured
- **Interval-class vector** -- used to determine the degree

# Key Properties
1. The degree always divides evenly into 12: possible values are 1, 2, 3, 4, 6, 12
2. Degree 1: only T0 (most set classes)
3. Degree 2: T0, T6
4. Degree 3: T0, T4, T8
5. Degree 4: T0, T3, T6, T9
6. Degree 6: T0, T2, T4, T6, T8, T10
7. The degree determines set class size: 24 / (total self-mapping operations)

# Construction / Recognition
To determine the degree:
1. Check the interval-class vector
2. Count entries equal to the set's cardinality (or half for ic6)
3. Add 1 for T0
4. Alternatively, read the first number from the (n, m) notation in the List of Set Classes

# Context & Application
The degree directly determines how many distinct transpositions of the set exist (12 / degree = number of distinct transpositions). Messiaen classified his "Modes of Limited Transposition" by this property, finding that their limited number of transpositions creates unique harmonic atmospheres.

# Examples
**Example 1** (p. 119): Representative degrees:

| Set Class | Name | Degree | Distinct Forms |
|-----------|------|--------|----------------|
| (06) | Tritone | 2 | 6 |
| (048) | Augmented triad | 3 | 4 |
| (0369) | Diminished 7th | 4 | 3 |
| (02468T) | Whole-tone | 6 | 2 |

**Example 2**: Most set classes (e.g., (0137), (0258)) have degree 1 -- they map onto themselves only at T0 and have 24 distinct members (before In-symmetry reduction).

# Relationships
## Builds Upon
- **Transpositional symmetry** -- this quantifies that property

## Enables
- **Symmetry and set class size** -- degree directly determines class size

## Related
- **Degrees of inversional symmetry** -- the complementary measure in the (n, m) pair

# Common Errors
- Forgetting to count T0 (every set has degree at least 1)
- Conflating degree of Tn-symmetry with degree of In-symmetry

# Common Confusions
- High degree does not mean "better" or "more useful" -- it means fewer distinct transpositions and a different compositional character
- The degree always divides 12, which explains why the possible values are limited

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.2.4, p. 119

# Verification Notes
Upgraded from old v2 card. Preserved all degree tables and formulas. Added distinct forms column and v3 template structure.
