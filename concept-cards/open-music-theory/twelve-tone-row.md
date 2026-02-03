---
concept: Twelve-Tone Row
category: theory
source: Open Music Theory
chapter: "Basics of Twelve-Tone Theory"
part: 9
---

# Twelve-Tone Row

## Quick Definition

A twelve-tone row (also called a series) is an ordered arrangement of all twelve pitch classes used as the foundational material in twelve-tone composition, where each pitch class appears exactly once—serving as a fixed sequence from which melodies, harmonies, and entire compositions are derived through various transformations (transposition, inversion, retrograde, and retrograde inversion), producing up to 48 related row forms that constitute a row class.

## Formal Definition

**Twelve-tone row** (also: series, tone row): An ordered set containing all twelve pitch classes exactly once.

**Mathematical properties**:
- Contains pitch classes 0-11 (C through B)
- Total possible rows: 12! = 479,001,600
- Each row generates up to 48 related forms (row class)
- Forms: P (prime), R (retrograde), I (inversion), RI (retrograde inversion)
- Each form can be transposed to 12 pitch levels

**Row class structure**:
```
P0, P1, P2 ... P11  (12 prime forms)
R0, R1, R2 ... R11  (12 retrograde forms)
I0, I1, I2 ... I11  (12 inversion forms)
RI0, RI1 ... RI11   (12 retrograde inversion forms)
Total: 48 forms (unless row has special symmetry)
```

**Basic constraints**:
1. Pitch classes played in order specified by row
2. Once played, pitch class not repeated until next row

## Musical Context

The twelve-tone row provides:
- **Unity**: All pitch material derives from single row
- **Equality**: All twelve pitch classes treated as equal (no tonic)
- **Variety**: 48 transformations provide diverse material
- **Coherence**: Interval content preserved across transformations
- **Structure**: Row properties determine compositional possibilities

The twelve-tone technique emerged in the 1920s, associated with:
- Arnold Schoenberg (developed the method)
- Anton Webern (explored row symmetry)
- Alban Berg (combined with tonal elements)

Not all serial music uses twelve-tone rows, and not all twelve-tone music is strictly serial.

## Examples

### Basic

**Chromatic scale as row** (for illustration):
```
P0: C-C#-D-D#-E-F-F#-G-G#-A-A#-B
    0  1  2  3 4 5  6 7  8 9 10 11

Intervals: +1, +1, +1, +1, +1, +1, +1, +1, +1, +1, +1
```

**Retrograde of above**:
```
R0: B-A#-A-G#-G-F#-F-E-D#-D-C#-C
    11 10 9  8 7  6 5 4  3 2  1 0

Same pitches, reversed order
Ends on same pitch as P0 starts
```

**Row construction principles**:
```
Row: [ordered pitch classes]
- Must include all 12 pitch classes
- Each appears exactly once
- Order is fixed (the "series")
- Intervals between adjacent pitches define character

Example row:
0-11-3-7-8-4-2-6-5-1-9-10
(from Lutyens's Motet Op. 27)
```

### From Repertoire

**Elisabeth Lutyens, Motet Op. 27**:
```
P0: 0-11-3-7-8-4-2-6-5-1-9-10
    C-B-Eb-G-Ab-E-D-F#-F-Db-A-Bb

Interval succession: -1, +4, +4, +1, -4, -2, +4, -1, -4, +8, +1
```

**Webern, Symphonie Op. 21**:
```
Row divides into symmetric hexachords:
First hexachord: 9-6-7-8-4-5 (chromatic cluster)
Second hexachord: 11-10-2-1-0-3 (chromatic cluster)

Both hexachords = set class 6-1 (chromatic hexachord)
Row is retrograde-equivalent (R6 = transposed P0)
```

**Webern, Konzert Op. 24**:
```
P0: 11-8-2 | 3-7-6 | 8-4-5 | 0-1-9

Four trichords, all set class (014)
Each trichord related by P, I, R, or RI
Only 12 distinct row forms (due to symmetry)
```

**Dallapiccola, Piccola Musica Notturna** (1954):
Row unfolds gradually with free repetition—demonstrates flexible approach to twelve-tone technique.

## Related Concepts

- **Prerequisite**: pitch-class, interval-class, set-theory-analysis
- **Leads to**: row-matrix, row-operations, combinatoriality
- **See also**: serialism, derived-row

## Common Confusions

- Twelve-tone row = ordered arrangement of all 12 pitch classes
- Each pitch class appears exactly once in the row
- The row is a fixed ORDER, not just a collection
- Row generates up to 48 forms (P, I, R, RI each at 12 transpositions)
- "Series" and "row" are synonymous terms
- Not the same as pitch-class set (row is ordered, set is unordered)
- Rows don't specify rhythm, register, or duration
- Different rows have different interval contents
- Some rows have symmetry properties reducing 48 to fewer forms
- Row forms maintain interval relationships when transformed
- "Twelve-tone" refers to pitch organization, not timbre or rhythm
- Not all music with 12 pitch classes is "twelve-tone"
- Twelve-tone is a technique, not a style
- Composers vary widely in how strictly they follow row ordering
- Repetition of notes within row statement varies by composer/piece

## Source Reference

Open Music Theory, Part IX: "Basics of Twelve-Tone Theory"
