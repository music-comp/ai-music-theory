---
concept: SLIDE Transformation
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
unit: null
authors: David Lewin
---

# Quick Definition
A Klang transformation that preserves the third of a triad while changing its mode: (F, +)SLIDE = (F#, -) and (F#, -)SLIDE = (F, +), keeping the pitch class A as common tone.

# Formal Definition
SLIDE preserves the third while changing mode:
- (p, +)SLIDE = (p + 1, -): Major Klang to minor Klang with root a semitone higher
- (p, -)SLIDE = (p - 1, +): Minor Klang to major Klang with root a semitone lower

SLIDE is its own inverse: (SLIDE)(SLIDE) = identity

# Mathematical Formulation
SLIDE transformation:
- (p, +)SLIDE = (p + 1, -) mod 12
- (p, -)SLIDE = (p + 11, +) mod 12
- SLIDE^2 = identity (SLIDE is an involution)

The preserved third:
- F major has third A; F# minor has third A
- The common tone is the third of both chords

# Musical Context/Application
SLIDE represents a chromatic, "exotic" relationship that nonetheless maintains a common tone. It appears in nineteenth-century chromatic harmony where themes transform between major and minor modes sharing a common third.

# Examples
From Beethoven's Eighth Symphony, last movement:
- Measures 379-91: F-major theme (beginning on A, the third) transforms into F# minor
- Measure 392: Theme slides back into F major
- The note A (third of F major, third of F# minor) is the pivot

From Schubert's posthumous Bb-Major Piano Sonata, slow movement:
- Measures 103-110: Material expected in C# minor appears in C major instead
- SLIDE relationship between (C, +) and (C#, -)

# Related Concepts
- Klang Representation
- PAR Transformation
- LT Transformation
- Chromatic Harmony
- Common-Tone Relationships

# Common Confusions
- SLIDE preserves the third, not the root or fifth
- The mode changes while the third stays fixed
- SLIDE is an involution (self-inverse)
- SLIDE is not common in Classical diatonic harmony but appears in Romantic chromatic practice

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.1.1
