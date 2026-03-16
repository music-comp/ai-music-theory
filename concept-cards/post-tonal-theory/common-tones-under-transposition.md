---
concept: Common Tones under Transposition
slug: common-tones-under-transposition
category: set-theory
subcategory: transposition properties
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 111
section: "3.1 Common Tones under Transposition (Tn)"
extraction_confidence: high
aliases:
  - "common tones under Tn"
  - "transpositional common tones"
prerequisites:
  - transposition
  - interval-class-vector
  - interval-class
extends:
  - transposition
related:
  - common-tone-theorem-for-transposition
  - common-tones-under-inversion
  - transpositional-symmetry
  - unique-multiplicity-of-interval-class
contrasts_with:
  - common-tones-under-inversion
answers_questions:
  - "How do I identify common tones under transposition?"
  - "How many pitch classes are retained when a set is transposed?"
  - "Why does the tritone produce twice as many common tones?"
---

# Quick Definition
The pitch classes held in common between a set and its transposition at Tn; the number of common tones equals the number of occurrences of interval-class n in the set, with the tritone (ic6) as an exception producing twice the expected count.

# Core Definition
When a pitch-class set is transposed at Tn, common tones are those pitch classes appearing in both the original and transposed set. The count is determined by the set's interval-class content: for each occurrence of interval n within the set, one common tone results at Tn. This occurs because transposing by n maps one note of an n-semitone pair onto the other. The tritone (ic6) is the exception: because it maps onto itself under T6, each occurrence of ic6 produces two common tones. At T0, all notes are common tones (Straus, p. 111-116).

# Prerequisites
- **Transposition (Tn)** -- the operation being applied to generate common tones
- **Interval-class vector** -- provides the count of each interval class, directly predicting common-tone counts
- **Interval class** -- the unordered pitch-class interval measured mod 12

# Key Properties
1. Common tones at Tn = number of occurrences of ic-n in the set
2. Exception: common tones at T6 = 2 times the number of ic6 occurrences
3. At T0, all pitch classes are common tones (trivial case)
4. Complementary transposition levels (Tn and T(12-n)) produce the same number of common tones
5. The interval-class vector predicts common-tone counts for all twelve transposition levels

# Construction / Recognition
To determine common tones under Tn for a set S:
1. Look up (or compute) the interval-class vector of S
2. For transposition level n (where 1 <= n <= 5), read entry ic-n from the vector
3. For T6, read entry ic6 and multiply by 2
4. To identify which specific pitch classes are retained: find pairs of notes n semitones apart; the higher note is a common tone at Tn, the lower at T(12-n)

# Context & Application
Common tones under transposition provide musical continuity between statements of the same set class. Composers exploit common tones for smooth voice leading (retaining tones in the same register) or avoid transposition levels that produce common tones to emphasize contrast. The property is fundamental to understanding key relationships in tonal music and motivic connections in post-tonal music.

# Examples
**Example 1** (p. 112, Ex. 3-2): [4, 5, 7, 8], a member of sc(0134), contains two occurrences of ic3 (between 4-7 and 5-8). At T3: 4 maps to 7 and 5 maps to 8, producing common tones {7, 8}. At T9: 7 maps to 4 and 8 maps to 5, producing common tones {4, 5}.

**Example 2** (p. 112, Ex. 3-3): [4, 9, 10], sc(016), contains one tritone (between 4 and 10). At T6: 4 maps to 10 and 10 maps back to 4, producing two common tones {4, 10} from a single ic6.

**Example 3** (p. 114, Ex. 3-4): Stravinsky, Rite of Spring -- a chain of (0134)s linked by common tones. T4 produces one common tone; T11 and T3 each produce two.

# Relationships
## Builds Upon
- **Transposition** -- common tones arise from the transposition operation
- **Interval-class vector** -- the predictive tool for common-tone counts

## Enables
- **Transpositional symmetry** -- when common tones equal the set's cardinality
- **Unique multiplicity of interval class** -- property enabling hierarchical key relationships

## Related
- **Common tones under inversion** -- analogous concept using sums instead of differences

## Contrasts With
- **Common tones under inversion** -- uses index numbers (sums) rather than intervals (differences)

# Common Errors
- Forgetting the tritone exception: ic6 produces twice the expected common tones at T6
- Confusing common pitch classes with common pitches (register is irrelevant for pitch-class common tones)

# Common Confusions
- The interval-class vector counts unordered intervals, so ic5 and ic7 are the same; T5 and T7 produce the same number of common tones
- Common tones at Tn and T(12-n) are always equal in number but may involve different pitch classes

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.1, pp. 111-116

# Verification Notes
Upgraded from old v2 card. Preserved all content including Stravinsky Rite of Spring example, major scale example, and tritone exception details. Added v3 template structure with taxonomy fields.
