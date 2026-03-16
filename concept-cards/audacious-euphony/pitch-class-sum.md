---
concept: Pitch-Class Sum
slug: pitch-class-sum

category: triadic-systems
subcategory: voice-leading zones
tier: intermediate

source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "A Unified Model of Triadic Voice-Leading Space"
chapter_number: 5
pdf_page: 101
section: null

extraction_confidence: high

aliases:
  - "pc sum"
  - "voice-leading zone index"

prerequisites:
  - voice-leading-zone
extends: []
related:
  - center-of-balance
  - augmented-triad-as-axis
contrasts_with: []

answers_questions:
  - "How is a chord's voice-leading zone determined?"
  - "How do you calculate the pitch-class sum of a triad?"
  - "What cognitive shortcut exists for quickly finding a triad's zone?"
---

# Quick Definition
The sum of a chord's pitch classes modulo 12, which determines its voice-leading zone and enables calculation of voice-leading distance without reference to specific pitch content.

# Core Definition
The pitch-class sum of a chord is computed by adding the integer values of its constituent pitch classes and taking the result modulo 12. This sum determines the chord's voice-leading zone. Pitch-class sums were first explored by Babbitt (2003, originally 1972) in a different context; their relevance to voice-leading zones was suggested by Jack Douthett.

# Prerequisites
- **Voice-leading Zone**: The classification that the pitch-class sum determines

# Key Properties
1. Calculated as sum of pitch-class integers modulo 12
2. Determines voice-leading zone uniquely
3. T4-related triads share the same pitch-class sum and zone
4. Augmented triads occupy zones 0, 3, 6, 9
5. Consonant triads occupy the remaining eight zones (1, 2, 4, 5, 7, 8, 10, 11)
6. Voice-leading distance between chords correlates with zone difference

# Construction / Recognition
Calculation method:
- Assign integers: C=0, C#=1, D=2, ..., B=11
- Sum the pitch classes
- Take result modulo 12

Examples:
- d minor {D, F, A} = {2, 5, 9} -> 16 mod 12 = 4
- G major {G, B, D} = {7, 11, 2} -> 20 mod 12 = 8
- C augmented {C, E, G#} = {0, 4, 8} -> 12 mod 12 = 0

Cognitive shortcut:
- Fix C augmented at zone 0
- Other augmented triads at zones 3, 6, 9 (ascending by semitone)
- Consonant triad zone = adjacent augmented zone, adjusted for upshift/downshift direction

# Context & Application
The pitch-class sum provides a musically meaningful way to determine zones without memorization. Since consonant triads displace augmented triads by one semitone, knowing the four augmented positions (0, 3, 6, 9) and whether the consonant triad upshifts or downshifts from its adjacent augmented determines its zone. Zone labels are used throughout Chapters 5-9 for tracking voice-leading trajectories.

# Examples
- **D minor**: upshifts from FAC# (zone 3) -> zone 4
- **G major**: downshifts from GBEb (zone 9) -> zone 8
- **All T4-related triads**: share the same pitch-class sum and zone
- **Tristan Prelude zones** (Chapter 7): zone labels track alternation of upshift/downshift in seventh-chord progressions (note: triadic and tetrachordal zone labels are "false friends")

# Relationships
## Builds Upon
- Voice-leading zones (what the sum determines)
- Modular arithmetic (the computational tool)

## Enables
- Quick zone identification without memorization
- Voice-leading distance calculation between any two triads
- Tracking voice-leading trajectories in analysis

## Related
- Center of balance (related to the pitch-class sum geometrically)
- Augmented triad as axis (the reference points at zones 0, 3, 6, 9)

## Contrasts With
- Root-based chord identification (the sum depends on all three pitch classes, not just the root)

# Common Errors
- **Error**: Pitch-class sum is the same as root pitch class
  **Correction**: The sum depends on all three pitch classes, not just the root

# Common Confusions
- **Confusion**: Triadic and tetrachordal zone labels refer to the same system
  **Clarification**: They are "false friends" -- the systems are not in communication with each other (p. 171, n. 13)

- **Confusion**: Enharmonic respelling changes the sum
  **Clarification**: Enharmonically equivalent pitch classes have the same integer value

# Source Reference
Cohn, Richard. *Audacious Euphony*, Chapter 5, pp. 122-124.

# Verification Notes
Re-extracted from v2 card to v3.1 format. Preserved: calculation examples, cognitive shortcut, Babbitt/Douthett attribution. Fresh extraction adds: false-friends warning for tetrachordal zones, T4 equivalence property, v3.1 structure.
