---
concept: Recognizing Augmented Sixth Chords
category: theory
source: Open Music Theory
chapter: "Augmented Sixth Chords"
pdf_page: null
chapter_number: 5
unit: null
authors: "Open Music Theory contributors"
---

# Recognizing Augmented Sixth Chords

## Quick Definition
Augmented sixth chords are recognized not by stacking thirds (which would reveal a confusing diminished third) but by identifying their characteristic context: a chromatic chord preceding V with ^b6 in the bass and ^#4 in an upper voice, forming an augmented sixth interval that expands outward to an octave on ^5.

## Formal Definition
**Recognition strategy** for augmented sixth chords:

**Do not use**: Traditional third-stacking (creates diminished third)

**Do use**: Context-based identification:
1. Look for chord preceding V (predominant position)
2. Find ^b6 in bass (lowered sixth scale degree)
3. Find ^#4 in upper voice (raised fourth scale degree)
4. Verify A6 interval between them
5. Identify type by remaining tones

**Why third-stacking fails**: The A6 interval inverted becomes d3

## The Diminished Third Problem

**Attempting to stack in thirds**:
```
Ger+6 in C: Ab-C-Eb-F#

Try stacking from F#:
F# - Ab = diminished third (d3)! Not a normal interval.

From Ab:
Ab - C = M3 (OK)
C - Eb = m3 (OK)
Eb - F# = augmented second (A2)! Also unusual.

No matter how you stack, unusual intervals appear.
```

**The chord has no root**:
```
Unlike triads and seventh chords:
- Major triad: M3 + m3 (stacks neatly)
- Minor triad: m3 + M3 (stacks neatly)
- Dominant 7th: M3 + m3 + m3 (stacks neatly)

Augmented sixth: Contains A6/d3 interval
Does not stack in traditional thirds
Has no conventional root
```

## Contextual Recognition Strategy

**Step 1: Find the harmonic context**:
```
Look for:
- Chord immediately before V
- Part of half cadence or authentic cadence
- Predominant function position

If chord precedes V, consider augmented sixth
```

**Step 2: Identify ^b6 in bass**:
```
In C major/minor: Look for Ab in bass
In G major/minor: Look for Eb in bass
In D major/minor: Look for Bb in bass

^b6 = lowered sixth scale degree
Usually the lowest sounding note
```

**Step 3: Find ^#4 in upper voices**:
```
In C major/minor: Look for F# above the bass
In G major/minor: Look for C# above the bass
In D major/minor: Look for G# above the bass

^#4 = raised fourth scale degree
Creates A6 interval with ^b6 bass
```

**Step 4: Verify A6 interval**:
```
Count half steps from ^b6 to ^#4:
Ab to F# = 10 half steps = A6 (augmented sixth)

Or check: m6 (8 half steps) + 2 = A6

The defining interval of all augmented sixths
```

**Step 5: Identify the type**:
```
After finding ^b6-^#4 (A6 interval), check middle voices:

Only ^1 present:              Italian sixth (It+6)
^1 and ^2 present:            French sixth (Fr+6)
^1 and ^b3 present:           German sixth (Ger+6)

In C: 
It+6:  Ab-C-F#       (^b6-^1-^#4)
Fr+6:  Ab-C-D-F#     (^b6-^1-^2-^#4)
Ger+6: Ab-C-Eb-F#    (^b6-^1-^b3-^#4)
```

## Quick Identification Table

| Look for | In C major/minor | Scale degrees |
|----------|-----------------|---------------|
| Bass note | Ab | ^b6 |
| Upper voice | F# | ^#4 |
| A6 interval | Ab to F# | ^b6 to ^#4 |
| Italian adds | C | ^1 |
| French adds | C, D | ^1, ^2 |
| German adds | C, Eb | ^1, ^b3 |

## Resolution Confirms Analysis

**Checking resolution**:
```
If your analysis is correct, the chord should resolve:

^b6 -> ^5 (down by half step)
^#4 -> ^5 (up by half step)
Other voices -> V chord tones

If Italian or French: Direct to V
If German: Through I64 to V
```

**Example verification**:
```
You see: Ab-C-D-F# followed by G-B-D-G

Check: Ab -> G (down, check)
       D  -> D (common tone, check)
       F# -> G (up, check)
       C  -> B (down, check)

Confirms: Fr+6 -> V
```

## Common Contexts to Check

**Where to look for augmented sixths**:
```
1. Before half cadence (... A6 - V)
2. Before authentic cadence (... A6 - I64 - V - I)
3. After predominant chords (iv - A6 - V)
4. In sequences with chromatic bass
5. At phrase endings
6. In development sections
```

**Not augmented sixth if**:
```
- No ^b6 in bass
- No ^#4 in chord
- Doesn't precede V (or I64-V)
- Part of different function (passing, etc.)
```

## Musical Context

Recognizing augmented sixth chords requires:
- **Contextual awareness**: Function before V
- **Interval identification**: Find the A6
- **Scale degree thinking**: ^b6 and ^#4 are key
- **Avoid third-stacking**: Will confuse analysis
- **Resolution verification**: Confirms identification
- **Type differentiation**: Check middle voices

## Examples

### Basic

**Step-by-step analysis**:
```
Given chord: Ab-C-Eb-F# in C minor, before G-B-D

Step 1: Context? Precedes G major (V in C minor) - YES
Step 2: ^b6 in bass? Ab = ^b6 in C - YES
Step 3: ^#4 in upper voice? F# = ^#4 in C - YES
Step 4: A6 interval? Ab to F# = 10 half steps = A6 - YES
Step 5: Middle voices? C (^1) and Eb (^b3) - GERMAN

Conclusion: Ger+6 in C minor
```

**Another example**:
```
Given chord: F-A-B-D# in E minor, before E-B-E

Step 1: Context? Precedes E major - wait, that's I not V!
        Actually look at bass: F in E minor = ^b2? 
        Not ^b6, so NOT standard augmented sixth
        
Possible: Neapolitan context, or other chord
```

**Correct identification**:
```
Given chord: C-E-F#-A# in E minor, before B-D#-F#-B

Step 1: Context? Precedes B major (V in E minor) - YES
Step 2: ^b6 in bass? C = ^b6 in E - YES
Step 3: ^#4 in upper voice? A# = ^#4 in E - YES
Step 4: A6 interval? C to A# = 10 half steps = A6 - YES
Step 5: Middle voices? E (^1) and F# (^2) - FRENCH

Conclusion: Fr+6 in E minor
```

### From Repertoire

Analysis practice in:
**Mozart, Symphony No. 40**: Identify augmented sixths by context and interval, not third-stacking.
**Beethoven, Piano Sonatas**: Locate ^b6 bass notes preceding V.
**Chopin, Nocturnes**: Find ^#4 creating A6 with ^b6 bass.

## Related Concepts

- **Prerequisite**: augmented-sixth-chords
- **Leads to**: italian-sixth, french-sixth, german-sixth
- **See also**: pre-dominant-function

## Common Confusions

- **Do not stack in thirds**: Augmented sixths have no root
- Stacking creates confusing intervals (d3, A2)
- **Use context instead**: Look for chord before V
- Find ^b6 in bass (lowered sixth scale degree)
- Find ^#4 in upper voice (raised fourth scale degree)
- Verify A6 interval between them (10 half steps)
- Identify type by middle voices: ^1 only = Italian, add ^2 = French, add ^b3 = German
- Resolution confirms analysis (A6 expands to P8 on ^5)
- ^b6 -> ^5 (down half step)
- ^#4 -> ^5 (up half step)
- Common contexts: before half cadence, before authentic cadence
- German sixth usually has I64 before V
- Italian and French resolve directly to V
- Not all chords with Ab-F# are augmented sixths (check function)

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Augmented Sixth Chords - Recognizing augmented sixth chords when analyzing"
