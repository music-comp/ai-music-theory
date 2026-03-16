---
concept: Pitch Class
slug: pitch-class

category: fundamentals
subcategory: pitch-systems
tier: advanced

source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Pitch and Pitch Class"
chapter_number: 8
pdf_page: null
section: "VIII.1"

extraction_confidence: high

aliases:
  - "pc"
  - "pitch-class integer"

prerequisites:
  - pitch-vs-pitch-class
extends: []
related:
  - integer-notation
  - pitch-class-set
  - interval-class
contrasts_with: []

answers_questions:
  - "What is a pitch class?"
  - "How are pitch classes represented numerically?"
  - "Why use integers instead of letter names in post-tonal theory?"
---

# Quick Definition
A pitch class is the group of all pitches sharing the same name regardless of octave, including enharmonic equivalents, represented in set theory by an integer from 0 (C) to 11 (B).

# Core Definition
A pitch class groups together all pitches related by octave equivalence and enharmonic equivalence. In integer notation, the twelve pitch classes are numbered 0-11, where C=0. This system replaces letter names because enharmonic spellings (e.g., C-sharp vs. D-flat) carry no functional distinction in post-tonal music. Integer notation enables mathematical operations -- transposition (adding mod 12), inversion (subtracting from 12), and comparison -- that letter names cannot easily support. The clock face diagram, with 0 at the top and integers proceeding clockwise, is the standard visualization.

# Prerequisites
- Pitch vs. pitch class distinction (understanding octave and enharmonic equivalence)

# Key Properties
1. Twelve pitch classes exist: integers 0 through 11
2. C=0, C-sharp/D-flat=1, D=2, ..., B=11
3. Operations use mod-12 arithmetic (numbers wrap around at 12)
4. The clock face places 0 (C) at 12 o'clock, proceeding clockwise
5. Integer notation sidesteps all enharmonic spelling issues
6. Pitch class is the fundamental unit of post-tonal set theory

# Context & Application
Pitch class is essential for analyzing music where octave placement is less important than pitch identity, particularly in post-tonal and twelve-tone music, and also in jazz chord-scale theory. The concept acknowledges that a C in any octave has a similar function and relationship to other pitch classes, even though specific register affects timbre and voice leading. All set-theory operations (Tn, In) work on pitch classes.

# Examples
**Example 1**: C4 (middle C, ~261.63 Hz) belongs to pitch class 0. So do C3, C5, B-sharp in any octave, and D-double-flat in any octave.

**Example 2**: Integer notation mapping -- C-sharp3, C-sharp4, D-flat2, D-flat6 all belong to pitch class 1. E2, E4, F-flat5 all belong to pitch class 4.

**Example 3** (Schoenberg, twelve-tone works): A tone row uses each of the twelve pitch classes exactly once, regardless of which octave each pitch appears in.

# Relationships
## Builds Upon
- **pitch-vs-pitch-class** -- The foundational distinction between concrete pitches and abstract pitch classes
## Related
- **integer-notation** -- The numbering system (0-11) for pitch classes
- **pitch-class-set** -- Groups of pitch classes analyzed as units
- **interval-class** -- Distances between pitch classes

# Common Confusions
- **Confusion**: Pitch and pitch class are interchangeable terms
  **Clarification**: A pitch includes octave information (C4); a pitch class does not (pc 0 = all Cs)
- **Confusion**: Pitch class 0 = A
  **Clarification**: By convention, C=0 in set theory (following integer notation, not MIDI)
- **Confusion**: In tonal contexts, C-sharp and D-flat should be treated as different pitch classes
  **Clarification**: Pitch-class theory assumes enharmonic equivalence; if enharmonic distinctions matter, pitch-class analysis may not be appropriate

# Source Reference
Open Music Theory, Part VIII, Chapter 1: "Pitch and Pitch Class."

# Verification Notes
- Definition source: Directly from 08-01 source chapter
- Confidence rationale: High -- clear definition with integer notation table
- Preserved from v2: Integer notation mapping, Schoenberg example
- Cross-reference status: Consistent with integer-notation card
