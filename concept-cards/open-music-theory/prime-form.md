---
concept: Prime Form
category: theory
source: Open Music Theory
chapter: "Set Class and Prime Form"
part: 8
---

# Prime Form

## Quick Definition

Prime form is the standard label for a set class: the version of a set that is (1) transposed to begin on 0 and (2) arranged in the most compact form possible, choosing between the normal order and its inversion, whichever is more compact to the left. Written in parentheses without commas: (014).

## Formal Definition

**Prime form**: The canonical representative of a set class, used as its name/label.

**Algorithm to find prime form**:

1. Put the pitch-class set in **normal order**
2. **Transpose to 0**: Subtract the first element from all elements
3. **Invert and compare**:
   - Apply I0 to the original normal order
   - Put the result in normal order
   - Transpose to 0
4. **Choose the most compact**: Compare step 2 and step 3; the one "most packed to the left" (smaller intervals appearing earlier) is the prime form
5. **Notation**: Write in parentheses without commas

**Packed to the left**: When comparing two candidates, the one with smaller intervals near the beginning wins. Compare element by element from left to right until one is smaller.

**Alternative conventions**: Some theorists (following Forte) use different algorithms, occasionally yielding different prime forms for the same set class. The method above follows Rahn's convention.

## Musical Context

Prime form serves as a **standardized label** for set classes, enabling:

1. **Lookup**: Find information about a set class in tables
2. **Communication**: Refer unambiguously to a set class
3. **Comparison**: Quickly see relationships between sets

**Important**: Prime form is just a **name**, not a privileged or "original" version. The set {3, 4, 7} is just as valid a member of set class (014) as {0, 1, 4} is. The prime form simply provides a consistent way to refer to the class.

## Examples

### Basic

**Finding prime form for {G#, A, D#} = {8, 9, 3}**:

```
Step 1: Normal order
  Rotations: [8, 9, 3], [9, 3, 8], [3, 8, 9]
  Spans: 7, 11, 6
  Normal order: [3, 8, 9]

Step 2: Transpose to 0
  [3, 8, 9] - 3 = [0, 5, 6]

Step 3: Invert and process
  I0[3, 8, 9] = [9, 4, 3]
  Normal order of {9, 4, 3}: [3, 4, 9]
  Transpose to 0: [0, 1, 6]

Step 4: Compare
  [0, 5, 6]: intervals 5, then 1
  [0, 1, 6]: intervals 1, then 5

  [0, 1, 6] has the smaller first interval (1 < 5)
  Prime form: (016)
```

**Finding prime form for a major triad {C, E, G} = {0, 4, 7}**:
```
Step 1: Normal order = [0, 4, 7]

Step 2: Already starts on 0: [0, 4, 7]

Step 3: Invert I0[0, 4, 7] = [0, 8, 5]
  Normal order: [5, 8, 0]
  Transpose to 0: [0, 3, 7]

Step 4: Compare
  [0, 4, 7]: intervals 4, 3
  [0, 3, 7]: intervals 3, 4

  [0, 3, 7] is more packed left (3 < 4)
  Prime form: (037)
```

**Common prime forms**:
```
(012): chromatic trichord
(013): minor second + major second
(014): "Viennese" trichord (common in Berg/Webern)
(015): semitone + major third
(016): semitone + tritone
(024): whole-tone trichord
(025): sus4 or incomplete dominant 7th
(027): stacked fourths/fifths
(036): diminished triad
(037): major/minor triad
(048): augmented triad
```

### From Repertoire

**Schoenberg, Pierrot lunaire, "Nacht"**: The opening motive E-G-Eb, when analyzed, yields prime form (014). This trichord pervades the movement.

**Webern, Concerto Op. 24**: The generating trichord is (014)--the same set class as "Nacht," demonstrating how different composers create entirely different works from the same abstract material.

**Stravinsky, Rite of Spring, "Augurs of Spring"**: The famous bitonal chord superimposes two triads. Each is (037); their combination forms a larger set class.

## Related Concepts

- **Prerequisite**: normal-order, transposition-Tn, inversion-In, set-class
- **Leads to**: forte-number, set-class-table, interval-vector
- **See also**: cardinality, z-relation, complement

## Common Confusions

- **Prime form is a label, not a special set**: {0, 1, 4} is no more "fundamental" than {5, 6, 9}--both are equally valid members of set class (014)
- **Parentheses without commas**: Write (014), not [0,1,4] or {014}
- **Compare after transposing to 0**: Both candidates must start on 0 before comparison
- **"Packed left" = smaller intervals early**: Compare leftmost intervals first
- **Inversion may or may not change the result**: Some sets are inversionally symmetrical, so inversion yields the same prime form
- **Different algorithms exist**: Forte's original algorithm occasionally gives different results than the Rahn algorithm presented here
- **T and E notation**: Some texts use T for 10 and E for 11 in prime forms: (01T), (01E)

## Source Reference

Open Music Theory, Part VIII, Chapter 4: "Set Class and Prime Form"
