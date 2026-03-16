---
concept: "Transposition (Twelve-Tone)"
slug: transposition-twelve-tone
category: twelve-tone
subcategory: operations
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Twelve-Tone Music"
chapter_number: 6
pdf_page: 313
section: "6.2.3 Transposition"
extraction_confidence: high
aliases:
  - "serial transposition"
  - "row transposition"
prerequisites:
  - twelve-tone-series
  - prime-ordering
  - content-and-order
extends:
  - transposition
related:
  - retrograde
  - inversion-twelve-tone
  - series-class
contrasts_with: []
answers_questions:
  - "What happens when a twelve-tone series is transposed?"
  - "How does transposition affect the interval succession?"
---

# Quick Definition
Transposition of a twelve-tone series shifts each pitch class by the same interval, preserving the interval succession while changing the order positions of pitch classes.

# Core Definition
When a twelve-tone series is transposed, "the order of the pitch classes changes: D was first, now it is toward the end; A was third, now it is first; and so on. In fact, no pitch class occupies the same order position it did. The content, of course, is the same... and, more important, so is the interval succession. That particular interval succession is what defines the prime ordering of this series" (Straus, p. 313).

P_n = the prime ordering beginning on pitch-class n. "We can produce that succession beginning on any of the twelve pitch classes."

# Prerequisites
- **Prime ordering** -- transposition generates other P-forms from the designated prime
- **Content and order** -- transposition changes order, not content

# Key Properties
1. P_n = T_n(P_0): each pitch class x becomes (x + n) mod 12
2. Interval succession remains invariant under transposition
3. No pitch class occupies the same order position after transposition (in general)
4. All twelve P-forms (P0--P11) share the same interval succession
5. Series related by transposition are "prime-related"

# Construction / Recognition
- Apply T_n to each pitch class in the series
- Verify the interval succession matches the original
- P_n begins on pitch-class n

# Context & Application
Transposition is one of the four basic operations generating the 48-member series class. "Prime-related" series (P and P, I and I, R and R, or RI and RI) "have the same intervals in the same order" (p. 317). Transposition is used for development, contrast, and continuity.

# Examples
**Example 1** (p. 313, Ex. 6-3): Schoenberg, String Quartet No. 4 -- P2 transposed up 7 semitones yields P9. The interval succession (11, 8, 1, 7, 10, 1, 8, 8, 11, 11, 5) is identical in both forms.

# Relationships
## Builds Upon
- **Transposition** -- extends the general pitch-class operation to ordered series

## Enables
- **Series class** -- the 12 P-forms plus their R, I, RI forms
- **Twelve-tone areas** -- areas defined by transposition levels

## Related
- **Retrograde** -- another basic serial operation
- **Inversion (twelve-tone)** -- another basic serial operation

# Common Errors
- Expecting transposition to change pitch-class content (it does for smaller sets, not for the aggregate)
- Confusing transposition level with order position

# Common Confusions
- **T_n on a set vs. T_n on a series**: For smaller sets, T_n usually introduces new pitch classes; for a complete twelve-tone series, T_n merely reorders

# Source Reference
Chapter 6, Section 6.2.3, p. 313

# Verification Notes
Preserved from old card: Schoenberg example, prime-related definition. Added: v3 template, direct source quotation, explicit interval succession.
