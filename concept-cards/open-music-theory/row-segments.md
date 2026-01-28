---
concept: Row Segments
category: theory
source: Open Music Theory
chapter: "Row Properties"
part: 9
---

# Row Segments

## Quick Definition

Row segments are subsections of a twelve-tone row that can be analyzed in two ways: discrete segments (non-overlapping divisions such as two hexachords, three tetrachords, four trichords, or six dyads) and overlapping segments (consecutive groups stepping through the row by one pitch at a time, such as trichords 1-2-3, 2-3-4, 3-4-5, etc.)—with discrete segments being key to understanding derived rows and combinatoriality, while overlapping segments are used to identify properties like all-interval rows.

## Formal Definition

**Row segments**: Subsections of a twelve-tone row.

**Discrete segments** (non-overlapping):
```
12-note row can divide into:
- 2 hexachords:   [1-6][7-12]           (2 x 6)
- 3 tetrachords:  [1-4][5-8][9-12]      (3 x 4)
- 4 trichords:    [1-3][4-6][7-9][10-12](4 x 3)
- 6 dyads:        [1-2][3-4][5-6][7-8][9-10][11-12] (6 x 2)

No pitch appears in more than one segment
```

**Overlapping segments** (consecutive):
```
Trichords stepping by 1:
[1-2-3], [2-3-4], [3-4-5], [4-5-6]...
Adjacent segments share 2 pitches

Dyads stepping by 1:
[1-2], [2-3], [3-4], [4-5]...
Adjacent segments share 1 pitch
(These give the 11 intervals of the row)
```

**Why 12 is useful**: 12 = 2 x 6 = 3 x 4 = 4 x 3 = 6 x 2 (many divisors).

## Musical Context

Segment analysis reveals:
- **Discrete segments**: Set-class content, derived rows, combinatoriality
- **Overlapping segments**: Interval content, all-interval property

**Compositional significance**:
- Discrete: Row broken into characteristic cells
- Overlapping: Row's intervallic profile revealed

**Analytical applications**:
- Derived row identification: Are discrete segments same set class?
- All-interval check: Do overlapping dyads give 11 intervals?
- Invariance: Do discrete segments recur under transformation?
- Combinatoriality: Do discrete hexachords complement?

## Examples

### Basic

**Discrete segmentation**:
```
Row: C-Db-E-F-Ab-A-Bb-D-Eb-G-Gb-B
     0  1  4 5  8 9 10  2  3 7  6 11

Hexachords:
H1: {C,Db,E,F,Ab,A} = {0,1,4,5,8,9}
H2: {Bb,D,Eb,G,Gb,B} = {10,2,3,7,6,11}

Tetrachords:
T1: {C,Db,E,F} = {0,1,4,5}
T2: {Ab,A,Bb,D} = {8,9,10,2}
T3: {Eb,G,Gb,B} = {3,7,6,11}

Trichords:
Tr1: {C,Db,E} = (014)
Tr2: {F,Ab,A} = (014)
Tr3: {Bb,D,Eb} = (024)
Tr4: {G,Gb,B} = (015)
```

**Overlapping segmentation**:
```
Same row: C-Db-E-F-Ab-A-Bb-D-Eb-G-Gb-B

Overlapping dyads (intervals):
C-Db:  ic1
Db-E:  ic3
E-F:   ic1
F-Ab:  ic3
Ab-A:  ic1
A-Bb:  ic1
Bb-D:  ic4
D-Eb:  ic1
Eb-G:  ic4
G-Gb:  ic1
Gb-B:  ic5

Not all-interval (missing some, duplicating others)
```

**All-interval row** (overlapping analysis):
```
All-interval row: Has all 11 intervals between adjacent pitches

Check overlapping dyads:
Position 1-2: interval a
Position 2-3: interval b
...
Position 11-12: interval k

If {a,b,c,d,e,f,g,h,i,j,k} = {1,2,3,4,5,6,7,8,9,10,11}
Then: all-interval row!
```

### From Repertoire

**Webern, Op. 28** (discrete trichords):
```
Row: B-Bb-D-Eb | F#-G-E-F | Ab-A-C-Db

Discrete trichords:
[B-Bb-D] = (013)
[Eb-F#-G] = wait, need correct segmentation...

Actually:
[B-Bb-D-Eb] = tetrachord (0123)
[F#-G-E-F] = tetrachord (0123)
[Ab-A-C-Db] = tetrachord (0123)

All discrete tetrachords = same set class (0123)
This is a DERIVED ROW (tetrachordal)
```

**Webern, Op. 24** (discrete trichords):
```
Row: B-Bb-D | Eb-G-F# | Ab-E-F | C-Db-A

Discrete trichords (4 x 3):
[B-Bb-D] = (014)
[Eb-G-F#] = (014)
[Ab-E-F] = (014)
[C-Db-A] = (014)

All discrete trichords = same set class (014)
This is a DERIVED ROW (trichordal)

Furthermore: each trichord is P, I, R, or RI of the cell
Maximum concentration
```

**Luigi Nono, Il Canto Sospeso** (overlapping dyads):
```
All-interval row
Check overlapping dyads:
Each adjacent pair gives different interval
All 11 intervals (1-11) represented once

Overlapping analysis reveals all-interval property
Discrete analysis might show different patterns
```

## Related Concepts

- **Prerequisite**: twelve-tone-row, interval, set-class
- **Leads to**: derived-row, all-interval-row, combinatoriality
- **See also**: hexachord, tetrachord, trichord

## Common Confusions

- Two types: discrete (non-overlapping) vs. overlapping
- Discrete: segments don't share pitches
- Overlapping: segments share some pitches
- Discrete hexachords: key to combinatoriality
- Discrete trichords/tetrachords: key to derived rows
- Overlapping dyads: reveal interval content
- 12's divisibility makes multiple segment sizes possible
- "Discrete" trichords = 4 groups of 3 (positions 1-3, 4-6, 7-9, 10-12)
- "Overlapping" trichords = 10 groups of 3 (positions 1-2-3, 2-3-4, etc.)
- All-interval property uses OVERLAPPING dyad analysis
- Derived row property uses DISCRETE segment analysis
- Same row analyzed both ways reveals different properties

## Source Reference

Open Music Theory, Part IX: "Row Properties"
