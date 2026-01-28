---
concept: All-Interval Row
category: theory
source: Open Music Theory
chapter: "Row Properties"
part: 9
---

# All-Interval Row

## Quick Definition

An all-interval row is a twelve-tone row in which all eleven distinct intervals (measured in semitones from 1 to 11) appear between adjacent pitches—achieved by examining the overlapping dyads formed by consecutive pitch pairs (1-2, 2-3, 3-4, etc.) and ensuring each of the eleven possible intervals occurs exactly once, resulting in maximum intervallic variety and a property shared by exactly 1,928 distinct row forms.

## Formal Definition

**All-interval row**: A twelve-tone row containing all eleven intervals between consecutive pitches.

**Mathematical properties**:
- 12 pitch classes produce 11 adjacent intervals
- Each interval class 1-11 appears exactly once
- Total distinct all-interval rows: 1,928
- By definition: tritone (interval 6) connects notes equidistant from center
  - Notes 1 and 12 are a tritone apart
  - Notes 2 and 11 are a tritone apart
  - Notes 3 and 10, 4 and 9, 5 and 8, 6 and 7

**Overlapping segment analysis**:
```
Row:      a - b - c - d - e - f - g - h - i - j - k - l
Dyads:    [a-b] [b-c] [c-d] [d-e] [e-f] [f-g] [g-h] [h-i] [i-j] [j-k] [k-l]
Intervals:  1     2     3     4     5     6     7     8     9    10    11
           (in some order, each appears once)
```

## Musical Context

All-interval rows provide:
- **Maximum variety**: Every interval represented equally
- **Balance**: No interval emphasized over others
- **Structural interest**: Row structure becomes self-defining
- **Historical importance**: Explored by many serialist composers

The all-interval property represents one approach to "using everything equally"—parallel to the twelve-tone principle of using all pitch classes.

Famous construction method (Grandmother chord / Slonimsky):
- Alternate odd and even intervals
- Odd intervals ascending (1, 3, 5, 7, 9, 11)
- Even intervals descending (10, 8, 6, 4, 2)
- Results in two interleaved chromatic scales

## Examples

### Basic

**Grandmother chord construction**:
```
Start on A
+1 (up semitone):     A  - Bb
-2 (down tone):       Bb - Ab
+3 (up minor 3rd):    Ab - B
-4 (down major 3rd):  B  - G
+5 (up perfect 4th):  G  - C
-6 (down tritone):    C  - F#
+7 (up perfect 5th):  F# - C#
-8 (down minor 6th):  C# - A (octave higher)
+9 (up major 6th):    A  - F#
-10 (down minor 7th): F# - E
+11 (up major 7th):   E  - D#

Result: A-Bb-Ab-B-G-C-F#-C#-A-F#-E-D#
        (two interleaved chromatic scales)
```

**Verifying all-interval property**:
```
Row: A-Bb-Ab-B-G-C-F#-C#-A-F#-E-D#
     9  10  8 11 7 0  6  1  9 6  4 3

Intervals between adjacent notes:
9→10: +1 (semitone)
10→8: -2 (= +10, whole tone)
8→11: +3 (minor 3rd)
11→7: -4 (= +8, major 3rd)
... etc.

All 11 intervals (1-11) appear exactly once
```

**Tritone relationships**:
```
In all-interval row, notes equidistant from center form tritones:

Position:  1   2   3   4   5   6   7   8   9  10  11  12
Note:      A  Bb  Ab  B   G   C   F#  C#  A  F#  E   D#

Pairs (tritone = interval 6):
1 and 12: A - D# (tritone)
2 and 11: Bb - E (tritone)
3 and 10: Ab - F# (tritone)
4 and 9:  B - A (not tritone in this row)
... pattern varies by specific row
```

### From Repertoire

**Luigi Nono, Il Canto Sospeso**:
```
Uses an all-interval row based on Grandmother chord principle

Row features:
- Chromatic wedge structure
- Two interleaved chromatic scales
- Maximum intervallic variety

Similar structure found in:
- Bach fugues (BWV 548) - chromatic wedge subjects
- Shostakovich fugues (Op. 87, No. 15)
```

**All-interval row in analysis**:
```
When analyzing, check:
1. List intervals between all 11 adjacent pairs
2. Confirm each interval 1-11 appears once
3. Note the tritone relationships
4. Consider how composer exploits the property
```

## Related Concepts

- **Prerequisite**: twelve-tone-row, interval-class, overlapping-segments
- **Leads to**: row-properties, derived-row, all-trichord-row
- **See also**: interval-vector, chromatic-wedge

## Common Confusions

- All-interval row = all 11 intervals between adjacent pitches
- NOT the same as containing all interval classes in any position
- Uses overlapping segment analysis (dyads 1-2, 2-3, 3-4, etc.)
- Exactly 1,928 distinct all-interval rows exist
- Every all-interval row has tritones between symmetrically-positioned notes
- Notes 1-12, 2-11, 3-10, 4-9, 5-8, 6-7 form tritones (by definition)
- "Grandmother chord" is a famous all-interval row construction
- Construction: alternate odd intervals up, even intervals down
- Results in two interleaved chromatic scales
- The property is about ADJACENT intervals, not all dyadic intervals
- Different from all-trichord rows (which use all trichord types)
- All-interval property preserved under transposition
- Inversion gives different interval sequence (inverted)
- Retrograde gives reversed interval sequence

## Source Reference

Open Music Theory, Part IX: "Row Properties"
