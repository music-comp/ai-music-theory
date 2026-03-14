---
concept: German Sixth
category: theory
source: Open Music Theory
chapter: "Augmented Sixth Chords"
pdf_page: null
chapter_number: 5
unit: null
authors: "Open Music Theory contributors"
---

# German Sixth

## Quick Definition
The German sixth (Ger+6) is an augmented sixth chord containing four notes (^b6, ^1, ^b3, and ^#4), which sounds identical to a dominant seventh chord but functions as a chromatic predominant, requiring resolution through the cadential I64 to avoid parallel fifths between ^b6-^b3 and ^5-^2.

## Formal Definition
**German sixth** (Ger+6) is an augmented sixth chord with:

**Structure**: Four notes
- ^b6 (lowered submediant) in bass
- ^1 (tonic) in middle voice
- ^b3 (lowered mediant) in middle voice
- ^#4 (raised subdominant) in upper voice

**Interval content**:
- Augmented sixth (A6) between ^b6 and ^#4
- Major third (M3) between ^b6 and ^1
- Minor third (m3) between ^1 and ^b3
- Augmented second (A2) between ^b3 and ^#4

**Enharmonic equivalence**: Sounds like Mm7 (dominant seventh)

**Function**: Chromatic predominant resolving via I64 to V

## The Parallel Fifths Problem

**Why German sixth requires I64**:
```
Ger+6: Ab-C-Eb-F#  ->  V: G-B-D-G
       |        |       |     |
       Ab------Eb      G-----D
       (P5)           (P5)

PARALLEL FIFTHS! Ab-Eb (P5) moves to G-D (P5)
This violates voice leading principles
```

**Solution - interpolate I64**:
```
Ger+6: Ab-C-Eb-F#  ->  I64: G-C-E-G  ->  V: G-B-D-G

Voice leading through I64:
Ab -> G  (^b6 -> ^5, down half step)
C  -> C  (^1 -> ^1, common tone)
Eb -> E  (^b3 -> ^3, up half step to avoid parallels!)
F# -> G  (^#4 -> ^5, up half step)

Then I64 resolves normally to V:
G  -> G  (^5 stays)
C  -> B  (^1 -> ^7)
E  -> D  (^3 -> ^2)
G  -> G  (^5 stays)
```

**Key insight**: ^b3 moves to ^3 in I64, then ^3 to ^2 in V

## Enharmonic Respelling as Dominant Seventh

**German sixth sounds like V7**:
```
Ger+6 in C: Ab-C-Eb-F#

Respell enharmonically:
Ab = G#, F# = Gb (enharmonic swap)
G#-C-Eb-Gb? No, that's not helpful

Better: Think of it as V7 of Db:
Ab-C-Eb-Gb = Ab7 = V7 of Db

Ger+6 in C sounds exactly like Ab7
But FUNCTIONS differently (predominant, not dominant)
```

**Enharmonic reinterpretation for modulation**:
```
Ger+6 in C (Ab-C-Eb-F#) = V7 of Db (Ab-C-Eb-Gb)

Can resolve:
1. As Ger+6: to I64 in C, then V-I in C
2. As V7: to Db major (modulation!)

This enables chromatic modulation
```

## Voice Leading

**Standard resolution through I64**:
```
Ger+6:    Ab-C-Eb-F#
          |  |   |   |
          v  v   v   v
I64:      G -C -E -G
          |  |   |   |
          v  v   v   v  
V:        G -B -D -G

Complete voice leading:
^b6 -> ^5 -> ^5 (Ab -> G -> G)
^1  -> ^1 -> ^7 (C -> C -> B)
^b3 -> ^3 -> ^2 (Eb -> E -> D) *critical for avoiding parallels*
^#4 -> ^5 -> ^5 (F# -> G -> G)
```

## Spelling in Various Keys

**In C major/minor**: Ab-C-Eb-F#
**In G major/minor**: Eb-G-Bb-C#
**In D major/minor**: Bb-D-F-G#
**In A major/minor**: F-A-C-D#
**In E major/minor**: C-E-G-A#
**In F major/minor**: Db-F-Ab-B

**In minor keys**: Only ^#4 is chromatic (^b3 and ^b6 are diatonic)
**In major keys**: ^b6, ^b3, and ^#4 all require accidentals

## Comparison with Other Augmented Sixths

| Chord | Notes | Distinctive Tone | Resolution Path |
|-------|-------|------------------|-----------------|
| It+6 | Ab-C-F# | None (doubled ^1) | Direct to V |
| Fr+6 | Ab-C-D-F# | ^2 (common tone) | Direct to V |
| **Ger+6** | Ab-C-Eb-F# | ^b3 (creates parallels) | Via I64 to V |

**German is richest**: Sounds like Mm7, requires I64

## Musical Context

The German sixth serves specific purposes:
- **Most common augmented sixth**: Preferred in Classical/Romantic music
- **Richest sonority**: Full four-note chord, Mm7 sound
- **Requires I64**: Avoids parallel fifths, creates two-chord anacrusis
- **Enharmonic potential**: Enables chromatic modulation
- **Minor key natural**: ^b3 and ^b6 diatonic in minor
- **Major key chromatic**: Requires three accidentals in major
- **Dramatic cadence**: Ger+6 -> I64 -> V -> I is powerful formula
- **Romantic favorite**: Wagner, Chopin, Brahms use extensively

## Examples

### Basic

**German sixth in C minor**:
```
Ger+6:  Ab-C-Eb-F#   I64: G-C-E-G   V: G-B-D-G
        |  |  |   |       |  |  |  |    |  |  |  |
Bass:   Ab    ->     G         ->   G
Alto:   C     ->     C         ->   B
Tenor:  Eb    ->     E(!)      ->   D
Sop:    F#    ->     G         ->   G

Eb -> E avoids parallel fifths
I64 interpolation essential
```

**Complete cadential progression**:
```
i  -  iv  -  Ger+6  -  I64  -  V7  -  i
Cm    Fm     Ab/C/Eb/F#  Cm/G    G7    Cm

Most common use of German sixth
Dramatic chromatic approach to cadence
```

### From Repertoire

**Beethoven, "Pathetique" Sonata, Op. 13**: German sixth in C minor, characteristic cadential usage.

**Mozart, Symphony No. 40, G minor**: German sixths at structural cadences, Classical style.

**Chopin, Ballade No. 1, G minor**: German sixths for expressive chromaticism.

**Wagner, Tristan und Isolde**: German sixths in extended chromatic progressions.

**Brahms, Symphony No. 1**: German sixth chords at climactic moments.

## Related Concepts

- **Prerequisite**: augmented-sixth-chords, italian-sixth, french-sixth, cadential-six-four
- **Leads to**: german-diminished-third, chromatic-modulation
- **See also**: dominant-seventh-chord, parallel-fifths, dominant-prolongation

## Common Confusions

- German sixth = four-note augmented sixth with ^b3 (Ger+6)
- Spelling: ^b6-^1-^b3-^#4 (in bass to soprano)
- In C: Ab-C-Eb-F# (includes lowered mediant Eb)
- **Sounds like V7**: Enharmonically equivalent to Mm7
- **Parallel fifths problem**: ^b6-^b3 (P5) would move to ^5-^2 (P5)
- **Requires I64**: Interpolate cadential 6/4 to avoid parallels
- ^b3 -> ^3 in I64 (Eb -> E), breaking the parallel motion
- Most common augmented sixth type in Classical/Romantic
- More common in minor (^b6 and ^b3 already diatonic)
- Can be enharmonically reinterpreted for modulation
- Ger+6 in C = V7 of Db (Ab-C-Eb-Gb)
- Named for supposed German origin (historical simplification)
- Function: Chromatic predominant (PD -> D)
- No root (unlike traditional chords)
- Richest sonority of three augmented sixth types

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Augmented Sixth Chords"
