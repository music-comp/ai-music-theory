---
# === CORE IDENTIFICATION ===
concept: Mod 12 Arithmetic
slug: mod-12-arithmetic

# === CLASSIFICATION ===
category: fundamentals
subcategory: mod 12
tier: foundational

# === PROVENANCE ===
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 22
section: "1.5 Arithmetic modulo 12 (mod 12)"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - modular arithmetic
  - arithmetic modulo 12
  - mod 12

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-class
  - integer-notation
extends: []
related:
  - pitch-class-clockface
  - pitch-class-space
  - complementary-intervals
contrasts_with:
  - pitch-space

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "What is mod 12 arithmetic?"
  - "How do I reduce integers to pitch-class values?"
---

# Quick Definition
Arithmetic modulo 12 is a number system where any integer is reduced to a value from 0 to 11 by adding or subtracting multiples of 12, reflecting the twelve-pitch-class cycle.

# Core Definition
In modular arithmetic with modulus 12 (abbreviated mod 12), any number larger than 11 or smaller than 0 is equivalent to some integer from 0 to 11. To find the equivalent, add or subtract 12 (or any multiple of 12). Twelve is called the modulus. In a mod 12 system, -12 = 0 = 12 = 24; similarly, -13, -1, 23, and 35 are all equivalent to 11 because they differ from 11 by multiples of 12.

# Prerequisites
- **Pitch Class** — mod 12 arithmetic operates on pitch-class integers
- **Integer Notation** — provides the integers that mod 12 arithmetic manipulates

# Key Properties
1. Any integer reduces to a unique value in {0, 1, 2, ..., 11}
2. Adding or subtracting 12 yields an equivalent value (same pitch class)
3. The modulus (12) corresponds to the number of pitch classes in equal temperament
4. All pitch-class interval and transformation calculations use mod 12

# Construction / Recognition
## To Construct:
1. Take any integer
2. If it is 0-11, it is already reduced
3. If larger than 11, subtract 12 repeatedly until in range
4. If smaller than 0, add 12 repeatedly until in range

## To Recognize:
1. Any operation where results wrap around after reaching 12
2. Calculations yielding values always in the range 0-11

# Context & Application
Mod 12 arithmetic is the mathematical backbone of pitch-class space. Going up an octave (adding 12 semitones) returns to the same pitch class: 3 + 12 = 15 = 3 (mod 12). The system is analogous to clock time (a mod 12 system where 11 + 1 = 0) and days of the week (a mod 7 system). Straus writes: "Just as our lives unfold simultaneously in linear and modular time, music unfolds simultaneously in pitch and pitch-class space."

# Examples
**From the text** (p. 22): Starting on the Eb above middle C (pitch class 3), going up 12 semitones returns to pitch class 3. In other words, 3 + 12 = 15 = 3 (mod 12).

**Reduction examples**:
- 15 mod 12 = 3
- 27 mod 12 = 3
- -1 mod 12 = 11
- -13 mod 12 = 11

# Relationships
## Builds Upon
- **Pitch Class** — mod 12 formalizes the cyclic nature of 12 pitch classes
- **Integer Notation** — provides the integers for calculation

## Enables
- **Pitch-Class Clockface** — visualizes mod 12 space as a circle
- **Ordered Pitch-Class Interval** — calculated as (y - x) mod 12
- **Complementary Intervals** — pairs that sum to 12 (= 0 mod 12)
- **Transposition (T_n)** — adds n mod 12

## Related
- **Pitch-Class Space** — the modular space described by mod 12 arithmetic

## Contrasts With
- **Pitch Space** — linear and unbounded, not modular

# Common Errors
- **Error**: Forgetting to reduce results to the 0-11 range
  **Correction**: Always apply mod 12 to final answers in pitch-class calculations.

- **Error**: Treating negative results as invalid
  **Correction**: Add 12 to any negative result. For example, 2 - 5 = -3 = 9 (mod 12).

# Common Confusions
- **Confusion**: Thinking mod 12 applies to pitch space
  **Clarification**: Mod 12 applies to pitch-class space only. Pitch space is linear; a pitch interval of 15 remains 15 and is not reduced.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.5, pp. 22-23.

# Verification Notes
- Definition source: direct (Straus Section 1.5 and "IN BRIEF" box)
- Confidence rationale: explicit definition with worked examples and time analogy
- Re-extraction notes: Re-extracted from v2 card; preserved: clock/week analogy, negative-number examples, Straus quote about linear and modular time
