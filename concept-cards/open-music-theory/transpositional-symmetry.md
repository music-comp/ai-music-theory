---
concept: Transpositional Symmetry
category: theory
source: Open Music Theory
chapter: "Collections"
part: 8
---

# Transpositional Symmetry

## Quick Definition

A pitch collection has transpositional symmetry if transposing it by some interval (other than the octave) maps it onto itself--producing the same collection of pitch classes. The whole-tone collection transposes onto itself at T2, the octatonic at T3, and the hexatonic at T4. This symmetry limits the number of unique transpositions possible.

## Formal Definition

**Transpositional symmetry**: A property of pitch-class sets where Tn(S) = S for some n between 1 and 11.

**Mathematical formulation**:
A set S has transpositional symmetry at level n if:
- For every pc x in S, (x + n) mod 12 is also in S
- Equivalently: Tn(S) = S

**Period of symmetry**: The smallest n > 0 for which Tn(S) = S

**Number of unique transpositions**: 12 / (period of symmetry)

**Symmetrical collections and their properties**:

| Collection | Period | Unique Transpositions |
|-----------|--------|----------------------|
| Chromatic | 1 | 1 (all 12 pcs) |
| Whole-tone | 2 | 2 |
| Octatonic | 3 | 3 |
| Hexatonic | 4 | 4 |
| Augmented triad | 4 | 4 |
| Diminished 7th | 3 | 3 |
| Tritone | 6 | 6 |

**Non-symmetrical collections** (12 unique transpositions):
- Diatonic collection
- Pentatonic collection
- Acoustic collection
- Major/minor triads

## Musical Context

Transpositional symmetry creates distinctive musical effects:

1. **Ambiguity**: Any "rotation" sounds like the same collection
2. **Limited transposition**: Fewer unique forms means greater coherence
3. **Modal equivalence**: Different starting points don't change the collection
4. **Compositional economy**: Exploring all forms doesn't take long

**Messiaen's interest**: Called these "modes of limited transposition"--scales that cannot be transposed in 12 unique ways. He valued their "charm of impossibilities."

**Bartok's usage**: Combined symmetrical subsets (like [0,1,6,7]) to build larger structures.

**The tritone's special status**: IC 6 is the only interval class that divides the octave exactly in half, giving it unique symmetrical properties.

## Examples

### Basic

**Whole-tone symmetry (period = 2)**:
```
WT0: {0, 2, 4, 6, 8, 10}

T2(WT0) = {2, 4, 6, 8, 10, 0} = WT0 (same set!)
T4(WT0) = {4, 6, 8, 10, 0, 2} = WT0 (same set!)

12 transposition levels / 2 period = 6 equivalent forms
But pairs are identical, so only 2 UNIQUE whole-tone collections
```

**Octatonic symmetry (period = 3)**:
```
OCT0,1: {0, 1, 3, 4, 6, 7, 9, 10}

T3(OCT0,1) = {3, 4, 6, 7, 9, 10, 0, 1} = OCT0,1 (same set!)

12 transposition levels / 3 period = 4 equivalent forms
But groups of 4 are identical, so only 3 UNIQUE octatonic collections
```

**Diminished seventh symmetry (period = 3)**:
```
Cdim7: {0, 3, 6, 9}

T3({0, 3, 6, 9}) = {3, 6, 9, 0} = same set!

Only 3 unique diminished seventh chords exist
```

**Non-symmetrical example (major triad)**:
```
C major: {0, 4, 7}

T1({0, 4, 7}) = {1, 5, 8} = C# major (different set)
T2({0, 4, 7}) = {2, 6, 9} = D major (different set)
...

All 12 transpositions yield different sets
Major triads have NO transpositional symmetry
```

### From Repertoire

**Debussy, "Voiles"**: The whole-tone passages exploit the collection's symmetry--the music can shift by whole steps without changing the pitch material, creating the floating quality.

**Stravinsky, Firebird**: Octatonic passages can be transposed by minor third without changing collection, facilitating the kaleidoscopic harmonic shifts.

**Messiaen, Quatuor pour la fin du temps**: The "Liturgie de cristal" uses modes of limited transposition, with different instruments cycling through symmetrical patterns at different rates.

**Bartok**: Often composes with symmetrical tetrachords (like the diminished seventh or [0,1,6,7]), exploiting their limited transpositions for structural coherence.

## Related Concepts

- **Prerequisite**: transposition-tn, pitch-class-set, interval-class
- **Leads to**: modes-of-limited-transposition, inversional-symmetry
- **See also**: whole-tone-collection, octatonic-collection, hexatonic-collection

## Common Confusions

- **Transpositional symmetry =/= inversional symmetry**: Different properties, often but not always co-present
- **Period is the smallest symmetrical interval**: If T3 works, T6 and T9 also work, but the period is 3
- **All sets are symmetrical at T0 and T12**: We don't count these as "symmetry"
- **Non-symmetrical sets have 12 unique forms**: Diatonic, pentatonic, major/minor triads
- **Symmetrical subsets can build non-symmetrical wholes**: Two dim7 chords a semitone apart form octatonic
- **"Mode of limited transposition" = transpositionally symmetrical scale**: Messiaen's term
- **The chromatic scale has maximum symmetry**: T1 maps it onto itself (period = 1)

## Source Reference

Open Music Theory, Part VIII, Chapter 8: "Collections"
