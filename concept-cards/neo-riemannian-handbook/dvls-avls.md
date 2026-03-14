---
concept: DVLS / AVLS Measurements
category: analysis
source: Oxford Handbook of Neo-Riemannian Music Theories
chapter: "Inversional Symmetry and Voice Leading"
authors: "Dmitri Tymoczko"
pdf_page: null
chapter_number: null
unit: null
---

# DVLS / AVLS Measurements

## Quick Definition

Quantitative measures of voice-leading efficiency: DVLS (Displacement Voice-Leading Size) sums the semitone distances moved by all voices, while AVLS (Average Voice-Leading Size) divides this by the number of voices to give average motion per voice.

## Formal Definition

**DVLS (Displacement Voice-Leading Size)**:
The total number of semitones traveled by all voices in a voice leading.

Formula: DVLS = |d1| + |d2| + |d3| + ... + |dn|

Where di is the displacement (in semitones) of voice i.

**AVLS (Average Voice-Leading Size)**:
The average displacement per voice.

Formula: AVLS = DVLS / n

Where n is the number of voices.

**Example Calculation**:
For the voice leading C major (C-E-G) -> C minor (C-Eb-G):
- Voice 1: C -> C = 0 semitones
- Voice 2: E -> Eb = 1 semitone
- Voice 3: G -> G = 0 semitones
- DVLS = 0 + 1 + 0 = 1
- AVLS = 1/3 ≈ 0.33

## Theoretical Framework

**Voice-Leading Spaces**:
Tymoczko develops these measures as part of a geometric approach to voice leading. DVLS and AVLS quantify distances in "voice-leading space" - a mathematical space where:
- Points represent chords
- Distances represent voice-leading effort
- Efficient progressions correspond to short paths

**Properties**:
- DVLS measures total effort required
- AVLS normalizes for chord size (useful when comparing different cardinalities)
- Both are non-negative
- DVLS = 0 only when chords are identical
- Inversion preserves both measures (I-related voice leadings have equal DVLS/AVLS)

**Relationship to Neo-Riemannian Transformations**:
| Transformation | DVLS (triads) | AVLS |
|---------------|---------------|------|
| P | 1 | 0.33 |
| L | 1 | 0.33 |
| R | 2 | 0.67 |
| T1 | 3 | 1.00 |
| T5/T7 | 5 | 1.67 |

P and L are "maximally smooth" - they achieve the minimum DVLS for distinct triads.

## Musical Context

**Defining "Efficient" Voice Leading**:
Tymoczko uses these measures to formalize efficiency:
- "Semitonal" voice leading: AVLS ≤ 1 (no voice moves more than a semitone on average)
- "Stepwise" voice leading: AVLS ≤ 2 (no voice moves more than a whole step on average)

These definitions are style-agnostic and allow cross-repertoire comparison.

**Cataloging Voice Leadings**:
The 16 semitonal voice leadings between triads (Tymoczko's Table) all have DVLS ≤ 2:
- DVLS = 1: P and L transformations
- DVLS = 2: Two voices move by semitone (LP, PL, and similar compounds)

**Chromatic vs. Diatonic Efficiency**:
Interesting insight: Motion by perfect fifth (dominant-to-tonic) has relatively high DVLS:
- G major (G-B-D) -> C major (G-C-E): DVLS = 4 (B->C + D->C + D->E... depending on voicing)
- This explains why V-I is "strong" harmonically but not voice-leading-efficient
- Chromatic harmony (low DVLS) and functional harmony (different criteria) operate differently

## Examples

### Comparing Voice Leadings

**P transformation** (C major to C minor):
```
C -> C: 0
E -> Eb: 1
G -> G: 0
DVLS = 1, AVLS = 0.33
```

**R transformation** (C major to A minor):
```
C -> C: 0
E -> E: 0
G -> A: 2
DVLS = 2, AVLS = 0.67
```

**Tritone transposition** (C major to F# major):
```
C -> C#: 1
E -> F#: 2
G -> A#: 3
DVLS = 6, AVLS = 2.0
```

### Wagner Analysis Application

Tarnhelm motive: G# minor to E minor
```
G# -> G: 1
B -> B: 0
D# -> E: 1
DVLS = 2, AVLS = 0.67
```

This is efficient (DVLS = 2) but not maximally smooth (DVLS > 1).

### Seventh Chord Voice Leadings

The Tristan chord to dominant seventh:
```
{F, G#, B, D#} -> {F, A, C, Eb}
F -> F: 0
G# -> A: 1
B -> C: 1
D# -> Eb: 1 (enharmonic)
DVLS = 3, AVLS = 0.75
```

Extremely efficient for a four-note chord transformation.

## Related Concepts

- **Prerequisite**: voice-leading, interval, semitone
- **Leads to**: voice-leading-zones
- **See also**: voice-leading-efficiency, inversional-symmetry

## Common Confusions

1. **DVLS vs. number of moving voices**: DVLS counts total semitones, not number of active voices

2. **Direction independence**: DVLS uses absolute values - motion up equals motion down

3. **Voicing matters**: Different voicings of the same chord progression can have different DVLS

4. **AVLS comparability**: AVLS allows comparison between voice leadings of different sizes (triads vs. seventh chords)

## Analytical Applications

**Identifying Efficient Progressions**:
- DVLS ≤ 2 for triads: Highly efficient (includes P, L, R)
- DVLS ≤ 3 for seventh chords: Highly efficient
- Higher DVLS suggests different organizational logic (functional rather than voice-leading)

**Explaining Chromatic Patterns**:
Why do major-third related triads appear so often in chromatic music?
- C major to Ab major: DVLS = 2 (minimal for same-quality triads at that interval)
- C major to E major: DVLS = 2 (same)
- These are more efficient than fifth-related progressions

**Limitations**:
DVLS/AVLS measure one aspect of voice leading. They don't capture:
- Voice crossing/uncrossing
- Harmonic function
- Registral concerns
- Timbral factors

## Source Reference

Oxford Handbook of Neo-Riemannian Music Theories, Part II, Chapter 8: Dmitri Tymoczko, "Inversional Symmetry and Voice Leading"

Extended treatment in: Tymoczko, *A Geometry of Music* (Oxford University Press, 2011)
