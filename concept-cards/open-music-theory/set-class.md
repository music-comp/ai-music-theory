---
concept: Set Class
category: theory
source: Open Music Theory
chapter: "Set Class and Prime Form"
part: 8
---

# Set Class

## Quick Definition

A set class is a collection of all pitch-class sets that are related by transposition (Tn) or inversion (In). Just as all major triads belong to one "type," all sets in a set class share the same interval content and thus sound fundamentally similar, regardless of which specific pitch classes they contain.

## Formal Definition

**Set class** (or pitch-class set class): The equivalence class of all pitch-class sets related by Tn and/or In operations.

**Equivalence relation**: Two pcsets A and B are members of the same set class if and only if B = Tn(A) or B = In(A) for some n.

**Properties of a set class**:
- All members share the same interval vector
- All members share the same prime form (the label for the set class)
- The number of members depends on the set's symmetry properties

**Naming conventions**:
- **Prime form**: The most compact form transposed to start on 0, written in parentheses: (014)
- **Forte number**: A catalog number (e.g., 3-3), where the first digit is cardinality and the second is position in Forte's list

**Terminology hierarchy**:
```
Pitch class: group of pitches (by octave/enharmonic equivalence)
Pitch-class set: group of pitch classes
Set class: group of pitch-class sets (by Tn/In equivalence)
```

## Musical Context

Set class provides the most abstract level of harmonic/melodic classification in set theory. Its utility:

1. **Explains similarity**: Why do certain sonorities in different transpositions/inversions sound related? They're members of the same set class.

2. **Simplifies analysis**: Instead of tracking individual pcsets, we can track set classes. A piece might use only 3-4 set classes despite dozens of different pcsets.

3. **Reveals compositional method**: Composers often build pieces around a small number of set classes, creating unity through consistent interval content.

The concept is analogous to (but more general than) tonal chord types: just as "major triad" encompasses C major, F# major, etc., set class (037) encompasses all major and minor triads.

## Examples

### Basic

**Major and minor triads as one set class**:
```
C major: {0, 4, 7}
D major: {2, 6, 9} = T2{0, 4, 7}
F minor: {5, 8, 0} = I0{0, 4, 7}
A minor: {9, 0, 4} = I4{0, 4, 7}

All are related by Tn or In
All are members of set class (037)
Forte number: 3-11
```

**A set class with its members**:
```
Set class (014):

Some members by transposition:
T0: {0, 1, 4}
T1: {1, 2, 5}
T5: {5, 6, 9}
...

Some members by inversion:
I0{0, 1, 4} = {0, 11, 8} = [8, 11, 0]
I5{0, 1, 4} = {5, 4, 1} = [1, 4, 5]
...

All 24 versions share interval vector <1,0,1,1,1,0>
```

**Comparing set classes**:
```
(013): semitone + whole tone
(014): semitone + minor third
(015): semitone + major third

Different set classes, different interval content, different sonic "color"
```

### From Repertoire

**Bartok, "Subject and Reflection"**: All four sets (two per passage, right and left hands) belong to the same set class (02357). Though they appear at different transposition levels and in inversional relationships, they share identical interval content.

**Schoenberg, Pierrot lunaire**: The recurring "Nacht" motive and its transformations all belong to set class (014). Tracking this set class through the movement reveals motivic unity.

**Webern, Concerto Op. 24**: Built from transformations of a single trichord--all manifestations belong to set class (014), creating extreme economy of materials.

## Related Concepts

- **Prerequisite**: pitch-class-set, transposition-Tn, inversion-In, interval-class
- **Leads to**: prime-form, interval-vector, forte-number, z-relation
- **See also**: normal-order, cardinality, set-class-table

## Common Confusions

- **Set class vs. pitch-class set**: A pcset is a specific collection; a set class is the group of all Tn/In-related pcsets
- **Prime form is a label, not a privileged set**: (014) doesn't make {0, 1, 4} more "important" than {3, 4, 7}
- **Major and minor triads share a set class**: They're inversionally related, hence the same set class (037)
- **Not all sets have 24 members in their class**: Symmetrical sets have fewer (e.g., augmented triad has only 4 members in its set class)
- **Forte numbers are arbitrary**: 3-3 doesn't mean "three of something"--it's just a catalog position
- **The finite number of set classes**: There are only 220 set classes total (including the null set and the aggregate)
- **Interval vector is the same for all members**: This is what makes set class membership aurally meaningful

## Source Reference

Open Music Theory, Part VIII, Chapter 4: "Set Class and Prime Form"
