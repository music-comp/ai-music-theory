---
concept: Tritone Exception
slug: tritone-exception
category: set-theory
subcategory: transposition properties
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 112
section: "3.1.1 Interval-class content"
extraction_confidence: high
aliases:
  - "T6 exception"
  - "tritone common-tone exception"
prerequisites:
  - interval-class
  - common-tones-under-transposition
extends:
  - common-tone-theorem-for-transposition
related:
  - transpositional-symmetry
contrasts_with: []
answers_questions:
  - "Why does the tritone produce twice as many common tones as expected?"
  - "What is special about T6 transposition?"
---

# Quick Definition
The special case in the common tone theorem where transposition at T6 produces twice the number of common tones predicted by the ic6 count, because the tritone maps onto itself under T6.

# Core Definition
When a set is transposed at T6, each occurrence of interval-class 6 in the set produces two common tones rather than one. This is because the tritone is the only interval class that maps onto itself under transposition: if pitch classes a and b are 6 semitones apart, then T6 maps a onto b and simultaneously maps b onto a. Both notes are therefore retained, yielding two common tones from a single ic6 occurrence (Straus, p. 112).

# Prerequisites
- **Interval class** -- ic6 is the unique self-mapping interval class
- **Common tones under transposition** -- the general principle this modifies

# Key Properties
1. The tritone (ic6) is the only interval that equals its own complement mod 12 (6 = 12 - 6)
2. At T6, each ic6 creates a bidirectional mapping: a -> b and b -> a
3. For all other interval classes, each occurrence creates only a unidirectional mapping at a given Tn

# Construction / Recognition
Given set [4, 9, 10] with one tritone (between 4 and 10):
- At T6: 4 maps to 10, and 10 maps to 4
- Both 4 and 10 are common tones
- Result: 2 common tones from 1 ic6

Compare with ic3 in [4, 5, 7, 8]:
- At T3: 4 maps to 7, 5 maps to 8 (one common tone per ic3 occurrence)
- The mapping is one-directional at T3

# Context & Application
The tritone exception must be accounted for when using the interval-class vector to predict common tones. Without this correction, T6 common-tone counts will be systematically underpredicted. The exception reflects the tritone's unique property as the only interval that bisects the octave exactly.

# Examples
**Example 1** (p. 112, Ex. 3-3): Set [4, 9, 10], sc(016), contains one tritone. At T6: [4, 9, 10] -> [10, 3, 4]. Common tones: {4, 10} -- two common tones from one ic6.

**Example 2**: Set (0167) has ic vector [200022], with two ic6 occurrences. At T6: 2 * 2 = 4 common tones, which equals the set's cardinality -- the set maps entirely onto itself.

# Relationships
## Builds Upon
- **Common tone theorem for transposition** -- this is the exception to the general rule

## Related
- **Transpositional symmetry** -- sets with ic6 count = half their cardinality are T6-symmetrical

# Common Errors
- Applying the standard rule (1 common tone per ic occurrence) to T6
- Forgetting to multiply the ic6 count by 2

# Common Confusions
- The exception applies only to T6, not to any other transposition level
- It arises from the mathematical property 6 + 6 = 12 = 0 (mod 12), not from any acoustic property of the tritone

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.1.1, p. 112

# Verification Notes
New card extracted from source. No prior card existed for this specific sub-concept.
