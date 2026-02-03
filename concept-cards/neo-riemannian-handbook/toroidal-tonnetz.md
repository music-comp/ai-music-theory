---
concept: "Toroidal Tonnetz (Conforming Tonnetz)"
tier: 3
category: geometric-models
sources:
  - Ch 9 (Gollin): Tonnetz from Acoustic to Metaphorical
  - Ch 11 (Cohn): Tonnetz as Analytical Apparatus
part: 3
---

# Toroidal Tonnetz (Conforming Tonnetz)

## Quick Definition

The geometric shape of the Tonnetz under equal temperament, where enharmonic equivalence causes the infinite plane to wrap into a torus (donut shape) with finite surface area containing exactly 12 pitch classes.

## Formal Definition

The **toroidal Tonnetz** arises when:
1. Equal temperament is assumed
2. Enharmonic equivalences are enforced
3. The generating intervals become cyclic

### From Plane to Torus

**Infinite Plane (Just Intonation)**:
- Perfect fifths generate infinitely (no cycle)
- Major thirds generate infinitely
- Enharmonically "same" pitches occupy different locations
- C# ≠ Db on the plane

**Torus (Equal Temperament)**:
- 12 perfect fifths = 7 octaves (closes the cycle)
- 3 major thirds = 1 octave (closes the cycle)
- C# = Db at the same location
- The plane wraps into a torus

### Mathematical Description
The torus is the product of two circles:
- **Circle 1**: 12 pitch classes by fifth (C-G-D-A-E-B-F#-C#-G#-D#-A#-E#/F-C)
- **Circle 2**: 4 pitch-class groups by major third (C-E-G#, Db-F-A, D-F#-A#, Eb-G-B)

Torus = S¹ × S¹ where each S¹ is a circle.

## Daniel Harrison's Terminology

### Conforming Tonnetz
The toroidal version "conforms" to equal temperament:
- Respects enharmonic equivalence
- Has finite, bounded geometry
- Each pitch class appears exactly once

### Nonconforming Tonnetz
The planar version "does not conform" to equal temperament:
- Maintains just-intonation distinctions
- Has infinite, unbounded geometry
- Same letter name appears multiple times

## Geometric Properties

### Surface Area
The torus surface contains:
- 12 pitch-class nodes
- 24 triadic regions (triangles)
- 36 edges connecting nodes

### Triadic Navigation
On the torus:
- Every triad has exactly 3 PLR neighbors
- LP cycles trace "longitudinal" loops
- PR cycles trace "latitudinal" loops
- LR spirals around the torus

### Homotopy
Paths on the torus can be classified by:
- How many times they wind around each axis
- Whether they return to starting point
- These correspond to transformation cycles

## Visualization

### Flat Torus Representation
The torus can be visualized as a rectangle with opposite edges identified:
```
A--E--B--F#--C#--G#--D#--A#--F--C--G--D--A
|  |  |   |   |   |   |   |  |  |  |  |  |
F--C--G--D--A--E--B--F#--C#--G#--D#--A#--F
|  |  |   |   |   |   |   |  |  |  |  |  |
Db-Ab-Eb--Bb--F--C--G--D--A--E--B--F#--Db
|  |  |   |   |   |   |   |  |  |  |  |  |
A--E--B--F#--C#--G#--D#--A#--F--C--G--D--A
```
Top edge connects to bottom; left edge connects to right.

### 3D Embedding
The torus can be embedded in 3D space:
- Major donut shape
- Pitch classes as points on the surface
- Triads as triangular regions

## Analytical Implications

### Finite Group Action
On the conforming torus:
- The PLR group has order 24
- Acts on exactly 24 triads
- All operations are well-defined and closed

### No "Edge" Effects
Unlike the infinite plane:
- No boundary to the space
- Every triad equivalent in position
- Cycles close naturally

### Hexatonic "Tubes"
Hexatonic systems appear as:
- Cylindrical regions on the torus
- 4 non-overlapping tubes
- LP motion stays within one tube

## Comparison with Other Representations

### Chicken-Wire Torus
The graph of PLR connections:
- Vertices = triads
- Edges = P, L, or R relations
- Embedding on torus = chicken-wire pattern

### Dual Representations
- **Pitch-class torus**: 12 nodes (pitches)
- **Triadic torus**: 24 nodes (triads)
- These are geometric duals

## Historical Development

### From Plane to Torus
The transition reflects:
- 19th century: Just intonation assumed (plane)
- Late 19th century: Equal temperament normalizing
- 20th century: Torus embraced for analytical convenience

### Neo-Riemannian Adoption
Modern theory prefers the torus:
- Finite, tractable space
- Group actions well-behaved
- Matches contemporary musical practice

## Three-Axis Hypertorus

### Gollin's Extension
When three generating intervals are used (IC3, IC4, IC5):
- The space becomes a 3-torus (hypertorus)
- Additional structure emerges
- More complex navigation possible

### Research Frontier
Multidimensional Tonnetze remain an active area of theoretical research.

## Related Concepts

- **Prerequisite**: tonnetz, enharmonic-equivalence
- **Leads to**: hexatonic-systems, plr-transformations
- **See also**: voice-leading-graph, regional-space

## Common Confusions

- **Torus vs. sphere**: A torus has a hole; a sphere doesn't. The Tonnetz is toroidal, not spherical.
- **Conforming doesn't mean "correct"**: Both plane and torus are valid models for different purposes
- **Still 2D**: The torus is a 2D surface, just embedded in 3D space

## Source References

- Oxford Handbook of Neo-Riemannian Music Theories, Part 3
- Ch 9: Edward Gollin, "From Acoustic to Metaphorical"
- Ch 11: Richard Cohn, "Tonnetz as Analytical Apparatus"
- Harrison, "Nonconformist Notions of Nineteenth-Century Enharmonicism" (2002)
