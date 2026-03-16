---
concept: Interval Class
slug: interval-class

category: fundamentals
subcategory: intervals
tier: advanced

source: "Open Music Theory"
source_slug: open-music-theory
authors: "Open Music Theory contributors"
chapter: "Intervals in Integer Notation"
chapter_number: 8
pdf_page: null
section: "VIII.2"

extraction_confidence: high

aliases:
  - "ic"
  - "unordered pitch-class interval"

prerequisites:
  - pitch-class
  - integer-notation
extends: []
related:
  - interval-vector
  - pitch-class-set
  - set-class
contrasts_with: []

answers_questions:
  - "What is an interval class?"
  - "How many interval classes exist?"
  - "Why does interval class max out at 6?"
---

# Quick Definition
The interval class (ic) is the smallest distance in semitones between two pitch classes, measured either clockwise or counterclockwise on the clock face -- whichever is shorter. There are only six interval classes (1 through 6), plus ic 0 for unison.

# Core Definition
Interval class is the most abstract of four interval types in atonal theory. The four types form a spectrum from most concrete to most abstract: ordered pitch intervals (specific direction and octave), unordered pitch intervals (no direction), ordered pitch-class intervals (always ascending around the clock face, mod 12), and interval classes (shortest distance between two pitch classes). Interval class collapses inversionally related intervals: a perfect fourth (5 semitones) and perfect fifth (7 semitones) are both ic 5. The six interval classes correspond to fundamental sonority types: ic 1 (semitonal clash), ic 2 (whole-tone), ic 3 (minor-third richness), ic 4 (major-third brightness), ic 5 (open fourth/fifth stability), ic 6 (tritone tension).

# Prerequisites
- Pitch class and integer notation (to measure distances between pitch classes)

# Key Properties
1. Range: 0, 1, 2, 3, 4, 5, 6 (seven possible values)
2. If ordered pitch-class interval exceeds 6, subtract from 12
3. ic 6 (tritone) is its own complement: 12-6=6
4. Inversionally related intervals share the same ic: P4 and P5 are both ic 5
5. ic groups: ic 1=m2/M7, ic 2=M2/m7, ic 3=m3/M6, ic 4=M3/m6, ic 5=P4/P5, ic 6=tritone
6. Direction is irrelevant: both C-to-E and E-to-C yield ic 4

# Context & Application
Interval class is the most abstract interval concept in set theory, answering: "What is the simplest measure of distance between two pitch classes?" By collapsing inversionally related intervals, it reveals fundamental sonority types. Two chords with the same interval-class content (as measured by the interval vector) will have similar sonic color regardless of voicing. The concept is essential for comparing sets and understanding why certain sonorities sound similar.

# Examples
**Example 1**: C and E -- ordered pc interval = 4, so ic = 4. E and C -- ordered pc interval = 8, but 8 > 6, so ic = 12-8 = 4.

**Example 2**: C and G -- ordered pc interval = 7, but 7 > 6, so ic = 12-7 = 5. G and C -- ordered pc interval = 5, so ic = 5.

**Example 3** (Webern, Concerto Op. 24): The generating trichord emphasizes ic 1 and ic 4, creating the characteristic "Webernian" pointillistic, chromatic sound.

# Relationships
## Builds Upon
- **pitch-class** -- Interval class measures distances between pitch classes
- **integer-notation** -- Uses integer arithmetic for calculation
## Related
- **interval-vector** -- Counts how many of each ic a set contains
- **set-class** -- Sets with the same interval-class content belong to the same set class

# Common Confusions
- **Confusion**: Interval class can be 7 or higher
  **Clarification**: If the ordered pc interval is 7 or more, subtract from 12; ic maxes at 6
- **Confusion**: Perfect 4th and perfect 5th are different interval classes
  **Clarification**: They are both ic 5 (inversionally related)
- **Confusion**: Tonal quality names apply
  **Clarification**: ic 3 encompasses both minor 3rd and major 6th; tonal names are irrelevant

# Source Reference
Open Music Theory, Part VIII, Chapter 2: "Intervals in Integer Notation."

# Verification Notes
- Definition source: Directly from 08-02 source chapter
- Confidence rationale: High -- clearly defined with four-type hierarchy
- Preserved from v2: ic calculation examples, Webern/Bartok/Berg repertoire references, ic-to-quality mapping
- Cross-reference status: Consistent with set theory quick reference sheet
