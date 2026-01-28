---
concept: Transpositional Equivalence
category: harmony
source: Open Music Theory
chapter: "Equal Divisions of the Octave"
part: 5
---

# Transpositional Equivalence

## Quick Definition

The property whereby transposing a pitch collection by a certain interval produces the same set of pitch classes (though starting on a different note)--this occurs in symmetrical collections where the interval pattern repeats within the octave, such as the whole-tone scale (equivalent under T2), octatonic scale (equivalent under T3), and augmented triad (equivalent under T4), resulting in fewer than 12 unique transpositions and contributing to the tonal ambiguity characteristic of these collections.

## Formal Definition

**Transpositional equivalence** occurs when a pitch-class set S maps onto itself under transposition by n semitones:

**Notation**: T(n)(S) = S

**This means**: Adding n semitones to every pitch class in S produces the same collection of pitch classes (possibly in different order).

**Consequences**:
1. The set is "transpositionally symmetric" at interval n
2. Fewer than 12 distinct transpositions exist
3. n is called the "period" of the symmetry

**Formula**:
- Number of unique transpositions = 12 / period
- Where period = smallest n > 0 such that T(n)(S) = S

## Examples of Transpositional Equivalence

### Whole-Tone Scale (Period 2)
```
WT0 = {C, D, E, F#, G#, A#} = {0, 2, 4, 6, 8, 10}

T(2)(WT0) = {D, E, F#, G#, A#, C} = {2, 4, 6, 8, 10, 0} = WT0 ✓

The collection maps onto itself under T(2)
Period = 2
Unique transpositions = 12/2 = 6... 

But wait: 6 transposition levels, yet only 2 distinct collections!
That's because transposing by 2, 4, 6, 8, or 10 all give WT0
Transposing by 1, 3, 5, 7, 9, or 11 all give WT1
```

### Octatonic Scale (Period 3)
```
OCT0,1 = {C, C#, D#, E, F#, G, A, Bb} = {0, 1, 3, 4, 6, 7, 9, 10}

T(3)(OCT0,1) = {3, 4, 6, 7, 9, 10, 0, 1} = OCT0,1 ✓

Period = 3
Unique transpositions = 3 distinct collections
```

### Augmented Triad (Period 4)
```
C+ = {C, E, G#} = {0, 4, 8}

T(4)(C+) = {E, G#, C} = {4, 8, 0} = C+ ✓

Period = 4
Unique transpositions = 4 distinct augmented triads
```

### Diminished Seventh Chord (Period 3)
```
C°7 = {C, Eb, Gb, A} = {0, 3, 6, 9}

T(3)(C°7) = {Eb, Gb, A, C} = {3, 6, 9, 0} = C°7 ✓

Period = 3
Unique transpositions = 3 distinct diminished seventh chords
```

### Tritone (Period 6)
```
{C, F#} = {0, 6}

T(6)({0, 6}) = {6, 0} = {0, 6} ✓

Period = 6
Unique transpositions = 6 tritone pairs
```

## Mathematical Framework

**Group theory perspective**:
```
The set of transpositions forms a cyclic group Z12
Transpositionally equivalent sets have non-trivial stabilizers
The stabilizer is the subgroup of transpositions that fix the set

For whole-tone: stabilizer = {T0, T2, T4, T6, T8, T10} ≅ Z6
For octatonic: stabilizer = {T0, T3, T6, T9} ≅ Z4
For augmented triad: stabilizer = {T0, T4, T8} ≅ Z3
```

**Orbit-stabilizer relationship**:
```
|orbit| × |stabilizer| = 12

Whole-tone: 2 collections × 6 stabilizing transpositions = 12 ✓
Octatonic: 3 collections × 4 stabilizing transpositions = 12 ✓
Augmented: 4 triads × 3 stabilizing transpositions = 12 ✓
```

## Contrast: Asymmetric Collections

**Diatonic collection (major scale)**:
```
C major = {C, D, E, F, G, A, B} = {0, 2, 4, 5, 7, 9, 11}

T(1)(C major) = {1, 3, 5, 6, 8, 10, 0} = C# major ≠ C major
T(2)(C major) = {2, 4, 6, 7, 9, 11, 1} = D major ≠ C major
...and so on

No non-zero transposition maps C major onto itself
Period = 12 (or undefined)
Unique transpositions = 12 (all different keys)
The diatonic collection is NOT transpositionally equivalent at any level < 12
```

**Major triad**:
```
C major = {C, E, G} = {0, 4, 7}

T(4)(C major) = {4, 8, 11} = E major ≠ C major
(E major = {E, G#, B} = {4, 8, 11})

Major triads have 12 unique transpositions
No transpositional equivalence
```

## Musical Implications

**For symmetric collections**:

1. **Tonal ambiguity**: Any pitch could be "tonic" since relationships are uniform
2. **Reduced modulatory distance**: Fewer distinct key areas to move between
3. **Enharmonic reinterpretation**: Same pitches, different spelling = different function
4. **Limited harmonic vocabulary**: Same material recycled at different transpositions

**For asymmetric collections**:

1. **Tonal clarity**: Each transposition is distinct, supporting key relationships
2. **Full chromatic space**: 12 different keys provide maximum variety
3. **Functional harmony**: Distinct positions enable dominant-tonic relationships
4. **Hierarchical structure**: Some positions privileged over others

## Historical and Theoretical Context

**19th-century chromaticism**: Composers discovered that symmetric collections allow smooth modulation without clear cadential confirmation, creating extended passages of harmonic ambiguity.

**Messiaen's "modes of limited transposition"**: Codified the idea that certain scales have fewer transpositions, viewing this as theologically meaningful.

**Pitch-class set theory**: Formalizes transpositional equivalence as a fundamental property distinguishing set classes.

**Neo-Riemannian theory**: Exploits transpositional equivalence of augmented and diminished sonorities for parsimonious voice leading.

## Examples

### Basic

**Demonstrating equivalence in whole-tone**:
```
Start: C-D-E-F#-G#-A# (WT0)
Add 2 to each: D-E-F#-G#-A#-C

Reordering: C-D-E-F#-G#-A# = D-E-F#-G#-A#-C
Same pitch classes, just reordered
This is transpositional equivalence
```

**Demonstrating non-equivalence in major scale**:
```
Start: C-D-E-F-G-A-B (C major)
Add 2 to each: D-E-F#-G-A-B-C#

C major: C-D-E-F-G-A-B (contains F natural)
D major: D-E-F#-G-A-B-C# (contains F#)

Different pitch content!
Not transpositionally equivalent
```

**Counting unique transpositions**:
```
Question: How many distinct octatonic collections exist?

Method: OCT has period 3 (symmetric under T3)
Answer: 12 semitones / 3 period = 4 equivalence classes

But each class has 4 positions mapping to itself...
So: 12 total positions / 4 positions per class = 3 distinct collections ✓
```

### From Repertoire

**Debussy, "Voiles"**: Uses only two whole-tone collections (WT0 and WT1), exploiting transpositional equivalence to create seamless modulation between the only two available "keys."

**Stravinsky, _Petrushka_**: Octatonic passages can shift by minor thirds without changing the underlying collection, enabling rapid harmonic motion without leaving the octatonic "key."

**Liszt, late works**: Augmented triads' transpositional equivalence allows any member to function as root, enabling smooth chromatic progressions.

**Wagner, chromatic sequences**: Diminished seventh chord's equivalence under T3 enables enharmonic modulation--same chord, respelled, resolves to distant key.

## Related Concepts

- **Prerequisite**: transposition, pitch-class, interval, octave-equivalence
- **Leads to**: modes-of-limited-transposition, set-class, symmetrical-pitch-collection
- **See also**: equal-divisions-of-the-octave, enharmonic-equivalence, interval-cycle

## Common Confusions

- Transpositional equivalence is NOT the same as enharmonic equivalence (though related)
- Enharmonic: same PITCH, different SPELLING (F# vs Gb)
- Transpositional: same COLLECTION after adding interval (WT0 + M2 = WT0)
- A set is equivalent "at" an interval, not equivalent "to" another transposition level
- Major triads are NOT transpositionally equivalent (all 12 are distinct)
- Diminished triads are also NOT transpositionally equivalent (despite having 4 diminished sevenths)
- The "period" is the smallest non-zero interval giving equivalence
- Number of unique forms = 12 / period (for pitch-class sets)
- Transpositional equivalence causes "limited transposition" (Messiaen's term)
- This is a property of SETS, not individual pitches
- Asymmetric sets enable tonal function; symmetric sets undermine it

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Equal Divisions of the Octave"
Open Music Theory, Part VIII: "Collections"
