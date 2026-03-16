---
concept: Combination Cycles
slug: combination-cycles
category: harmony
subcategory: interval cycles
tier: advanced
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Motive, Voice Leading, and Harmony"
chapter_number: 4
pdf_page: 175
section: "4.2.4 Combination cycles"
extraction_confidence: high
aliases: []
prerequisites:
  - interval-cycles
extends:
  - interval-cycles
related:
  - ri-chain
  - octatonic-collection
  - hexatonic-collection
contrasts_with:
  - interval-cycles
answers_questions:
  - "What are combination cycles?"
---

# Quick Definition

Cyclic patterns produced by alternating two different intervals rather than repeating a single interval, notated with angle brackets (e.g., <1, 8>) and generating chains of related trichords.

# Formal Definition

A **combination cycle** is a sequence of pitch classes produced by systematically alternating two different ordered pitch-class intervals. Unlike simple interval cycles (which repeat one interval), combination cycles alternate between two intervals in strict succession. The notation <a, b> indicates alternation of intervals a and b.

Key properties:
- Every combination cycle can be understood as two intertwined simple cycles
- The underlying simple cycle uses the interval sum: <a, b> involves two (a+b)-cycles
- Every three-note segment of a combination cycle belongs to the same set class
- Combination cycles provide systematic pathways through members of a set class

The number of pitch classes in a combination cycle before returning to the starting point depends on the specific intervals involved.

# Mathematical Formulation/Recognition

**Notation:** <a, b> means alternate intervals a and b (ordered pitch-class intervals)

**Underlying structure:**
- <a, b> intertwines two simple (a+b)-cycles
- Example: <1, 8> intertwines two 9-cycles (since 1+8=9, and 9-cycle = 3-cycle backwards)

**Common combination cycles and their trichords:**

| Combination Cycle | Trichord Generated | Number of Distinct Cycles |
|-------------------|-------------------|---------------------------|
| <1, 3> | (014) | 4 (hexatonic scales) |
| <1, 8> | (014) | 3 (octatonic scales) |
| <3, 8> | (014) | 1 (24 moves to return) |
| <1, 6> | (016) | 1 (24 moves to return) |

**Cycle length:**
- Some combination cycles exhaust all 12 pitch classes once before returning
- Others require 24 moves (each pitch class appears twice)

# Musical Context/Application

Combination cycles provide:
- **Melodic pathways**: Systematic ways to traverse members of a trichord class
- **Harmonic chains**: Connecting related harmonies through consistent intervals
- **Compositional resources**: Pre-determined sequences for pitch organization
- **Analytical tools**: Revealing underlying structure in post-tonal melodies

The three combination cycles for (014):
- <1, 3>: Generates four hexatonic-related cycles
- <1, 8>: Generates three octatonic-related cycles
- <3, 8>: Generates one large 24-member cycle

Each provides a different way of systematically moving through members of sc(014).

# Examples

**<1, 8> combination cycle** (Example 4-11a):
- Starting on any pitch class, alternate intervals 1 and 8
- Three distinct <1, 8> cycles correspond to the three octatonic scales
- Every three-note segment is a member of sc(014)
- Underlying structure: two intertwined 3-cycles (diminished seventh chords)

**<1, 3> combination cycle** (Example 4-11b):
- Alternation of intervals 1 and 3
- Four distinct cycles corresponding to the four hexatonic scales
- Generated from different combinations of 4-cycles (augmented triads)
- Every three-note segment is sc(014)

**<3, 8> combination cycle** (Example 4-11c):
- Alternation of intervals 3 and 8
- Requires 24 moves to return to starting point (each pitch class appears twice)
- Can be thought of as two interlocked 1-cycles
- Every three-note segment is sc(014)

**Webern, Concerto for Nine Instruments, op. 24, second movement** (Example 4-12):
- Melody consists almost entirely of (014) trichords
- Presented as alternations of intervals 1 and 8 (or their complements 11 and 4)
- Melodic motions traceable on the three <1, 8> cycles
- Measures 17-20 and 22-27 make complete clockwise circuits around cycles

**Lutoslawski, Funeral Music** (Example 4-13):
- Two cellos play canon at the tritone
- Both voices follow <1, 6> combination cycle counterclockwise
- Alternating intervals 6 and -1 (or equivalently, 6 and 11)
- All melodic trichords are members of sc(016)
- 24-move cycle traversed many times throughout the piece

# Related Concepts

- Interval Cycles
- Cyclic Sets
- Cyclic Linear Motion
- Octatonic collection (<1, 8> cycles)
- Hexatonic collection (<1, 3> cycles)
- Set class (014)
- Set class (016)
- RI-chain (retrograde-inversion chain)

# Common Confusions

- **Combination vs. simple cycles**: Simple cycles repeat one interval; combination cycles alternate two intervals
- **Order matters**: <1, 8> and <8, 1> are different combination cycles (different starting directions)
- **Sum determines underlying cycle**: The sum of the two intervals reveals the underlying simple cycle structure
- **Multiple cycles possible**: Some interval pairs generate multiple distinct cycles; others generate just one
- **Complement intervals**: Moving by <a, b> clockwise is equivalent to moving by <12-a, 12-b> counterclockwise

# Source Reference

Chapter 4: Motive, Voice Leading, and Harmony, Section 4.2.4, pages 187-190
