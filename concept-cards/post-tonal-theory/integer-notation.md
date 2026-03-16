---
# === CORE IDENTIFICATION ===
concept: Integer Notation
slug: integer-notation

# === CLASSIFICATION ===
category: fundamentals
subcategory: notation systems
tier: foundational

# === PROVENANCE ===
source: "Introduction to Post-Tonal Theory"
source_slug: post-tonal-theory
authors: "Joseph N. Straus"
chapter: "Basic Concepts of Pitch and Interval"
chapter_number: 1
pdf_page: 21
section: "1.4 Integer Notation"

# === CONFIDENCE ===
extraction_confidence: high

# === VARIANTS ===
aliases:
  - pitch-class integers
  - fixed-do integer notation

# === TYPED RELATIONSHIPS ===
prerequisites:
  - pitch-class
  - enharmonic-equivalence
extends: []
related:
  - c-equals-zero-convention
  - mod-12-arithmetic
contrasts_with: []

# === COMPETENCY QUESTIONS ===
answers_questions:
  - "How are pitch classes represented numerically?"
  - "What is integer notation in post-tonal theory?"
---

# Quick Definition
Integer notation uses the numbers 0 through 11 to represent the twelve pitch classes, with C assigned to 0 and subsequent pitch classes numbered chromatically upward.

# Core Definition
Integer notation is a system for labeling the twelve pitch classes using integers from 0 through 11, following a "fixed do" convention where pitch class C (and its enharmonic equivalents B# and Dbb) is assigned 0, C#/Db is 1, D is 2, and so on chromatically. Integers are traditional in music theory (comparable to figured-bass numbers) and enable precise mathematical operations on pitch classes. The system does not make the music itself "mathematical"; rather, numbers and arithmetic help us think about musical relationships.

# Prerequisites
- **Pitch Class** — understanding that there are exactly 12 equivalence classes of pitches
- **Enharmonic Equivalence** — all enharmonic spellings map to the same integer

# Key Properties
1. Exactly 12 integers (0-11), one per pitch class
2. Follows "fixed do" convention: C = 0
3. Integers larger than 11 or smaller than 0 reduce via mod 12 arithmetic
4. Enables arithmetic operations (transposition, inversion, interval calculation)

# Construction / Recognition
## To Construct:
1. Start from C = 0
2. Count upward chromatically: C#/Db = 1, D = 2, D#/Eb = 3, E = 4, F = 5, F#/Gb = 6, G = 7, G#/Ab = 8, A = 9, A#/Bb = 10, B = 11

## To Recognize:
1. Any integer 0-11 represents a pitch class
2. Map the integer to its letter-name equivalent(s)

# Context & Application
Integer notation is used throughout post-tonal theory for calculating intervals, transpositions, inversions, and set-class membership. When referring to pitch classes, analysts use either traditional letter names or pitch-class integers, whichever is clearest in a given context.

# Examples
**Example 1-6** (p. 21): The twelve pitch classes and their integer names, showing all enharmonic spellings:

| Integer | Pitch-Class Content |
|---------|---------------------|
| 0 | B#, C, Dbb |
| 1 | C#, Db |
| 2 | C##, D, Ebb |
| 3 | D#, Eb |
| 4 | D##, E, Fb |
| 5 | E#, F, Gbb |
| 6 | F#, Gb |
| 7 | F##, G, Abb |
| 8 | G#, Ab |
| 9 | G##, A, Bbb |
| 10 | A#, Bb |
| 11 | A##, B, Cb |

**Example 1-7** (p. 22): Pitch-class integers assigned to the opening of Schoenberg, String Quartet No. 3, with octave and enharmonic equivalence assumed throughout.

# Relationships
## Builds Upon
- **Pitch Class** — integers label the twelve pitch classes
- **Enharmonic Equivalence** — multiple spellings map to a single integer

## Enables
- **Mod 12 Arithmetic** — arithmetic on pitch-class integers
- **Ordered Pitch-Class Interval** — calculated as (y - x) mod 12
- **Transposition** — T_n adds n to each pitch-class integer
- **Inversion** — I_n subtracts each integer from n

## Related
- **C=0 Convention** — the specific assignment convention used

## Contrasts With
- (no direct contrast within this chapter)

# Common Errors
- **Error**: Confusing pitch-class integers with pitch numbers (e.g., MIDI note numbers)
  **Correction**: Pitch-class integers are mod 12 (0-11); MIDI numbers are linear and register-specific (0-127).

# Common Confusions
- **Confusion**: Thinking integers make the music "mathematical"
  **Clarification**: "The music itself is not 'mathematical' any more than our lives are 'mathematical' because we count our ages in integers" (Straus). Integers are simply a tool for representing and calculating pitch-class relationships.

# Source Reference
Chapter 1: Basic Concepts of Pitch and Interval, Section 1.4, pp. 21-22.

# Verification Notes
- Definition source: direct (Straus Section 1.4 and "IN BRIEF" box)
- Confidence rationale: explicit definition with complete integer table
- Re-extraction notes: Re-extracted from v2 card; preserved: integer table, "not mathematical" clarification, Schoenberg example reference
