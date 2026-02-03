---
concept: Mod 12 Arithmetic
category: theory
source: Open Music Theory
chapter: "Pitch and Pitch Class"
part: 8
---

# Mod 12 Arithmetic

## Quick Definition

Mod 12 arithmetic (modular arithmetic with modulus 12) is the mathematical system underlying pitch-class operations. Numbers "wrap around" at 12, like hours on a clock: 11 + 3 = 2 (not 14), and 2 - 5 = 9 (not -3). This models the cyclic nature of pitch-class space where octave equivalence makes pc 12 identical to pc 0.

## Formal Definition

**Modular arithmetic**: A system of arithmetic where numbers wrap around after reaching a certain value (the modulus).

**Mod 12** (modulo 12): The result of any operation is the remainder when divided by 12.

**Formal definition**: a mod 12 = r, where r is the unique integer with 0 <= r < 12 such that a = 12k + r for some integer k.

**Basic operations**:
- **Addition**: (a + b) mod 12 = remainder of (a + b) / 12
- **Subtraction**: (a - b) mod 12 = remainder of (a - b) / 12 (always positive)
- **Negation**: -a mod 12 = (12 - a) mod 12

**Properties**:
- Results are always in the range 0-11
- Cyclical: 12 = 0, 13 = 1, 14 = 2, etc.
- Negative results wrap: -1 = 11, -2 = 10, -3 = 9, etc.

**Why 12?**: There are 12 pitch classes in equal temperament, and the octave (12 semitones) brings us back to the same pitch class.

## Musical Context

Mod 12 arithmetic models the cyclic nature of pitch-class space:

1. **Octave equivalence**: C4 and C5 are 12 semitones apart but belong to pc 0
2. **Wraparound**: Moving up from B (11) by semitone gives C (0), not 12
3. **Interval calculation**: The interval from G (7) to D (2) is 2 - 7 = -5 = 7 (mod 12)
4. **Transposition**: T5 of B (11) = 11 + 5 = 16 = 4 (mod 12) = E

**Clock face visualization**: A clock with pitch classes at each hour helps visualize mod 12:
- 12 o'clock = C = 0
- Moving clockwise = adding
- Moving counterclockwise = subtracting (or adding the complement)

## Examples

### Basic

**Addition (mod 12)**:
```
7 + 3 = 10 (no wraparound needed)
8 + 7 = 15 = 3 (15 - 12 = 3)
11 + 5 = 16 = 4
11 + 1 = 12 = 0
6 + 6 = 12 = 0 (tritone above tritone = octave)
```

**Subtraction (mod 12)**:
```
7 - 3 = 4 (no wraparound needed)
3 - 7 = -4 = 8 (-4 + 12 = 8)
0 - 5 = -5 = 7
2 - 11 = -9 = 3
```

**Finding complements (12 - x)**:
```
Complement of 0 = 12 - 0 = 0 (special case)
Complement of 1 = 11
Complement of 3 = 9
Complement of 6 = 6 (tritone is self-complementary)
Complement of 11 = 1
```

**Applied to pitch classes**:
```
G (7) + perfect 5th (7) = D (2)
  7 + 7 = 14 = 2 (mod 12)

E (4) - major 3rd (4) = C (0)
  4 - 4 = 0

A (9) transposed up by T5:
  9 + 5 = 14 = 2 (mod 12) = D
```

### From Repertoire

**Any transposition operation**: When Schoenberg transposes a row by T5, each pc x becomes (x + 5) mod 12.

**Calculating inversion**: I7 of pc 3 is 7 - 3 = 4. I7 of pc 11 is 7 - 11 = -4 = 8.

**Finding Tn relationships**: If set A is [3, 7, 10] and set B is [8, 0, 3], then B = Tn(A) where n = 8 - 3 = 5. Verify: 7 + 5 = 0, 10 + 5 = 3. Yes, B = T5(A).

## Related Concepts

- **Prerequisite**: integer-notation, pitch-class
- **Leads to**: transposition-tn, inversion-in, interval-class
- **See also**: pitch-class-set

## Common Confusions

- **Negative results must be converted**: -3 mod 12 = 9, not -3
- **12 = 0**: There is no pitch class 12; it wraps to 0
- **Adding 12 (or any multiple) doesn't change the pc**: 5 = 17 = 29 = etc.
- **Complementary intervals**: If interval up is n, interval down is 12-n
- **Not regular arithmetic**: In regular math, 11 + 3 = 14; in mod 12, 11 + 3 = 2
- **Why mod 12?**: Because there are 12 pitch classes in equal temperament
- **The clock face is your friend**: Visualize operations as clockwise (add) or counterclockwise (subtract)

## Source Reference

Open Music Theory, Part VIII, Chapters 1-3
