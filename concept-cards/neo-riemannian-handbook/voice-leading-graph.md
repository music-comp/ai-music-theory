---
concept: Voice-Leading Graph / Voice Leadability
tier: 3
category: geometric-models
sources:
  - Ch 11 (Cohn): Tonnetz as Analytical Apparatus
  - Ch 13 (Gollin): Doppelklänge & Transformational Analysis
part: 3
---

# Voice-Leading Graph / Voice Leadability

## Quick Definition

Geometric representations of chords as points in multidimensional space where proximity reflects voice-leading efficiency, enabling the visualization and calculation of parsimonious connections between harmonies.

## Formal Definition

**Voice-leading graphs** are spatial models where:
- **Nodes** represent chords (as points in pitch-class space)
- **Edges** connect chords with efficient voice-leading relationships
- **Distance** corresponds to total voice-leading displacement

### Voice Leadability (Cohn)
The property of chord types that participate in smooth voice-leading networks. A chord class is highly "voice-leadable" when:
1. Multiple other chords lie at minimal voice-leading distance
2. These connections form regular patterns (cycles, grids)
3. The chord can participate in parsimonious progressions

Consonant triads ([037]) exhibit optimal voice leadability in 12-TET.

## Tonnetz as Voice-Leading Graph

### Triadic Tonnetz Properties
The Tonnetz can be read as a voice-leading graph:
- Each triangle (triad) is adjacent to three other triangles
- Adjacent triangles share two pitch classes (one voice-leading step)
- The P, L, R operations connect adjacent triangles

### Voice-Leading Distance on Tonnetz
| Transformation | Common Tones | Moving Voice | Semitone Distance |
|----------------|--------------|--------------|-------------------|
| P | 2 | 1 (semitone) | 1 |
| L | 2 | 1 (semitone) | 1 |
| R | 2 | 1 (whole tone) | 2 |
| LP | 1 | 2 | 2 |
| PR | 1 | 2 | 3 |

### Geometric Interpretation
- P and L are "maximally parsimonious"
- R is "next most parsimonious"
- Compounds increase voice-leading distance

## DOUTH2 Relation (Douthett)

### Definition
Two chords of the same cardinality are **DOUTH2-related** when:
- Two voices remain stationary
- The remaining voices move by semitone in parallel motion

### Application to Triads
P and L transformations are DOUTH2 relations:
- P: Root and fifth fixed; third moves by semitone
- L: Third and fifth fixed; root moves by semitone

### Extension to Larger Sets
DOUTH2 applies to tetrachords and beyond:
- [0148] tetrachords have DOUTH2 networks
- Seventh chords form voice-leading graphs
- Any set class can be analyzed for DOUTH2 connections

## Higher-Dimensional Voice-Leading Spaces

### Tymoczko's Chord Geometry
Dmitri Tymoczko developed comprehensive voice-leading geometry:
- Chords as points in n-dimensional space (n = number of voices)
- Voice-leading as linear paths between points
- Orbifold structure accounts for permutation equivalence

### Properties
- Distance = sum of individual voice motions
- Efficient voice leadings = short paths
- Maximally smooth cycles = closed geodesics

### Relation to Tonnetz
The Tonnetz is a 2D projection of:
- 3-voice chord space for triads
- The projection preserves voice-leading relationships
- Other projections yield different (complementary) views

## Parsimonious Graphs

### Douthett & Steinbach's Contribution
Jack Douthett and Peter Steinbach formalized:
- Graphs of parsimonious connections
- Conditions for graph connectivity
- Cycles within parsimonious networks

### Chicken-Wire Torus
The graph of triadic P, L, R connections forms:
- A torus (in equal temperament)
- Regular hexagonal tiling ("chicken wire")
- 24 vertices (triads), 36 edges (connections)

### Graph Properties
- Vertex degree: 3 (each triad has 3 parsimonious neighbors)
- Connected: Any triad reachable from any other
- Regular: All vertices equivalent

## Analytical Applications

### Voice-Leading Parsimony Analysis
Given a progression:
1. Plot chords on voice-leading graph
2. Identify the path traversed
3. Measure total voice-leading distance
4. Compare to alternatives (was this the "smoothest" path?)

### Motivic Voice Leading
Recurring voice-leading patterns:
- May not be visible in traditional notation
- Become apparent in geometric representation
- Connect surface harmony to deeper logic

### Chromatic Saturation
As voice-leading distance decreases toward zero:
- Chords cluster in pitch space
- Maximal common tones
- Minimal motion
- "Saturation" of chromatic space

## Related Concepts

- **Prerequisite**: voice-leading, tonnetz, parsimonious-trichords

## Common Confusions

- **Voice leading vs. transformation**: Voice-leading graphs measure motion; transformation networks classify operations
- **Distance vs. path**: Distance is measured by the edge; path is the sequence of edges traversed
- **Projection issues**: The Tonnetz is a 2D projection; some information is lost compared to full chord space

## Source References

- Oxford Handbook of Neo-Riemannian Music Theories, Part 3
- Ch 11: Richard Cohn, "Tonnetz as Analytical Apparatus"
- Ch 13: Edward Gollin, "Doppelklänge"
- Douthett & Steinbach, "Parsimonious Graphs" (1998)
- Tymoczko, *A Geometry of Music* (2011)
