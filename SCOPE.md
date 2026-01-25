# SCOPE.md — Music Theory Skill

## Purpose

This skill provides Claude with comprehensive knowledge of modern mathematical
music theory, enabling support for:

1. **Analysis** — Understanding the structure of existing music across all
   Western style periods
2. **Composition** — Generating possibilities, exploring "what if?" questions,
   finding voice leadings, working with collections
3. **Theory exploration** — Deep dives into the mathematical foundations of
   musical phenomena
4. **Pedagogy** — Explaining concepts at appropriate levels, connecting
   abstract structures to musical intuition

## Intellectual Orientation

### Core Lineage

- **David Lewin** — Foundational: Generalized Interval Systems, transformation
  graphs, the "transformational attitude"
- **Dmitri Tymoczko** — Central: Geometric voice-leading theory, orbifolds,
  scalar collections, the "extended common practice"
- **Richard Cohn** — Key ally: Neo-Riemannian theory, hexatonic systems,
  parsimonious voice leading

### Important Bridge Figure

- **Olivier Messiaen** — Not a theorist in the Lewin/Tymoczko sense, but:
  - Modes of limited transposition are pitch-class sets invariant under translation
  - Non-retrogradable rhythms are temporal symmetry
  - Symmetrical permutations are group theory in action
  - Bridges "working composer" and "systematic thinker"
  - See Papadopoulos (2014) for formalization

### Guiding Principles

1. **Mathematical rigor** — Precise definitions, correct proofs, honest about
   limitations
2. **Musical grounding** — Theory serves music, not vice versa; examples from
   real repertoire
3. **Intellectual honesty** — Acknowledge debates, don't paper over
   disagreements, distinguish established results from speculation
4. **Compositional utility** — Not just "what is?" but "what can I do with
   this?"

### The Inquiry Pattern

The skill should support this pattern of investigation:

```
Musical phenomenon → Pattern recognition → Mathematical structure → 
Generalization → Return to music
```

Not just "what is X?" but "what does X connect to?" and "how do I use X?"

### Explicitly Out of Scope

- **Mazzola's topos-theoretic apparatus** — Impressive but problematic; see
  Tymoczko's critique (weak arguments, mathematical contradictions, failed
  predictions). We may reference individual concepts but don't build on this
  foundation.
- **Speculative extensions** — We go deep, but stay grounded. Category theory
  and homotopy theory appear where Tymoczko points toward them, but we don't
  venture into uncharted territory without clear musical motivation.

## Stylistic Scope

### Covered Periods/Styles

- Renaissance polyphony
- Baroque (Bach, counterpoint traditions)
- Classical (Mozart, Haydn, sonata forms)
- Romantic (Schubert, Wagner, chromatic harmony)
- Early 20th century (Debussy, Ravel, extended tonality)
- Atonal (Schoenberg, Webern, Berg pre-serial)
- Serial/12-tone
- Minimalism (Reich, Glass, Adams)
- Post-tonal/contemporary
- Messiaen and spectralism

### Style-Awareness

Different theoretical tools have different applicability:

| Tool | Primary Application |
|------|---------------------|
| Voice-leading geometry | Nearly universal |
| Functional harmony | Baroque through late Romantic |
| Neo-Riemannian | Romantic chromaticism, film music |
| Set theory | Atonal and serial |
| Scalar/collectional | Broad applicability |
| Modes of limited transposition | Messiaen, some impressionism |

Guides should indicate which tools apply to which contexts using style-period
tags (see CONVENTIONS.md).

## Mathematical Depth

### Assumed Foundations (via abstract-algebra-topology skill)

- Set theory basics
- Group theory (groups, subgroups, cosets, quotients, group actions)
- Basic topology (spaces, continuity, compactness)
- Metric spaces

### Developed Within This Skill

- Pitch-class spaces as quotient structures
- Voice-leading spaces as orbifolds
- Transformation groups (T/I, PLR, UTTs)
- Generalized Interval Systems
- Geometric models (Tonnetz, Cube Dance, chord space)

### Horizon (Referenced But Not Fully Developed)

- Category-theoretic formulations
- Homotopy-theoretic perspectives
- Fourier analysis on pitch-class sets

## Validation Criteria

### The Quartal/Quintal Test

The skill should be able to support a deep exploration of quartal/quintal
harmony, including:

- Generator-based construction of quartal/quintal collections
- Duality between quartal and quintal (related by inversion in Z₁₂)
- Supporting scales and why they work
- Connection to modes of limited transposition (non-generator cases)
- Group-theoretic foundations
- Compositional applications

If the skill materials can support this inquiry journey — from musical
phenomenon through mathematical structure to compositional application —
the skill is working.

### The Messiaen Test

Can the skill support analysis of:
- Modes of limited transposition (as Z₁₂ subgroups)
- Non-retrogradable rhythms (as palindromic sequences)
- Symmetrical permutations (as permutation groups)

### Novel Query Test

The skill should handle questions not explicitly covered in sources:
- "What are the voice-leading options from this chord to that chord?"
- "How does Cohn's hexatonic system relate to Tymoczko's orbifolds?"
- "Generate a chord progression using only parsimonious voice leading"

## Versioning

| Version | Content | Bump Criteria |
|---------|---------|---------------|
| v0.1 | OMT concept cards, basic MCP | First usable state |
| v0.2 | + Lewin concepts | Foundational theory complete |
| v0.3 | + Both Tymoczko books | Geometric theory integrated |
| v0.4 | + Cohn, unified cards | Neo-Riemannian integrated |
| v0.5 | DEBATES.md, INDEX.md | Cross-referencing complete |
| v0.6 | Initial guide set | Guides cover fundamentals |
| v0.7 | Full guide coverage | All major topics have guides |
| v0.8 | SKILL.md, full MCP | Entry point complete |
| v0.9 | Validation tests pass | Quality verified |
| v1.0 | Polish, documentation | Release ready |

### Version Bump Criteria

- **Patch** (0.1.1): Bug fixes, typo corrections, minor clarifications
- **Minor** (0.2.0): New concept cards, new guide, expanded coverage
- **Major** (1.0.0): Validation complete, production ready

## License

CC0 — Public Domain Dedication

All original content in this skill is released to the public domain. Source
materials retain their original licenses (noted in SOURCES.md).
