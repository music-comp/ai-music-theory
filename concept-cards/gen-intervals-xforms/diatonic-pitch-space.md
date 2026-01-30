---
concept: Diatonic Pitch Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Diatonic pitch space is a GIS where the musical space consists of pitches arranged in scalar order, and intervals measure the number of scale steps between pitches.

# Formal Definition
In Example 2.1.1, the musical space S is a diatonic gamut of pitches arranged in scalar order. Given pitches s and t, int(s, t) is the number of scale steps one must move in an upward-oriented sense to get from s to t. The interval group IVLS is the integers under addition, extended to include indefinitely high and low "pitches" to satisfy Condition (B).

# Mathematical Formulation
- S = diatonic pitches (extended indefinitely up and down)
- IVLS = (Z, +), the integers under addition
- int(s, t) = number of scale steps up from s to t
- Negative intervals represent downward motion
- int(C4, C4) = 0, int(C4, D4) = 1, int(C4, E4) = 2, int(C4, C5) = 7

# Musical Context/Application
This GIS models step-wise melodic motion in diatonic music. Unlike traditional interval naming (where "3rd + 3rd = 5th"), this system uses additive intervals: 2 steps + 2 steps = 4 steps. This makes the algebra consistent and allows for proper mathematical analysis of melodic patterns, sequences, and voice-leading.

# Examples
From Example 2.1.1:
- int(C4, C4) = 0 (unison, no steps)
- int(C4, D4) = 1 (one step up)
- int(C4, E4) = 2 (two steps up)
- int(C4, C5) = 7 (seven steps up = octave)
- int(C4, A3) = -2 (two steps down = "-2 steps up")

Interval composition: int(C4, E4) + int(E4, G4) = 2 + 2 = 4 = int(C4, G4). Taking 2 steps up from C4 to E4, then 2 more steps up from E4 to G4, gives 4 steps total from C4 to G4.

# Related Concepts
- Chromatic Pitch Space
- Generalized Interval System
- Pitch-Class Space
- Modular Diatonic Space
- Integers Under Addition

# Common Confusions
- Traditional "3rd + 3rd = 5th" is replaced by 2 + 2 = 4 (counting steps, not notes)
- The space must extend beyond the audible range for mathematical completeness
- -n steps up means n steps down
- This differs from chromatic pitch space (which counts semitones, not scale steps)

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.1, Section 2.4
