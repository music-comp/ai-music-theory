---
concept: Tristan-Genus Voice-Leading Zones
slug: tristan-genus-voice-leading-zones

category: dissonance
subcategory: tetrachordal systems
tier: advanced

source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "Dissonance"
chapter_number: 7
pdf_page: 171
section: "Boretz-group transformations"

extraction_confidence: high

aliases:
  - "tetrachordal voice-leading zones"

prerequisites:
  - tristan-genus
  - voice-leading-zone
  - pitch-class-sum
extends:
  - voice-leading-zone
related:
  - four-cube-trio
  - boretz-region
  - circumnavigation-tetrachordal
contrasts_with:
  - voice-leading-zones

answers_questions:
  - "How are voice-leading zones calculated for Tristan-genus chords?"
  - "What do the zone numbers mean in the tetrachordal system?"
---

# Quick Definition
Tristan-genus chords occupy the six odd-numbered voice-leading zones (1, 3, 5, 7, 9, 11), calculated by summing their pitch-class content modulo 12, with diminished seventh, minor seventh, and French sixth chords filling the even zones.

# Core Definition
**Voice-leading zones** for the Tristan genus are derived via the same summing protocol used for triadic zones (Chapter 5). Each Tristan-genus chord is assigned to an odd-numbered zone by summing its pitch-class integers modulo 12. Cohn warns that "the labels for zones acquire meaning only within the limited context of the tetrachordal system of voice leading. Some of the labels are identical to those used for voice-leading zones in triadic space. These are 'false friends': the triadic and tetrachordal systems are not in communication with each other" (p. 172 n. 13).

# Prerequisites
- **Tristan genus**: The chords being zoned
- **Voice-leading zone (triadic)**: The analogous concept for triads
- **Pitch-class sum**: The computation method

# Key Properties
1. Tristan-genus chords occupy only odd zones (1, 3, 5, 7, 9, 11)
2. Diminished sevenths at even zones 2, 6, 10 (at spider centers on 4-Cube Trio)
3. Minor sevenths and French sixths at even zones 0, 2, 4, 6, 8, 10 (bridges)
4. Boretz-region motion alternates between two adjacent odd zones
5. Octatonic bridges jump zones, leaving gaps fillable by even-zone chords
6. Zone labels are "false friends" with triadic zone labels

# Construction / Recognition
To determine a Tristan-genus chord's zone:
1. List four pitch classes as integers (C=0, C#=1, ..., B=11)
2. Sum them modulo 12
3. Result is an odd number

Zone behavior:
- **Balanced (Boretz-region)**: Alternates between two adjacent odd zones (e.g., 3 and 1)
- **Directed (circumnavigation)**: Tours all six odd zones systematically
- **Even zones engaged**: By diminished sevenths, minor sevenths, French sixths

# Context & Application
Zones enable tracking directional trajectory. In the Tristan Prelude opening, alternation between zones 3 and 1 reveals balanced motion; when chord 6 enters zone 5, it signals modulation. In the Chopin Mazurka, zone 1 plays a central articulating role, represented by successive dominant seventh chords (G7, Bb7, Db7) that mark structural junctures.

# Examples
- **Tristan Prelude opening**: Zones alternate 3-1-3-1-3, balancing about B-D-F-G# diminished seventh (zone 2). Chord 6 jumps to zone 5 (pp. 171-172)
- **Chopin Mazurka Op. 68 no. 4**: Zone 1 returns at G7 (m. 33), Bb7 (m. 36), Db7 (m. 39), each marking structural juncture (pp. 178-179)
- **Chopin e minor Prelude**: First phrase descends 15 cumulative semitones through the zones (pp. 180-181)

# Relationships
## Builds Upon
- Voice-leading zone (triadic) (analogous concept, same computation, different system)
## Enables
- 4-Cube Trio navigation
- Tracking upshift/downshift trajectories
## Related
- Boretz region (balanced motion between adjacent zones)
- Circumnavigation (touring all six odd zones)
## Contrasts With
- Triadic voice-leading zones (same labels but "false friends" -- independent systems)

# Common Errors
- **Error**: Assuming tetrachordal zone 3 and triadic zone 3 refer to the same harmonic space
  **Correction**: They are "false friends" in independent, non-communicating systems

# Common Confusions
- **Confusion**: Thinking zone numbers have tonal significance
  **Clarification**: Zones describe voice-leading position only, not key or tonal center

# Source Reference
Cohn, R. *Audacious Euphony*, Chapter 7: "Dissonance," pp. 171-172, 177-179.

# Verification Notes
Re-extracted from v2 card; preserved: "false friends" warning, zone behavior types, Chopin examples. Fresh extraction adds direct quotation and computation method.
