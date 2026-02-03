---
concept: Tonnetz
category: harmony
source: Open Music Theory
chapter: "Neo-Riemannian Triadic Progressions"
part: 5
---

# Tonnetz

## Quick Definition

A visual representation of pitch relationships arranged in a two-dimensional grid where perfect fifths run horizontally (left to right), major thirds run diagonally (top-left to bottom-right), and minor thirds run diagonally (bottom-left to top-right), allowing any three adjacent pitches forming a triangle to represent a major or minor triad, and Neo-Riemannian transformations to be visualized as "flipping" a triangle along one of its edges to reach an adjacent triad.

## Formal Definition

**Tonnetz** (German for "tone network"):

**Grid structure**:
```
                    major third (M3) axis
                        ↘
         E ---- B ---- F# --- C# --- G# --- D#
        / \    / \    / \    / \    / \    / \
       /   \  /   \  /   \  /   \  /   \  /   \
      C ---- G ---- D ---- A ---- E ---- B ----
       \   /  \   /  \   /  \   /  \   /  \   /
        \ /    \ /    \ /    \ /    \ /    \ /
         Ab--- Eb --- Bb --- F ---- C ---- G ---
                        ↗
                  minor third (m3) axis
    
    ←―――――――――― perfect fifth (P5) axis ―――――――――→
```

**Reading the grid**:
- **Horizontal** (left to right): Perfect fifths (C-G-D-A-E-B...)
- **Diagonal ↘** (top-left to bottom-right): Major thirds (C-E-G#...)
- **Diagonal ↗** (bottom-left to top-right): Minor thirds (C-Eb-Gb...)

**Triads as triangles**:
- Any three adjacent pitches forming a triangle = a triad
- **Upward-pointing triangle** (▲) = major triad
- **Downward-pointing triangle** (▼) = minor triad

## Visualizing Transformations

**P, L, R as triangle flips**:

**P (Parallel) = flip along perfect fifth edge**:
```
     E           Eb
    / \          / \
   /   \   P    /   \
  C --- G  →   C --- G
  (C major)    (C minor)

Flip along the C-G (P5) edge
E moves to Eb
```

**R (Relative) = flip along major third edge**:
```
     E            E
    / \          / \
   /   \   R    /   \
  C --- G  →   A --- C
  (C major)    (A minor)

Flip along the C-E (M3) edge
G moves to A
```

**L (Leading-tone) = flip along minor third edge**:
```
     E            E
    / \          / \
   /   \   L    /   \
  C --- G  →   B --- G
  (C major)    (E minor)

Flip along the E-G (m3) edge
C moves to B
```

## Tonnetz Properties

**Proximity = voice-leading distance**:
- Adjacent triads share an edge (2 common tones)
- Nearby triads are voice-leading close
- Distant triads on Tonnetz = more chromatic voice-leading

**Key areas cluster together**:
```
Triads in C major are close on the Tonnetz:
C major, A minor, G major, E minor, F major, D minor

Distant keys are far apart:
C major and F# major are maximally distant
```

**Wrap-around (in 12-tone equal temperament)**:
- The Tonnetz theoretically wraps around (torus shape)
- After 12 fifths: back to starting pitch (enharmonic)
- After 3 major thirds: back to starting pitch (enharmonic)
- After 4 minor thirds: back to starting pitch (enharmonic)

## Musical Context

The Tonnetz serves multiple purposes:
- **Visualization**: See relationships between triads spatially
- **Analysis**: Trace chord progressions as paths on the grid
- **Composition**: Discover new progressions by exploring the space
- **Historical**: Derived from 18th/19th-century theory (Euler, Riemann)
- **Neo-Riemannian analysis**: Standard visualization tool
- **Film music analysis**: Map non-functional progressions
- **Understanding key relationships**: Close keys cluster together

## Examples

### Basic

**C major triad on Tonnetz**:
```
         E
        /|\
       / | \
      /  |  \
     C---+---G
     
Triangle vertices: C, E, G
Upward-pointing = major triad
```

**Performing transformations**:
```
C major position:
         E
        / \
       /   \
      C --- G

After P (flip along C-G):
         Eb
        / \
       /   \
      C --- G
= C minor

After L on C major (flip along E-G):
         E
        / \
       /   \
      B --- G
= E minor

After R on C major (flip along C-E):
         E
        / \
       /   \
      C --- A
Wait, let me reconsider...
A-C-E forms A minor (downward triangle)
= A minor
```

**Tracing a progression (Brahms example)**:
```
Ab → Abm → E → Em → C → Cm → Ab

On Tonnetz:
Start at Ab major (Ab-C-Eb triangle)
P: flip to Ab minor
L: flip to E major
P: flip to E minor  
L: flip to C major
P: flip to C minor
L: flip to Ab major (back to start!)

Creates a path moving up the Tonnetz
Closed loop after 6 transformations
```

### From Repertoire

**Wagner, Tristan prelude**: Complex Tonnetz paths showing chromatic mediant relationships.

**Brahms, late piano works**: Progressions that trace geometric paths on the Tonnetz.

**Film music (Howard Shore)**: Distinct Tonnetz regions for different characters/places in Lord of the Rings.

**Schubert, "Der Doppelgänger"**: Tonnetz analysis reveals underlying triadic logic.

## Related Concepts

- **Prerequisite**: triad, interval, neo-riemannian-theory, plr-transformations
- **Leads to**: neo-riemannian-cycles, weitzmann-region, cube-dance
- **See also**: pitch-space, transformational-theory, voice-leading-offset

## Common Confusions

- Tonnetz = visual representation of pitch/triad relationships (not the theory itself)
- Horizontal axis = perfect fifths (C-G-D-A-E-B...)
- Two diagonal axes = major thirds and minor thirds
- Triangles = triads (upward ▲ = major, downward ▼ = minor)
- P, L, R = "flipping" a triangle along an edge
- P flips along the fifth (horizontal) edge
- R flips along the major third (one diagonal) edge
- L flips along the minor third (other diagonal) edge
- Adjacent triads share 2 common tones (one transformation apart)
- Tonnetz shows voice-leading proximity (close = smooth voice leading)
- In 12-TET, the Tonnetz wraps around (topologically a torus)
- Triads in same key cluster together on Tonnetz
- Distant keys are far apart on Tonnetz
- Great for visualizing AND generating chord progressions

## Source Reference

Open Music Theory, Part V: "Chromaticism"
Open Music Theory, Part V: "Neo-Riemannian Triadic Progressions"
