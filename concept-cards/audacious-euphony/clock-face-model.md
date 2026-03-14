---
concept: Clock-Face Model
category: analysis
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
chapter: "A Unified Model of Triadic Voice-Leading Space"
chapter_number: 5
pdf_page: 101
unit: null
authors: Richard Cohn
---

# Quick Definition
A representation of the twelve voice-leading zones as stations on a clock face (0-11), exploiting the isomorphism between zone structure and the numbers modulo 12.

# Formal Definition
The clock-face model (Figure 5.24) superimposes a modified clock face over Cube Dance, labeling each voice-leading zone with numbers 0-11. Zone labels correspond to pitch-class sums modulo 12. The assignment enables voice-leading distance to be computed as subtraction modulo 12, tapping into intuitions about number structure.

# Construction/Recognition
Zone assignments:
- Augmented triads at multiples of 3: zones 0, 3, 6, 9
- Consonant triads in remaining zones: 1, 2, 4, 5, 7, 8, 10, 11
- Each consonant zone contains 3 T4-related triads

Distance computation:
- Voice-leading distance = |zone1 - zone2| mod 12
- Example: d minor (zone 4) to G major (zone 8) = |8-4| = 4 units

Conventions:
- Zero substitutes for twelve at top
- Multiples of three in special typeface (adjacent to augmented triads)
- All numbers underlined to distinguish from other uses

# Musical Context/Application
The clock-face model leverages familiar intuitions about time-reckoning to understand voice-leading zones. The isomorphism with pitch-class integers means that voice-leading operations parallel pitch-class operations, enabling transfer of knowledge from atonal theory.

# Examples
- Figure 5.24: Clock face superimposed over Cube Dance
- Zone 0: C augmented (0+4+8 = 12 = 0 mod 12)
- Zone 4: d minor (2+5+9 = 16 = 4 mod 12)
- Zone 8: G major (7+11+2 = 20 = 8 mod 12)

# Related Concepts
- Voice-Leading Zones
- Pitch-Class Sum
- Cube Dance
- Modular Arithmetic
- T4 Equivalence

# Common Confusions
- The numbers are zone labels, not pitch-class integers (though related)
- Hexatonic pole is the only exception to distance = zone difference rule
- The clock metaphor is for visualization, not temporal relationships

# Source Reference
Chapter 5: A Unified Model of Triadic Voice-Leading Space, pages 121-124, Figure 5.24
