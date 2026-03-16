---
concept: Common Tone Theorem for Transposition
slug: common-tone-theorem-for-transposition
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
  - "common tone theorem (transposition)"
  - "Tn common tone theorem"
prerequisites:
  - transposition
  - interval-class-vector
  - common-tones-under-transposition
extends:
  - common-tones-under-transposition
related:
  - common-tones-under-inversion
  - transpositional-symmetry
contrasts_with:
  - common-tones-under-inversion
answers_questions:
  - "How can I predict the number of common tones without computing the transposition?"
  - "What is the common tone theorem for transposition?"
---

# Quick Definition
The theorem stating that when a set is transposed at Tn, the number of common tones equals the number of occurrences of interval-class n in the set, except at T6 where it equals twice the number of tritones.

# Core Definition
The Common Tone Theorem for Transposition formalizes the relationship between a set's interval content and its behavior under transposition. For any pitch-class set S transposed at Tn, the count of pitch classes held in common between S and Tn(S) equals the number of times interval-class n appears in the interval-class vector of S. The sole exception is T6, where the count equals twice the number of ic6 occurrences, because the tritone maps onto itself. The theorem allows prediction of common-tone behavior directly from the interval-class vector without calculating actual transpositions (Straus, p. 116).

# Prerequisites
- **Transposition (Tn)** -- the operation whose common-tone behavior is predicted
- **Interval-class vector** -- the data structure from which predictions are read
- **Common tones under transposition** -- the phenomenon the theorem formalizes

# Key Properties
1. Common tones at T1 or T11 = ic1 count
2. Common tones at T2 or T10 = ic2 count
3. Common tones at T3 or T9 = ic3 count
4. Common tones at T4 or T8 = ic4 count
5. Common tones at T5 or T7 = ic5 count
6. Common tones at T6 = 2 * ic6 count
7. Common tones at T0 = cardinality of S

# Construction / Recognition
Given a set with interval-class vector [a, b, c, d, e, f]:
- Read each entry to determine common tones at the corresponding transposition level
- For T6, double the ic6 entry
- Results hold for all members of the set class

**Example**: sc(0134) has vector [212100]
- T1/T11: 2 common tones
- T2/T10: 1 common tone
- T3/T9: 2 common tones
- T4/T8: 1 common tone
- T5/T7: 0 common tones
- T6: 0 common tones

# Context & Application
The theorem is essential for predicting voice-leading smoothness between transposed set forms, understanding key relationships in tonal music, and identifying transpositionally symmetrical sets (when common tones equal set cardinality at some Tn other than T0).

# Examples
**Example 1** (p. 115): The major scale vector [254361] predicts:
- T7: 6 common tones (dominant = closely related key)
- T1: 2 common tones (key of leading tone = remote)
- T6: 2 common tones (1 tritone, doubled)

**Example 2** (p. 116): The whole-tone scale vector [060603]:
- Even transpositions: 6 common tones (complete duplication)
- Odd transpositions: 0 common tones (no overlap)

# Relationships
## Builds Upon
- **Common tones under transposition** -- the theorem formalizes this concept
- **Interval-class vector** -- provides the data for the theorem

## Enables
- **Transpositional symmetry** -- identified when common tones equal cardinality
- **Unique multiplicity of interval class** -- hierarchy property readable from the theorem

## Contrasts With
- **Common tones under inversion** -- uses addition (sums) rather than the interval-class vector

# Common Errors
- Forgetting that T6 is the exception (multiply ic6 count by 2)
- Assuming the theorem identifies which specific pitch classes are common (it only gives the count)

# Common Confusions
- Complementary transposition levels (Tn and T12-n) always produce the same count, since they correspond to the same interval class
- The theorem describes a property of set classes, not just individual sets

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.1, pp. 111-116 ("In Brief" summary on p. 116)

# Verification Notes
Upgraded from old v2 card. Preserved all mathematical formulations and examples. Added explicit theorem statement format and taxonomy fields per v3 template.
