---
concept: Pitch-Class Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Pitch-class space is a GIS of 12 pitch classes (octave-equivalent pitches) arranged on a clock face, with intervals measured as clockwise hours.

# Formal Definition
In Example 2.1.3, the musical space S comprises the twelve pitch classes under equal temperament. If we arrange the pitch classes around the face of a clock following the order of a chromatic scale, then int(s, t) is the number of hours traversed in proceeding clockwise from s to t. The interval group IVLS is the integers under addition modulo 12.

# Mathematical Formulation
- S = {C, C#, D, D#, E, F, F#, G, G#, A, A#, B} (12 pitch classes)
- IVLS = Z12 = integers under addition mod 12
- int(s, t) = clockwise distance from s to t on a 12-hour clock
- int(s, t) is always in {0, 1, 2, ..., 11}
- int(s, t) + int(t, s) = 12 = 0 mod 12

# Musical Context/Application
This is the foundational GIS for twelve-tone and pitch-class set theory. Octave equivalence collapses all C's (C1, C2, C3, ...) into a single pitch class C. This abstraction allows analysis of properties that are invariant under octave transposition, which is essential for understanding twelve-tone rows, set classes, and transformational relationships.

# Examples
From Example 2.1.3:
- int(E, E) = 0 (unison)
- int(E, F) = 1 (semitone up)
- int(F, E) = 11 (11 hours clockwise = semitone "down" in mod 12)
- If s is at 8 o'clock and t is at 1 o'clock, int(s, t) = 5

Note: int(s, t) does not depend on which pitch class is positioned at 12 o'clock.

Condition (A) example: int(C, E) = 4, int(E, G) = 3, int(C, G) = 7, and 4 + 3 = 7 mod 12.

Every element in Z12 is reachable: For any pitch class s and interval i (0-11), exactly one pitch class t satisfies int(s, t) = i.

# Related Concepts
- Chromatic Pitch Space
- Generalized Interval System
- Integers Mod 12
- Pitch Class
- Twelve-Tone Theory
- Set Theory

# Common Confusions
- There are no "negative" intervals in mod 12 - instead, int(F, E) = 11, not -1
- int(s, t) + int(t, s) = 0 mod 12, so int(t, s) = -int(s, t) = 12 - int(s, t)
- The clock positioning is arbitrary; only relative distances matter
- This is finite (12 elements), unlike chromatic pitch space (infinite)

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.3, Section 2.4
