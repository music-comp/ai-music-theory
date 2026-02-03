---
concept: Normal Order
category: theory
source: Open Music Theory
chapter: "Pitch-Class Sets, Normal Order, and Transformations"
part: 8
---

# Normal Order

## Quick Definition

Normal order (or normal form) is the most compact way to arrange the pitch classes of a set in ascending order, analogous to how "root position" standardizes triads. It provides a consistent way to compare and label pitch-class sets.

## Formal Definition

**Normal order**: The ascending arrangement of a pitch-class set that spans the smallest possible interval from first to last element.

**Algorithm to find normal order**:

1. **List all rotations**: Arrange the pitch classes in ascending order within an octave, then create all rotations
2. **Find the smallest outer interval**: Compare the interval from first to last pc in each rotation
3. **Choose the rotation with the smallest span**
4. **Tiebreaker**: If spans are equal, choose the rotation most "packed to the left" (smallest intervals at the beginning)

**Notation**: Normal order is written in square brackets: [0, 4, 7]

**Mathematical representation**:
For a set of cardinality n:
- Calculate all n rotations
- For each rotation, measure interval from element 1 to element n (mod 12)
- Select the rotation with minimum span; if tied, select the one with minimum interval between elements 1 and 2, then 2 and 3, etc.

## Musical Context

Just as root position provides a standard form for triads, normal order provides a standard form for any pitch-class set. This standardization:

- Allows consistent comparison between sets
- Facilitates identification of transpositional relationships
- Serves as the starting point for finding prime form
- Makes sets recognizable regardless of how they appear in the music

Normal order is **descriptive, not prescriptive**: the normal order [0, 4, 7] doesn't mean C should be the bass note--it's simply a standard representation of the set.

## Examples

### Basic

**Finding normal order for {G#, A, D#}**:
```
Step 1: Write pitch classes in ascending order
  8, 9, 3 (G#, A, D#)

Step 2: List all rotations and their spans
  [8, 9, 3]: span from 8 to 3 = 7 (3 + 12 - 8 = 7)
  [9, 3, 8]: span from 9 to 8 = 11
  [3, 8, 9]: span from 3 to 9 = 6 <-- smallest!

Step 3: Normal order is [3, 8, 9]
```

**A set with a tie**:
```
Set: {C, D, F#, G#} = {0, 2, 6, 8}

Rotations and spans:
  [0, 2, 6, 8]: span = 8
  [2, 6, 8, 0]: span = 10
  [6, 8, 0, 2]: span = 8  <-- tie!
  [8, 0, 2, 6]: span = 10

Tiebreaker: Compare [0, 2, 6, 8] and [6, 8, 0, 2]
  [0, 2, 6, 8]: first interval = 2
  [6, 8, 0, 2]: first interval = 2  <-- still tied!

  [0, 2, 6, 8]: second interval = 4
  [6, 8, 0, 2]: second interval = 4  <-- still tied!

  [0, 2, 6, 8]: third interval = 2
  [6, 8, 0, 2]: third interval = 2

Both are equally packed. Convention: choose the one starting with the lower number.
Normal order: [0, 2, 6, 8]
```

**Clock face method**:
```
1. Plot pitch classes on clock face
2. Find the largest "gap" (arc without notes)
3. Read clockwise from the note after the gap
4. The first note you encounter starts the normal order
```

### From Repertoire

**Schoenberg, Pierrot lunaire, "Nacht"**: The "Nacht" motive {E, G, Eb} = {3, 4, 7}
```
Rotations: [3, 4, 7], [4, 7, 3], [7, 3, 4]
Spans: 4, 11, 9
Normal order: [3, 4, 7]
```

**Debussy, La mer**: Finding normal orders of recurring sonorities helps identify relationships between seemingly different chords that are actually transpositions of the same set.

## Related Concepts

- **Prerequisite**: pitch-class-set, integer-notation, mod-12-arithmetic
- **Leads to**: prime-form, set-class, transposition-tn, inversion-in
- **See also**: interval-vector

## Common Confusions

- **Normal order is not the same as prime form**: Normal order is the most compact arrangement; prime form requires additional steps (transposing to 0 and comparing with inversion)
- **Normal order is not root position**: It's a standardization for comparison, not a claim about which note is "fundamental"
- **The starting pitch class can be any integer**: Normal order [3, 8, 9] is valid--it doesn't need to start on 0
- **Square brackets signal normal order**: Use [brackets] for normal order, {braces} for unordered sets
- **"Packed to the left" means smallest intervals first**: In a tie, the set with smaller intervals early wins
- **Span is measured mod 12**: The interval from the first to the last element, going ascending (clockwise)

## Source Reference

Open Music Theory, Part VIII, Chapter 3: "Pitch-Class Sets, Normal Order, and Transformations"
