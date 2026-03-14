---
concept: Interval Class
category: theory
source: Open Music Theory
chapter: "Intervals in Integer Notation"
pdf_page: null
chapter_number: 8
unit: null
authors: "Open Music Theory contributors"
---

# Interval Class

## Quick Definition

The interval class (ic) is the smallest distance in semitones between two pitch classes, measured either ascending or descending--whichever is shorter. Interval classes range from 0 to 6, with ic 6 (the tritone) being the maximum, since any larger ordered interval has a smaller complement.

## Formal Definition

**Interval class (ic)**: The unordered pitch-class interval; the shortest distance between two pitch classes on the pitch-class "clock."

**Calculation**:
Given two pitch classes x and y:
- ic = min(|x - y| mod 12, |y - x| mod 12)
- Or: If ordered PC interval > 6, subtract from 12

**Range**: 0, 1, 2, 3, 4, 5, 6

**Complementary relationships**:
```
Ordered PCI:  0  1  2  3  4  5  6  7  8  9  10  11
Interval class: 0  1  2  3  4  5  6  5  4  3   2   1
```

**Interval class groups related intervals**:
- IC 1: minor 2nd, major 7th (and their compounds)
- IC 2: major 2nd, minor 7th
- IC 3: minor 3rd, major 6th
- IC 4: major 3rd, minor 6th
- IC 5: perfect 4th, perfect 5th
- IC 6: tritone (augmented 4th, diminished 5th)

## Musical Context

Interval class is the most abstract interval concept in set theory. It answers the question: "What is the simplest measure of distance between two pitch classes?"

By collapsing inversionally related intervals (P4 and P5; m3 and M6; etc.), interval class reveals fundamental sonority types:

- **IC 1**: Semitonal "clash" quality
- **IC 2**: Whole-tone spacing
- **IC 3**: Minor-third richness
- **IC 4**: Major-third brightness
- **IC 5**: Open fifth/fourth stability
- **IC 6**: Tritone tension/ambiguity

This abstraction is powerful for comparing sets: two chords with the same interval-class content will have similar sonic "color" regardless of specific voicing.

## Examples

### Basic

**Calculating interval class**:
```
C and E: Ordered PCI = 4, so IC = 4
E and C: Ordered PCI = 8, but 8 > 6, so IC = 12 - 8 = 4

C and G: Ordered PCI = 7, but 7 > 6, so IC = 12 - 7 = 5
G and C: Ordered PCI = 5, so IC = 5

F# and C: Ordered PCI = 6, IC = 6 (tritone, its own complement)
```

**The six interval classes**:
```
IC 1: C-C# (semitone)
IC 2: C-D (whole tone)
IC 3: C-Eb (minor 3rd)
IC 4: C-E (major 3rd)
IC 5: C-F or C-G (P4 or P5)
IC 6: C-F# (tritone)
```

**Clock face visualization**:
```
On the clock face, IC = shorter arc between two points:
- C (12 o'clock) to E (4 o'clock): 4 steps clockwise OR 8 counter
- Shorter path = 4, so IC = 4
- C to G: 7 clockwise, 5 counter - IC = 5
```

### From Repertoire

**Webern, Concerto Op. 24**: The trichord that generates the row emphasizes IC 1 and IC 4, creating a characteristic "Webernian" sound--pointillistic and chromatic.

**Bartok, Music for Strings, Percussion, and Celesta**: The fugue subject emphasizes IC 1 (semitones), with the climax at the tritone (IC 6), then returns via the same interval classes--a symmetrical "wedge" shape.

**Berg, Wozzeck**: The opera's harmonic language shows a preference for IC 4 (major 3rds), giving even the atonal passages a richer, more "Romantic" quality compared to Webern's leaner textures.

## Related Concepts

- **Prerequisite**: pitch-class, ordered-pitch-class-interval, semitone
- **Leads to**: interval-vector, set-class, z-relation
- **See also**: interval-types-integer, interval-class-content

## Common Confusions

- **IC maxes at 6**: If ordered PCI is 7 or more, subtract from 12
- **IC 6 is unique**: The tritone is its own "complement" (12-6=6)
- **IC ignores direction**: Both C-to-E and E-to-C yield IC 4
- **IC ignores octave**: C4-E5 has the same IC as C4-E4
- **IC 0 exists**: Unison (same pitch class) has IC 0
- **Perfect 4th and 5th share IC 5**: They're inversionally related
- **"Interval class" is not "intervals by class"**: It's a specific technical term for the shortest PC distance
- **Tonal quality names are irrelevant**: IC 3 encompasses both minor 3rd and major 6th

## Source Reference

Open Music Theory, Part VIII, Chapter 2: "Intervals in Integer Notation"
