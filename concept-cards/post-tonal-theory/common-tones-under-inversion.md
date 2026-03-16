---
concept: Common Tones under Inversion
slug: common-tones-under-inversion
category: set-theory
subcategory: inversion properties
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 119
section: "3.3 Common Tones Under Inversion (In)"
extraction_confidence: high
aliases:
  - "common tones under In"
  - "inversional common tones"
prerequisites:
  - inversion
  - index-number
  - common-tones-under-transposition
extends:
  - inversion
related:
  - addition-table
  - inversional-symmetry
contrasts_with:
  - common-tones-under-transposition
answers_questions:
  - "How do I calculate common tones under inversion?"
  - "How do common tones under inversion differ from those under transposition?"
---

# Quick Definition
The pitch classes held in common between a set and its inversion at In; determined by examining the sums (index numbers) formed by pairs of notes within the set, where two different notes summing to n yield two common tones, and a note summing with itself to n yields one.

# Core Definition
When a pitch-class set is inverted at In, common tones are determined by index numbers (sums) rather than intervals (differences). The sum of any pair of notes equals the index number n such that In maps those notes onto each other. If two different notes x and y in the set satisfy x + y = n (mod 12), both are common tones at In (they map onto each other). If a note x satisfies x + x = n (mod 12), then x alone is a common tone at In (it maps onto itself) (Straus, pp. 119-123).

# Prerequisites
- **Inversion (In)** -- the operation being applied
- **Index number** -- the sum that determines the inversion level
- **Common tones under transposition** -- the analogous concept using differences

# Key Properties
1. Two different notes summing to n produce 2 common tones at In
2. A note added to itself equaling n produces 1 common tone at In
3. The addition table systematically computes all sums
4. Unlike transposition, there is no simple vector-based shortcut
5. Common tones under In depend on the specific set, not just its set class

# Construction / Recognition
Method 1 -- Direct calculation:
1. For each pair of distinct notes (x, y), compute x + y (mod 12)
2. For each note x, compute 2x (mod 12)
3. Each sum of two different notes: 2 common tones at that In
4. Each self-sum: 1 common tone at that In

Method 2 -- Addition table:
1. Write the set along both axes
2. Fill cells with pairwise sums (mod 12)
3. Count occurrences of each sum; each occurrence = 1 common tone

# Context & Application
Common tones under inversion provide voice-leading connections between inversionally related forms. Composers use them for smooth transitions, pitch continuity, and establishing inversional axes as compositional centers. The calculation is more complex than under transposition but equally important for post-tonal voice-leading analysis.

# Examples
**Example 1** (p. 120, Ex. 3-10): [1, 3, 6, 9], sc(0258):
- 1 + 3 = 4: notes 1 and 3 are common tones at I4
- 1 + 6 = 7: notes 1 and 6 are common tones at I7
- 1 + 9 = 10: notes 1 and 9 are common tones at I10
- 3 + 6 = 9: notes 3 and 6 are common tones at I9
- 3 + 3 = 6: note 3 is a common tone at I6
- 6 + 6 = 0: note 6 is a common tone at I0

**Example 2** (p. 121, Ex. 3-12): Webern, Movements for String Quartet, op. 5, no. 3 -- intensive use of sc(014). At I0, [8, 9, 0] retains C as a common tone, kept in the same register and instrument. At I3, [0, 3, 4] retains both C and Eb, arranged to sound simultaneously.

# Relationships
## Builds Upon
- **Inversion (In)** -- the operation producing common tones
- **Index number** -- the sum used in calculations

## Enables
- **Inversional symmetry** -- the maximal case where all tones are common

## Related
- **Addition table** -- the computational tool for systematic calculation

## Contrasts With
- **Common tones under transposition** -- uses intervals (differences) not sums

# Common Errors
- Confusing sums (for inversion) with differences (for transposition)
- Forgetting that two different notes mapping onto each other counts as 2 common tones, while a note mapping onto itself counts as 1

# Common Confusions
- In and I(12-n) may produce different numbers of common tones (unlike Tn and T(12-n), which always match)
- The addition table counts each occurrence as exactly 1 common tone, whether on or off the diagonal

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.3, pp. 119-123

# Verification Notes
Upgraded from old v2 card. Preserved all calculation examples, Webern example, and addition table method. Added systematic distinction between pair-sums and self-sums per v3 template.
