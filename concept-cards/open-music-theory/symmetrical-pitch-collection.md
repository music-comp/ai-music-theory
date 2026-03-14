---
concept: Symmetrical Pitch Collection
category: theory
source: Open Music Theory
chapter: "Equal Divisions of the Octave"
pdf_page: null
chapter_number: 5
unit: null
authors: "Open Music Theory contributors"
---

# Symmetrical Pitch Collection

## Quick Definition

A set of pitch classes that maps onto itself under one or more operations of transposition or inversion--such collections exhibit internal regularity that eliminates the hierarchical distinctions between pitches found in asymmetrical scales like major and minor, creating inherent ambiguity about which pitch should serve as tonic and becoming foundational materials for 20th-century composers seeking alternatives to functional tonality while still working with recognizable intervallic patterns.

## Formal Definition

**Symmetrical pitch collection** is a pitch-class set that is invariant under certain transformations:

**Types of symmetry**:

1. **Transpositional symmetry**: Collection maps onto itself when transposed by interval i
   - Written: T(i) where T(i)(collection) = collection
   - Collection is "transpositionally symmetric at level i"

2. **Inversional symmetry**: Collection maps onto itself under inversion around some axis
   - The collection is its own inversion
   - Has an "axis of symmetry" in pitch-class space

**Degree of symmetry**: Number of transpositional and/or inversional operations that map the collection onto itself.

## Transpositional Symmetry

**Collections with transpositional symmetry**:

| Collection | Symmetric at | Transpositions | Unique Forms |
|------------|--------------|----------------|--------------|
| Chromatic | T(1) | Every level | 1 |
| Whole-tone | T(2) | 0, 2, 4, 6, 8, 10 | 2 |
| Octatonic | T(3) | 0, 3, 6, 9 | 3 |
| Hexatonic | T(4) | 0, 4, 8 | 4 |
| Dim7 chord | T(3) | 0, 3, 6, 9 | 3 |
| Aug triad | T(4) | 0, 4, 8 | 4 |
| Tritone | T(6) | 0, 6 | 6 |

**Formula for unique forms**:
```
Number of unique transpositions = 12 / (period of symmetry)

Whole-tone: period 2, 12/2 = 6 positions, but 6/3 pairs = 2 unique
Octatonic: period 3, 12/3 = 4 positions... actually 3 unique
The formula: 12 / (cardinality / period) or simply GCD relationships
```

## Inversional Symmetry

**Collections with inversional symmetry**:

All the above transpositionally symmetric collections are ALSO inversionally symmetric.

**Axis of inversion**:
```
For a collection to be inversionally symmetric, there must be an axis
(a pitch or point between pitches) around which the collection reflects.

Example - C major triad: C - E - G
Invert around Eb/E axis: C→F#, E→D, G→B = {F#, D, B} ≠ original
C major triad is NOT inversionally symmetric

Example - Diminished 7th: C - Eb - Gb - A
Multiple axes of inversional symmetry exist
Diminished 7th IS inversionally symmetric
```

## Asymmetry Creates Tonality

**Why diatonic scales are asymmetrical**:
```
C major scale: C - D - E - F - G - A - B
Interval pattern: 2 - 2 - 1 - 2 - 2 - 2 - 1

The two half steps are positioned asymmetrically:
- E-F (between ^3 and ^4)
- B-C (between ^7 and ^1)

This asymmetry creates:
- Unique position for each scale degree
- Leading tone function (^7 → ^1)
- Tritone between ^4 and ^7 resolving to ^3 and ^1
- Tonal hierarchy with clear tonic
```

**Why symmetric scales lack tonality**:
```
Whole-tone scale: C - D - E - F# - G# - A#
Interval pattern: 2 - 2 - 2 - 2 - 2 - 2

Every note has identical intervallic surroundings:
- No unique position
- No leading tone
- No distinguishable tonic
- All notes equally "stable" or "unstable"
```

## Properties of Symmetrical Collections

**Common characteristics**:

1. **Limited transposition**: Fewer than 12 unique transpositions
2. **Ambiguous tonality**: No pitch naturally functions as tonic
3. **Equal interval content**: More uniform distribution of intervals
4. **Enharmonic flexibility**: Can be respelled multiple ways
5. **Smooth voice leading**: Often allow parsimonious connections

**Mathematical properties**:
```
A collection C is transpositionally symmetric at level n if:
T(n)(C) = C, where T(n) adds n semitones to each pitch class

The "period" of a symmetric collection:
Smallest non-zero n such that T(n)(C) = C

Number of distinct transpositions = 12 / period
```

## Hierarchy of Symmetry

**From most to least symmetrical**:

```
1. Chromatic aggregate (12 notes)
   - Symmetric at every level
   - Only 1 unique form
   
2. Whole-tone (6 notes)
   - Symmetric at T(2)
   - 2 unique forms
   
3. Octatonic (8 notes) / Diminished 7th (4 notes)
   - Symmetric at T(3)
   - 3 unique forms
   
4. Hexatonic (6 notes) / Augmented triad (3 notes)
   - Symmetric at T(4)
   - 4 unique forms
   
5. Tritone dyad (2 notes)
   - Symmetric at T(6)
   - 6 unique forms

6. Diatonic (7 notes)
   - NO transpositional symmetry
   - 12 unique transpositions (one for each key)
```

## Musical Context

**Why composers use symmetrical collections**:

1. **Escape from tonality**: Symmetric collections don't point to a tonic
2. **New organizational systems**: Replace tonal hierarchy with intervallic consistency
3. **Coloristic effects**: Distinctive sound worlds (whole-tone = dreamy, octatonic = magical)
4. **Modulatory freedom**: Move between transpositions without functional "rules"
5. **Motivic consistency**: Same intervals available at all transposition levels

**20th-century significance**:
- Debussy: Whole-tone for Impressionist ambiguity
- Stravinsky, Bartok: Octatonic as structural basis
- Messiaen: "Modes of limited transposition" = transpositionally symmetric scales
- Twelve-tone music: Uses the most symmetric collection (chromatic aggregate)

## Examples

### Basic

**Testing for transpositional symmetry**:
```
Is {C, E, G#} transpositionally symmetric?

Transpose by 1: {C#, F, A} - different set, NO
Transpose by 2: {D, F#, A#} - different set, NO
Transpose by 3: {D#, G, B} - different set, NO
Transpose by 4: {E, G#, C} = {C, E, G#} - SAME SET, YES!

{C, E, G#} is symmetric at T(4)
Period = 4
Unique transpositions = 12/4 = 3... wait, but augmented triads have 4 unique forms

Actually: 12 starting points / 3 notes that map to same = 4 distinct aug triads
```

**Comparing symmetric and asymmetric**:
```
C major scale (asymmetric):
Intervals from C: 0-2-4-5-7-9-11
Each scale degree has UNIQUE intervallic context

Whole-tone scale (symmetric):
Intervals from C: 0-2-4-6-8-10
OR from D: 2-4-6-8-10-0 (same pattern)
OR from E: 4-6-8-10-0-2 (same pattern)
Every note has IDENTICAL intervallic context
```

### From Repertoire

**Debussy, "Voiles"**: Built almost entirely on whole-tone collection, creating suspended, directionless quality that matches the title's ambiguity (sails? veils?).

**Stravinsky, _The Firebird_**: Octatonic for supernatural elements (symmetric = otherworldly), diatonic for human characters (asymmetric = grounded).

**Bartok, "Mikrokosmos"**: Systematic exploration of symmetric and asymmetric collections, including synthetic scales.

**Messiaen, _Quartet for the End of Time_**: Uses modes of limited transposition (transpositionally symmetric scales) for theological symbolism--limited transposition suggests eternity/infinity.

**Liszt, late works**: Augmented triads (symmetric) as structural sonorities challenging the primacy of asymmetric major/minor.

## Related Concepts

- **Prerequisite**: pitch-class, transposition, inversion, interval
- **Leads to**: modes-of-limited-transposition
- **See also**: whole-tone-scale, octatonic-scale, hexatonic-scale, equal-divisions-of-the-octave

## Common Confusions

- "Symmetrical" in music theory means TRANSPOSITIONAL and/or INVERSIONAL symmetry
- NOT the same as "bilateral symmetry" or "mirror image" in the everyday sense
- Symmetry is about pitch CLASSES, ignoring octave placement
- Diatonic collections are NOT transpositionally symmetric (12 unique keys)
- BUT major scale IS inversionally symmetric (around ^2/^6 axis in some sense)
- More symmetry = fewer unique transpositions = more ambiguity
- Chromatic aggregate is maximally symmetric (only 1 form) AND maximally ambiguous
- Tritone is symmetric (maps onto itself at T(6)) even though it's only 2 notes
- Symmetry explains WHY these collections lack tonal pull
- "Limited transposition" IS transpositional symmetry (Messiaen's term)

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Equal Divisions of the Octave"
Open Music Theory, Part VIII: "Collections"
