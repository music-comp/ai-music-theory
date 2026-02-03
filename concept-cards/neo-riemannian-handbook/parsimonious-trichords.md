---
concept: Parsimonious Trichords / Generated Trichords
tier: 2
category: voice-leading-theory
sources:
  - Ch 10 (Clark): Klangvertretung & Tone Representation
  - Ch 11 (Cohn): Tonnetz as Analytical Apparatus
  - Ch 12 (Engebretsen): Harmonieschritte & Neo-Riemannian Appropriations
  - Ch 13 (Gollin): Doppelklänge & Transformational Analysis
part: 3
---

# Parsimonious Trichords / Generated Trichords

## Quick Definition

Three-note chords (trichords) that can be connected through maximally smooth voice leading, where only one voice moves by a single semitone, forming the basis for neo-Riemannian transformational systems.

## Formal Definition

**Parsimonious trichords** are set classes that participate in efficient voice-leading networks:

### Voice-Leading Parsimony
A voice-leading connection is **parsimonious** when:
- Total voice-leading distance is minimal
- Individual voices move by small intervals (semitones preferred)
- The number of moving voices is minimal

For triads (set class [037]):
- **Most parsimonious**: P and L operations (1 semitone total)
- **Next most parsimonious**: R operation (2 semitones total)

### The Consonant Triad's Special Status
Richard Cohn demonstrated that consonant triads [037] are optimal for parsimonious voice leading because:

1. **Acoustic foundation**: Triads are the largest-cardinality consonant collections
2. **Chromatic positioning**: Triads bear optimal voice-leading properties in 12-tone equal temperament
3. **Near-evenness**: The interval pattern (4-3-5) is close to even division of the octave

### Maximally Even Sets
Parsimonious behavior relates to **maximal evenness**—how close a set comes to dividing the octave equally:
- Perfectly even: Augmented triad [048]
- Nearly even: Major/minor triads [037]
- The slight deviation from evenness enables the semitone displacements

## Generation Through Transformation

### PLR as Generators
The PLR operations generate all 24 consonant triads from any starting triad:
- P, L, R are **involutions** (self-inverse)
- Their combinations produce all possible triadic relations
- The group has a rich internal structure

### Cycle Generation
Repeated application of single transformations or compounds:

| Operation | Cycle | Length | Set Class Traversed |
|-----------|-------|--------|---------------------|
| P alone | Toggles between parallel | 2 | Major/minor pair |
| L alone | Toggles between L-related | 2 | Major/minor pair |
| R alone | Toggles between relatives | 2 | Major/minor pair |
| LP | Hexatonic cycle | 6 | 6 triads in [014589] |
| PR | Octatonic cycle | 8 | 8 triads in [0134679T] |
| LR | Full chromatic | 24 | All 24 triads |

## Douthett's DOUTH2 Relation

Jack Douthett formalized **DOUTH2**: the relation between two sets of the same cardinality where:
- Two tones remain fixed
- Two tones move by semitone in parallel motion

This captures the voice-leading logic underlying:
- P and L transformations on triads
- Transformations on larger sets (e.g., [0148] tetrachords in Ravel)
- Contextual inversions preserving specific interval content

## Analytical Applications

### Tracking Common Tones
Parsimonious voice leading foregrounds **common tones** as anchoring points:
- P: Fifth (root and fifth) held, third moves
- L: Minor third (third and fifth) held, root moves
- R: Major third (root and third) held, fifth moves

### Network Analysis
Parsimonious connections create **transformation networks** where:
- Nodes represent triads
- Edges represent single parsimonious moves
- Paths trace voice-leading trajectories through pitch-class space

### Hexatonic and Octatonic Spaces
Parsimonious cycles partition triads into subsystems:
- 4 hexatonic systems (each containing 6 triads)
- 3 octatonic systems (each containing 8 triads)
- These provide intermediate structures between individual triads and full chromatic space

## Extension to Other Set Classes

### [0148] Tetrachords
Gollin's analysis of Ravel's Forlane demonstrates parsimonious transformations on [0148]:
- Inversion about the semitone (analogous to PLR on triads)
- W₃ operation maps embedded triads between hexatonic poles
- Voice-leading logic extends beyond triadic contexts

### General Principles
Parsimonious behavior is possible for any set class that:
- Contains intervals of 1 or 2 semitones
- Has cardinality allowing minimal-motion connections
- Exhibits near-evenness in its interval structure

## Related Concepts

- **Prerequisite**: triad, voice-leading, set-class, interval
- **Leads to**: hexatonic-systems, maximally-smooth-cycles, voice-leading-graph
- **See also**: plr-transformations, tonnetz, common-tones

## Common Confusions

- **Parsimonious ≠ efficient**: "Efficient" can mean many things; "parsimonious" specifically means minimal voice-leading displacement
- **Not just triads**: While triads are the focus of neo-Riemannian theory, parsimony applies to other cardinalities
- **Acoustic vs. voice-leading**: Triads are acoustically optimal (consonance) AND voice-leading optimal (parsimony)—these are independent properties that happen to coincide

## Source References

- Oxford Handbook of Neo-Riemannian Music Theories, Part 3
- Ch 11: Richard Cohn, "Tonnetz as Analytical Apparatus"
- Ch 13: Edward Gollin, "Doppelklänge"
- Cohn, "Neo-Riemannian Operations, Parsimonious Trichords, and Their Tonnetz Representations" (1997)
- Douthett & Steinbach, "Parsimonious Graphs" (1998)
