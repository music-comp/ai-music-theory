---
concept: Unique Multiplicity of Interval Class
slug: unique-multiplicity-of-interval-class
category: set-theory
subcategory: transposition properties
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 114
section: "3.1.2 Some special set classes (major scale and whole-tone scale)"
extraction_confidence: high
aliases:
  - "unique multiplicity"
  - "graduated interval content"
prerequisites:
  - interval-class-vector
  - common-tone-theorem-for-transposition
extends:
  - common-tone-theorem-for-transposition
related:
  - transpositional-symmetry
contrasts_with:
  - transpositional-symmetry
answers_questions:
  - "Why does the major scale create a hierarchy of closely and distantly related keys?"
  - "What is unique multiplicity of interval class?"
---

# Quick Definition
A property of the major scale (and its set class) where each interval class occurs a different number of times, producing a graduated continuum of common-tone relationships at different transposition levels and enabling the hierarchy of key relationships in tonal music.

# Core Definition
Unique multiplicity of interval class describes an interval-class vector in which each entry is distinct (with the minor exception that ic1 and ic6 produce the same common-tone count due to the tritone exception). The major scale, sc(013568T) with vector [254361], exemplifies this property: it has 2 ic1s, 5 ic2s, 4 ic3s, 3 ic4s, 6 ic5s, and 1 ic6. Because each interval class has a different multiplicity, transposition at each level yields a different number of common tones, creating a graduated hierarchy from closely related keys (T7: 6 common tones) to distantly related keys (T1: 2 common tones) (Straus, pp. 114-115).

# Prerequisites
- **Interval-class vector** -- the data structure exhibiting unique multiplicity
- **Common tone theorem for transposition** -- connects interval counts to common-tone behavior

# Key Properties
1. The major scale vector [254361] has all different entries
2. This produces a different common-tone count at (almost) every transposition level
3. T5/T7: 6 common tones (closest relationship -- dominant/subdominant)
4. T2/T10: 5 common tones
5. T3/T9: 4 common tones
6. T4/T8: 3 common tones
7. T1/T11: 2 common tones; T6: 2 common tones (the one overlap, due to tritone exception)
8. This graduated continuum is the foundation of the circle of fifths

# Construction / Recognition
To check for unique multiplicity:
1. Compute the interval-class vector
2. Verify that all six entries are distinct
3. Note: T1/T11 and T6 may coincide due to the tritone doubling rule

Contrast with whole-tone scale [060603]: entries are 0, 6, 0, 6, 0, 3 -- no gradation, only extreme values.

# Context & Application
Unique multiplicity explains why the major/minor scale system supports hierarchical key relationships: modulation to the dominant (T7) retains 6 of 7 pitch classes, while modulation by semitone retains only 2. The whole-tone scale, lacking this property, cannot support graduated key relationships -- it offers only complete overlap or complete contrast.

# Examples
**Example 1** (p. 115, Ex. 3-5): Major scale common tones at each transposition:
- T7 (to dominant): 6 common tones -- closest relationship
- T2: 5 common tones
- T3: 4 common tones
- T4: 3 common tones
- T1 or T6: 2 common tones -- most distant relationships

**Example 2** (p. 116, Ex. 3-6): Whole-tone scale [060603]:
- Even transpositions (T2, T4, T6, T8, T10): 6 common tones (complete duplication)
- Odd transpositions (T1, T3, T5, T7, T9, T11): 0 common tones
- Stark either/or, no hierarchy possible

# Relationships
## Builds Upon
- **Common tone theorem for transposition** -- unique multiplicity is visible through this theorem

## Enables
- Understanding of tonal key hierarchies and circle of fifths

## Contrasts With
- **Transpositional symmetry** -- symmetrical sets lack unique multiplicity; they have redundant interval counts

# Common Errors
- Overlooking that T1/T11 and T6 both yield 2 common tones (the one exception to strict uniqueness, due to the tritone rule)
- Confusing unique multiplicity with transpositional symmetry (they are nearly opposite properties)

# Common Confusions
- The property is not unique to the major scale -- other set classes could theoretically have it, though the major scale is the most musically significant example
- The whole-tone scale is the extreme counterexample: maximum redundancy in interval counts

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.1.2, pp. 114-116

# Verification Notes
Upgraded from old v2 card. Preserved all content including major scale analysis, whole-tone contrast, and circle-of-fifths connection. Added v3 taxonomy and structural fields.
