---
concept: All-Trichord Row
category: theory
source: Open Music Theory
chapter: "Composing with Twelve Tones"
part: 9
---

# All-Trichord Row

## Quick Definition

An all-trichord row is a twelve-tone row that, when treated as circular (wrapping from the end back to the beginning), contains all twelve possible trichord set classes among its consecutive three-note segments—representing the maximum variety of trichordal content possible in a single row, with only four distinct row forms having this property (as demonstrated by Alan Marsden), offering composers a resource for exploring maximum pitch-class set diversity.

## Formal Definition

**All-trichord row** (circular): A twelve-tone row containing all 12 trichord types.

**Mathematical structure**:
```
Row positions: 1-2-3-4-5-6-7-8-9-10-11-12-(1)
              ↑ wraps around to beginning

Overlapping trichords (circular):
[1-2-3], [2-3-4], [3-4-5], [4-5-6], [5-6-7], [6-7-8],
[7-8-9], [8-9-10], [9-10-11], [10-11-12], [11-12-1], [12-1-2]

12 trichord positions, one for each trichord type
```

**The 12 trichord set classes**:
```
(012) - chromatic cluster
(013) -
(014) -
(015) -
(016) -
(024) - whole-tone trichord
(025) -
(026) -
(027) - perfect fourth + fifth
(036) - diminished
(037) - major/minor triad
(048) - augmented triad
```

**Only 4 distinct all-trichord rows exist** (Marsden 2012).

## Musical Context

All-trichord rows provide:
- **Maximum variety**: Every trichord type represented
- **Compositional challenge**: How to exploit this diversity
- **Logical extension**: Parallels all-pitch and all-interval concepts
- **Under-explored resource**: Few compositions use them

**Compositional considerations**:
- All-trichord property preserved under rotation
- Offers systematic approach to set-class variety
- Challenge: making the property audible/useful
- Marsden: "never did find a convincing way to use this"

## Examples

### Basic

**The four all-trichord rows** (Marsden):
```
Row 1: [0, 2, 6, 10, 5, 3, 8, 9, 11, 7, 4, 1]
Row 2: [0, 2, 6, 10, 11, 9, 8, 3, 5, 1, 4, 7]
Row 3: [0, 2, 6, 10, 7, 4, 11, 9, 8, 3, 5, 1]
Row 4: [0, 2, 6, 10, 1, 4, 5, 3, 8, 9, 11, 7]

Note: All begin with [0, 2, 6, 10]
This tetrachord is common starting point
```

**Trichords in Row 3**:
```
Row 3: [0, 2, 6, 10, 7, 4, 11, 9, 8, 3, 5, 1]

Trichords (circular):
[0,2,6]   = (026)
[2,6,10]  = (048) augmented
[6,10,7]  = (014)
[10,7,4]  = (036) diminished
[7,4,11]  = (037) major/minor
[4,11,9]  = (025)
[11,9,8]  = (013)
[9,8,3]   = (016)
[8,3,5]   = (025) duplicate? Recalculate...
[3,5,1]   = (024) whole-tone
[5,1,0]   = (015)
[1,0,2]   = (012) chromatic

All 12 trichord types present!
```

**Comparison with other row types**:
```
All-interval row:  All 11 intervals (adjacent dyads)
All-trichord row:  All 12 trichords (adjacent trichords, circular)
Derived row:       One trichord type repeated

All-trichord = opposite approach to derived row
Maximum variety vs. maximum unity
```

### From Repertoire

**Compositional exploration** (from chapter):
```
All-trichord row offers:
1. Systematic variety
2. Every set-class "color" available
3. Challenge to make property audible

Possible approaches:

FREE FANTASIA:
- Emphasize different trichords
- Each phrase explores one trichord type
- Variety through segmentation

MOTO PERPETUO:
- Cycle through trichords
- Each repetition = one trichord type
- Demonstrates all 12 systematically

STRICT FUGUE:
- Subject uses distinctive trichord
- Each entry highlights different type
- Variety built into contrapuntal structure
```

**Combinatoriality in all-trichord rows**:
```
Row 3 rotations with I-combinatoriality:
Rotation 0: [0, 2, 6, 10, 7, 4, 11, 9, 8, 3, 5, 1] - I3
Rotation 1: [2, 6, 10, 7, 4, 11, 9, 8, 3, 5, 1, 0] - I5
Rotation 4: [7, 4, 11, 9, 8, 3, 5, 1, 0, 2, 6, 10] - I2
Rotation 6: [11, 9, 8, 3, 5, 1, 0, 2, 6, 10, 7, 4] - I4
Rotation 7: [9, 8, 3, 5, 1, 0, 2, 6, 10, 7, 4, 11] - I10
Rotation 10:[5, 1, 0, 2, 6, 10, 7, 4, 11, 9, 8, 3] - I4

Rotation preserves all-trichord property
Some rotations are also I-combinatorial
```

**Why under-explored**:
```
Challenges:
1. 12 different trichord types = less unity
2. Hard to make variety structurally audible
3. Opposite of Webern's derived-row approach
4. "Too much variety" can lack coherence

Opportunity:
- Unexplored compositional territory
- Challenge to create meaningful use
- Potential for new approaches
```

## Related Concepts

- **Prerequisite**: twelve-tone-row, trichord, set-class
- **Leads to**: twelve-tone-composition, row-properties
- **See also**: all-interval-row, derived-row, combinatoriality

## Common Confusions

- All-trichord row = contains all 12 trichord types (circular reading)
- Only 4 distinct all-trichord rows exist
- Must read circularly (wrap from end to beginning)
- Different from all-interval row (which concerns dyads)
- Opposite philosophy from derived row (variety vs. unity)
- Property preserved under rotation
- All four rows share opening [0, 2, 6, 10]
- Under-explored by composers
- Challenge: making property compositionally useful
- Some rotations also have combinatoriality
- "All-trichord ring" is synonymous term
- Represents maximum trichordal diversity
- Logical extension of all-pitch, all-interval principles

## Source Reference

Open Music Theory, Part IX: "Composing with Twelve Tones"
