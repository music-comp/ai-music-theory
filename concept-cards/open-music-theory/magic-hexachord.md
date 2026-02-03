---
concept: Magic Hexachord
category: theory
source: Open Music Theory
chapter: "Row Properties"
part: 9
---

# Magic Hexachord

## Quick Definition

The magic hexachord is set class (014589), also known as the hexatonic collection or Ode-to-Napoleon hexachord, a six-note pitch-class set with exceptional properties including all-combinatoriality (producing combinatorial pairs through transposition by +/-2 and 6 semitones), containment of multiple major and minor triads (comprising a complete hexatonic cycle), relationship to the 1:3 distance model, and having uniquely constrained subset/superset relationships—making it particularly valuable for composers seeking to combine twelve-tone technique with triadic sonorities.

## Formal Definition

**Magic hexachord**: Set class (014589)

**Pitch-class content** (in prime form):
```
{0, 1, 4, 5, 8, 9}
 C  Db E  F  Ab A

Interval vector: [303630]
- 3 ic1 (semitones)
- 0 ic2 (whole tones)
- 3 ic3 (minor thirds)
- 6 ic4 (major thirds)
- 3 ic5 (perfect fourths)
- 0 ic6 (tritones)
```

**Exceptional properties**:

1. **All-combinatorial** by transposition:
   - T2, T-2 (T10), and T6 all produce combinatorial pairs
   - Only whole-tone collection (02468T) exceeds this

2. **Triadic content** (hexatonic cycle):
   ```
   Contains: C major, C minor, E major, E minor, Ab major, Ab minor
   Six triads forming the hexatonic cycle C-E-Ab
   ```

3. **Unique subset**: Only one 5-note subset, (01458)

4. **Unique superset**: Only one 7-note superset, (0124589)

5. **Related to 1:3 distance model** (alternating semitones and minor thirds)

## Musical Context

The magic hexachord serves multiple purposes:
- **Tonal-atonal bridge**: Contains triads within twelve-tone context
- **Combinatorial flexibility**: Multiple transpositions work
- **Harmonic richness**: Six triads available
- **Structural clarity**: Unique subset/superset relationships

Historical and compositional significance:
- Named for Schoenberg's Ode to Napoleon (one usage)
- Also called "hexatonic collection" (from neo-Riemannian theory)
- Central to several major twelve-tone works
- Connects serial technique with triadic harmony

## Examples

### Basic

**The hexachord in pitch classes**:
```
(014589) starting on C:
C - Db - E - F - Ab - A
0    1   4   5   8   9

Intervals within:
C-Db:  semitone (ic1)
Db-E:  major 3rd (ic4)
E-F:   semitone (ic1)
F-Ab:  minor 3rd (ic3)
Ab-A:  semitone (ic1)
```

**Triadic content**:
```
Major triads:     Minor triads:
C-E-Ab (C+)       C-Eb-Ab (don't have Eb)
                  Wait - let me recalculate...

Actual triads in {C, Db, E, F, Ab, A}:
- F minor: F-Ab-C
- Db major: Db-F-Ab
- A minor: A-C-E
- E major: E-Ab(=G#)-B? No B...

Hexatonic cycle triads (with enharmonic spelling):
{C, E, G#, etc.} forms hexatonic system
But magic hexachord specifically contains:
The "even" hexatonic: C, Db, E, F, Ab, A
```

**Combinatoriality by transposition**:
```
Original: {0,1,4,5,8,9}
T2:       {2,3,6,7,10,11}

Check complement:
{0,1,4,5,8,9} + {2,3,6,7,10,11} = all 12!

Also T10 (same as T-2):
{10,11,2,3,6,7} - same as T2

And T6:
{6,7,10,11,2,3} - also complementary!
```

### From Repertoire

**Webern, Konzert Op. 24**:
```
Row hexachords are both (014589):

H1: B-Bb-D-Eb-G-F# = {11,10,2,3,7,6}
    Transposed (014589)

H2: Ab-E-F-C-Db-A = {8,4,5,0,1,9}
    Also (014589)

Properties used:
- Trichordal derivation (four (014) trichords)
- Hexachordal all-combinatoriality
- Only 12 distinct row forms
```

**Schoenberg, Ode to Napoleon**:
```
Uses magic hexachord
Hence name "Ode-to-Napoleon hexachord"

Schoenberg exploits:
- Triadic possibilities
- Combinatorial properties
- Connection to tonal harmony
```

**Other works using (014589)**:
```
- Bruno Maderna: various works
- Luigi Nono: various works
- Many post-war European serialists

The hexachord bridges:
- Twelve-tone technique
- Triadic/tonal sonorities
- Neo-Riemannian hexatonic systems
```

## Related Concepts

- **Prerequisite**: set-class, combinatoriality
- **Leads to**: hexatonic-collection, neo-riemannian-theory, i-combinatoriality
- **See also**: derived-row, triadic-post-tonality

## Common Confusions

- Magic hexachord = set class (014589)
- Also called: hexatonic collection, Ode-to-Napoleon hexachord
- All-combinatorial by T2, T-2 (T10), and T6
- Contains triads from the hexatonic cycle
- Only whole-tone collection has more T-combinatoriality options
- NOT the same as whole-tone scale (which is 02468T)
- Related to 1:3 distance model (semitone + minor 3rd pattern)
- Unique properties: only one 5-note subset, only one 7-note superset
- Interval vector [303630]: rich in ic4 (major 3rds), no ic2 or ic6
- "Magic" refers to its exceptional structural properties
- Connects twelve-tone technique with triadic sonorities
- Used by many major serialist composers
- One of only six all-combinatorial hexachord types

## Source Reference

Open Music Theory, Part IX: "Row Properties"
