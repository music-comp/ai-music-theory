---
concept: Inversion (In)
category: theory
source: Open Music Theory
chapter: "Pitch-Class Sets, Normal Order, and Transformations"
part: 8
---

# Inversion (In)

## Quick Definition

Inversion (In) is a two-step operation: first invert each pitch class (take its complement mod 12, turning x into 12-x), then transpose by n. The result preserves interval content but reverses the arrangement of intervals. In subtraction form: In(x) = n - x (mod 12).

## Formal Definition

**Inversion operation In**: For each pitch class x in a set:
In(x) = n - x (mod 12)

**Two methods**:

1. **Invert-then-transpose method**:
   - Step 1: Invert each pc (replace x with -x mod 12, i.e., 12 - x for x > 0)
   - Step 2: Transpose by n (add n mod 12)

2. **Subtraction method**:
   - Simply calculate n - x for each pitch class x (mod 12)

**Index number (n)**: The sum of corresponding pitch classes in inversionally related pairs.
- If In(x) = y, then x + y = n

**Interval reversal**: Where Tn preserves the exact sequence of intervals, In reverses it.
```
Original: intervals 3, 5, 2
Inverted: intervals 2, 5, 3 (reversed order)
```

**Finding the index number between two sets**:
If sets A and B are inversionally related:
- Write A in normal order, B in normal order
- Pair elements crosswise (first of A with last of B, etc.)
- Add paired elements; all sums should equal n

## Musical Context

Inversion in set theory differs from tonal melodic inversion:
- Tonal inversion: Up becomes down within a scale/mode
- Set-theory inversion: A mathematical operation that reverses interval order

Inversionally related sets share the same interval-class content but with intervals arranged in mirror fashion. This creates:
- **Audible similarity**: Same "color" or sonority type
- **Structural opposition**: Mirror relationship, useful for creating contrast-within-unity

Composers like Webern exploited inversional symmetry extensively, while others (like Schoenberg) used In relationships more freely as one technique among many.

## Examples

### Basic

**Inverting [2, 4, 7] by I8**:

Method 1 (Invert-then-transpose):
```
Step 1 - Invert (12 - x):
  2 -> 10
  4 -> 8
  7 -> 5
Inverted set: {10, 8, 5}

Step 2 - Transpose by 8:
  10 + 8 = 18 = 6 (mod 12)
  8 + 8 = 16 = 4 (mod 12)
  5 + 8 = 13 = 1 (mod 12)

I8[2, 4, 7] = {6, 4, 1} = [1, 4, 6] (in normal order)
```

Method 2 (Subtraction):
```
I8: calculate 8 - x for each pc
  8 - 2 = 6
  8 - 4 = 4
  8 - 7 = 1

I8[2, 4, 7] = {6, 4, 1} = [1, 4, 6]
```

**Finding the index number**:
```
Set A: [2, 4, 7]
Set B: [1, 4, 6]

Cross-add (first with last, second with second-to-last, etc.):
  2 + 6 = 8
  4 + 4 = 8
  7 + 1 = 8

Index number = 8, so B = I8(A)
```

**Comparing intervals**:
```
[2, 4, 7]: intervals are 2, 3 (between adjacent pcs)
[1, 4, 6]: intervals are 3, 2 (reversed!)
```

### From Repertoire

**Chen Yi, "Duo Ye"**: The sets [2, 4, 7] and [1, 4, 6] appear in succession. They share the same intervals (2 and 3, plus the outer interval 5) but in reversed arrangement. This is the I8 relationship.

**Bartok, "Subject and Reflection"**: Within each passage, the right and left hands are inversionally related--one hand "reflects" the other. The title itself references this technique.

**Webern, Symphony Op. 21**: The entire work is built on inversional symmetry. The 12-tone row is constructed so that its second half is the inversion of its first half, creating palindromic structures.

## Related Concepts

- **Prerequisite**: pitch-class-set, integer-notation, mod-12-arithmetic, transposition-Tn
- **Leads to**: set-class, prime-form, Tn/In-equivalence, index-number, inversional-symmetry
- **See also**: normal-order, interval-vector, twelve-tone-operations

## Common Confusions

- **In is NOT just "flip"**: It's specifically invert-then-transpose by n (or equivalently, subtract from n)
- **I0 inverts around C/F#**: The axis of inversion passes through pc 0 and pc 6
- **Index number = sum of corresponding pcs**: In inversionally related pairs, each pair sums to n
- **The "n" in In is NOT the transposition after inversion**: It's the index, which determines where the inversion axis lies
- **Interval content is preserved**: Same interval classes, but the order reverses
- **Cross-addition for finding n**: First element of one set pairs with last element of the other
- **In and Tn are distinct operations**: Tn preserves interval sequence; In reverses it
- **I0 is not the identity**: I0 is inversion around 0 (mapping x to -x)

## Source Reference

Open Music Theory, Part VIII, Chapter 3: "Pitch-Class Sets, Normal Order, and Transformations"
