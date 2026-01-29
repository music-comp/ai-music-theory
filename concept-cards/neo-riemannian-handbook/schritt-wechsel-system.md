---
concept: Schritt/Wechsel System (S/W System)
tier: 2
category: transformational-theory
sources:
  - Ch 12 (Engebretsen): Harmonieschritte & Neo-Riemannian Appropriations
  - Ch 13 (Gollin): Doppelklänge & Transformational Analysis
part: 3
---

# Schritt/Wechsel System (S/W System)

## Quick Definition

The complete group of 24 transformations on consonant triads, comprising 12 mode-preserving Schritte (steps) and 12 mode-reversing Wechsel (exchanges), isomorphic to the PLR group but organized by root-interval relationships.

## Formal Definition

The **Schritt/Wechsel system** formalizes Riemann's *Harmonieschritte* as a mathematical group:

### Schritte (Steps)
Mode-preserving transformations:
- Major triad → Major triad
- Minor triad → Minor triad
- Named by root interval (0-11 semitones)

**Notation**: Sₙ where n = semitones of root motion in the direction of chord generation
- S₀ = Identity
- S₇ = Quintschritt (up fifth for major, down fifth for minor)
- S₄ = Terzschritt (up major third / down major third)
- S₂ = Ganztonschritt (up whole tone / down whole tone)

### Wechsel (Exchanges)
Mode-reversing transformations:
- Major triad → Minor triad
- Minor triad → Major triad
- Named by root interval between source and target

**Notation**: Wₙ where n = semitones between roots
- W₀ = Seitenwechsel (P in PLR notation)
- W₄ = Terzwechsel (R in PLR notation)
- W₃ = Leittonwechsel (L in PLR notation)
- W₇ = Quintwechsel

## Klumpenhouwer's Recovery

Henry Klumpenhouwer (1994) revived the S/W system as:
- A corrective to Lewin/Hyer's "conflicted" mixing of dualist and monist operations
- A fully dualized system with consistent behavior
- A historically grounded alternative to PLR

### The Conflation Problem
In Lewin/Hyer's original system:
- P, L, R are dualist (effect depends on chord quality)
- D (Dominant) is monist (always moves up fifth regardless of quality)

This mixing produces counterintuitive results:
- LR applied to major = subdominant direction
- LR applied to minor = dominant direction

The S/W system avoids this by being fully dualist throughout.

## Group Structure

### Presentation
⟨S₁, W₀ | S₁¹² = W₀² = (S₁W₀)² = e⟩

Or equivalently with different generators:
⟨⊕, Q, T | ⊕² = (Q⊕)² = (T⊕)² = QT-TQ = e⟩

where ⊕ = Seitenwechsel, Q = Quintschritt, T = Terzschritt

### Order and Structure
- Total operations: 24
- 12 Schritte (including identity)
- 12 Wechsel
- Isomorphic to dihedral group D₁₂ or Z₁₂ ⋊ Z₂

### Isomorphism with PLR Group
Every S/W operation has a PLR equivalent:
| S/W | PLR Equivalent |
|-----|----------------|
| W₀ | P |
| W₃ | L |
| W₄ | R |
| S₇ | LR |
| S₅ | RL |
| S₄ | LPL or RPR |
| W₈ | LP or PL |

## Combinatorial Foundation

### Generators
The complete group can be generated from:
- Q (Quintschritt): Fifth relation
- T (Terzschritt): Major third relation
- ⊕ (Seitenwechsel): Mode exchange

### Derivation
All 24 operations derive from combinations:
- S₂ = Q² (two fifths = whole tone)
- W₇ = Q⊕ (fifth then mode change)
- S₆ = Q³T (tritone = three fifths plus third)

### Word Length
Measuring "distance" by generator count:
- Primary operations (⊕, Q, Q⁻¹, T, T⁻¹): Length 1
- Compounds (Q², Q⊕, T⊕...): Length 2
- Further compounds: Length 3+

## Comparison: S/W vs. PLR

| Feature | S/W System | PLR System |
|---------|------------|------------|
| Basis | Root interval | Voice leading |
| Generators | Q, T, ⊕ | P, L, R |
| Fifth relation | Direct (S₇) | Compound (LR) |
| Parsimony emphasis | No | Yes |
| Historical basis | Riemann | Neo-Riemannian |
| Distance metric | Root motion | Voice-leading steps |

### Kopp's Critique
David Kopp noted that PLR makes fifth relations (like dominant) seem "indirect" (requiring LR compound), obscuring their acoustic and perceptual directness. The S/W system preserves this directness.

## Analytical Implications

### When S/W is Preferable
- Analyzing root-motion patterns
- Passages emphasizing acoustic relationships
- Historical reconstruction of 19th-century hearing
- Comparison with function theory

### When PLR is Preferable
- Voice-leading analysis
- Maximally smooth cycles
- Hexatonic/octatonic spaces
- Passages emphasizing parsimonious motion

### Hybrid Approaches
Many analysts use both:
- S/W for classifying relationships
- PLR for tracing voice-leading paths
- The isomorphism ensures compatibility

## Related Concepts

- **Prerequisite**: Harmonieschritte, PLR-transformations, group-theory
- **Leads to**: neo-Riemannian-operations, chromatic-transformation-networks
- **See also**: Klang, harmonic-dualism, root-interval

## Common Confusions

- **S/W is not "better" than PLR**: They're isomorphic; preference depends on analytical goals
- **Riemann didn't formalize the group**: Klumpenhouwer extracted group structure from Riemann's writings
- **Schritte preserve mode, Wechsel reverse it**: This is the fundamental distinction

## Source References

- Oxford Handbook of Neo-Riemannian Music Theories, Part 3
- Ch 12: Nora Engebretsen, "Harmonieschritte & Neo-Riemannian Appropriations"
- Klumpenhouwer, "Some Remarks on the Use of Riemann Transformations" (1994)
- Hook, "Uniform Triadic Transformations" (2002)
