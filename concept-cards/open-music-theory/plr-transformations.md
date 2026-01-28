---
concept: PLR Transformations
category: harmony
source: Open Music Theory
chapter: "Neo-Riemannian Triadic Progressions"
part: 5
---

# PLR Transformations

## Quick Definition

The three basic Neo-Riemannian operations that transform one triad into another while preserving two common tones and changing the mode: Parallel (P) preserves the perfect fifth and moves the third by half step; Leading-tone exchange (L) preserves the minor third and moves the remaining note by half step; Relative (R) preserves the major third and moves the remaining note by whole step—each transformation toggling between a specific major-minor triad pair.

## Formal Definition

**The three basic transformations**:

**P (Parallel)**:
- Preserves the **perfect fifth**
- Moves the **third** by **half step**
- Connects triads sharing the **same root** (parallel major/minor)
- Example: C major ↔ C minor

**L (Leading-tone exchange)**:
- Preserves the **minor third**
- Moves the remaining note by **half step**
- Connects triads related by **major third**
- Example: C major ↔ E minor

**R (Relative)**:
- Preserves the **major third**
- Moves the remaining note by **whole step**
- Connects triads related by **minor third** (relative major/minor)
- Example: C major ↔ A minor

**All transformations**:
- Toggle between ONE major and ONE minor triad
- Applying the same transformation twice returns to original
- Change the MODE of the triad (major ↔ minor)

## Voice-Leading Detail

**P transformation (Parallel)**:
```
C major: C - E - G
         |   ↓   |      (fifth C-G stays, third moves)
C minor: C - Eb- G

Preserves: P5 (C-G)
Moves: E → Eb (half step down)
Mode change: Major → Minor

Reverse (minor to major):
Cm: C - Eb - G  →  C: C - E - G
    E  moves up by half step (Eb → E)
```

**L transformation (Leading-tone exchange)**:
```
C major: C - E - G
         ↓   |   |      (minor third E-G stays, root moves)
E minor: B - E - G

Preserves: m3 (E-G)
Moves: C → B (half step down, C becomes leading tone of Em)
Mode change: Major → Minor

Why "leading-tone"? The moving note BECOMES a leading tone
C major: C moves to B (leading tone of the new chord Em)
E minor: B moves to C (B was leading tone, now becomes root)
```

**R transformation (Relative)**:
```
C major: C - E - G
         |   |   ↓      (major third C-E stays, fifth moves)
A minor: C - E - A

Preserves: M3 (C-E)
Moves: G → A (whole step up)
Mode change: Major → Minor

This connects relative major and minor
C major = relative major of A minor
A minor = relative minor of C major
```

## The Toggle Principle

**Each transformation is an involution**:
```
P(P(C)) = C     Applying P twice returns to original
L(L(C)) = C     Applying L twice returns to original  
R(R(C)) = C     Applying R twice returns to original

Example:
C major -P→ C minor -P→ C major
C major -L→ E minor -L→ C major
C major -R→ A minor -R→ C major

Like a light switch or caps lock:
On/Off/On = Off/On/Off = back to original state
```

**Successive identical transformations**:
```
C -L→ Em -L→ C -L→ Em -L→ C -L→ Em ...

Alternates between only TWO chords
Used in minimalist music (Laurie Anderson, "O Superman")
```

## Compound Transformations

**Combining P, L, R**:
```
RP: C -R→ Am -P→ Am... wait, that's wrong
    Actually: C -R→ Am, then Am -P→ A major
    C -RP→ A major (!)

LP: C -L→ Em -P→ E major
    C -LP→ E major

PL: C -P→ Cm -L→ Ab major  
    C -PL→ Ab major

PR: C -P→ Cm -R→ Eb major
    C -PR→ Eb major
```

**Any two triads in 5 or fewer steps**:
```
C major to F# minor:
C -L→ Em -P→ E -L→ G#m/Abm -R→ E -... 
or
C -R→ Am -L→ F -P→ Fm -R→ Ab -L→ Cm -...

Multiple paths exist between any two triads
```

## Musical Context

PLR transformations appear in various contexts:
- **Late Romantic music**: Brahms, Wagner, Wolf, Liszt
- **Chromatic mediant relationships**: P connects parallel keys, R connects relatives
- **Film music**: Triadic progressions that resist functional analysis
- **Minimalism**: Repetitive transformation chains
- **Analysis tool**: Understand non-functional triadic progressions
- **Composition tool**: Generate interesting progressions through transformation

## Examples

### Basic

**All three transformations from C major**:
```
Starting chord: C major (C-E-G)

P: C major → C minor
   C-E-G → C-Eb-G
   (Fifth stays, third moves down by half step)

L: C major → E minor  
   C-E-G → B-E-G
   (Minor third E-G stays, C moves to B)

R: C major → A minor
   C-E-G → A-C-E  
   (Major third C-E stays, G moves to A)
```

**All three from C minor**:
```
Starting chord: C minor (C-Eb-G)

P: C minor → C major
   C-Eb-G → C-E-G
   (Fifth stays, third moves up by half step)

L: C minor → Ab major
   C-Eb-G → C-Eb-Ab
   (Minor third Eb-G becomes Eb-C in Ab; wait...
   Actually: C-Eb-G → Ab-C-Eb
   Minor third Eb-G stays?... No:
   In Cm, the minor 3rd is C-Eb. L preserves m3.
   C-Eb stays, G moves to Ab. Result: Ab-C-Eb = Ab major)

R: C minor → Eb major
   C-Eb-G → Eb-G-Bb
   (Major third Eb-G stays, C moves to Bb... 
   Actually: major 3rd in Cm is Eb-G, R preserves M3
   Eb-G stays, C moves to Bb. Result: Bb-Eb-G... that's Eb major)
```

**Compound transformation example**:
```
C major to Ab major via PL:
C major -P→ C minor -L→ Ab major

Step 1: C-E-G → C-Eb-G (P: fifth stays, third down)
Step 2: C-Eb-G → Ab-C-Eb (L: minor third C-Eb stays, G→Ab)

C -PL→ Ab (down major third)
This is the hexatonic pole relationship!
```

### From Repertoire

**Brahms, Violin/Cello Concerto, mm. 270-76**:
```
Ab -P→ Abm -L→ E -P→ Em -L→ C -P→ Cm -L→ Ab

Alternating P and L transformations
Creates a closed PL cycle
Returns to starting chord after 6 steps
```

**Laurie Anderson, "O Superman"**: Successive L transformations throughout.

**Schubert, "Der Wanderer"**: R and L transformations between distantly related triads.

## Related Concepts

- **Prerequisite**: triad, major-minor-mode, common-tone, voice-leading
- **Leads to**: neo-riemannian-cycles, tonnetz, transformational-theory
- **See also**: neo-riemannian-theory, secondary-neo-riemannian-transformations, parsimonious-voice-leading

## Common Confusions

- P, L, R are the THREE basic Neo-Riemannian transformations
- All three preserve TWO common tones and change the mode
- P (Parallel): Same root, mode changes (C ↔ Cm)
- L (Leading-tone): Major third apart, mode changes (C ↔ Em)
- R (Relative): Minor third apart, mode changes (C ↔ Am)
- P preserves the FIFTH, moves the THIRD (half step)
- L preserves the MINOR THIRD, moves the other note (half step)
- R preserves the MAJOR THIRD, moves the other note (whole step)
- R is "twice as much work" as P or L (whole step vs. half step)
- All transformations TOGGLE (applying twice returns to original)
- "Leading-tone exchange" = the moving note becomes a leading tone
- Can combine into compound transformations (PL, RP, LPL, etc.)
- Any two triads connected in 5 or fewer basic transformations
- Used for analysis and composition of non-functional triadic music

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Neo-Riemannian Triadic Progressions"
