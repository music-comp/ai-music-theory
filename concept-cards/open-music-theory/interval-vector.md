---
concept: Interval Vector
category: theory
source: Open Music Theory
chapter: "Set Class and Prime Form"
pdf_page: null
chapter_number: 8
unit: null
authors: "Open Music Theory contributors"
---

# Interval Vector

## Quick Definition

The interval vector (or interval-class vector) is a summary of all interval classes present in a pitch-class set, written as six numbers in angle brackets. Each position counts how many times a specific interval class (1 through 6) appears between pairs of pitch classes in the set. All members of a set class share the same interval vector.

## Formal Definition

**Interval vector (ic vector)**: A six-element array showing the multiplicity of each interval class in a pitch-class set.

**Format**: <ic1, ic2, ic3, ic4, ic5, ic6>

Where each position counts:
- ic1: number of semitones (minor 2nds/major 7ths)
- ic2: number of whole tones (major 2nds/minor 7ths)
- ic3: number of minor 3rds/major 6ths
- ic4: number of major 3rds/minor 6ths
- ic5: number of perfect 4ths/5ths
- ic6: number of tritones

**Calculation**: For a set of cardinality n:
- Total intervals = n(n-1)/2 (each pair counted once)
- Determine the ic between each pair of pitch classes
- Tally the count for each ic

**Property**: All members of a set class share the same interval vector. This is what makes set-class membership aurally meaningful--shared interval content produces similar sonic "color."

## Musical Context

The interval vector reveals the **sonic fingerprint** of a set class:

- **<101100>** for (013): one ic1, one ic3, one ic4--lean, with no tritones
- **<000300>** for (048): three ic4s only--the augmented triad's unique sound
- **<001110>** for (037): one each of ic3, ic4, ic5--the familiar major/minor triad

Composers may choose set classes based on their interval vectors:
- Favoring certain intervals (Webern loved ic1 and ic4)
- Avoiding tritones (or embracing them)
- Creating contrast between sections using sets with different interval profiles

The interval vector is also crucial for identifying the **Z-relation**: two different set classes with identical interval vectors.

## Examples

### Basic

**Calculating interval vector for {0, 1, 4}**:
```
Pairs and their interval classes:
  0 and 1: ic = 1
  0 and 4: ic = 4
  1 and 4: ic = 3

Tally:
  ic1: 1
  ic2: 0
  ic3: 1
  ic4: 1
  ic5: 0
  ic6: 0

Interval vector: <1,0,1,1,0,0>
```

**Major triad {0, 4, 7}**:
```
Pairs:
  0 and 4: ic = 4 (major 3rd)
  0 and 7: ic = 5 (perfect 5th)
  4 and 7: ic = 3 (minor 3rd)

Interval vector: <0,0,1,1,1,0>

(Same for all major triads and all minor triads!)
```

**Whole-tone collection {0, 2, 4, 6, 8, 10}**:
```
15 pairs total (6 choose 2)
All pairs are ic2, ic4, or ic6

Interval vector: <0,6,0,6,0,3>

(No semitones, no minor 3rds, no perfect 5ths)
```

**Comparison of trichord interval vectors**:
```
(012) <2,1,0,0,0,0> - chromatic cluster
(013) <1,1,1,0,0,0> - balanced
(014) <1,0,1,1,0,0> - "Viennese" trichord
(015) <1,0,0,1,1,0> - spans P5
(016) <1,0,0,0,1,1> - semitone + tritone
(024) <0,2,0,1,0,0> - whole-tone
(025) <0,1,1,0,1,0> - suspended quality
(027) <0,1,0,0,2,0> - quartal
(036) <0,0,2,0,0,1> - diminished
(037) <0,0,1,1,1,0> - major/minor triad
(048) <0,0,0,3,0,0> - augmented
```

### From Repertoire

**Webern, Concerto Op. 24**: The generating trichord (014) has interval vector <1,0,1,1,0,0>--emphasizing ic1 and ic4. This gives the work its characteristic sound: pointillistic and "crystalline."

**Bartok, Music for Strings, Percussion, and Celesta**: The fugue subject emphasizes sets with high ic1 content, creating the chromatic intensity associated with this work.

**Messiaen**: His modes of limited transposition (including the octatonic) have distinctive interval vectors. The octatonic <4,4,8,4,4,2> is rich in every interval class, explaining its harmonic versatility.

## Related Concepts

- **Prerequisite**: interval-class, pitch-class-set, set-class
- **Leads to**: z-relation, rondo-character
- **See also**: prime-form, forte-number, cardinality

## Common Confusions

- **Vector positions 1-6 correspond to interval classes 1-6**: Position 1 = ic1 (semitone), not ic0
- **Total entries sum to n(n-1)/2**: Where n = cardinality of the set
- **Major and minor triads have the same vector**: They're in the same set class
- **Angle brackets for interval vector**: Write <0,0,1,1,1,0>, not (001110) or [001110]
- **Z-related sets have identical vectors**: Different prime forms can share a vector (see Z-relation)
- **Vector doesn't specify interval arrangement**: Two sets with <1,0,1,1,0,0> may place those intervals differently
- **ic0 is not included**: The vector only counts ics 1-6

## Source Reference

Open Music Theory, Part VIII, Chapter 4: "Set Class and Prime Form"
