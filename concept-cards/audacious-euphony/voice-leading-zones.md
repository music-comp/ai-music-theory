---
concept: Voice-Leading Zones
slug: voice-leading-zones

category: voice-leading
subcategory: equivalence classes
tier: advanced

source: "Audacious Euphony: Chromaticism and the Consonant Triad's Second Nature"
source_slug: audacious-euphony
authors: "Richard Cohn"
chapter: "A Unified Model of Triadic Voice-Leading Space"
chapter_number: 5
pdf_page: 120
section: "Voice-Leading Zones"

extraction_confidence: high

aliases:
  - "sum classes"
  - "voice-leading zone system"

prerequisites:
  - cube-dance-graph
  - hexatonic-weitzmann-interaction
  - pitch-class-sum
extends:
  - transformational-substitution
related:
  - transformation-class
  - zone-diametric-relations
  - clock-face-model
contrasts_with: []

answers_questions:
  - "What is a voice-leading zone?"
  - "How do voice-leading zones organize triadic space?"
  - "How do I determine the voice-leading zone of a triad?"
---

# Quick Definition
Twelve equivalence classes of trichords that share a center of balance, labeled 0-11 like a clock face, allowing voice-leading distance to be computed as subtraction modulo 12, with zone labels equal to the pitch-class sum of each triad.

# Core Definition
Voice-leading zones are "equivalence classes of trichords with the capacity to share a center of balance" (p. 121). Consonant triads share a zone if transpositionally related by major third (T4). The twenty-four consonant triads partition into eight zones; the four augmented triads occupy the remaining four zones, for twelve total. On Cube Dance, zone-equivalent triads occupy the same radius. On the Tonnetz, they occupy the same hexatonic strip with equivalently oriented triangles. The concept was first developed under the term "sum classes" (Cohn 1998b).

# Prerequisites
- **Cube Dance graph**: Zones are radial positions on Cube Dance
- **Hexatonic-Weitzmann interaction**: T4 equivalence arises from the intersection of the two systems
- **Pitch-class sum**: Zone number equals the pc sum mod 12

# Key Properties
1. 8 zones for consonant triads (3 triads per zone, T4-related)
2. 4 zones for augmented triads (1 triad per zone, at positions 0, 3, 6, 9)
3. 12 zones total, labeled 0-11 (clock face)
4. Zone number = sum of pitch classes, modulo 12
5. Voice-leading distance = |zone1 - zone2| mod 12, taking absolute value (with hexatonic pole as sole exception)
6. H-class operations move between zones X and X +/- 1
7. W-class operations move between zones Y and Y +/- 2

# Construction / Recognition
**Calculating zone from pitch-class sum:**
- d minor {D, F, A} = {2, 5, 9} -> 2+5+9 = 16 = 4 (mod 12) -> Zone 4
- G major {G, B, D} = {7, 11, 2} -> 7+11+2 = 20 = 8 (mod 12) -> Zone 8
- Distance: |8-4| = 4 units of voice-leading work

**Quick determination method** (p. 123):
1. Fix C augmented triad at zone 0 (piggyback on C=0 convention)
2. Secure remaining augmented triads to ascending multiples of 3
3. Each consonant triad's zone = which augmented triad it displaces, and in which direction
4. D minor is zone 4 because it upshifts FAC# at zone 3; G major is zone 8 because it downshifts GBEb at zone 9

# Context & Application
The concept is previewed in Chapter 2 (pp. 44-46) where hexatonic cycles show that the center of balance "toggles back and forth between two pitches separated by one-third of a semitone" (p. 45), producing a "prolongation not of a tonic, in any standard construal of the term, but rather of a zone of voice-leading space." As Cohn writes: "Like a walker or a waterfall, the incessant local fluctuations are underlain by a global stasis" (p. 45). This arises from "the quasi-uniform size of the intervals of which the triad is composed, which fall within a narrow compass of from three to five semitones."

Voice-leading zones provide the fundamental abstraction enabling all analytical work in chapters 5-6. They allow tracking voice-leading trajectories at a level that transcends specific triadic choices, enabling the identification of compositional scripts (neighborhood, departure-return, continuous upshift). The zone system also enables precise measurement of disjunction and entropy.

# Examples
- **Figure 5.24** (p. 122): The twelve voice-leading zones depicted on a clock face superimposed over Cube Dance
- **Distance example** (p. 121): d minor (zone 4) to G major (zone 8) = 4 voice-leading units; this holds for any of the 9 pairs of triads from these zones
- **Equivalence economy** (p. 122): "What is proper to one pair is ipso facto true of the other eight. Behold the beautiful cognitive economy of equivalence!"

# Relationships
## Builds Upon
- Cube Dance (zones are radial positions)
- Transformational substitution (T4 equivalence underlies zone equivalence)
## Enables
- Transformation classes (zone-shifting equivalence classes for operations)
- Zone-diametric relations and disjunction analysis
- All compositional scripts in chapter 6
## Related
- Clock face model (the labeling scheme)
- Pitch-class sum (the computational method)
## Contrasts With
- Individual triad identity (zones abstract away from specific triads)

# Common Errors
- **Error**: Zone number is the root pitch class
  **Correction**: Zone number is the pitch-class SUM of all three notes, mod 12
- **Error**: Distance formula always works
  **Correction**: The hexatonic pole is the sole exception to distance = zone difference, due to its contrary motion (p. 121n15)

# Common Confusions
- **Confusion**: Zones are keys or tonal centers
  **Clarification**: Zones are defined by voice-leading geometry (center of balance), not by tonal function; functional interpretations may be layered on but are not intrinsic
- **Confusion**: Augmented triads have the same kind of zone membership as consonant triads
  **Clarification**: Augmented triads each occupy their own zone (at multiples of 3); consonant triads share zones in T4-related triplets

# Source Reference
Cohn, Richard. *Audacious Euphony*, Chapter 5, pp. 120-124, Figure 5.24.

# Verification Notes
Re-extracted from v2 card (voice-leading-zones.md); preserved: calculation examples, quick determination method, equivalence economy quote. High confidence -- the most thoroughly defined concept in ch5.
Consolidated content from v2 card [voice-leading-zone]: added Chapter 2 preview material (pp. 44-46) -- "toggles between two pitches" quote, "walker or waterfall" analogy, "global stasis" concept, quasi-uniform interval size rationale.
Consolidated content from v2 card [zone-distance-calculation]: No unique content to consolidate (calculation method, d minor/G major example, quick determination, hexatonic pole exception all already present).
