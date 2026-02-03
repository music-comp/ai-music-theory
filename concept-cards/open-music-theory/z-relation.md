---
concept: Z-Relation
category: theory
source: Open Music Theory
chapter: "Set Class and Prime Form"
part: 8
---

# Z-Relation

## Quick Definition

Two set classes are Z-related if they share the same interval vector but are not related by transposition or inversion--they are distinct set classes that happen to have identical interval-class content. The "Z" stands for "zygotic" (twin-like). Z-related sets sound similar due to shared interval content but are structurally distinct.

## Formal Definition

**Z-relation**: A relationship between two distinct set classes that share the same interval vector.

**Properties**:
- Z-related sets have **identical interval vectors**
- Z-related sets are **not** Tn or In equivalent (otherwise they'd be the same set class)
- Z-related sets have **different prime forms**
- Z-relation occurs only in sets of cardinality 4 and higher

**Notation**: Z-related sets are often paired in set-class tables. Forte numbers include "Z" to indicate the relationship:
- 4-Z15 and 4-Z29 are Z-related
- 6-Z17 and 6-Z43 are Z-related

**Frequency**:
- No Z-related trichords (cardinality 3)
- 3 pairs of Z-related tetrachords (cardinality 4)
- 3 pairs of Z-related pentachords (cardinality 5)
- 15 pairs of Z-related hexachords (cardinality 6)

**Hexachordal theorem**: Z-related hexachords are always complements of each other. (The complement of a set is the collection of all pitch classes not in the set.)

## Musical Context

Z-relation presents a puzzle: How can two sets have identical interval content yet not be transpositions or inversions of each other?

The answer lies in the **arrangement** of intervals. While both Z-related sets contain the same intervals, those intervals are distributed among the pitch classes differently.

**Analytical implications**:
- Z-related sets may sound similar (same "color")
- But they cannot be connected by Tn or In
- A piece using both Z-related set classes creates unity through interval content while maintaining structural distinction

**Compositional implications**:
- Some composers exploit Z-relations for subtle variety-within-unity
- The ear perceives similarity while the structure remains distinct

## Examples

### Basic

**The Z-related tetrachords (0146) and (0137)**:
```
Prime form (0146):
  Example: {0, 1, 4, 6}
  Interval vector: <1,1,1,1,1,1>

Prime form (0137):
  Example: {0, 1, 3, 7}
  Interval vector: <1,1,1,1,1,1>

Same vector, but:
  No Tn of (0146) yields (0137)
  No In of (0146) yields (0137)

They are structurally distinct despite identical interval content.
```

**Verifying they're not Tn/In related**:
```
(0146): intervals between consecutive pcs: 1, 3, 2
(0137): intervals between consecutive pcs: 1, 2, 4

These interval sequences are neither the same nor retrogrades
Therefore, not transpositionally or inversionally related
```

**Z-related hexachords (all are complementary)**:
```
6-Z17 (012478) and 6-Z43 (012568)
Interval vector: <3,2,2,3,3,2>
Complements of each other

6-Z19 (013478) and 6-Z44 (012569)
Interval vector: <3,1,3,2,3,3>
```

**Why no Z-related trichords?**
```
For trichords, the interval vector fully determines the set class.
With only 3 pitch classes (3 intervals), there aren't enough
degrees of freedom to create structurally different arrangements.
```

### From Repertoire

**Bartok**: Some analysts have found Z-related set classes in Bartok's music, where the shared interval content creates unity across sections that use structurally distinct sets.

**Elliott Carter**: Known for using all-interval tetrachords, Carter was interested in sets with rich interval content, including Z-related pairs.

**Webern, Variations Op. 27**: Analysts have noted relationships involving Z-related hexachords in the twelve-tone structure.

## Related Concepts

- **Prerequisite**: interval-vector, set-class, prime-form
- **Leads to**: hexachordal-combinatoriality, all-interval-tetrachords
- **See also**: forte-number

## Common Confusions

- **Z-related sets are NOT in the same set class**: They have different prime forms
- **Z-relation is about identical interval vectors**: Not about similar prime forms
- **"Z" doesn't mean "zero"**: It comes from "zygotic" (twin-like)
- **Z-relation is rare in small sets**: Only exists in cardinality 4 and above
- **Z-related hexachords are always complements**: This is not true for tetrachords or pentachords
- **Same interval vector does NOT mean same set class**: Z-relation proves this
- **Sound similarity =/= structural identity**: Z-related sets sound similar but can't be connected by Tn/In

## Source Reference

Open Music Theory, Part VIII, Chapter 4: "Set Class and Prime Form"
