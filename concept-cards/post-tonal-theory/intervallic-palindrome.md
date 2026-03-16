---
concept: Intervallic Palindrome
slug: intervallic-palindrome
category: set-theory
subcategory: symmetry
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Some Additional Properties and Relationships"
chapter_number: 3
pdf_page: 123
section: "3.4.1 Intervallic palindrome (mirror image)"
extraction_confidence: high
aliases:
  - "mirror image intervals"
  - "interval palindrome"
prerequisites:
  - inversional-symmetry
  - normal-form
extends:
  - inversional-symmetry
related:
  - pitch-symmetry
contrasts_with: []
answers_questions:
  - "How can I tell if a set is inversionally symmetrical by looking at its intervals?"
  - "What does inversional symmetry look like when written out?"
---

# Quick Definition
The characteristic property of inversionally symmetrical sets: when written in normal form, the sequence of intervals between adjacent notes reads the same from left to right as from right to left.

# Core Definition
Sets that are inversionally symmetrical can be written so that the intervals reading from left to right are the same as the intervals reading from right to left. This intervallic palindrome is usually apparent when the set is written in normal form, though occasionally a note must be written twice to capture the modular wraparound. The palindromic structure reflects the fact that every note has an inversional partner within the set (Straus, p. 123).

# Prerequisites
- **Inversional symmetry** -- the property that produces palindromes
- **Normal form** -- the standard ordering that makes the palindrome visible

# Key Properties
1. The interval sequence reads identically forward and backward
2. Usually visible in normal form; sometimes requires modular wraparound notation
3. On the clockface, corresponds to mirror (reflective) symmetry
4. The center of the palindrome identifies the axis of symmetry

# Construction / Recognition
To check for an intervallic palindrome:
1. Write the set in normal form
2. Compute intervals between adjacent pitch classes (mod 12)
3. Check if the interval sequence is a palindrome
4. If not immediately palindromic, try writing the set with wraparound (repeating the first note an octave higher)

# Context & Application
The intervallic palindrome is the most direct way to recognize inversional symmetry by inspection. It transforms the abstract algebraic property into a visible pattern, making it useful for quick identification in analytical work.

# Examples
**Example 1** (p. 123, Ex. 3-13): Three palindromic sets:
- [B, C, D, E, F]: intervals 1-2-2-1 (palindrome)
- [D, E, F, A, Bb]: intervals 2-1-4-1 (palindrome with wraparound)
- [F#, G, Bb, B, D, Eb]: intervals 1-3-1-3-1 (palindrome)

**Example 2**: Non-symmetrical set [0, 1, 3, 7]: intervals 1-2-4 -- not a palindrome, confirming the set is not inversionally symmetrical.

# Relationships
## Builds Upon
- **Inversional symmetry** -- the palindrome is its visible manifestation

## Related
- **Pitch symmetry** -- when the palindrome is realized in pitch intervals (register), not just pitch-class intervals

# Common Errors
- Assuming the palindrome will always be visible without wraparound notation
- Computing pitch-class intervals incorrectly (must account for mod 12)

# Common Confusions
- The palindrome is in the intervals, not the pitch-class numbers themselves
- A set can be inversionally symmetrical even if the palindrome requires wraparound to see

# Source Reference
Chapter 3: Some Additional Properties and Relationships, Section 3.4.1, p. 123

# Verification Notes
New card extracted from source. This sub-concept was mentioned in the inversional-symmetry card but not given its own card previously.
