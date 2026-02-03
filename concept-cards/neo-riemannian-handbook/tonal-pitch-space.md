---
concept: "Lerdahl's Tonal Pitch Space"
tier: 3
category: comparison-model
sources:
  - Ch 11 (Cohn): Tonnetz as Analytical Apparatus
part: 3
---

# Lerdahl's Tonal Pitch Space

## Quick Definition

Fred Lerdahl's hierarchical model of tonal organization comprising three distinct levels (pitch-class, chord, and regional spaces), each with its own distance metric, offered as a contrast to the Tonnetz's unified representation.

## Formal Definition

**Tonal Pitch Space** (TPS) is Lerdahl's comprehensive model from his 2001 book, which proposes:

### Three Hierarchical Levels

**1. Pitch-Class Space (Chromatic)**
- All 12 pitch classes arranged by proximity
- Primarily a chromatic/diatonic circle
- Distance measured in semitones or scale steps

**2. Chordal Space**
- All triads (and optionally seventh chords)
- Distance based on voice-leading and common-tone relationships
- Organized around a circle of fifths with mode distinction

**3. Regional Space**
- All 24 major and minor keys
- Distance based on shared pitch content
- Organized as a toroidal structure

### Basic Space
Lerdahl's **basic space** is a reduction showing:
- Current pitch-class collection
- Current chord (as subset)
- Current key (as reference point)

Stability is measured by **level of embedding** in this hierarchy.

## Distance Metrics

### Chord Distance (δ)
Lerdahl's chord distance calculation includes:
1. **j**: Number of steps on circle of fifths between roots
2. **k**: Change in chord type (major/minor)
3. **i**: Non-common tones between chords

Formula: δ = i + j + k

### Regional Distance
Similar calculation for keys:
- Fifth-circle distance between tonics
- Mode change penalty
- Scale-degree difference counting

### Aggregate Distance
Total tonal distance sums:
- Event-to-event chord changes
- Regional implications
- Weighted by structural importance

## Cohn's Critique

### The Conflation Argument
Richard Cohn challenges Lerdahl's separation:

**Lerdahl**: Pitch-class, chord, and regional spaces are distinct structures requiring separate representations.

**Cohn**: A single Tonnetz can represent all three levels:
- Pitch classes are nodes
- Chords are triangles (groupings of nodes)
- Regions are parallelograms (groupings of triangles)

### Mutual Implication
Cohn argues these levels **mutually imply** each other:
- Knowing any two levels determines the third
- Separate representations are redundant
- The Tonnetz is more parsimonious

### Counterexample Cases
Lerdahl's model handles:
- Clear tonal passages with established keys
- Hierarchically structured music
- Pieces following conventional harmonic progressions

But Cohn argues it struggles with:
- Regionally indeterminate passages
- Chromatic sequences without clear tonal center
- Late Romantic and early modernist repertoire

## Points of Agreement

Both Lerdahl and Cohn acknowledge:
- Voice-leading proximity is analytically significant
- Chord relations can be modeled spatially
- Regional relations exist at a higher structural level
- Some form of distance metric is valuable

## Comparison Table

| Feature | Tonal Pitch Space | Tonnetz (Cohn) |
|---------|-------------------|----------------|
| Levels | 3 distinct | 1 unified |
| Distance | Calculated formula | Path length |
| Emphasis | Hierarchical embedding | Geometric position |
| Tonality | Always regionally determined | Optionally indeterminate |
| Application | Tonal music primarily | Chromatic music emphasis |
| Theoretical basis | Cognition/linguistics | Geometry/group theory |

## Integration Possibilities

### Complementary Use
The models can be complementary:
- TPS for establishing hierarchical relationships
- Tonnetz for tracking chromatic voice leading
- Each illuminates different aspects of the music

### Hybrid Approaches
Some analysts use:
- TPS concepts (stability, distance) within Tonnetz framework
- Tonnetz visualization with Lerdahl-style metrics
- Multiple representations for different analytical questions

## Related Concepts

- **Prerequisite**: tonnetz, voice-leading
- **Leads to**: regional-space
- **See also**: tonic-prolongation

## Common Confusions

- **TPS is not "wrong"**: Cohn's critique is that the Tonnetz does the same work more efficiently, not that TPS is incorrect
- **Different purposes**: TPS was designed for cognitive modeling; Tonnetz for transformation tracking
- **Levels are not independent**: Even in TPS, the levels interact hierarchically

## Source References

- Oxford Handbook of Neo-Riemannian Music Theories, Part 3
- Ch 11: Richard Cohn, "Tonnetz as Analytical Apparatus"
- Lerdahl, *Tonal Pitch Space* (2001)
- Lerdahl & Jackendoff, *A Generative Theory of Tonal Music* (1983)
