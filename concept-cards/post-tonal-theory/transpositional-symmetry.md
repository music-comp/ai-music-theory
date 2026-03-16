---
concept: Transpositional Symmetry
slug: transpositional-symmetry
category: set-theory
subcategory: symmetry
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 116
section: "3.2 Transpositional Symmetry"
extraction_confidence: high
aliases:
  - "Tn-symmetry"
  - "rotational symmetry"
prerequisites:
  - transposition
  - interval-class-vector
  - common-tones-under-transposition
extends:
  - common-tone-theorem-for-transposition
related:
  - degrees-of-transpositional-symmetry
  - inversional-symmetry
  - modes-of-limited-transposition
  - symmetry-and-set-class-size
contrasts_with:
  - unique-multiplicity-of-interval-class
answers_questions:
  - "What is transpositional symmetry?"
  - "Which sets map onto themselves under transposition?"
  - "How do I detect transpositional symmetry from the interval-class vector?"
---

# Quick Definition
A property of sets that map entirely onto themselves under transposition at some level other than T0; equivalently, sets with rotational symmetry on the pitch-class clockface.

# Core Definition
A set is transpositionally symmetrical if it maps entirely onto itself under some transposition Tn where n is not 0. This occurs when the interval-class vector contains an entry equal to the number of notes in the set (or half that number for ic6). Such sets have fewer than twelve distinct transpositions, as some transpositions reproduce the same pitch-class content. Mathematicians call this property rotational symmetry (Straus, pp. 116-119).

# Prerequisites
- **Transposition (Tn)** -- the operation under which the set is invariant
- **Interval-class vector** -- diagnostic tool for detecting the property
- **Common tones under transposition** -- transpositional symmetry is the extreme case where all tones are common

# Key Properties
1. A set of cardinality c is Tn-symmetrical when ic-n count = c (or ic6 count = c/2)
2. Every set is trivially T0-symmetrical
3. Transpositionally symmetrical sets can be rotated on the clockface to fit back onto themselves
4. Only 14 set classes (excluding the aggregate and null set) have this property
5. The levels of symmetry always divide evenly into 12

# Construction / Recognition
To detect transpositional symmetry:
1. Compute the interval-class vector
2. Check if any entry equals the set's cardinality (for ic1-5) or half the cardinality (for ic6)
3. On the clockface: does rotating by n positions map the set onto itself?

The 14 Tn-symmetrical set classes (from Ex. 3-7):

| Set Class | Familiar Name | Levels | Degree |
|-----------|---------------|--------|--------|
| 2-6 (06) | Tritone | T0, T6 | 2 |
| 3-12 (048) | Augmented triad | T0, T4, T8 | 3 |
| 4-9 (0167) | -- | T0, T6 | 2 |
| 4-25 (0268) | French aug. 6th | T0, T6 | 2 |
| 4-28 (0369) | Dim. 7th chord | T0, T3, T6, T9 | 4 |
| 6-7 (012678) | -- (Mode 5) | T0, T6 | 2 |
| 6-30 (013679) | Petrushka chord | T0, T6 | 2 |
| 6-20 (014589) | Hexatonic scale | T0, T4, T8 | 3 |
| 6-35 (02468T) | Whole-tone (Mode 1) | T0, T2, T4, T6, T8, T10 | 6 |
| 8-9 (01236789) | -- (Mode 4) | T0, T6 | 2 |
| 8-25 (0124678T) | -- (Mode 6) | T0, T6 | 2 |
| 8-28 (0134679T) | Octatonic (Mode 2) | T0, T3, T6, T9 | 4 |
| 9-12 (01245689T) | Enneatonic (Mode 3) | T0, T4, T8 | 3 |
| 10-6 (012346789T) | -- (Mode 7) | T0, T6 | 2 |

# Context & Application
Transpositionally symmetrical sets have attracted composers including Messiaen, who identified seven "Modes of Limited Transposition." Their symmetry creates distinctive harmonic colors and a static, non-tonal quality. The most familiar examples include the augmented triad, diminished seventh chord, whole-tone scale, and octatonic scale.

# Examples
**Example 1** (p. 118, Ex. 3-8): Bartok, Suite for Piano, op. 14, Scherzo -- augmented triads (048) at T4 produce the same pitch classes despite the pitch level changing. The second phrase begins four semitones higher than the first, but since (048) maps onto itself at T4, the pitch-class content is identical.

**Example 2** (p. 119, Ex. 3-9): [0, 2, 6, 8] on the clockface -- rotating by 6 semitones maps [0, 2] onto [6, 8] and [6, 8] onto [0, 2], demonstrating T6 symmetry as rotational symmetry.

# Relationships
## Builds Upon
- **Common tone theorem for transposition** -- symmetry is the maximal case

## Enables
- **Symmetry and set class size** -- symmetry reduces the number of distinct sets in the class
- **Modes of limited transposition** -- Messiaen's compositional application

## Related
- **Inversional symmetry** -- virtually all Tn-symmetrical sets are also In-symmetrical

## Contrasts With
- **Unique multiplicity of interval class** -- graduated hierarchy vs. symmetrical redundancy

# Common Errors
- Forgetting that T0 symmetry is trivial (all sets have it)
- Confusing transpositional symmetry with inversional symmetry

# Common Confusions
- The tritone exception applies here too: for ic6, the threshold is half the cardinality
- (013679) is the only set class that is Tn-symmetrical but not In-symmetrical at more than one level

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.2, pp. 116-119

# Verification Notes
Upgraded from old v2 card. Preserved complete table of 14 Tn-symmetrical set classes, Bartok example, and Messiaen modes connection. Added clockface/rotational symmetry discussion and v3 fields.
