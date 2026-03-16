---
concept: Integer Notation for Pitches
slug: integer-notation-pitches
category: analysis
subcategory: post-tonal-analysis
tier: advanced
source: "Music Theory for the 21st-Century Classroom"
source_slug: 21st-century-classroom
authors: "Robert Hutchinson"
chapter: "Set Theory"
chapter_number: 33
pdf_page: 477
section: "33.1.2 Integer Notation for Pitches"
extraction_confidence: high
aliases:
  - "pitch integers"
  - "pitch-class integers"
prerequisites:
  - pitch-class
extends: []
related:
  - set-theory-overview
  - interval-class
contrasts_with: []
answers_questions:
  - "How are pitches represented as integers in set theory?"
  - "What is modulo 12 arithmetic?"
---

# Quick Definition
Integer notation represents pitches as numbers 0-11 (C=0), removing tonal implications and enabling mathematical operations on pitch collections.

# Core Definition
In set theory, pitches are represented by integers 0 through 11, corresponding to the twelve pitch classes. C=0, C#/Db=1, D=2, through B=11. This system assumes enharmonic equivalence (D, C double-sharp, and E double-flat all equal 2) and octave equivalence (any C is pitch class 0). The system operates in modulo 12, meaning it cycles after 11 (0,1,...11,0,1...) like a clock (Hutchinson, Ch. 33, p. 477).

# Prerequisites
- **Pitch class** -- Integer notation represents pitch classes

# Key Properties
1. C=0 through B=11
2. Enharmonic equivalence assumed
3. Octave equivalence assumed
4. Modulo 12 system
5. Removes tonal implications of staff notation
6. Memory aid: C major triad = 0, 4, 7

# Construction / Recognition
C=0, C#/Db=1, D=2, D#/Eb=3, E=4, F=5, F#/Gb=6, G=7, G#/Ab=8, A=9, A#/Bb=10, B=11

Intervals also use integers: the number of semitones (m2=1, M2=2, m3=3, M3=4, P4=5, TT=6, P5=7, etc.)

# Context & Application
Integer notation strips tonal implications. In atonal music, an augmented 5th and minor 6th have the same sound (both span 8 semitones). Integer notation reveals these equivalences.

# Examples
- C major triad: 0, 4, 7 (p. 477)
- Webern chord Eb, B, D: 3, 11, 2
- Interval from 11 to 2: 3 semitones (modulo 12, like 11am to 2pm = 3 hours)

# Relationships
## Related
- **Set theory overview** -- Integer notation is fundamental to set theory
- **Interval class** -- Intervals also represented as integers

# Common Errors
- **Error**: Forgetting to reduce results larger than 11 by subtracting 12
  **Correction**: Always apply modulo 12 arithmetic

# Common Confusions
- **Confusion**: Thinking integers represent specific pitches (e.g., a specific C)
  **Clarification**: Integers represent pitch classes (all Cs, any octave)

# Source Reference
Chapter 33, Section 33.1.2, PDF page 477. Interval table (Table 33.1.2).

# Verification Notes
- Mapping table directly from source, p. 477
- Modulo 12 clock analogy from source
- Re-extracted from v2 card; preserved: C major triad memory aid, clock analogy
- Confidence: HIGH -- source provides explicit mapping table
