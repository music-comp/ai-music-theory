---
concept: PLR Transformations
category: technique
pdf_page: null
chapter_number: null
unit: null
source: "The Oxford Handbook of Neo-Riemannian Music Theories"
chapter: "Klangvertretung & Tone Representation"
authors: "Clark"
---

# PLR Transformations

## Quick Definition

The three fundamental neo-Riemannian operations (Parallel, Leittonwechsel, Relative) that transform major and minor triads by moving a single voice by semitone or whole tone while preserving two common tones.

## Formal Definition

The **PLR transformations** are contextual, mode-reversing operations on consonant triads:

### P (Parallel)
- **Common tones**: Perfect fifth (e.g., C and G)
- **Displaced tone**: Third moves by semitone (E to Eb or vice versa)
- **Example**: C major (C-E-G) ↔ C minor (C-Eb-G)
- Connects parallel major/minor on the same root

### L (Leittonwechsel / Leading-Tone Exchange)
- **Common tones**: Minor third (e.g., E and G)
- **Displaced tone**: Root moves by semitone (C to B or vice versa)
- **Example**: C major (C-E-G) ↔ E minor (B-E-G)
- The "leading tone exchange" moves the root to/from a leading tone

### R (Relative)
- **Common tones**: Major third (e.g., C and E)
- **Displaced tone**: Fifth moves by whole tone (G to A or vice versa)
- **Example**: C major (C-E-G) ↔ A minor (A-C-E)
- Connects relative major/minor keys

## Mathematical Properties

### Voice-Leading Parsimony
PLR operations exemplify **parsimonious voice leading**:
- P and L: One voice moves by semitone (1 semitone total displacement)
- R: One voice moves by whole tone (2 semitones total displacement)

These are the **maximally smooth** transformations between consonant triads.

### Group Structure
The PLR operations generate a group with presentation:
⟨L, P, R; L², P², R², (LP)³, (PR)⁴, (LR)¹²⟩

Key group properties:
- Order 24 (acts on all 24 major/minor triads)
- All three operations are involutions (self-inverse): P² = L² = R² = identity
- Isomorphic to the Schritt/Wechsel group

### Binary Compounds
| Compound | Effect | Cycle Length |
|----------|--------|--------------|
| LP (or PL) | Hexatonic poles | 3 (creates hexatonic cycle) |
| PR (or RP) | Minor third related | 4 (creates octatonic cycle) |
| LR (or RL) | Dominant/Subdominant | 12 (chromatic circle) |

Note: LR from C+ yields G+ (dominant direction); RL yields F+ (subdominant direction).

## Origin in Riemann's Writings

### Figure 3 from "Ideen" (1914-1915)
Riemann's late article presented three pairs of triads illustrating interval relationships:
1. C major / C minor (P relationship - perfect fifth common tones)
2. C major / A minor (R relationship - major third common tones)
3. C major / E minor (L relationship - minor third common tones)

Riemann observed that imagining any perfect fifth, major third, or minor third yields exactly **one major and one minor triad** sharing those tones.

### Terminological Note
- Riemann's "Parallele" = modern "Relative" (R)
- Riemann's "Variante" = modern "Parallel" (P)
- "Leittonwechsel" = same in both systems (L)

## Analytical Applications

### Chromatic Progressions
PLR analysis reveals coherence in chromatic progressions that resist functional analysis:
- Wagner's *Schlafakkorde*: LP compounds
- Late Romantic mediant chains: L and R progressions
- Schubert's third relations: Multiple PLR pathways

### Tonnetz Navigation
On the Tonnetz:
- P: Vertical move (same column)
- L: Diagonal move
- R: Horizontal move
- Compounds trace specific geometric paths

### Functional Independence
Unlike traditional theory, PLR operations:
- Describe relationships **independent of key**
- Do not require tonic reference
- Provide coherence through **transformational logic** rather than functional syntax

## Relationship to Harmonieschritte

Neo-Riemannian PLR operations correspond to specific Schritte/Wechsel:
- P = Seitenwechsel (mode change on same Hauptton)
- R = Terzwechsel (relative relationship)
- L = Leittonwechsel (leading-tone exchange)

However, the systems differ in:
1. **Generative emphasis**: PLR privileges parsimonious voice leading; Schritte privileges root intervals
2. **Compound status**: A *Quintschritt* (fifth relation) requires LR compound in PLR system
3. **Derivational distance**: PLR groups by voice-leading steps; Schritte by acoustic intervals

## Kopp's Critique and Alternative

David Kopp argued that PLR compounds for fifth relations (LR for dominant) misrepresent their directness. His alternative includes direct transformations for all common-tone relations:
- D (Dominant), S (Subdominant)
- M/m (Mediant types)
- F (Flat relations)
- All derivable from single common-tone groups

## Related Concepts

- **Prerequisite**: triad, interval, voice-leading, common-tones
- **Leads to**: hexatonic-systems, tonnetz, neo-riemannian-operations, maximally-smooth-cycles
- **See also**: harmonieschritte, schritt-wechsel-system, chromatic-transformation-networks

## Common Confusions

- **P, L, R are contextual**: Their effect depends on the triad quality they act upon
- **Not the same as transposition**: PLR operations change mode, not just pitch level
- **Compounds aren't "two steps"**: LP can be conceived as a single transformation with independent identity
- **Riemann's terminology differs**: His "Parallele" is our "Relative"

## Source References

- Oxford Handbook of Neo-Riemannian Music Theories, Part 3
- Ch 10: Suzannah Clark, "Klangvertretung"
- Ch 11: Richard Cohn, "Tonnetz as Analytical Apparatus"
- Ch 12: Nora Engebretsen, "Harmonieschritte"
- Cohn, "Neo-Riemannian Operations, Parsimonious Trichords" (1997)
