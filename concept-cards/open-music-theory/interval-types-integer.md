---
concept: Interval Types in Integer Notation
category: theory
source: Open Music Theory
chapter: "Intervals in Integer Notation"
part: 8
---

# Interval Types in Integer Notation

## Quick Definition

Post-tonal theory distinguishes four interval types based on two criteria: pitch vs. pitch class, and ordered vs. unordered. These range from the most specific (ordered pitch interval: +16 semitones ascending) to the most abstract (interval class: simply 4, the shortest distance between two pitch classes).

## Formal Definition

Four interval types exist, organized from most concrete to most abstract:

### 1. Ordered Pitch Interval (OPI)
- Measures distance between **specific pitches** in **semitones**
- Includes **direction** (+ for ascending, - for descending)
- Example: C4 to E5 = +16 (4 semitones for m3 + 12 for octave)
- Example: E4 to C4 = -4

### 2. Unordered Pitch Interval (UPI)
- Measures distance between **specific pitches** in **semitones**
- **No direction** indicated (always positive)
- Example: C4 to E5 = 16
- Example: E4 to C4 = 4

### 3. Ordered Pitch-Class Interval (OPCI)
- Measures distance between **pitch classes** (octave-independent)
- Always measured **ascending** (clockwise on clock face)
- Range: 0-11
- Example: C to E = 4; E to C = 8

### 4. Interval Class (IC) / Unordered Pitch-Class Interval
- Measures **shortest distance** between pitch classes
- Either direction allowed (shortest path on clock face)
- Range: 0-6 only
- Example: C to E = 4; E to C = 4

**Summary table**:
| Type | Pitch/PC | Ordered? | Range | Example (C-E) |
|------|----------|----------|-------|---------------|
| OPI | Pitch | Yes | unlimited | +4 or -8 |
| UPI | Pitch | No | 0 to unlimited | 4 or 8 |
| OPCI | PC | Yes (ascending) | 0-11 | 4 |
| IC | PC | No | 0-6 | 4 |

## Musical Context

In tonal music, intervals carry qualities (major, minor, etc.) tied to diatonic context: G-Bb is a minor third, while G-A# is an augmented second--different intervals despite sounding the same.

In atonal music, without tonal context, G-Bb and G-A# become equivalent. We measure intervals in semitones rather than by quality, and we choose the appropriate level of abstraction:

- **Ordered pitch intervals**: Useful for melodic contour and voice leading
- **Unordered pitch intervals**: Useful for harmonic analysis of specific sonorities
- **Ordered PC intervals**: Useful for understanding clockwise motion in PC space
- **Interval classes**: Most abstract; useful for comparing sets regardless of voicing or direction

The choice of interval type depends on analytical goals--whether octave, direction, and register matter for the passage being analyzed.

## Examples

### Basic

**Given pitches: C4 and E5**

```
Ordered Pitch Interval:
  C4 to E5 = +16 (up 16 semitones)
  E5 to C4 = -16 (down 16 semitones)

Unordered Pitch Interval:
  C4 to E5 = 16 (direction irrelevant)
  E5 to C4 = 16

Ordered Pitch-Class Interval:
  C to E = 4 (clockwise from C to E)
  E to C = 8 (clockwise from E to C)

Interval Class:
  C and E = 4 (shortest path, either direction)
```

**Given pitches: G3 and Db4**
```
OPI: +6 (ascending tritone)
UPI: 6
OPCI: G to Db = 6 (clockwise); Db to G = 6 (clockwise)
IC: 6 (tritone is its own complement)
```

**Why IC maxes at 6**:
```
If OPCI = 7, the other direction = 5, so IC = 5
If OPCI = 8, the other direction = 4, so IC = 4
...and so on. IC 6 (tritone) is the largest possible.
```

### From Repertoire

**Webern, Symphony Op. 21**: Analysis often focuses on interval classes, revealing the concentration on ic 1, 3, and 4 regardless of how they're voiced in the texture.

**Debussy, "La cathédrale engloutie"**: The opening motive D-E-B can be described:
- As ordered pitch intervals: +2, +7
- As interval classes: 2, 5
Both reveal the open, non-triadic quality without imposing tonal labels.

**Berg, Violin Concerto**: The opening arpeggiated series uses alternating interval classes 3 and 4 (minor thirds and major thirds), creating the triadic sonority that gives the work its distinctive character.

## Related Concepts

- **Prerequisite**: pitch, pitch-class, semitone, integer-notation
- **Leads to**: interval-vector, pitch-class-set, normal-order
- **See also**: clock-face-representation, mod-12-arithmetic, interval-class-content

## Common Confusions

- **Ordered PC interval is always ascending** (clockwise): C to G = 7, G to C = 5 (not -7 or -5)
- **Interval class has only values 0-6**: Once you reach 7, the other direction is shorter (5)
- **IC 6 (tritone) is unique**: It's the only IC that equals its complement (12-6=6)
- **Tonal interval names don't apply**: "Minor third" vs. "augmented second" distinction disappears
- **Ordered vs. unordered is about direction**, not size: unordered intervals are still measured, just without +/-
- **Pitch intervals can exceed 12**: They measure actual semitone distance including octaves
- **Pitch-class intervals max at 11**: Octave is equivalent to 0 in PC space

## Source Reference

Open Music Theory, Part VIII, Chapter 2: "Intervals in Integer Notation"
