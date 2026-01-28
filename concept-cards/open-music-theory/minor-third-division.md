---
concept: Minor Third Division
category: harmony
source: Open Music Theory
chapter: "Equal Divisions of the Octave"
part: 5
---

# Minor Third Division

## Quick Definition

Division of the octave into four equal parts of three semitones (minor thirds) each, producing the diminished seventh chord--with only three unique transpositions due to its symmetry at T(3), this division enables the famous enharmonic modulation technique where any note of a diminished seventh can be reinterpreted as the leading tone to a new key, allowing sudden modulation to four different tonal areas from a single chord, and also providing the structural basis for the octatonic scale (which combines two minor-third cycles a semitone apart).

## Formal Definition

**Minor third division** is the equal partition of the octave by minor thirds:

**Structure**:
- **Interval**: 3 semitones (minor third)
- **Division**: 12 semitones / 3 = 4 parts
- **Cardinality**: 4 pitch classes per collection
- **Unique collections**: 3 distinct diminished seventh chords

**Interval cycle**: C(3) -- traverses 4 pitch classes before returning

**Set class**: (0,3,6,9) -- the diminished seventh chord

**Mathematical properties**:
- Period of symmetry: T(3)
- Number of distinct forms: 12/4 = 3

## The Three Diminished Seventh Chords

**All unique diminished seventh chords**:
```
Dim7 chord 1: {C, Eb, Gb, A} = {0, 3, 6, 9}
              C°7 = Eb°7 = Gb°7 = A°7 (all same chord)

Dim7 chord 2: {C#, E, G, Bb} = {1, 4, 7, 10}
              C#°7 = E°7 = G°7 = Bb°7

Dim7 chord 3: {D, F, Ab, B} = {2, 5, 8, 11}
              D°7 = F°7 = Ab°7 = B°7
```

**Together they partition all 12 pitch classes**:
```
3 chords × 4 notes = 12 pitch classes total
Each pitch class belongs to exactly one dim7 chord
```

## Symmetry and Enharmonic Equivalence

**Each note can be the root**:
```
{C, Eb, Gb, A} can be spelled as:
- C°7: C - Eb - Gb - Bbb (A)
- Eb°7: Eb - Gb - Bbb - Dbb (C)
- Gb°7: Gb - Bbb - Dbb - Fbb (Eb)
- A°7: A - C - Eb - Gb

Same sound, four different notations
Each spelling suggests different resolution
```

**Resolution possibilities**:
```
Any note of a dim7 can act as leading tone (^7):

{C, Eb, Gb, A} (the chord):
- C as ^7 → resolves to Db major/minor
- Eb as ^7 → resolves to Fb (E) major/minor
- Gb as ^7 → resolves to Abb (G) major/minor
- A as ^7 → resolves to Bb major/minor

Four possible resolutions from ONE chord!
This enables modulation to keys a minor third apart
```

## Generating the Octatonic Scale

**Two dim7 chords a semitone apart**:
```
Dim7 chord 1: {C, Eb, Gb, A} = {0, 3, 6, 9}
Dim7 chord 2: {C#, E, G, Bb} = {1, 4, 7, 10}

Combined: {C, C#, Eb, E, Gb, G, A, Bb}
        = {0, 1, 3, 4, 6, 7, 9, 10}
        = OCT0,1 (octatonic collection)

The octatonic scale IS two interlocking dim7 chords
```

**Three ways to pair dim7 chords for octatonics**:
```
Chords 1 + 2: {0,3,6,9} + {1,4,7,10} = OCT0,1
Chords 2 + 3: {1,4,7,10} + {2,5,8,11} = OCT1,2
Chords 3 + 1: {2,5,8,11} + {0,3,6,9} = OCT2,3

Three pairs → three octatonic collections
```

## Voice Leading from Diminished Seventh

**Minimal motion resolutions**:
```
From C°7 = C - Eb - Gb - Bbb(A):

To C major (C as passing tone):
C → C, Eb → D, Gb → G, A → G
(Not the most common resolution)

As vii°7 of Db (C as leading tone):
C°7 → Db major: C → Db, Eb → Db, Gb → F, A → Ab

As vii°7 of E (Eb=D# as leading tone):  
C°7 → E major: C → B, D# → E, Gb → E, A → G#

Each resolution requires respelling
```

**Omnibus progression**:
```
The "omnibus" is a 19th-century voice-leading pattern
that cycles through dim7 chords chromatically:

Bb - D - F - G# (Bb°7)
↓
A - D - F - G# (incomplete)
↓
Ab - D - F - G# → respell → Ab - D - F - Ab... 

Creates chromatic lines while remaining dim7-based
```

## Minor Third Division in Root Motion

**Roots moving by minor third**:
```
Chord progression: C - Eb - Gb - A - C

All four roots form the diminished seventh chord
Progression cycles through all notes of one dim7

Each chord is equally "important"
No dominant-tonic function
Rapid traversal of distant keys
```

**With tonicization**:
```
C major - [V7/Eb] - Eb major - [V7/Gb] - Gb major - [V7/A] - A major - C major

Each key tonicized before moving to next
Creates sense of four equal tonal areas
19th-century modulatory technique
```

## Historical and Theoretical Context

**Baroque and Classical**:
- Dim7 as dominant substitute (vii°7)
- Resolves predictably to tonic
- Enharmonic potential occasionally exploited

**Romantic period**:
- Full exploitation of enharmonic ambiguity
- Modulation to distant keys via dim7 reinterpretation
- Chains of dim7 chords for suspense
- "Omnibus" and similar chromatic patterns

**20th century**:
- Dim7 as source of octatonic collection
- Structural use in Bartok's axis system
- Less "charged" harmonically in post-tonal context

## Musical Context

**Why three unique forms?**:
- Symmetry at T(3) means transposing by m3 gives same chord
- 12 pitch classes / 4 per chord = 3 distinct chords
- Each of the 12 transposition levels maps to one of three chords

**Modulatory power**:
- From any dim7, access four different keys
- All four are minor third apart
- Maximum modulatory flexibility from single sonority
- "Get-out-of-jail-free card" for distant modulation

**Relation to other divisions**:
- Contains two tritones: C-Gb and Eb-A in C°7
- Subset of octatonic (8 notes = 2 dim7 chords)
- Superset of tritone pairs

## Examples

### Basic

**The three dim7 chords spelled out**:
```
Starting from C (or any note of the chord):

Chord 1: C - Eb - Gb - A (roots at 0, 3, 6, 9)
        C°7 = Eb°7 = Gb/F#°7 = A°7

Chord 2: C# - E - G - Bb (roots at 1, 4, 7, 10)
        C#/Db°7 = E°7 = G°7 = Bb°7

Chord 3: D - F - Ab - B (roots at 2, 5, 8, 11)
        D°7 = F°7 = Ab°7 = B°7

Any transposition maps to one of these three
```

**Enharmonic modulation example**:
```
In C minor: vii°7 = B - D - F - Ab (dim7 chord 3)

Reinterpret Ab as G#, treat as leading tone to A:
G#°7 in A minor: G# - B - D - F → A minor

What happened:
Started in C minor (1 flat)
Ended in A minor (0 flats/sharps)
Used same dim7 chord, different spelling

Keys C, A, F#/Gb, Eb are all accessible from one dim7
```

**Minor third root cycle**:
```
C major → Eb major → Gb major → A major → C major

Four keys, roots form dim7 chord {C, Eb, Gb, A}
Each key a minor third from neighbors
Creates kaleidoscopic, centerless progression
```

### From Repertoire

**Mozart, Fantasy in C minor, K. 475**: Famous enharmonic reinterpretation of dim7 for remote modulation.

**Beethoven, "Appassionata" Sonata, development**: Extended dim7 sequences create harmonic tension.

**Schubert, String Quintet in C major, slow movement**: Enharmonic dim7 reinterpretation for expressive modulation.

**Chopin, Ballade in G minor**: Dim7 as pivot for modulatory passages.

**Wagner**: Systematic exploitation of dim7 ambiguity throughout the music dramas.

**Romantic opera generally**: Dim7 for dramatic suspense, resolution deferred.

## Related Concepts

- **Prerequisite**: minor-third, diminished-seventh-chord, enharmonic-equivalence
- **Leads to**: octatonic-scale, enharmonic-modulation, chromatic-harmony
- **See also**: equal-divisions-of-the-octave, tritone-division, symmetrical-pitch-collection

## Common Confusions

- Only THREE unique dim7 chords exist (not 4 or 12)
- Each dim7 has FOUR valid root spellings (any note can be root)
- The dim7 chord IS the interval-3 cycle C(3)
- Dim7 is NOT the same as half-diminished (m7b5/ø7)
- Two dim7 chords combine to make octatonic; one dim7 is not octatonic
- Each dim7 note can be leading tone to a different key (4 possible resolutions)
- Transposing dim7 by minor third gives the SAME chord
- The three dim7 chords together include all 12 pitch classes
- "Enharmonic modulation" via dim7 is respelling, not true transposition
- Dim7 contains TWO tritones (overlapping at distance of m3)

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Equal Divisions of the Octave"
Open Music Theory, Part V: "Reinterpreting Diminished Seventh Chords"
