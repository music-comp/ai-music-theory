---
concept: Weitzmann Region
category: theory
source: Open Music Theory
chapter: "Neo-Riemannian Triadic Progressions"
pdf_page: null
chapter_number: 5
unit: null
authors: "Open Music Theory contributors"
---

# Weitzmann Region

## Quick Definition

A network of six triads (three major, three minor) that all connect to a central augmented triad by moving a single voice by half step, named after theorist Carl Friedrich Weitzmann who wrote extensively about the augmented triad's versatility in 19th-century treatises; since only four distinct augmented triads exist due to the chord's symmetry, there are exactly four Weitzmann regions that together encompass all 24 major and minor triads, with each region providing pathways for R, S, and N transformations through the augmented triad intermediary.

## Formal Definition

**Weitzmann region** structure:

**Central augmented triad**:
- Symmetrical chord (only 4 distinct ones exist)
- Each note can serve as root (enharmonic respelling)
- Example: C+ = E+ = G#+ (same pitches, different spellings)

**Connected triads** (6 per region):
- **Three major triads**: Each a half step away from augmented triad (one voice moves)
- **Three minor triads**: Each a half step away from augmented triad (one voice moves)

**The four Weitzmann regions**:
```
Region 1 (C+ = E+ = Ab+):
Major: C, E, Ab
Minor: Cm, Em, Abm

Region 2 (Db+ = F+ = A+):
Major: Db, F, A
Minor: Dbm, Fm, Am

Region 3 (D+ = Gb+ = Bb+):
Major: D, Gb, Bb
Minor: Dm, Gbm, Bbm

Region 4 (Eb+ = G+ = B+):
Major: Eb, G, B
Minor: Ebm, Gm, Bm
```

## Augmented Triad Connections

**How major triads connect to augmented**:
```
C major (C-E-G) → C+ (C-E-G#)
Move: G → G# (fifth up by half step)
One voice, one half step

E major (E-G#-B) → C+/E+ (E-G#-C)
Move: B → C (fifth up by half step)
One voice, one half step

Ab major (Ab-C-Eb) → Ab+/C+ (Ab-C-E)
Move: Eb → E (fifth up by half step)
One voice, one half step
```

**How minor triads connect to augmented**:
```
A minor (A-C-E) → C+/E+/Ab+ (respelled as needed)
A-C-E: Move A down to Ab = Ab-C-E = C+ (respelled Ab+)
One voice, one half step

C minor (C-Eb-G) → C+
Move: Eb → E (third up by half step)
C-E-G... wait, that's C major, not C+

Let me reconsider:
C minor: C-Eb-G
C+: C-E-G#
To get from Cm to C+: Eb→E AND G→G#? That's two moves.

Actually, the connection is:
C minor (C-Eb-G) connects to Eb+ (Eb-G-B)
Move: C down to B = B-Eb-G = Eb+ (respelled)
One voice, one half step

So minor triads connect to a DIFFERENT augmented triad than "their" major.
```

## Transformations Through Weitzmann Regions

**R, S, and N traverse Weitzmann regions**:

Each of these transformations moves two voices by half step total.
The augmented triad serves as an intermediary:

```
R transformation (C → Am):
C (C-E-G) → C+ (C-E-G#) → Am (A-C-E)
        +1 half step    +1 half step
        (G→G#)          (G#→A)

The augmented triad "fills the gap" in the R transformation

S transformation (C → C#m):
C (C-E-G) → C+ (C-E-G#) → C#m (C#-E-G#)
        +1 half step    +1 half step
        (G→G#)          (C→C#)

N transformation (C → Fm):
Traced through the augmented triad as intermediary
```

**Weitzmann regions and PL cycles**:
```
Each Weitzmann region contains triads from TWO different PL cycles
The augmented triad connects the two cycles

C+ region: C, E, Ab (major) + Cm, Em, Abm (minor)
- C, Cm, Ab, Abm = part of one PL cycle
- E, Em = part of the same PL cycle
- But the REGION connects to adjacent PL cycles via the augmented triad
```

## Musical Context

Weitzmann regions serve specific purposes:
- **19th-century analysis**: Weitzmann (1853) described augmented triad versatility
- **Enharmonic pivot**: Augmented triad can resolve three ways
- **Modulation tool**: Navigate between distant keys
- **Neo-Riemannian network**: Shows how all triads interconnect
- **Chromatic mediant**: R, S, N relationships visualized
- **Composition**: Generate progressions through augmented intermediaries
- **Romantic harmony**: Explains augmented triad usage in Liszt, Wagner

## Examples

### Basic

**The C augmented Weitzmann region**:
```
Central chord: C+ (C-E-G#) = E+ = Ab+

Three major triads (one half step from C+):
C:  C-E-G   (G up to G# = C+)
E:  E-G#-B  (B up to C = C+)
Ab: Ab-C-Eb (Eb up to E = C+)

Three minor triads (one half step from C+):
Am:  A-C-E   (A down to G# = C+)  
C#m: C#-E-G# (C# down to C = C+)
Fm:  F-Ab-C  (F down to E = C+)

All six triads connect to C+ by moving ONE note ONE half step
```

**Visualizing the region**:
```
           C major
              |
              | (G→G#)
              |
    Fm ---- C+ ---- Am
    |    /    \     |
    |   /      \    |
    |  /        \   |
   Ab           E 
    |            |
    |            |
   C#m         
   
Lines represent single half-step connections through C+
```

**Traversing R through augmented triad**:
```
C major → A minor (R transformation)

Step 1: C (C-E-G) → C+ (C-E-G#)
        G moves up to G#
        
Step 2: C+ → Am (A-C-E)
        G# moves up to A (enharmonically)
        
Total motion: G → G# → A (whole step in two half-step moves)
The augmented triad "fills in" the R transformation
```

### From Repertoire

**Liszt, late piano works**: Extensive use of augmented triads as pivot points between Weitzmann regions.

**Wagner, Tristan und Isolde**: Augmented triads connect distantly related key areas.

**Schubert, "Der Doppelgänger"**: Augmented triad passages analyzed through Weitzmann lens.

**Film music**: Augmented chords create modulatory transitions between cues.

## Related Concepts

- **Prerequisite**: augmented-triad, neo-riemannian-theory, plr-transformations
- **Leads to**: cube-dance, enharmonic-modulation, chromatic-mediant
- **See also**: neo-riemannian-cycles, secondary-neo-riemannian-transformations

## Common Confusions

- Weitzmann region = 6 triads around a central augmented triad
- Only FOUR Weitzmann regions exist (because only 4 distinct augmented triads)
- C+ = E+ = Ab+ (same pitches, enharmonic spellings)
- Each region has 3 major + 3 minor triads
- All triads connect to the augmented by ONE half step
- Named after Carl Friedrich Weitzmann (19th-century theorist)
- The augmented triad acts as a "hub" or "pivot"
- R, S, and N transformations can be traced THROUGH the augmented
- Major triads connect by raising their fifth
- Minor triads connect by lowering a different pitch
- Weitzmann regions overlap with PL cycles (but not identical)
- Great for understanding chromatic mediant relationships
- Explains 19th-century composers' love of the augmented triad

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Neo-Riemannian Triadic Progressions"
