---
concept: Row Operations
category: theory
source: Open Music Theory
chapter: "Basics of Twelve-Tone Theory"
pdf_page: null
chapter_number: 9
unit: null
authors: "Open Music Theory contributors"
---

# Row Operations

## Quick Definition

Row operations are the four fundamental transformations applied to a twelve-tone row that preserve its essential intervallic character while creating related forms: prime (P, the original row), retrograde (R, the row played backwards), inversion (I, the row with intervals reversed in direction), and retrograde inversion (RI, combining both retrograde and inversion)—together generating up to 48 distinct row forms when each operation is transposed to all 12 pitch levels.

## Formal Definition

**Row operations** (serial transformations): Four ways to transform a twelve-tone row.

**The four operations**:

1. **Prime (P)**: Original row or its transposition
   - Intervals unchanged
   - P0, P1, P2 ... P11 (12 transposition levels)

2. **Retrograde (R)**: Row played in reverse order
   - Last pitch becomes first
   - Intervals reversed in order
   - R0, R1, R2 ... R11

3. **Inversion (I)**: Direction of intervals reversed
   - Rising intervals become falling
   - Falling intervals become rising
   - Preserves interval size in semitones
   - I0, I1, I2 ... I11

4. **Retrograde Inversion (RI)**: Both operations combined
   - Invert, then retrograde (or vice versa)
   - RI0, RI1, RI2 ... RI11

**Mathematical relationships**:
```
If P = a, b, c, d, e, f, g, h, i, j, k, l

R = l, k, j, i, h, g, f, e, d, c, b, a
    (reverse order)

I = a, (2a-b), (2a-c), (2a-d), ... mod 12
    (invert intervals around first note)

RI = reverse of I
```

**Total forms**: 4 operations x 12 transpositions = 48 row forms

## Musical Context

Row operations serve fundamental purposes:
- **Variety**: 48 forms provide diverse pitch material
- **Unity**: All forms share intervallic DNA
- **Development**: Transformations enable musical development
- **Structure**: Different forms can articulate sections
- **Counterpoint**: Inversionally-related rows combine well

The operations parallel earlier contrapuntal techniques:
- Transposition: familiar from tonal music
- Inversion: like melodic inversion in counterpoint
- Retrograde: like crab canon (cancrizans)
- Retrograde inversion: combination technique

## Examples

### Basic

**Operations on chromatic scale** (C-C#-D-D#-E-F-F#-G-G#-A-A#-B):
```
P0:  C-C#-D-D#-E-F-F#-G-G#-A-A#-B
     0  1  2  3 4 5  6 7  8 9 10 11

R0:  B-A#-A-G#-G-F#-F-E-D#-D-C#-C
     (same pitches, reversed order)

I0:  C-B-A#-A-G#-G-F#-F-E-D#-D-C#
     0 11 10  9  8 7  6 5 4  3 2  1
     (intervals inverted: +1 becomes -1)

RI0: C#-D-D#-E-F-F#-G-G#-A-A#-B-C
     (I0 reversed)
```

**Interval preservation under operations**:
```
P0:  C  -  E  -  G   (intervals: +4, +3)
     0     4     7

I0:  C  -  Ab -  F   (intervals: -4, -3 = +8, +9 mod 12)
     0     8     5

Same interval classes, opposite directions
```

**Transposition example**:
```
P0:  C-E-G-B-...     (starts on C = 0)
P5:  F-A-C-E-...     (starts on F = 5)

P5 = P0 transposed up 5 semitones
Same intervals, different pitch level
```

### From Repertoire

**Lutyens, Motet Op. 27** (showing all four forms):
```
P0:  0-11-3-7-8-4-2-6-5-1-9-10
     C-B-Eb-G-Ab-E-D-F#-F-Db-A-Bb

R10: 10-9-1-5-6-2-4-8-7-3-11-0
     (P0 reversed, labeled by ending pitch)

I0:  0-1-9-5-4-8-10-6-7-11-3-2
     C-Db-A-F-E-Ab-Bb-Gb-G-B-Eb-D
     (intervals inverted)

RI0: 2-3-11-7-6-10-8-4-5-9-1-0
     (I0 reversed)
```

**Webern, Op. 28 String Quartet**:
```
P0:  B-Bb-D-Eb | Gb-G-E-F | Ab-A-C-Db
     Discrete tetrachords all set class (0123)

When transposed by T8 (up 8 semitones):
- First tetrachord of P0 becomes last of P8
- Middle tetrachord stays in middle
- Last tetrachord of P0 becomes first of P8

Demonstrates segmental invariance under transposition
```

**Tavener, The Lamb** (serial but not twelve-tone):
```
Soprano (prime):    G-B-A-F#-G
Alto (inversion):   G-Eb-F-Ab-G

Only 5 pitch classes, but demonstrates
P and I relationship clearly
```

## Related Concepts

- **Prerequisite**: twelve-tone-row, interval, transposition
- **Leads to**: row-matrix, row-naming-conventions
- **See also**: canzona, inversion, retrograde, combinatoriality

## Common Confusions

- P = Prime (original row or transposition)
- R = Retrograde (row reversed)
- I = Inversion (intervals reversed in direction)
- RI = Retrograde Inversion (both combined)
- Each operation has 12 transposition levels (subscript 0-11)
- P and I start on same pitch class when same subscript
- R and RI end on same pitch class as P and I start
- Inversion preserves interval SIZE (semitones), reverses DIRECTION
- Inversion is NOT the same as pitch-class inversion around an axis
- Order matters: invert-then-retrograde = retrograde-then-invert
- Transposition by semitones only (not diatonic steps)
- All operations preserve interval class content
- Retrograde preserves actual intervals (just reversed in time)
- The 48 forms constitute a "row class"
- Some rows have symmetry, giving fewer than 48 distinct forms
- Operations are mathematical, not performance instructions

## Source Reference

Open Music Theory, Part IX: "Basics of Twelve-Tone Theory"
