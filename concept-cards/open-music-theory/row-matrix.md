---
concept: Row Matrix
category: theory
source: Open Music Theory
chapter: "Basics of Twelve-Tone Theory"
part: 9
---

# Row Matrix

## Quick Definition

A row matrix (also called a magic square or twelve-tone matrix) is a 12x12 grid that compactly displays all 48 forms of a twelve-tone row class, organized so that prime forms (P) are read left to right across rows, retrogrades (R) right to left, inversions (I) top to bottom down columns, and retrograde inversions (RI) bottom to top—providing a complete reference for identifying and working with all possible transformations of a row.

## Formal Definition

**Row matrix**: A 12x12 grid displaying all 48 row forms.

**Structure and reading directions**:
```
          I0  I11  I3  I7  I8  I4  I2  I6  I5  I1  I9  I10
    +----------------------------------------------------+
P0  |  0   11   3   7   8   4   2   6   5   1   9   10  | R10
P1  |  1    0   4   8   9   5   3   7   6   2  10   11  | R11
P9  |  9    8   0   4   5   1  11   3   2  10   6    7  | R7
P5  |  5    4   8   0   1   9   7  11  10   6   2    3  | R3
... (continues for all 12 rows)
    +----------------------------------------------------+
          RI2 RI1  RI5 RI9 RI10 RI6 RI4 RI8 RI7 RI3 RI11 RI0
```

**Conventions**:
- **P (Prime)**: Read rows left to right
- **R (Retrograde)**: Read rows right to left
- **I (Inversion)**: Read columns top to bottom
- **RI (Retrograde Inversion)**: Read columns bottom to top

**Labeling conventions**:
- P0 and I0 start on the same pitch class (first note of P0)
- R0 and P0 are exact retrogrades (R0 labeled by ending pitch = P0's first pitch)
- RI0 and I0 are exact retrogrades
- Subscript = starting pitch for P and I, ending pitch for R and RI

## Mathematical Construction

**Building a matrix** (starting from P0):

1. Place P0 across the top row
2. The first column becomes I0 (same starting note as P0)
3. Each subsequent row: transpose P0 to start on the I column's pitch
4. Verify: diagonal from top-left to bottom-right shows same pitch

**Example construction**:
```
Given P0: 0-11-3-7-8-4-2-6-5-1-9-10

Step 1: Top row is P0
Step 2: First column is I0 (invert intervals from 0)
        P0 intervals: -1, +4, +4, +1, -4, -2, +4, -1, -4, +8, +1
        I0 intervals: +1, -4, -4, -1, +4, +2, -4, +1, +4, -8, -1
        I0: 0-1-9-5-4-8-10-6-7-11-3-2

Step 3: Each row is P form starting on I column pitch
        Row 2: P1 (starts on 1, from I0)
        Row 3: P9 (starts on 9, from I0)
        etc.
```

## Musical Context

The row matrix provides:
- **Complete reference**: All 48 forms visible at once
- **Analysis tool**: Quickly identify row forms in a score
- **Composition aid**: See relationships between row forms
- **Invariance detection**: Spot shared pitch content across forms
- **Planning**: Map out form usage for a composition

The matrix reveals structural relationships:
- Main diagonal always shows same pitch (starting pitch of P0)
- Symmetric rows create matrix symmetries
- Combinatorial pairs appear in predictable positions

## Examples

### Basic

**Matrix for Lutyens, Motet Op. 27**:
```
       I0  I11  I3  I7  I8  I4  I2  I6  I5  I1  I9 I10
  +--------------------------------------------------+
P0 |  0  11   3   7   8   4   2   6   5   1   9  10 | R10
P1 |  1   0   4   8   9   5   3   7   6   2  10  11 | R11
P9 |  9   8   0   4   5   1  11   3   2  10   6   7 | R7
P5 |  5   4   8   0   1   9   7  11  10   6   2   3 | R3
P4 |  4   3   7  11   0   8   6  10   9   5   1   2 | R2
P8 |  8   7  11   3   4   0  10   2   1   9   5   6 | R6
P10| 10   9   1   5   6   2   0   4   3  11   7   8 | R8
P6 |  6   5   9   1   2  10   8   0  11   7   3   4 | R4
P7 |  7   6  10   2   3  11   9   1   0   8   4   5 | R5
P11| 11  10   2   6   7   3   1   5   4   0   8   9 | R9
P3 |  3   2   6  10  11   7   5   9   8   4   0   1 | R1
P2 |  2   1   5   9  10   6   4   8   7   3  11   0 | R0
  +--------------------------------------------------+
      RI2 RI1 RI5 RI9 RI10 RI6 RI4 RI8 RI7 RI3 RI11 RI0
```

**Reading the matrix**:
```
To find P5: Read row starting "P5" left to right
   P5 = 5-4-8-0-1-9-7-11-10-6-2-3

To find R3: Read P5 row right to left (ends on 3)
   R3 = 3-2-6-10-11-9-1-0-8-4-5

To find I7: Read column under "I7" top to bottom
   I7 = 7-8-4-0-11-3-5-1-2-6-10-9

To find RI9: Read I9 column bottom to top
   RI9 = 1-0-4-8-7-11-9-5-6-10-2-3
```

### From Repertoire

**Webern, Symphonie Op. 21** (symmetrical row):
```
P0: 9-6-7-8-4-5-11-10-2-1-0-3

Because row is retrograde-equivalent:
P0 = R6 (transposed retrograde)

Matrix shows only 24 distinct forms
(P and R pairs are equivalent)
```

**Webern, Konzert Op. 24** (highly symmetric):
```
P0: 11-8-2-3-7-6-8-4-5-0-1-9

Four trichords related by P, I, R, RI
P0 = RI7 (when rotated from 7th note)

Matrix shows only 12 distinct forms!
(P=RI equivalence, I=R equivalence)
```

## Related Concepts

- **Prerequisite**: twelve-tone-row, row-operations, pitch-class
- **Leads to**: naming-conventions, invariance, combinatoriality
- **See also**: transposition, inversion, row-class

## Common Confusions

- Matrix = 12x12 grid showing all 48 row forms
- Read P (prime) left to right across rows
- Read R (retrograde) right to left across same rows
- Read I (inversion) top to bottom down columns
- Read RI (retrograde inversion) bottom to top up columns
- P0 is in top row, I0 is in first column
- Both P0 and I0 start with the same pitch class
- R and RI labels indicate ENDING pitch (not starting)
- Main diagonal shows same pitch throughout (P0's first pitch)
- Matrix construction: place P0 on top, derive I0 down first column
- Some rows have fewer than 48 distinct forms due to symmetry
- Different naming conventions exist (fixed-zero vs. moveable-zero)
- Matrix is an analytical tool, not a performance instruction
- Reading a form from matrix gives pitch-class numbers (not note names)
- Matrix doesn't show rhythm, register, or other parameters

## Source Reference

Open Music Theory, Part IX: "Basics of Twelve-Tone Theory"
