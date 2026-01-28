---
concept: Hexatonic Scale
category: harmony
source: Open Music Theory
chapter: "Equal Divisions of the Octave"
part: 5
---

# Hexatonic Scale

## Quick Definition

A six-note symmetrical scale built by alternating half steps and minor thirds (semitones and intervals of 3 semitones), creating a collection based on the major-third division of the octave--containing two interlocking augmented triads, plus three major and three minor triads connected by Neo-Riemannian P and L transformations, this scale became important in late Romantic harmony (particularly Liszt and Wagner) and provides the theoretical foundation for understanding "parsimonious" triadic voice leading where all voices move by semitone or stay put.

## Formal Definition

**Hexatonic scale** (hexatonic collection) is a six-note pitch collection with the following properties:

**Structure**:
- **Interval pattern**: 1-3-1-3-1-3 (alternating semitone and minor third)
- **Total**: 3(1) + 3(3) = 12 semitones (one octave)
- **Cardinality**: 6 pitch classes

**Symmetry**:
- **Rotational symmetry**: Pattern repeats every major third
- **Transpositional equivalence**: Only FOUR unique hexatonic collections exist
- **Contains two interlocking augmented triads a semitone apart**

**Naming convention**:
- HEX0,1: Contains semitone C-C# (pitch classes 0 and 1)
- HEX1,2: Contains semitone C#-D (pitch classes 1 and 2)
- HEX2,3: Contains semitone D-Eb (pitch classes 2 and 3)
- HEX3,4: Contains semitone Eb-E (pitch classes 3 and 4)

Also known as: Augmented scale (jazz), 1:3 collection, Mode 3^1 (Messiaen)

## Construction

**Building the hexatonic scale**:
```
Starting on C (HEX0,1):
C - C# - E - F - G# - A - (C)
   1    3   1   3    1   3

Pitch classes: 0, 1, 4, 5, 8, 9
Intervals: semitone, minor 3rd, semitone, minor 3rd, semitone, minor 3rd
```

**The four unique collections**:
```
HEX0,1: C - C# - E - F - G# - A
        Pitch classes: 0, 1, 4, 5, 8, 9

HEX1,2: C# - D - F - F# - A - Bb
        Pitch classes: 1, 2, 5, 6, 9, 10

HEX2,3: D - Eb - F# - G - Bb - B
        Pitch classes: 2, 3, 6, 7, 10, 11

HEX3,4: Eb - E - G - Ab - B - C
        Pitch classes: 3, 4, 7, 8, 11, 0
```

**Two interlocking augmented triads**:
```
HEX0,1 contains:
  - C+ = C - E - G#    (augmented triad)
  - C#+ = C# - F - A   (= Db - F - A, augmented triad a semitone higher)

Two augmented triads a semitone apart combine to form one hexatonic collection
```

## Harmonic Content

**Triadic content**:
```
Within HEX0,1 (C - C# - E - F - G# - A):

Major triads (3):
  - C major  (C - E - G#... wait, G# not G, so NO)
  - Actually: E major (E - G# - B? No B), need to recalculate

Let me correct:
HEX0,1 = C, C#(Db), E, F, G#(Ab), A

Major triads (3):
  - C major: C - E - ? (no G, only G#) NO
  - Actually the triads are:
  
  Ab major: Ab - C - Eb (no Eb) NO
  
Let me reconsider by checking each possible root:

E major: E - G# - B (no B) - NO
F major: F - A - C - YES!
Ab major: Ab - C - Eb (no Eb) - NO  
A major: A - C# - E - YES!
C major: C - E - G (no G, only G#) - NO
Db major: Db - F - Ab - YES!

Major triads: Db (C#), F, A (roots form augmented triad)

Minor triads:
C#m: C# - E - G# - YES!
Fm: F - Ab - C - YES!
Am: A - C - E - YES!

Minor triads: C#m, Fm, Am (roots form same augmented triad!)

Augmented triads (2):
C+ = C - E - G# - YES!
Db+ = Db - F - A - YES!
```

**Correct triadic summary**:
```
Within HEX0,1:

3 Major triads: Db, F, A       (roots: Db+ augmented triad)
3 Minor triads: C#m, Fm, Am    (roots: C#+ augmented triad)
2 Augmented triads: C+, Db+    (the generating triads)

Total: 8 triads (3 maj + 3 min + 2 aug)
```

**Neo-Riemannian structure**:
```
The 6 consonant triads form a PL cycle:

C#m --L--> Db/C# major --P--> C#m... 

Actually:
Db --P--> Dbm? No, Dbm not in collection

The cycle is:
F --L--> Am --P--> A --L--> C#m --P--> Db --L--> Fm --P--> F

This is the hexatonic "PL cycle"
```

## Transpositional Equivalence

**Only four unique forms**:
```
Why four?
- Transposition by M3 (4 semitones) maps collection onto itself
- Period of transposition = 4
- 12 semitones / 4 semitone period = 3 equivalent transposition positions per collection
- But 12 pitch classes / 6 notes per collection gives us 4 distinct collections
```

**Testing collection identity**:
```
Find any half step in the scale:
- C-Db present → HEX0,1
- C#-D present → HEX1,2  
- D-Eb present → HEX2,3
- Eb-E present → HEX3,4
```

## The PL Cycle

**Connection to Neo-Riemannian theory**:
```
The hexatonic scale contains exactly the triads of one PL cycle:

Starting from C major (in HEX3,4):
C --P--> Cm --L--> Ab --P--> Abm --L--> E --P--> Em --L--> C

This cycle:
- Alternates P (parallel) and L (leading-tone exchange) transformations
- Uses only triads from one hexatonic collection
- Returns to start after 6 transformations
- All voice leading is by semitone or common tone
```

**Hexatonic poles**:
```
In the PL cycle, triads opposite each other are "hexatonic poles"
Example in HEX3,4:
  C major and Abm are hexatonic poles
  E major and Cm are hexatonic poles
  Ab major and Em are hexatonic poles

Hexatonic poles:
- Share NO common tones
- Related by the H (hexpole) transformation
- Maximum voice-leading distance within the hexatonic system
- Each voice moves by semitone (in opposite directions)
```

## Musical Characteristics

**Voice leading properties**:
- Maximum smoothness between adjacent triads
- All transformations involve semitone motion
- "Parsimonious" connections throughout

**Tonal implications**:
- Contains major and minor triads (unlike whole-tone)
- But no dominant-tonic relationships (no roots a fifth apart)
- Third relations predominate
- Creates chromatic wandering without functional direction

**Sonic quality**:
- "Uncanny" mixture of consonance and disorientation
- Triadic but non-functional
- Associated with late Romantic chromaticism

## Musical Context

**Late Romantic usage**: Composers like Liszt and Wagner used hexatonic-based progressions for moments of harmonic suspension and chromatic wandering. The smooth voice leading allows continuous triadic motion without functional cadences.

**Neo-Riemannian analysis**: The hexatonic collection provides the foundation for understanding PL cycles in 19th-century chromatic harmony.

**Messiaen**: The hexatonic scale is Messiaen's "Mode 3" (first rotation), one of his modes of limited transposition.

**Jazz**: Known as the "augmented scale" because of its two augmented triads. Used over augmented chord symbols.

## Examples

### Basic

**HEX3,4 spelled out**:
```
Eb - E - G - Ab - B - C - (Eb)
  1   3   1   3    1   3

Contains:
- C+ = C - E - G# (enharmonic: Ab)
- Eb+ = Eb - G - B

Major triads: C, E, Ab (roots = C+)
Minor triads: Cm, Em, Abm (roots = C+)
```

**The PL cycle in HEX3,4**:
```
C major → Cm (P: E moves to Eb)
Cm → Ab major (L: G moves to Ab)  
Ab major → Abm (P: C moves to B)
Abm → E major (L: Eb moves to E)
E major → Em (P: G# moves to G)
Em → C major (L: B moves to C)

Each step: one voice moves by semitone
Total: 6 steps to return to C major
```

**Generating hexatonic from two augmented triads**:
```
Take any augmented triad: C - E - G#
Take another a semitone away: Db - F - A (= C# - F - A)

Combine: C - C# - E - F - G# - A
This is HEX0,1

Any two augmented triads a semitone apart generate a hexatonic collection
```

### From Repertoire

**Liszt, late piano works**: Extensive use of PL cycles and hexatonic progressions, particularly in religious and contemplative pieces.

**Wagner, _Parsifal_**: Hexatonic progressions create the "uncanny" harmonic atmosphere appropriate to the opera's mystical subject.

**Brahms, Concerto for Violin and Cello, I, mm. 270-76**: PL cycle connects two Ab major triads through hexatonic space.

**Richard Strauss, _Also sprach Zarathustra_**: Opening C-G-C progression followed by chromatic drift suggests hexatonic voice leading.

**Laurie Anderson, "O Superman"**: Uses repeated L transformations, cycling through hexatonic-related triads.

## Related Concepts

- **Prerequisite**: augmented-triad, semitone, minor-third, major-third
- **Leads to**: neo-riemannian-theory, PL-cycle, modes-of-limited-transposition, parsimonious-voice-leading
- **See also**: equal-divisions-of-the-octave, octatonic-scale, whole-tone-scale, symmetrical-collection

## Common Confusions

- "Hexatonic" means 6 notes (hexa = 6)--same as whole-tone, but DIFFERENT interval pattern
- Only FOUR unique hexatonic collections exist (not 6 or 12)
- Interval pattern is 1-3-1-3-1-3 (half step, minor third), NOT all whole steps
- Contains two augmented triads (unlike whole-tone's single endless augmented sound)
- Also contains 3 major and 3 minor triads (whole-tone has NONE)
- The PL cycle triads ARE the hexatonic collection (they're the same thing viewed differently)
- "Augmented scale" (jazz term) = hexatonic scale
- NOT the same as "blues scale" which is also sometimes called hexatonic
- Hexatonic poles share NO common tones (maximum distance within system)
- Transposing by major third gives the SAME collection

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Equal Divisions of the Octave"
Open Music Theory, Part V: "Neo-Riemannian Triadic Progressions"
Open Music Theory, Part VIII: "Collections"
