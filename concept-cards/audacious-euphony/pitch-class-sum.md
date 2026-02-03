---
concept: Pitch-Class Sum
category: theory
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
chapter: "A Unified Model of Triadic Voice-Leading Space"
chapter_number: 5
pdf_page: 101
unit: null
authors: Richard Cohn
---

# Quick Definition
The sum of a chord's pitch classes modulo 12, which determines its voice-leading zone and enables calculation of voice-leading distance without reference to specific pitch content.

# Formal Definition
The pitch-class sum of a chord is computed by adding the integer values of its constituent pitch classes and taking the result modulo 12. This sum determines the chord's voice-leading zone. Pitch-class sums were first explored by Babbitt (2003, originally 1972) in a different context; their relevance to voice-leading zones was suggested by Jack Douthett.

# Musical/Mathematical Formulation
Calculation method:
- Assign integers to pitch classes: C=0, C#=1, D=2, ..., B=11
- Sum the pitch classes
- Take result modulo 12

Examples:
- d minor {D, F, A} = {2, 5, 9} -> 2+5+9 = 16 -> 16 mod 12 = 4
- G major {G, B, D} = {7, 11, 2} -> 7+11+2 = 20 -> 20 mod 12 = 8
- C augmented {C, E, G#} = {0, 4, 8} -> 0+4+8 = 12 -> 12 mod 12 = 0

Cognitive shortcut:
- Fix C augmented at zone 0
- Other augmented triads at zones 3, 6, 9 (ascending by semitone)
- Consonant triad zone = which augmented it displaces, adjusted for direction

# Musical Context/Application
The pitch-class sum provides a musically meaningful way to determine zones without memorization. Since consonant triads displace augmented triads by one semitone, knowing the four augmented positions (0, 3, 6, 9) and whether the consonant triad upshifts or downshifts from its adjacent augmented determines its zone.

# Examples
- D minor upshifts FAC# (zone 3) -> zone 4
- G major downshifts GBEb (zone 9) -> zone 8
- All T4-related triads share the same pitch-class sum and zone

# Related Concepts
- Voice-Leading Zones
- Center of Balance
- T4 Equivalence
- Augmented Triad
- Modular Arithmetic

# Common Confusions
- Pitch-class sum is not the same as root pitch class
- The sum depends on all three pitch classes, not just one
- Enharmonically equivalent pitch classes have the same integer value

# Source Reference
Chapter 5: A Unified Model of Triadic Voice-Leading Space, pages 122-124
