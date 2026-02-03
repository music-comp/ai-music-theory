---
concept: "Transposition (Tn)"
category: theory
source: Open Music Theory
chapter: "Pitch-Class Sets, Normal Order, and Transformations"
part: 8
---

# Transposition (Tn)

## Quick Definition

In set theory, transposition (Tn) is an operation that adds a fixed integer n to every pitch class in a set, shifting the entire collection by n semitones. T4 means "transpose up by 4 semitones" (mod 12). Transposition preserves intervallic content--the distances between pitch classes remain identical.

## Formal Definition

**Transposition operation Tn**: For each pitch class x in a set, the transposed pitch class is (x + n) mod 12.

**Notation**: Tn(set) or Tn[set]
- T0 = identity (no change)
- T6 = tritone transposition
- T12 = T0 (mod 12)

**Mathematical properties**:
- **Preserves intervals**: If ic(x,y) = k in the original, then ic(Tn(x), Tn(y)) = k
- **Preserves interval ordering**: The sequence of intervals between adjacent elements is unchanged
- **Bijective (one-to-one)**: Each pc maps to exactly one other pc
- **Group operation**: Tm(Tn(x)) = Tm+n(x)

**Index number (n)**: The transposition level; the number of semitones added to each pitch class.

**Calculating transposition between sets**:
If set A is transposed to become set B, find n by:
n = (any element of B) - (corresponding element of A) mod 12

## Musical Context

Transposition in post-tonal music works similarly to transposition in tonal music--moving a melody or chord up or down--but without the constraints of maintaining key or mode.

In analysis, Tn relationships reveal:
- **Motivic development**: A motive transposed creates recognizable variation
- **Structural parallelism**: Sections or phrases at different transposition levels
- **Compositional unity**: Sets related by transposition share identical interval content

Unlike tonal transposition (which might involve mode changes or respelling), Tn is mathematically precise: T7 always means "add 7 to each pc."

## Examples

### Basic

**Transposing a trichord by T4**:
```
Original set: [11, 2, 4] (B, D, E)
Operation: T4

11 + 4 = 15 = 3 (mod 12)
 2 + 4 = 6
 4 + 4 = 8

T4[11, 2, 4] = [3, 6, 8] (Eb, F#, G#)
```

**Identifying the transposition between two sets**:
```
Set A: [11, 2, 4]
Set B: [3, 6, 8]

Subtract corresponding elements:
3 - 11 = -8 = 4 (mod 12)
6 - 2 = 4
8 - 4 = 4

All differences = 4, so B = T4(A)
```

**Transposition levels**:
```
T0:  no change (identity)
T1:  up a semitone
T2:  up a whole tone
T3:  up a minor third
T4:  up a major third
T5:  up a perfect fourth
T6:  up a tritone
T7:  up a perfect fifth (= down a P4)
T8:  up a minor sixth (= down a M3)
T9:  up a major sixth (= down a m3)
T10: up a minor seventh (= down a M2)
T11: up a major seventh (= down a m2)
```

### From Repertoire

**Debussy, "La cathedrale engloutie"**: The opening motive <D, E, B> = [2, 4, 11] returns at m. 18 as <F#, G#, D#> = [6, 8, 3]. This is T4--the "cathedral" motive has risen 4 semitones, depicting its ascent from the water.

**Bartok, "Subject and Reflection"**: The right-hand set [10, 0, 2, 3, 5] is T5-related to [3, 5, 7, 8, 10]. Bartok uses transposition structurally across passages.

**Schoenberg, Pierrot lunaire**: Tracking Tn relationships of the "Nacht" motive [3, 4, 7] reveals its appearances throughout the movement at various transposition levels.

## Related Concepts

- **Prerequisite**: pitch-class-set, integer-notation, mod-12-arithmetic, normal-order
- **Leads to**: set-class, prime-form, twelve-tone-series
- **See also**: inversion-in, index-number

## Common Confusions

- **Tn adds n to each pc**: It doesn't multiply or apply a different operation to each
- **Mod 12 is essential**: 11 + 4 = 15 = 3, not 15
- **Tn =/= tonal transposition**: Tn doesn't change key signature or mode--it's purely mathematical
- **T0 is the identity**: The set remains unchanged
- **T12 = T0**: Because of mod 12 arithmetic
- **Both sets should be in normal order** when comparing to identify Tn relationship
- **Negative transposition**: T-3 = T9 (add -3 mod 12 = add 9)
- **Tn preserves set class**: A transposed set is a member of the same set class

## Source Reference

Open Music Theory, Part VIII, Chapter 3: "Pitch-Class Sets, Normal Order, and Transformations"
