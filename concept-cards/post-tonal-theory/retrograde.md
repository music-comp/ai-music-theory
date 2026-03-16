---
concept: Retrograde
slug: retrograde
category: twelve-tone
subcategory: operations
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Twelve-Tone Music"
chapter_number: 6
pdf_page: 313
section: "6.2.4 Retrograde"
extraction_confidence: high
aliases:
  - "R-form"
  - "retrograde ordering"
prerequisites:
  - twelve-tone-series
  - prime-ordering
extends: []
related:
  - inversion-twelve-tone
  - retrograde-inversion
  - series-class
contrasts_with:
  - prime-ordering
answers_questions:
  - "What is the retrograde of a twelve-tone series?"
  - "How does retrograding affect intervals?"
  - "Why does R_n end on pitch-class n rather than begin on it?"
---

# Quick Definition
The retrograde (R) of a series is the prime ordering played backward, reversing the order of pitch classes and producing a reversed interval succession with each interval replaced by its complement mod 12.

# Core Definition
"The *retrograde* simply reverses the prime ordering." When the series is retrograded, "the intervals are heard in reverse order, and each interval is replaced by its complement mod 12 (1 becomes 11, 2 becomes 10, etc.)" (Straus, p. 313--314). There are twelve retrograde forms: R0, R1, R2, ..., R11. "R0 is the retrograde of P0, R1 the retrograde of P1, and so on. R0 thus ends rather than begins on 0" (p. 314).

# Prerequisites
- **Prime ordering** -- the retrograde reverses the prime
- **Twelve-tone series** -- the object being retrograded

# Key Properties
1. R_n = P_n played backward
2. R_n *ends* on pitch-class n (not begins)
3. Intervals are reversed in order
4. Each interval is replaced by its complement mod 12 (i becomes 12 - i)
5. Series related by retrograde (P and R, I and RI) have "complementary intervals in reverse order" (p. 317)

# Construction / Recognition
- Write out P_n and reverse the order of pitch classes
- Verify: interval succession is reversed and complemented
- Example: If P has interval 11, R has interval 1 in the corresponding reversed position

# Context & Application
Retrograde is one of the four basic orderings of the twelve-tone series. It creates palindromic and arch-form structures. In Webern's music, series overlap frequently occurs at the junction of series forms, where "a single note can be simultaneously the last note of one series form and the first note of the next" (p. 322).

# Examples
**Example 1** (p. 314, Ex. 6-4): Schoenberg, String Quartet No. 4 -- R2 is P2 played backward. If P2 has intervals <11, 8, 1, 7, 10, 1, 8, 8, 11, 11, 5>, then R2 has intervals <7, 1, 1, 4, 4, 11, 2, 5, 11, 4, 1>.

**Example 2** (p. 321): Webern, "Wie bin ich froh!" -- uses only P7, R7, I7, and RI7 throughout the entire song.

# Relationships
## Builds Upon
- **Prime ordering** -- R is P reversed

## Related
- **Inversion (twelve-tone)** -- another basic operation
- **Retrograde-inversion** -- the retrograde of the inversion

## Contrasts With
- **Prime ordering** -- opposite ordering direction

# Common Errors
- Assuming R_n begins on pitch-class n (it *ends* on n)
- Expecting intervals to remain the same (they are complemented *and* reversed)

# Common Confusions
- **Retrograde vs. inversion**: Retrograde reverses order; inversion complements pitch classes
- **Retrograde vs. retrograde-inversion**: R reverses P; RI reverses I

# Source Reference
Chapter 6, Section 6.2.4, pp. 313--314

# Verification Notes
Preserved from old card: interval complementation, Schoenberg example. Added: v3 template, direct quotation, Webern example, emphasis on R_n ending convention.
