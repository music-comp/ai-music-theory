---
concept: Vibrating Strings and Fret Positioning
slug: vibrating-strings-and-fret-positioning

category: pitch-and-intervals
subcategory: acoustics
tier: intermediate

source: "Mathematics and Music"
source_slug: maths-and-music
authors: "David Wright"
chapter: "Ratios and Musical Intervals"
chapter_number: 4
pdf_page: 58
section: "Vibration of Strings"

extraction_confidence: high

aliases:
  - string vibration and pitch
  - inverse proportionality of frequency and length

prerequisites:
  - interval-as-frequency-ratio
  - semitone-ratio
extends: []
related:
  - multiplicative-composition-of-intervals
contrasts_with: []

answers_questions:
  - "How does string length relate to pitch?"
  - "How do you calculate fret positions for a given interval?"
  - "Why are frets not equally spaced?"
---

# Quick Definition

The vibrating frequency of a string is inversely proportional to its length (F = k/L). To raise the pitch by an interval ratio r, a fret must be placed to shorten the string to a fraction q = r^(-1) of its original length.

# Core Definition

For a string of length L under constant tension and weight per unit length, the frequency F satisfies:

F = k/L    (Formula 4.5)

for some constant k in R+. If a fret divides the string so that the vibrating portion has length L' = qL, the new frequency F' satisfies:

F'/F = L/L' = 1/q    (Formula 4.6)

To achieve an upward interval with ratio r >= 1, the fret position must satisfy:

q = r^(-1)    (Formula 4.7)

(Wright, pp. 64-65)

# Prerequisites

- **Interval as Frequency Ratio** -- Must understand intervals as ratios to connect string length to interval
- **Semitone Ratio** -- Needed to calculate fret positions for chromatic intervals

# Key Properties

1. Frequency is inversely proportional to string length: F = k/L (Formula 4.5)
2. The interval ratio equals the inverse ratio of lengths: F'/F = L/L' (Formula 4.6)
3. For a fret at fraction q of the string length: interval ratio = 1/q (Formula 4.6)
4. To achieve ratio r: place fret at q = 1/r of the string length (Formula 4.7)
5. Frets are not equally spaced because equal intervals correspond to equal ratios of lengths, not equal differences
6. Frets get closer together as pitch rises (toward the bridge)

# Construction / Recognition

## To Position a Fret for a Given Interval

1. Determine the desired interval ratio r
2. Compute q = r^(-1) = 1/r
3. The fret should be placed at distance qL from the end of the string (where L is the total string length)
4. The vibrating portion has length qL

# Context & Application

The inverse proportionality of frequency and string length was known to the ancient Greeks and is fundamental to all stringed instruments. Fretted instruments (like guitars) have fixed fret positions; unfretted instruments (like violins) allow continuous pitch adjustment via finger placement. The non-equal spacing of frets on a guitar is a visible manifestation of the exponential relationship between pitch and string length (Wright, pp. 64-65).

# Examples

**Example 1** (p. 65): To raise pitch by a major third (r = 2^(1/3)): q = 2^(-1/3) ~ 0.7937. The fret is at about 79.37% of the string length, close to 4/5 of the length.

**Example 2**: To raise pitch by one semitone (r = 2^(1/12)): q = 2^(-1/12) ~ 0.9439.

**Example 3** (Exercise 6): A 50 cm banjo string's 12 frets for one chromatic octave would be non-equidistant, getting closer together as pitch rises.

# Relationships

## Builds Upon

- **Interval as Frequency Ratio** -- String length changes produce interval ratios
- **Semitone Ratio** -- Used to calculate chromatic fret positions

## Related

- **Multiplicative Composition of Intervals** -- Multiple fret positions compound multiplicatively

# Common Errors

- **Error**: Placing frets at equal distances for equal intervals
  **Correction**: Equal intervals require equal ratios of remaining length, not equal absolute distances

# Common Confusions

- **Confusion**: Thinking frequency is directly proportional to length
  **Clarification**: Frequency is inversely proportional to length (shorter string = higher pitch)
- **Confusion**: Assuming the formula F = k/L works regardless of tension
  **Clarification**: The constant k depends on the string's tension and linear density; the formula assumes these are held constant

# Source Reference

Chapter 4: "Ratios and Musical Intervals," pp. 64-65. Formulas 4.5, 4.6, and 4.7.

# Verification Notes

- Definition source: Direct from pp. 64-65, with three numbered formulas
- Confidence rationale: High -- explicit derivation with worked example
- Uncertainties: None
- Cross-reference status: All slugs verified
- Re-extraction notes: Re-extracted from v2 card; preserved: ancient Greeks reference, major third fret example (~4/5 length), non-equidistance observation, all three formula references
