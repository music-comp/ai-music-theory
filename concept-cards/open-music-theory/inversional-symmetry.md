---
concept: Inversional Symmetry
category: theory
source: Open Music Theory
chapter: "Collections"
part: 8
---

# Inversional Symmetry

## Quick Definition

A pitch-class set has inversional symmetry if inverting it around some axis maps the set onto itself. Inversionally symmetrical sets include the diatonic collection (symmetrical around D in the white-note collection), the whole-tone, octatonic, and hexatonic collections, and simpler structures like the tritone and diminished seventh chord. Such sets have a natural "balance" or "mirror" quality.

## Formal Definition

**Inversional symmetry**: A property of pitch-class sets where In(S) = S for some n.

**Mathematical formulation**:
A set S has inversional symmetry at index n if:
- For every pc x in S, (n - x) mod 12 is also in S
- Equivalently: In(S) = S

**Axis of symmetry**: The index n determines an axis in pitch-class space:
- If n is even, the axis passes through two pitch classes (n/2 and n/2+6)
- If n is odd, the axis passes between pitch classes

**Examples of inversionally symmetrical sets**:
- Tritone {0, 6}: I0(0,6) = {0, 6}
- Diminished 7th {0, 3, 6, 9}: I0(0,3,6,9) = {0, 9, 6, 3} = same
- Whole-tone WT0 {0, 2, 4, 6, 8, 10}: Multiple axes of symmetry
- Diatonic {0, 2, 4, 5, 7, 9, 11}: Axis through 2 and 8 (D and Ab)

**Relationship to transpositional symmetry**: Many collections have both types, but they're independent properties. The major triad has neither; the tritone has both.

## Musical Context

Inversional symmetry creates a sense of balance or mirroring:

1. **Mirror structures**: Upper and lower "halves" reflect each other
2. **Pitch-axis composition**: Melodies that mirror around a central pitch
3. **Symmetrical voice leading**: Voices moving in contrary motion
4. **Bartok's "axis system"**: Organizing tonality around symmetrical pitch relationships

**Compositional applications**:
- **Palindromic structures**: Retrograde equals original when inverted
- **Wedge motion**: Lines moving apart or together symmetrically
- **Inversional complement**: A melody and its inversion can sound simultaneously

**Analytical applications**:
- Identifying the axis helps understand voice-leading possibilities
- Symmetrical sets have special properties in twelve-tone contexts
- Neo-Riemannian operations often involve inversional relationships

## Examples

### Basic

**Testing for inversional symmetry**:
```
Set: {0, 1, 6, 7} - is it inversionally symmetrical?

Try I0: Each pc x maps to (0 - x) mod 12
  0 -> 0
  1 -> 11 (not in set!)

Try I7: Each pc x maps to (7 - x) mod 12
  0 -> 7 (in set!)
  1 -> 6 (in set!)
  6 -> 1 (in set!)
  7 -> 0 (in set!)

Yes! {0, 1, 6, 7} is symmetrical under I7
Axis passes between 3.5 and 9.5 (between Eb-E and Bb-B)
```

**The diatonic collection's axis**:
```
White notes: {0, 2, 4, 5, 7, 9, 11} = {C, D, E, F, G, A, B}

Under I4:
  0 -> 4, 4 -> 0 (C and E swap)
  2 -> 2 (D maps to itself - on the axis!)
  5 -> 11, 11 -> 5 (F and B swap)
  7 -> 9, 9 -> 7 (G and A swap)

D is on the axis of symmetry for the white-note collection
```

**Tritone (maximally symmetrical for a dyad)**:
```
{0, 6} is symmetrical under I0, I6, I2, I4, I8, I10...
Every even index works!
The tritone divides the octave in half, creating maximum symmetry
```

### From Repertoire

**Bartok, Music for Strings, Percussion, and Celesta**: The fugue subject is constructed with inversional symmetry around A. The work as a whole uses axis-based tonal organization.

**Webern, Symphony Op. 21**: The twelve-tone row is constructed to be inversionally symmetrical, and the formal structure mirrors this property.

**Bartok, "Subject and Reflection"**: The title explicitly references inversional relationships--material in one hand is inverted in the other.

**Berg, Lyric Suite**: Uses a row with special inversional properties that create structural relationships throughout the work.

## Related Concepts

- **Prerequisite**: inversion-in, pitch-class-set, interval-class
- **See also**: transpositional-symmetry

## Common Confusions

- **Inversional symmetry =/= transpositional symmetry**: Different properties
- **The axis is determined by the index n**: I0 has axis through C and F#; I2 through C#/Db and G
- **Not all sets are inversionally symmetrical**: Major triads are not
- **Multiple axes can exist**: Highly symmetrical sets have many axes
- **Symmetry under In means set equals its own inversion**: In(S) = S
- **The axis is where pitch-class pairs "balance"**: Pairs sum to the index n
- **Clock-face visualization**: The axis passes through the clock, dividing the set into mirrored halves

## Source Reference

Open Music Theory, Part VIII, Chapter 8: "Collections"
