---
concept: Segmental Invariance
category: theory
source: Open Music Theory
chapter: "Row Properties"
part: 9
---

# Segmental Invariance

## Quick Definition

Segmental invariance occurs when a pitch-class segment of a twelve-tone row remains unchanged in absolute pitch content (not just set class) when the row undergoes transformation—happening when two segments within a row are related by the same transposition or inversion that is subsequently applied to the entire row, causing those segments to swap positions while retaining their exact pitches, providing composers with predictable pitch connections across different row forms.

## Formal Definition

**Segmental invariance**: Preservation of exact pitch-class content in a segment when a row is transformed.

**Mechanism**:
```
If segments A and B within a row are related by Tn:
When the entire row is transposed by Tn:
- Segment A becomes segment B (by position swap)
- Segment B becomes segment A
- Both retain their exact pitch classes
```

**Requirements for invariance**:
1. Find two segments of same set class within row
2. Determine their transpositional/inversional relationship (Tn or TnI)
3. Apply SAME transformation to entire row
4. Original segments are "held invariant"

**Types**:
- **Transpositional invariance**: Segments related by Tn, row transposed by Tn
- **Inversional invariance**: Segments related by TnI, row inverted by TnI

## Musical Context

Segmental invariance provides:
- **Pitch continuity**: Same pitches recur across row forms
- **Motivic connection**: Links between different sections
- **Compositional control**: Predictable pitch relationships
- **Voice leading**: Smooth connections between row forms

Invariance is distinct from set-class preservation:
- Set-class content ALWAYS preserved under transformation
- SEGMENTAL invariance = exact PITCH CLASSES preserved
- Much rarer and more structurally significant

## Examples

### Basic

**How invariance works** (abstract example):
```
Row P0: [A-B-C] [D-E-F] [G-H-I] [J-K-L]
        Segment1 Segment2 Segment3 Segment4

If Segment1 = T8 of Segment4:
Then transposing by T8:

Row P8: [J-K-L] [?-?-?] [?-?-?] [A-B-C]
        (was Seg4) ........ (was Seg1)

Segments 1 and 4 swap positions
Their PITCHES stay the same
They are "held invariant"
```

**Finding invariance**:
```
Step 1: Find equivalent set-class segments in row
        Example: two (0123) tetrachords

Step 2: Determine relationship between them
        Example: first = T8 of third

Step 3: Transform row by that relationship
        Example: transpose entire row by T8

Step 4: Those segments are now invariant
        Example: first and third tetrachords swap
                 but keep same pitch classes
```

### From Repertoire

**Webern, String Quartet Op. 28**:
```
P0: B-Bb-D-Eb | F#-G-E-F | Ab-A-C-Db
    Tet 1       Tet 2      Tet 3

All tetrachords = set class (0123)

Relationship:
Tet 1 to Tet 2: T8 (or T-4)
Tet 2 to Tet 3: T4

When row transposed by T8 (= P8):
P8: Ab-A-C-Db | B-Bb-D-Eb | F#-G-E-F
    (was Tet3)  (was Tet1)   (was Tet2)

Invariance:
- Tetrachord 1 of P0 = Tetrachord 2 of P8 (same pitches!)
- Tetrachord 2 of P0 = Tetrachord 3 of P8 (same pitches!)
- Tetrachord 3 of P0 = Tetrachord 1 of P8 (same pitches!)

Segments "rotate" but pitches held invariant
```

**Visual representation**:
```
P0:  [B-Bb-D-Eb] - [F#-G-E-F] - [Ab-A-C-Db]
         |              |             |
     light blue        red          navy

P8:  [Ab-A-C-Db] - [B-Bb-D-Eb] - [F#-G-E-F]
         |              |             |
        navy       light blue        red

Colors show how segments rotate
Pitch content stays identical
```

**Compositional use**:
```
Webern exploits this invariance:
- Can move between P0 and P8
- Tetrachord pitches remain constant
- Creates continuity across row changes
- Segments function as stable reference points
```

## Related Concepts

- **Prerequisite**: twelve-tone-row, row-operations, transposition
- **Leads to**: combinatoriality, twelve-tone-analysis
- **See also**: derived-row, set-class, pitch-class-set

## Common Confusions

- Segmental invariance = exact pitch classes preserved (not just set class)
- Set class is ALWAYS preserved; segmental invariance is special
- Occurs when segments within row are related by Tn or TnI
- Apply SAME transformation to row, segments become invariant
- The segments SWAP POSITIONS but keep same pitches
- Different from "row class" (all 48 forms of a row)
- Derived rows often have invariance properties
- To find invariance:
  1. Find equivalent segments in row
  2. Determine their relationship (Tn or TnI)
  3. That transformation produces invariance
- Not all rows have useful invariance properties
- Invariance gives composers predictable pitch connections
- Webern particularly exploited segmental invariance
- Invariance is about ABSOLUTE pitches, not interval patterns

## Source Reference

Open Music Theory, Part IX: "Row Properties"
