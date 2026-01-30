---
concept: Modular Diatonic Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Modular diatonic space is a GIS of 7 pitch classes (scale degrees) arranged on a 7-hour clock, with intervals measured as clockwise hours.

# Formal Definition
In Example 2.1.4, the musical space S comprises seven pitch classes corresponding to the seven mode degrees of a diatonic system. If we wrap the scale around the face of a seven-hour clock, then int(s, t) is the number of hours traversed on that clock in proceeding clockwise from s to t. The interval group IVLS is the integers under addition modulo 7.

# Mathematical Formulation
- S = {C, D, E, F, G, A, B} (7 diatonic pitch classes) or scale degrees {1, 2, 3, 4, 5, 6, 7}
- IVLS = Z7 = integers under addition mod 7
- int(s, t) = clockwise distance on a 7-hour clock
- int(s, t) is always in {0, 1, 2, 3, 4, 5, 6}
- int(s, t) + int(t, s) = 7 = 0 mod 7

# Musical Context/Application
This GIS models diatonic interval relationships when octave equivalence is assumed. It's useful for analyzing tonal music where the seven scale degrees are the primary structural elements. The mod-7 arithmetic captures relationships like "a third plus a third equals a fifth" (2 + 2 = 4) in step-class terms.

# Examples
From Example 2.1.4:
- int(D, D) = 0 (unison)
- int(D, E) = 1 (one step up)
- int(D, C) = 6 (six hours clockwise = one step "down")

Other examples:
- int(C, E) = 2 (third = 2 steps)
- int(E, G) = 2 (third = 2 steps)
- int(C, G) = 4 (fifth = 4 steps)
- Verification: 2 + 2 = 4 mod 7

Analogs: We could derive similar modular spaces for other scales:
- Octatonic scale: 8 pitch classes, IVLS = Z8
- Pentatonic scale: 5 pitch classes, IVLS = Z5

# Related Concepts
- Diatonic Pitch Space
- Pitch-Class Space
- Generalized Interval System
- Integers Mod 7
- Scale Degree

# Common Confusions
- This collapses all octave-equivalent scale degrees together
- The 7-hour clock is distinct from the 12-hour pitch-class clock
- Intervals are measured in scale steps, not semitones
- int(D, C) = 6, not -1 (mod 7 arithmetic uses only 0-6)

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.4, Section 2.4
