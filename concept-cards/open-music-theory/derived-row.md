---
concept: Derived Row
category: theory
source: Open Music Theory
chapter: "Row Properties"
part: 9
---

# Derived Row

## Quick Definition

A derived row is a twelve-tone row constructed from multiple instances of a single pitch-class set (typically a trichord or tetrachord), where the row's discrete segments all belong to the same set class—ensuring that when the row is transposed, inverted, or otherwise transformed, these characteristic set classes recur constantly throughout the composition, providing motivic unity and coherence through the consistent circulation of related pitch collections.

## Formal Definition

**Derived row**: A twelve-tone row whose discrete segments are instances of the same set class.

**Types of derivation**:
- **Trichordal**: 4 trichords from one set class (4 x 3 = 12)
- **Tetrachordal**: 3 tetrachords from one set class (3 x 4 = 12)
- **Hexachordal**: 2 hexachords from one set class (2 x 6 = 12)

**Discrete segment analysis**:
```
Trichordal:   [1-2-3] [4-5-6] [7-8-9] [10-11-12]
Tetrachordal: [1-2-3-4] [5-6-7-8] [9-10-11-12]
Hexachordal:  [1-2-3-4-5-6] [7-8-9-10-11-12]

For derived row: all segments same set class
```

**Compositional significance**:
- Set class content unchanged under transformation
- Same sets circulate throughout piece
- Guarantees motivic recurrence
- Unity through consistent intervallic content

## Musical Context

Derived rows provide:
- **Unity**: Limited set-class vocabulary
- **Coherence**: Same intervals recur constantly
- **Motivic identity**: Characteristic sonority pervades piece
- **Compositional clarity**: Clear structural building blocks

The derived row concept connects to:
- "Composing with pitch-class sets"
- Cell-based compositional thinking
- Webern's condensed musical language

Note: "Derived" can mean either:
1. Row built from one set class (definition above)
2. New row created from subsegment of existing row

## Examples

### Basic

**Trichordal derivation** (set class 014):
```
Row: B-C-Eb | E-G-Ab | Bb-D-Db | F-F#-A
     0-1-4  | 5-8-9  | 11-3-2  | 6-7-10

All four trichords = set class (014)
Each contains: minor 3rd + semitone

Set (014) interval content: ic1, ic3, ic4
These intervals pervade entire row
```

**Tetrachordal derivation** (set class 0123):
```
Row: B-Bb-D-Eb | F#-G-E-F | Ab-A-C-Db
     11-10-2-3 | 6-7-4-5  | 8-9-0-1

All three tetrachords = set class (0123)
Each is a chromatic tetrachord

From Webern, String Quartet Op. 28
```

**Why derivation creates unity**:
```
Any transposition of the row:
- Still contains same 4 trichord types
- Still contains same 3 tetrachord types
- Set-class content invariant under T, I, R, RI

Result: same pitch-class sets throughout piece
```

### From Repertoire

**Webern, String Quartet Op. 28**:
```
Row: B-Bb-D-Eb | F#-G-E-F | Ab-A-C-Db
     (0123)     (0123)     (0123)

Properties:
- Three chromatic tetrachords
- Six semitone dyads (all adjacent pairs)
- Set class (0123) = chromatic cluster
- Maximum pitch density

Compositional result:
- Chromatic clusters pervade texture
- Semitone motion predominates
- Tight, compressed sound world
```

**Webern, Konzert Op. 24**:
```
Row: B-Bb-D | Eb-G-F# | Ab-E-F | C-Db-A
     (014)   (014)     (014)    (014)

All trichords = set class (014)
Each contains ic1, ic3, ic4

Furthermore:
- Cell 1 (B-Bb-D): P form
- Cell 2 (Eb-G-F#): RI form
- Cell 3 (Ab-E-F): R form
- Cell 4 (C-Db-A): I form

Trichords related by P, I, R, RI operations!
```

**Derivation from existing row**:
```
Given row with (014) subsegment:
...C-E-Eb...

Can derive new row:
C-E-Eb | F-A-Ab | G-B-Bb | D-F#-F
(014)   (014)   (014)    (014)

Each new trichord = transposition/inversion of original
New row "derived from" the subsegment
```

## Related Concepts

- **Prerequisite**: twelve-tone-row, set-class, discrete-segments
- **Leads to**: combinatoriality, invariance, row-properties
- **See also**: trichord, tetrachord, pitch-class-set

## Common Confusions

- Derived row = row whose discrete segments share one set class
- "Discrete" segments don't overlap (unlike "overlapping" segments)
- Common types: trichordal (4x3), tetrachordal (3x4), hexachordal (2x6)
- Set-class content preserved under all row operations
- Guarantees same sets circulate throughout entire piece
- Different from all-interval rows (which focus on intervals)
- "Derived" has two meanings:
  1. Row built from one set class
  2. Row created from subsegment of existing row
- Webern particularly associated with derived rows
- Not all twelve-tone rows are derived rows
- Derived rows offer motivic unity (same sets recur)
- The discrete segments may be DIFFERENT transpositions/inversions
- They belong to same SET CLASS, not identical pitch content
- Derivation relates to cell-based compositional thinking
- Interval content of set class becomes pervasive in piece

## Source Reference

Open Music Theory, Part IX: "Row Properties"
