---
concept: Distance Model
category: harmony
source: Open Music Theory
chapter: "Equal Divisions of the Octave"
part: 5
---

# Distance Model

## Quick Definition

A method of generating pitch collections by alternating between two fixed intervals in a repeating pattern, notated as i:j where i and j are the alternating intervals in semitones--for example, 1:2 produces the octatonic scale (alternating half and whole steps), 1:3 produces the hexatonic scale (alternating semitones and minor thirds), and this systematic approach to scale construction was particularly important to Bela Bartok and other 20th-century composers seeking alternatives to diatonic organization.

## Formal Definition

**Distance model** (interval model) is a method of constructing pitch collections using an alternating pattern of two intervals:

**Notation**: i:j
- i = first interval (in semitones)
- j = second interval (in semitones)
- Pattern: i, j, i, j, i, j, ... repeating

**Properties**:
- Collection size depends on when pattern completes an octave
- i + j determines the period of symmetry
- Collections are transpositionally symmetric at interval (i + j)

**Common distance models**:
| Model | Intervals | Result | Notes |
|-------|-----------|--------|-------|
| 1:1 | m2, m2 | Chromatic scale | Trivial |
| 2:2 | M2, M2 | Whole-tone scale | Same as 2:0 |
| 1:2 | m2, M2 | Octatonic scale | 8 notes |
| 2:1 | M2, m2 | Octatonic scale | Different rotation |
| 1:3 | m2, m3 | Hexatonic scale | 6 notes |
| 3:1 | m3, m2 | Hexatonic scale | Different rotation |
| 1:5 | m2, P4 | Bartok's favorite | 4 notes per cycle |

## Generating Collections with Distance Models

### 1:2 Model (Octatonic)
```
Pattern: half step, whole step, half step, whole step...

Starting on C:
C -[1]-> C# -[2]-> D# -[1]-> E -[2]-> F# -[1]-> G -[2]-> A -[1]-> Bb -[2]-> C

Result: C - C# - D# - E - F# - G - A - Bb (8 notes)
This is the octatonic scale (half-whole form)
```

### 2:1 Model (Octatonic, different rotation)
```
Pattern: whole step, half step, whole step, half step...

Starting on C:
C -[2]-> D -[1]-> D# -[2]-> F -[1]-> F# -[2]-> G# -[1]-> A -[2]-> B -[1]-> C

Result: C - D - D# - F - F# - G# - A - B (8 notes)
Same pitches as 1:2 starting on C#, different rotation
```

### 1:3 Model (Hexatonic)
```
Pattern: half step, minor third, half step, minor third...

Starting on C:
C -[1]-> C# -[3]-> E -[1]-> F -[3]-> G# -[1]-> A -[3]-> C

Result: C - C# - E - F - G# - A (6 notes)
This is the hexatonic scale
```

### 1:5 Model (Bartok tetrachord)
```
Pattern: half step, perfect fourth, half step, perfect fourth...

Starting on C:
C -[1]-> C# -[5]-> F# -[1]-> G -[5]-> C

Result: C - C# - F# - G (4 notes)
Set class (0167)
Bartok's "Z-cell"
```

## Mathematical Properties

**Collection size**:
```
For model i:j:
Total semitones = i + j (per cycle of two intervals)
Number of cycles to complete octave = 12 / GCD(i+j, 12)
Notes per cycle = 2
Total notes = 2 × (12 / GCD(i+j, 12)) / ((i+j) / GCD(i+j, 12))

Simplified: if i + j divides 12 evenly, notes = 24/(i+j)

Examples:
1:2 model: i+j = 3, notes = 24/3 = 8 ✓
1:3 model: i+j = 4, notes = 24/4 = 6 ✓
1:5 model: i+j = 6, notes = 24/6 = 4 ✓
```

**Symmetry period**:
```
Collections from i:j model are symmetric at interval (i + j)

1:2 model: symmetric at T(3) - octatonic property
1:3 model: symmetric at T(4) - hexatonic property
1:5 model: symmetric at T(6) - tritone symmetry
```

## Relationship to Equal Divisions

**Distance models and equal divisions**:
```
Model 1:2 (octatonic):
- Sum = 3 semitones (minor third)
- 4 × 3 = 12 (four minor thirds span octave)
- Built on minor-third equal division

Model 1:3 (hexatonic):
- Sum = 4 semitones (major third)
- 3 × 4 = 12 (three major thirds span octave)
- Built on major-third equal division

Model 1:5 (Z-cell):
- Sum = 6 semitones (tritone)
- 2 × 6 = 12 (two tritones span octave)
- Built on tritone equal division
```

## Bartok's 1:5 Model

**The "Z-cell" or (0167) tetrachord**:
```
Pattern: 1:5:1(:5 would continue...)

C - C# - F# - G forms set class (0167)
Contains two tritones: C-F# and C#-G
Inversionally symmetric around Eb/E axis

Bartok used this as a fundamental building block:
- Two (0167) sets can combine to form octatonic
- Partitions the octatonic into two tetrachords
```

**In Bartok's "From the Island of Bali"**:
```
Right hand: B - C - F - G (0167 starting on B)
Left hand: E - F - Bb - B (0167 starting on E)

Combined: B - C - E - F - F# - G - Bb - (B)
This is an octatonic collection!

Two 1:5 cells combine to create 1:2 (octatonic)
```

## Musical Context

**Why distance models?**:

1. **Systematic construction**: Precise method for generating scales
2. **Symmetry**: All distance model collections are transpositionally symmetric
3. **Interval consistency**: Repeating patterns create intervallic unity
4. **Compositional resource**: Different models suggest different motives
5. **Analytical tool**: Identify underlying intervallic structure

**Association with Bartok**:
- Bartok systematically explored distance model collections
- Used them as melodic and harmonic material
- Combined cells from different transpositions
- Created "axis systems" based on symmetric divisions

**Other 20th-century uses**:
- Messiaen's modes of limited transposition share properties
- Twelve-tone composers use interval cycles
- Spectral composers relate models to overtone series

## Examples

### Basic

**Building 1:2 vs 2:1**:
```
1:2 starting on C (half-whole):
C -1-> Db -2-> Eb -1-> E -2-> F# -1-> G -2-> A -1-> Bb -2-> C
Intervals from C: 0, 1, 3, 4, 6, 7, 9, 10

2:1 starting on C (whole-half):
C -2-> D -1-> Eb -2-> F -1-> Gb -2-> Ab -1-> A -2-> B -1-> C
Intervals from C: 0, 2, 3, 5, 6, 8, 9, 11

Different rotations of the same octatonic collection!
```

**Combining two 1:5 cells**:
```
Cell 1 (starting C): C - Db - Gb - G = {0, 1, 6, 7}
Cell 2 (starting Eb): Eb - E - A - Bb = {3, 4, 9, 10}

Combined: {0, 1, 3, 4, 6, 7, 9, 10}
This is OCT0,1!

The octatonic can be partitioned into two (0167) cells
```

**Identifying distance model in a passage**:
```
Given melody: C - C# - E - F - G# - A - C

Intervals: 1 - 3 - 1 - 3 - 1 - 3
Pattern: 1:3 (alternating semitone and minor third)

This is the hexatonic collection, model 1:3
```

### From Repertoire

**Bartok, "From the Island of Bali"**: Opens with two hands playing different (0167) tetrachords (1:5 model), combining to form octatonic.

**Bartok, _Music for Strings, Percussion, and Celesta_**: Extensive use of octatonic (1:2 model) and symmetric axis-based structures.

**Bartok, "Mikrokosmos"**: Systematic exploration of various scale types including distance model collections.

**Stravinsky, _The Rite of Spring_**: Octatonic passages (1:2 model) create the famous "primitive" harmonic language.

**Messiaen**: Modes 2-7 can be understood as distance models or extensions thereof.

## Related Concepts

- **Prerequisite**: interval, semitone, pitch-collection
- **Leads to**: octatonic-scale, hexatonic-scale, bartok-axis-system
- **See also**: equal-divisions-of-the-octave, symmetrical-pitch-collection, modes-of-limited-transposition

## Common Confusions

- Distance model i:j means alternating i semitones then j semitones, repeating
- The ORDER matters: 1:2 and 2:1 are different rotations of octatonic
- The SUM (i + j) determines the period of transpositional symmetry
- Distance models always produce transpositionally symmetric collections
- 1:1 model just produces chromatic scale (trivial case)
- 2:2 model produces whole-tone scale (equivalent to just repeating 2)
- The 1:5 model produces only 4 notes (not 6 or 8)
- Two (0167) cells combine to make octatonic, not vice versa
- Not all symmetric collections come from simple distance models
- "Distance model" is primarily associated with Bartok scholarship

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Equal Divisions of the Octave"
Open Music Theory, Part VIII: "Collections"
