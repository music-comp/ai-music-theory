---
concept: Integer Notation
slug: integer-notation

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
  - "pitch-class integers"
  - "pc integers"

prerequisites:
  - pitch-class
extends: []
related:
  - interval-class
  - pitch-class-set
contrasts_with: []

answers_questions:
  - "What is integer notation in set theory?"
  - "How are pitch classes numbered 0-11?"
  - "Why use numbers instead of letter names for pitches?"
---

# Quick Definition
A system for representing pitch classes as integers 0-11, where C=0, C-sharp/D-flat=1, D=2, and so on chromatically, eliminating the ambiguity of letter names and enharmonic spellings for post-tonal analysis.

# Core Definition
Integer notation assigns a number from 0 to 11 to each of the twelve pitch classes: 0=C, 1=C-sharp/D-flat, 2=D, 3=D-sharp/E-flat, 4=E, 5=F, 6=F-sharp/G-flat, 7=G, 8=G-sharp/A-flat, 9=A, 10=A-sharp/B-flat, 11=B. Operations use mod-12 arithmetic (numbers wrap around at 12, like a clock face). Because enharmonic equivalents share the same integer (G-sharp and A-flat are both 8), the system avoids the ambiguity of letter names in non-tonal contexts. Integer notation emerged in the mid-20th century as theorists (particularly Allen Forte and Milton Babbitt) developed systematic methods for analyzing atonal and twelve-tone music.

# Prerequisites
- Pitch class (understanding the grouping of pitches by octave and enharmonic equivalence)

# Key Properties
1. Twelve integers: 0 through 11, one per pitch class, with C=0
2. Arithmetic is mod 12 (e.g., 11+3=14 mod 12=2; 2-5=-3 mod 12=9)
3. Enharmonic equivalents share the same integer
4. Enables mathematical operations: transposition (Tn = add n), inversion (In = subtract from n)
5. Visualized on the clock face with 0 at 12 o'clock, proceeding clockwise
6. Each pitch class has exactly one unambiguous integer name

# Context & Application
Letter names work well for tonal music where spelling reflects function (G-sharp as leading tone vs. A-flat as upper neighbor). In post-tonal music, tonal function is absent, seven letter names inadequately represent twelve pitch classes, and mathematical relationships become analytically significant. The clock face diagram is the standard visualization, reinforcing the cyclical nature of pitch-class space.

# Examples
**Example 1**: Converting letter names -- E=4, F-sharp=6, B-flat=10, A-flat=8.

**Example 2**: Mod 12 arithmetic -- 7+8=15=3 (mod 12); 3-7=-4=8 (mod 12); 0-1=-1=11 (mod 12).

**Example 3** (Schoenberg, "Nacht" from Pierrot lunaire): The recurring motive E-G-E-flat can be notated as [4, 7, 3], facilitating tracking of transpositions: T5 yields [9, 0, 8] (A-C-A-flat).

# Relationships
## Builds Upon
- **pitch-class** -- Integer notation is the numbering system for pitch classes
## Related
- **interval-class** -- Intervals measured using integer semitones
- **pitch-class-set** -- Collections represented as sets of integers

# Common Confusions
- **Confusion**: 0 represents A (as in some MIDI systems)
  **Clarification**: In set theory, 0 always represents C
- **Confusion**: Integers represent specific pitches
  **Clarification**: They represent pitch classes -- C4 and C5 are both pc 0
- **Confusion**: 10 and 11 are problematic two-digit numbers
  **Clarification**: Some texts use T and E (or A and B) for compactness, but 10 and 11 are standard

# Source Reference
Open Music Theory, Part VIII, Chapter 1: "Pitch and Pitch Class," section on Integer Notation.

# Verification Notes
- Definition source: Directly from 08-01 source chapter
- Confidence rationale: High -- clearly defined with complete mapping in source
- Preserved from v2: Integer-to-pitch mapping table, mod-12 examples, Schoenberg/Webern/Bartok repertoire references
- Cross-reference status: Consistent with set theory quick reference sheet
