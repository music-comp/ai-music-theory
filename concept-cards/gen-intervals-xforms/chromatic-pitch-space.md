---
concept: Chromatic Pitch Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Chromatic pitch space is a GIS where the musical space consists of equally-tempered pitches, and intervals measure the number of semitones between pitches.

# Formal Definition
In Example 2.1.2, the musical space S is a gamut of chromatic pitches under twelve-tone equal temperament. Given pitches s and t, int(s, t) is the number of semitones one must move in an upward-oriented sense to get from s to t, not counting s itself. The interval group IVLS is the integers under addition.

# Mathematical Formulation
- S = chromatic pitches (extended indefinitely up and down)
- IVLS = (Z, +), the integers under addition
- int(s, t) = number of semitones up from s to t (not counting s)
- Negative intervals represent downward motion
- int(C4, C5) = 12 (octave = 12 semitones)

# Musical Context/Application
This is the most common GIS for pitch analysis in twelve-tone and post-tonal music. Every pitch interval can be expressed as an integer number of semitones. This system underlies set theory, twelve-tone analysis, and much of modern music theory. The integer representation allows powerful mathematical techniques.

# Examples
From Example 2.1.2:
- int(C4, D4) = 2 (whole tone)
- int(C4, G4) = 7 (perfect fifth)
- int(C4, C5) = 12 (octave)
- int(C4, F3) = -7 (perfect fifth down)
- int(C4, F2) = -19 (octave + fifth down)

Verification of Condition (A): int(C4, E4) = 4, int(E4, G4) = 3, int(C4, G4) = 7, and 4 + 3 = 7.

Condition (B): For any pitch s and any integer i, there exists exactly one pitch t with int(s, t) = i.

# Related Concepts
- Diatonic Pitch Space
- Pitch-Class Space (mod 12)
- Generalized Interval System
- Twelve-Tone Equal Temperament
- Integers Under Addition

# Common Confusions
- Intervals don't count the starting pitch (C to D = 2 semitones, not 3)
- The space extends theoretically to inaudible frequencies
- This differs from diatonic space (which counts scale steps, not semitones)
- Negative intervals are valid and necessary for downward motion

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.2, Section 2.4
