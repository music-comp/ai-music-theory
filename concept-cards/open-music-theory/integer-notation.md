---
concept: Integer Notation
category: theory
source: Open Music Theory
chapter: "Pitch and Pitch Class"
part: 8
---

# Integer Notation

## Quick Definition

A system for representing pitch classes as integers 0-11, where C=0, C#/Db=1, D=2, and so on chromatically. This notation eliminates the ambiguity of letter names and enharmonic spellings, making it ideal for post-tonal analysis.

## Formal Definition

**Integer notation** assigns a number from 0 to 11 to each of the twelve pitch classes:

| Integer | Pitch Class |
|---------|-------------|
| 0 | C (B#, Dbb) |
| 1 | C#, Db |
| 2 | D (Cx, Ebb) |
| 3 | D#, Eb |
| 4 | E (Fb) |
| 5 | F (E#) |
| 6 | F#, Gb |
| 7 | G (Fx) |
| 8 | G#, Ab |
| 9 | A |
| 10 | A#, Bb |
| 11 | B (Cb) |

**Mathematical properties**:
- Operations use **mod 12 arithmetic** (numbers "wrap around" at 12, like a clock)
- 11 + 3 = 14 mod 12 = 2
- 2 - 5 = -3 mod 12 = 9

**Advantages over letter names**:
- No enharmonic ambiguity (G# and Ab are both simply "8")
- Enables mathematical operations (transposition, inversion)
- Each pitch class has exactly one name
- Twelve pitch classes match twelve integers (0-11)

## Musical Context

Letter names work well for tonal music where spelling reflects function (G# as leading tone vs. Ab as upper neighbor). But in post-tonal music:

1. Tonal function is absent, so spelling is arbitrary
2. Seven letter names inadequately represent twelve pitch classes
3. Mathematical relationships become analytically significant

Integer notation emerged in the mid-20th century as theorists (particularly Allen Forte and Milton Babbitt) developed systematic methods for analyzing atonal and twelve-tone music.

The **clock face diagram** is a common visualization: pitch classes arranged like hours on a clock (0=12 o'clock, moving clockwise). This image reinforces the cyclical nature of pitch-class space and mod 12 arithmetic.

## Examples

### Basic

**Converting letter names to integers**:
```
C  -> 0     F# -> 6     Bb -> 10
C# -> 1     G  -> 7     B  -> 11
Db -> 1     Ab -> 8
D  -> 2     A  -> 9
```

**A melody in integer notation**:
```
Notes:  E  - F  - G# - A  - C
Staff:  E4 - F4 - G#4 - A4 - C5
PC int: 4  - 5  - 8  - 9  - 0
```

**Mod 12 arithmetic examples**:
```
7 + 8 = 15 = 3 (mod 12)
3 - 7 = -4 = 8 (mod 12)
0 - 1 = -1 = 11 (mod 12)
```

### From Repertoire

**Schoenberg, "Nacht" from Pierrot lunaire**: The recurring motive E-G-Eb can be notated as [4, 7, 3]. This facilitates tracking transpositions: T5 yields [9, 0, 8] (A-C-Ab).

**Webern, Concerto Op. 24**: The opening row can be expressed entirely in integers, revealing the derived set structure: the row is built from four transformations of a single trichord.

**Bartok, "Subject and Reflection"**: Integer notation reveals that passages in both hands share pitch-class set content related by transposition or inversion.

## Related Concepts

- **Prerequisite**: pitch-class, enharmonic-equivalence, mod-12-arithmetic
- **Leads to**: pitch-class-set, ordered-pitch-class-interval, transposition-operation, inversion-operation
- **See also**: normal-order, interval-class

## Common Confusions

- **C=0, not A**: Unlike some MIDI systems, set theory places C at 0
- **Integers represent pitch classes, not pitches**: C4 and C5 are both pc 0
- **10 and 11 are single digits conceptually**: Some texts use T and E (or A and B) to avoid two-digit numbers in sets
- **Mod 12 means "remainder when divided by 12"**: Negative numbers wrap around (e.g., -3 mod 12 = 9)
- **Integer notation is not merely "shorthand"**: It enables mathematical operations that letter names cannot support cleanly
- **The clock face is pitch-class space, not pitch space**: It has no vertical (octave) dimension

## Source Reference

Open Music Theory, Part VIII, Chapter 1: "Pitch and Pitch Class"
