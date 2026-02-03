---
concept: Pitch-Class Set
category: theory
source: Open Music Theory
chapter: "Pitch-Class Sets, Normal Order, and Transformations"
part: 8
---

# Pitch-Class Set

## Quick Definition

A pitch-class set (pcset) is an unordered collection of pitch classes treated as a single analytical unit. Any group of notes can form a pcset--a chord, a melodic fragment, or even non-contiguous pitches that an analyst groups together for structural reasons.

## Formal Definition

**Pitch-class set (pcset)**: An unordered collection of distinct pitch classes, typically written in curly braces or square brackets.

**Notation conventions**:
- Unordered set: {0, 4, 7} or {C, E, G}
- Normal order: [0, 4, 7] (most compact ascending arrangement)
- Duplicates eliminated: {C, E, G, C} = {C, E, G} = {0, 4, 7}

**Properties**:
- **Cardinality**: The number of distinct pitch classes in the set (e.g., {0, 4, 7} has cardinality 3)
- **Order-independent**: {0, 4, 7} = {7, 0, 4} = {4, 7, 0}
- **Duplicates collapse**: Each pc appears at most once
- **Octave-independent**: C4, E5, G3 yields the same pcset as C5, E4, G6

**Set vs. Class terminology**:
- A **set** is any group collected by the analyst
- A **class** is a group related by some equivalence (octave, enharmonic, etc.)
- "Pitch-class set" is a set of pitch classes (not a class of sets--that's "set class")

## Musical Context

In tonal analysis, we name harmonies by their root and quality (C major, F#dim7). But this system assumes:
- Triadic/seventh-chord structure
- Root position as referential
- Tonal function

Post-tonal music often uses sonorities that don't fit these categories. A pcset approach allows us to:
- Analyze any combination of pitch classes
- Find relationships between seemingly different harmonies
- Track motivic pitch content regardless of octave or voicing

Pcsets are fundamental units in set theory analysis--much as "triad" or "seventh chord" are fundamental units in tonal analysis.

## Examples

### Basic

**A C major chord as a pcset**:
```
Pitches: C4, E4, G4
Pitch classes: C, E, G
In integers: 0, 4, 7
As a set: {0, 4, 7}
In normal order: [0, 4, 7]
```

**A more complex example**:
```
Pitches: Bb3, E4, F#4, C5
Pitch classes: Bb, E, F#, C
In integers: 10, 4, 6, 0
As a set: {0, 4, 6, 10}
Cardinality: 4 (a tetrachord)
```

**Cardinality names**:
```
2 pc: dyad
3 pc: trichord
4 pc: tetrachord
5 pc: pentachord
6 pc: hexachord
7 pc: septachord
8 pc: octachord
9 pc: nonachord
```

### From Repertoire

**Schoenberg, "Nacht" from Pierrot lunaire**: The recurring motive E-G-Eb forms pcset {3, 4, 7}. This "night" motive appears throughout in various transpositions and inversions, all sharing the same set-class identity.

**Debussy, "La cathedrale engloutie"**: The opening motive <D, E, B> or <2, 4, 11> forms a trichord that returns at T4 as <F#, G#, D#> or <6, 8, 3>--both are the same set class.

**Chen Yi, "Duo Ye"**: Pcsets [2, 4, 7] and [1, 4, 6] appear in the opening, related by inversion--they share the same interval content in reversed arrangement.

## Related Concepts

- **Prerequisite**: pitch-class, integer-notation, interval-class
- **Leads to**: normal-order, prime-form, set-class, transposition-tn, inversion-in
- **See also**: interval-vector, cardinality, segmentation

## Common Confusions

- **Set vs. set class**: A pcset is a specific collection; a set class groups all transpositions and inversions of that collection
- **Order doesn't matter in sets**: {0, 4, 7} = {4, 7, 0} (but order matters for ordered sets and melodies)
- **Duplicates are eliminated**: The chord C-E-G-C' is still pcset {0, 4, 7}
- **Curly braces vs. square brackets**: {0, 4, 7} is unordered; [0, 4, 7] implies normal order
- **Any pitch collection can be a pcset**: It doesn't need to be a traditional chord or scale
- **"Pitch-class set" is not the same as "set class"**: The former is a specific set; the latter is a class of related sets
- **Cardinality = number of elements**: A pcset with 3 pitch classes has cardinality 3, regardless of how many actual notes are played

## Source Reference

Open Music Theory, Part VIII, Chapter 3: "Pitch-Class Sets, Normal Order, and Transformations"
