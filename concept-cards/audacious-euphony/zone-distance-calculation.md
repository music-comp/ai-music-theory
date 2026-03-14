---
concept: Zone Distance Calculation
category: theory
source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
chapter: "A Unified Model of Triadic Voice-Leading Space"
chapter_number: 5
pdf_page: 101
unit: null
authors: "Richard Cohn"
---

# Quick Definition
Voice-leading distance between any two consonant triads can be calculated by subtracting their voice-leading zone numbers modulo 12 and taking the absolute value, with a single exception for the hexatonic pole relation.

# Formal Definition
Voice-leading zone numbers are isomorphic to numbers modulo 12, enabling voice-leading distance to be modeled as subtraction. For triads in zones X and Y, the voice-leading distance is |X - Y| modulo 12, taking the smaller of the two complementary values. This works because zone-equivalent triads (related by T4) are equally distant from any given target zone. The single exception is the hexatonic pole (**H**), whose contrary motion gives it a maverick distance status.

# Construction/Recognition
To calculate zone distance:
1. Determine each triad's zone number (= sum of pitch classes mod 12)
2. Subtract: |zone1 - zone2| mod 12
3. Take the smaller value (compare with 12 minus the result)
4. The result equals the voice-leading work in semitones

Quick zone determination:
- Fix C augmented triad at zone 0
- Remaining augmented triads at zones 3, 6, 9 (ascending order)
- Each consonant triad's zone = its nearby augmented triad's zone +/- 1 (upshift = +1, downshift = -1)

# Musical Context
Zone distance calculation allows rapid assessment of voice-leading proximity without consulting a graph or counting edges. It leverages the isomorphism between voice-leading zones and the mod 12 number system.

# Examples
- d minor (zone 4) to G major (zone 8): |8 - 4| = 4 units of voice-leading work
- d minor: {2, 5, 9}, sum = 16 = 4 mod 12
- G major: {7, 11, 2}, sum = 20 = 8 mod 12
- D minor is at zone 4 because it upshifts FAC# (zone 3)
- G major is at zone 8 because it downshifts GBEb (zone 9)

# Related Concepts
- Voice-Leading Zones
- Pitch-Class Sum
- Voice-Leading Work
- Cube Dance
- Compound Transformation Classes

# Common Confusions
- Zone number = pitch-class sum mod 12, not the root pitch class
- The hexatonic pole is the sole exception to the distance = zone difference rule
- Any pair from the same two zones has the same distance: "What is proper to one pair is ipso facto true of the other eight"

# Source Reference
Chapter 5: A Unified Model of Triadic Voice-Leading Space, pp. 121-123, Figure 5.24
