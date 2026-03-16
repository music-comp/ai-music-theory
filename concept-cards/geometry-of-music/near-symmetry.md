---
concept: Near Symmetry
slug: near-symmetry

category: geometric-theory
subcategory: chord-structure
tier: intermediate

source: "A Geometry of Music"
source_slug: geometry-of-music
authors: "Dmitri Tymoczko"
chapter: "A Geometry of Chords"
chapter_number: 3
pdf_page: 109
section: "3.8-3.10"

extraction_confidence: high

aliases:
  - "approximate symmetry"
  - "near transpositional symmetry"

prerequisites:
  - nearly-even-chords
  - three-note-chord-space
extends: []
related:
  - harmonic-consistency-and-efficient-voice-leading
  - voice-leading-lattices
contrasts_with: []

answers_questions:
  - "What does 'near symmetry' mean for chords?"
  - "Why does near symmetry enable efficient voice leading?"
  - "How does a chord's proximity to a perfectly symmetric chord determine its voice-leading properties?"
---

# Quick Definition
Chords that are close to perfectly symmetric (transpositionally self-symmetric) chords. Their near symmetry enables efficient voice leading to transpositions at the symmetric chord's interval of symmetry — major third for triads (near augmented), minor third for seventh chords (near diminished seventh).

# Core Definition
A perfectly symmetric n-note chord divides the octave into n exactly equal parts and is unchanged by transposition by 12/n semitones. A nearly symmetric chord is close (in chord space) to such a perfectly symmetric chord. This proximity means that the nearly symmetric chord can be linked to its 12/n-semitone transposition by efficient voice leading — specifically, by a short path that crosses through or near the center of chord space. For three-note chords, the symmetry interval is 4 semitones (major third), because augmented triads divide the octave into three equal parts. For four-note chords, it is 3 semitones (minor third), because diminished seventh chords divide the octave into four equal parts. The decomposition of efficient voice leading into purely contrary and small parallel components (Section 3.10) shows that near-symmetry accounts for the prevalence of major-third root relations among triads and minor-third root relations among seventh chords.

# Prerequisites
- Nearly even chords and their position in chord space
- Three-note chord space and the central lattice

# Key Properties
1. Near the center of chord space, close to a perfectly symmetric chord
2. Can be linked by efficient voice leading to transpositions at the symmetry interval
3. For triads (3-note): near augmented triads, linked efficiently at major-third transposition
4. For seventh chords (4-note): near diminished sevenths, linked efficiently at minor-third transposition
5. Adding a small parallel component extends connections to nearby transpositions (e.g., perfect fifth for triads)

# Construction / Recognition
## To Identify:
1. Compare the chord to the nearest perfectly even chord of the same cardinality
2. Determine the interval of symmetry (12/n semitones for n-note chords)
3. The available efficient transpositions cluster around this symmetry interval

# Context & Application
Near symmetry explains a central fact about tonal music: triads tend to be connected by root motions of major thirds (and nearby intervals like perfect fourths and fifths), while seventh chords favor minor-third root motions (and tritone substitutions). This is not arbitrary convention but a geometric consequence of the chords' positions in their respective chord spaces. Chapters 6 and 8 use this principle extensively to analyze chromatic music.

# Examples
**Example 1** (p. 109): Figure 3.8.8 shows that in three-note chord space, nearly even chords near the center connect to their major-third transpositions by pure contrary motion.
**Example 2** (p. 117-118): Nirvana's "Heart-Shaped Box" switches from major-third root motion (triads) to minor-third root motion when it introduces a seventh chord — a direct consequence of near symmetry changing from 3-fold to 4-fold.
**Example 3** (p. 120): Figure 3.10.8 shows the pure contrary transpositions for chords of 2 through 5 notes, determined by near symmetry in each dimension.

# Relationships
## Builds Upon
- **nearly-even-chords** — Near symmetry is a property of nearly even chords
- **three-note-chord-space** — Where triadic near symmetry is visible
## Enables
- **voice-leading-lattices** — The lattices encode the consequences of near symmetry
## Related
- **harmonic-consistency-and-efficient-voice-leading** — Near symmetry is the mechanism

# Common Errors
- **Error**: Assuming near symmetry means the chord is literally symmetric
  **Correction**: Near symmetry means the chord is *close to* a symmetric chord but not identical to it. A major triad is close to an augmented triad but is not itself symmetric.

# Common Confusions
- **Confusion**: Why do triads favor major-third relations but seventh chords favor minor thirds?
  **Clarification**: Because 12/3=4 (major third) for triads near augmented, and 12/4=3 (minor third) for seventh chords near diminished seventh. The geometry dictates different transposition intervals for different chord sizes.

# Source Reference
Chapter 3: A Geometry of Chords, Sections 3.8 and 3.10, pages 109, 113-121.

# Verification Notes
- Definition source: Synthesized from Sections 3.8 and 3.10
- Confidence rationale: High — central theoretical result with extensive musical evidence
- Cross-reference status: Verified against Nirvana, Brahms, Schumann examples
