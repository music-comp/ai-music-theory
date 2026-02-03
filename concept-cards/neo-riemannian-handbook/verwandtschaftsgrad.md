---
concept: "Verwandtschaftsgrad (Degree of Relatedness)"
tier: 3
category: distance-metrics
sources:
  - Ch 9 (Gollin): Tonnetz from Acoustic to Metaphorical
part: 3
---

# Verwandtschaftsgrad (Degree of Relatedness)

## Quick Definition

Riemann's concept of measuring harmonic distance between chords or keys by counting steps on the Tonnetz, marking the transition from understanding the Tonnetz as a frequency calculator to viewing it as a navigable space with path-based distance metrics.

## Formal Definition

**Verwandtschaftsgrad** (German: "degree of relationship" or "grade of kinship") quantifies:
- The closeness between two tones, chords, or keys
- Measured by the number of generator steps required
- Realized as path length on the Tonnetz

### Riemann's Introduction
In the *Musik-Lexikon* (1894 edition), Riemann introduced:
- Explicit distance measurement between Klangs
- Path counting as the measure
- The Tonnetz as the space where distance is computed

### Path-Based Calculation
Given generators Q (fifth) and T (third):
- C to G: 1 step (Q)
- C to E: 1 step (T)
- C to D: 2 steps (Q²)
- C to A: 2 steps (Q⁻¹T or TQ⁻¹)
- C to F#: 3+ steps

## Significance for Tonnetz Interpretation

### From Matrix to Landscape
The introduction of Verwandtschaftsgrad marked a conceptual shift:

**Before (Oettingen)**:
- Tonnetz entries are calculation results
- Spatial position records exponents
- "Distance" is not a primary concept

**After (Riemann 1894)**:
- Tonnetz is a navigable space
- Paths between entries are meaningful
- Distance = path length = degree of relatedness

### Compositional Implications
Composers "navigate" the Tonnetz:
- Close modulations: short paths
- Distant modulations: long paths
- Chromatic progressions: complex trajectories

## Calculation Methods

### Using Q and T
Distance in the standard Tonnetz:
- Count horizontal moves (Q or Q⁻¹)
- Count vertical moves (T or T⁻¹)
- Sum = total Verwandtschaftsgrad

### Example: C major to E major
- Path 1: C → G → D → A → E (4 fifths) = 4 steps
- Path 2: C → E (1 third) = 1 step
- Verwandtschaftsgrad = 1 (shortest path)

### Including Mode Change
For major/minor relationships:
- Add Seitenwechsel (⊕) where needed
- Each mode change adds 1 step
- C major to A minor: T⊕ = 2 steps

## Relationship to Modern Distance Metrics

### Voice-Leading Distance
Neo-Riemannian theory often measures distance by voice-leading displacement:
- P: 1 semitone
- L: 1 semitone
- R: 2 semitones

This is related but not identical to Verwandtschaftsgrad.

### Lerdahl's Tonal Distance
Fred Lerdahl's chord distance formula incorporates:
- Circle-of-fifths distance (similar to Q count)
- Mode change
- Non-common tones

This operationalizes similar intuitions with different weighting.

### Comparison

| Metric | Basis | C to G | C to E | C+ to c- |
|--------|-------|--------|--------|----------|
| Verwandtschaftsgrad | Path length | 1 | 1 | 1 |
| PLR steps | Transformation count | 2 (LR) | 2 (LP or RL) | 1 (P) |
| Voice-leading | Semitone motion | 2 | 3 | 1 |

## Analytical Applications

### Key Distance
Verwandtschaftsgrad applies to keys:
- C major to G major: 1 (one fifth)
- C major to A minor: 2 (relative via third + mode)
- C major to F# major: 3+ (depending on path)

### Modulation Complexity
Complex modulations traverse more steps:
- Diatonic modulation: 1-2 steps
- Chromatic modulation: 3-4 steps
- Enharmonic modulation: potentially infinite (in just intonation)

### Formal Analysis
Large-scale tonal plans can be charted:
- Exposition: Tonic region
- Development: Increasing Verwandtschaftsgrad
- Recapitulation: Return to minimal Verwandtschaftsgrad

## Historical Position

### Between Acoustics and Transformation
Verwandtschaftsgrad represents:
- A move beyond pure acoustics (frequency ratios)
- Not yet fully transformational (operations on objects)
- An intermediate stage: "distance in navigable space"

### Prefiguring Neo-Riemannian Theory
The concept anticipates:
- Lewin's "characteristic gesture" between points
- Cohn's emphasis on path and cycle
- Geometric models of harmonic space

## Related Concepts

- **Prerequisite**: Tonnetz, Harmonieschritte, just-intonation
- **Leads to**: tonal-pitch-space, voice-leading-graph
- **See also**: key-distance, combinatorial-group-theory

## Common Confusions

- **Path vs. destination**: Verwandtschaftsgrad measures the journey, not just the endpoint
- **Multiple paths**: Different paths between same endpoints may have different lengths
- **Generator choice matters**: Distance depends on which generators define the space

## Source References

- Oxford Handbook of Neo-Riemannian Music Theories, Part 3
- Ch 9: Edward Gollin, "From Acoustic to Metaphorical"
- Riemann, *Musik-Lexikon* (1894 and later editions)
