---
concept: Augmented Sixth Voice Leading
category: harmony
source: Open Music Theory
chapter: "Augmented Sixth Chords"
part: 5
---

# Augmented Sixth Voice Leading

## Quick Definition
Augmented sixth voice leading is characterized by the "wedge" or "pincer" motion in which the augmented sixth interval expands outward to a perfect octave on the dominant, with ^b6 descending by half step to ^5 and ^#4 ascending by half step to ^5, creating dramatic chromatic contrary motion at the cadence.

## Formal Definition
**Augmented sixth voice leading** features:

**Defining motion**: A6 interval expands to P8
- ^b6 (bass) descends by half step to ^5
- ^#4 (upper voice) ascends by half step to ^5
- Result: Both arrive at ^5 (octave)

**Contrary motion**: Outer voices move in opposite directions

**Chromatic approach**: Half-step motion from both directions

**Resolution target**: Dominant (V) or cadential I64 then V

## The Expanding Interval

**Augmented sixth to perfect octave**:
```
Starting interval:
Ab --------- F#
     A6 (10 half steps)

Resolution:
G --------- G
     P8 (12 half steps)

Ab -> G (down half step)
F# -> G (up half step)
A6 expands outward to P8
```

**Interval mathematics**:
```
A6 = 10 half steps (8 + 2 = minor 6th + 2)
P8 = 12 half steps

A6 + 2 half steps (1 from each voice) = P8
```

**The "wedge" motion**:
```
    F# (^#4)
     ↑
     |  ascending
     |
   [G] <- convergence point (^5)
     |
     |  descending
     ↓
    Ab (^b6)
    
Both voices converge on G (^5) from opposite directions
```

## Voice Leading by Type

**Italian sixth (It+6)**:
```
It+6:  Ab - C - F#    V:  G - B/D - G
       ^b6  ^1  ^#4        ^5  ^7/^2  ^5

Ab -> G   (^b6 -> ^5, down half step)
C  -> B   (^1 -> ^7, down half step)
     or D (^1 -> ^2, up whole step, if doubled)
F# -> G   (^#4 -> ^5, up half step)

Outer voices: wedge motion
Middle voice: descends to leading tone
```

**French sixth (Fr+6)**:
```
Fr+6: Ab - C - D - F#   V:  G - B - D - G
      ^b6  ^1  ^2  ^#4        ^5  ^7  ^2  ^5

Ab -> G   (^b6 -> ^5, down half step)
C  -> B   (^1 -> ^7, down half step)
D  -> D   (^2 -> ^2, COMMON TONE)
F# -> G   (^#4 -> ^5, up half step)

Outer voices: wedge motion
^2 is common tone with V (smooth!)
```

**German sixth (Ger+6)** via I64:
```
Ger+6: Ab - C - Eb - F#   I64: G - C - E - G   V: G - B - D - G
       ^b6  ^1  ^b3  ^#4         ^5  ^1  ^3  ^5      ^5  ^7  ^2  ^5

Ab -> G    (^b6 -> ^5)
C  -> C    (^1 stays in I64)   then C -> B (to V)
Eb -> E    (^b3 -> ^3, UP!)    then E -> D (to V)
F# -> G    (^#4 -> ^5)

Key: Eb -> E avoids parallel fifths!
```

## The German Sixth Problem

**Parallel fifths without I64**:
```
Ger+6: Ab - Eb   V: G - D
       P5           P5

Ab and Eb form perfect fifth
G and D form perfect fifth
Moving directly = PARALLEL FIFTHS (forbidden)
```

**Solution with I64**:
```
Ger+6:    Ab - Eb    I64: G - E     V: G - D
          |    |           |   |        |   |
          ↓    ↑           |   |        |   |
          G    E (!)       G   E        G   D
                
Eb moves UP to E (not down to D)
Breaks parallel motion
I64 provides landing spot for ^b3 -> ^3
```

**Alternative: ^b3 leaps**:
```
Some composers have ^b3 leap to ^7:
Ger+6: Ab - C - Eb - F#   V: G - B - B - G
                 ↓             ↑
                 Eb ----------→ B (leap of d5)

Less common, but avoids I64
```

## Inner Voice Motion

**^1 in all three types**:
```
^1 -> ^7 (most common)
C -> B in C major/minor

Descends by half step to leading tone
Consistent across all three types
```

**Fourth voice varies**:
```
Italian: ^1 doubled, one moves to ^7, one to ^2
French: ^2 stays as common tone with V
German: ^b3 moves to ^3 (via I64), then ^2
```

## Spacing and Voicing

**Typical voicing**:
```
Soprano: ^#4 (F#) - resolves up to ^5
Alto:    varies by type (^1, ^2, or ^b3)
Tenor:   ^1 (C) - resolves down to ^7
Bass:    ^b6 (Ab) - resolves down to ^5

Outer voices carry the wedge motion
Inner voices fill out harmony
```

**Inversion possibilities**:
```
Standard: ^b6 in bass (A6 interval above)
Inverted: ^#4 in bass (d3 interval above = German °3)

When inverted:
d3 contracts to P1 (unison) instead of expanding
Still resolves to ^5, but from opposite direction
```

## Musical Context

Augmented sixth voice leading is significant because:
- **Dramatic resolution**: Maximum chromatic tension release
- **Contrary motion**: Creates "wedge" effect
- **Half-step approach**: Smoothest possible voice leading to ^5
- **Predominant intensity**: Most chromatic approach to V
- **Type-specific variations**: Each type has unique inner-voice behavior
- **German sixth problem**: Parallel fifths require I64 solution
- **Cadential power**: Strong preparation for dominant

## Examples

### Basic

**Italian sixth voice leading**:
```
It+6:     Ab - C - C - F#
          ↓    ↓   ↑   ↑
V:        G  - B - D - G

^b6 -> ^5 (Ab -> G, down)
^1  -> ^7 (C -> B, down)
^1  -> ^2 (C -> D, up) [doubled ^1]
^#4 -> ^5 (F# -> G, up)
```

**French sixth voice leading**:
```
Fr+6:     Ab - C - D - F#
          ↓    ↓   |   ↑
V:        G  - B - D - G

^b6 -> ^5 (Ab -> G, down)
^1  -> ^7 (C -> B, down)
^2  -> ^2 (D -> D, common tone)
^#4 -> ^5 (F# -> G, up)
```

**German sixth voice leading (with I64)**:
```
Ger+6:    Ab - C - Eb - F#
          ↓    |   ↑    ↑
I64:      G  - C - E  - G
          |    ↓   ↓    |
V:        G  - B - D  - G

Stage 1: Ger+6 -> I64
Ab -> G, C -> C, Eb -> E(!), F# -> G

Stage 2: I64 -> V
G -> G, C -> B, E -> D, G -> G
```

### From Repertoire

**Mozart, Don Giovanni, "Catalogue Aria"**: Clear augmented sixth voice leading, textbook wedge motion.

**Beethoven, Symphony No. 5, II**: German sixth with I64, proper parallel-fifth avoidance.

**Chopin, Ballade No. 1**: Various augmented sixth resolutions, Romantic voice leading.

**Brahms, Piano Concerto No. 2**: Extended augmented sixth passages, rich chromatic voice leading.

## Related Concepts

- **Prerequisite**: augmented-sixth-chords, voice-leading, contrary-motion
- **Leads to**: chromatic-voice-leading, cadential-intensification
- **See also**: parallel-fifth-avoidance, cadential-six-four, predominant-to-dominant

## Common Confusions

- Augmented sixth voice leading = A6 expands to P8 ("wedge" motion)
- ^b6 descends by half step to ^5
- ^#4 ascends by half step to ^5
- Both converge on ^5 (dominant root)
- **Italian**: Doubled ^1 splits to ^7 and ^2
- **French**: ^2 is common tone (smoothest voice leading)
- **German**: ^b3 -> ^3 via I64, then ^3 -> ^2 (avoids parallel fifths)
- German without I64 creates parallel fifths (^b6-^b3 -> ^5-^2)
- Eb -> E in I64 breaks the parallel motion
- Inner voice ^1 -> ^7 in all three types
- Can be inverted (^#4 in bass = German °3)
- Inverted: d3 contracts to P1 (opposite of A6 expanding)
- Maximum chromatic intensity before dominant

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Augmented Sixth Chords"
