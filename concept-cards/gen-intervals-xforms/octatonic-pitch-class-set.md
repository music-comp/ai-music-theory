---
concept: Octatonic Pitch-Class Set
slug: octatonic-pitch-class-set

category: generalized-interval-systems
subcategory: octatonic-structures
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups"
chapter_number: null
pdf_page: 282
section: null

extraction_confidence: high

aliases:
  - "octatonic collection"
  - "octatonic set S"

prerequisites:
  - simply-transitive-group
extends: []
related:
  - strans1-group
  - strans2-group
  - gis1-octatonic
  - gis2-octatonic
contrasts_with: []

answers_questions:
  - "What must I know before understanding the octatonic GIS?"
  - "What is the octatonic pitch-class set used in Lewin's Appendix B?"
---

# Quick Definition
The eight-note collection S = {C, C#, D#, E, F#, G, A, A#}, comprising two interlocking diminished-seventh chords, which supports two distinct non-commutative GIS structures through dual simply transitive groups.

# Core Definition
"Let S be the octatonic family of pitch classes comprising C, C#, D#, E, F#, G, A, and A#. Eight of the standard 'atonal' operations on the twelve pitch-classes transform S into itself; these operations are T_0, T_3, T_6, T_9, I_5, I_E, I_8, and I_{A#}" (Lewin, Appendix B, p. 251). These eight operations form a group that is simply transitive on S, becoming STRANS1. A second simply transitive group STRANS2 also exists on S, creating dual GIS structures.

# Prerequisites
- **Simply transitive group** — Both STRANS1 and STRANS2 are simply transitive on S

# Key Properties
1. S = {C, C#, D#, E, F#, G, A, A#} (8 pitch classes)
2. Two diminished-seventh chords: {C, D#, F#, A} and {C#, E, G, A#}
3. Eight standard atonal operations preserve S: T_0, T_3, T_6, T_9, I_5, I_E, I_8, I_{A#}
4. These operations form a group isomorphic to D_4 (dihedral group of order 8)
5. The group is simply transitive on S
6. Supports two distinct GIS structures (GIS1 and GIS2) via dual groups

# Construction / Recognition
## To Construct:
1. Start with any pitch class
2. Alternate whole steps and half steps (or half and whole)
3. The resulting 8-note collection is an octatonic set
## To Recognize:
1. Eight pitch classes forming two interlocking diminished-seventh chords
2. Alternating intervals of 1 and 2 semitones

# Context & Application
The octatonic collection is fundamental to music of Bartok, Stravinsky, Messiaen, and others. Its symmetric structure creates unique transformational possibilities that differ from the full chromatic. The dual GIS structures demonstrate that "the same musical space can support multiple equally valid GIS structures."

# Examples
**Example 1** (p. 251): S = {C, C#, D#, E, F#, G, A, A#}. Two dim7 chords: {C, D#, F#, A} and {C#, E, G, A#}.

**Example 2** (p. 252): Exercise: consider S' = {C, C#, E, F, G#, A} and develop analogous simply transitive groups (the hexatonic collection).

# Relationships
## Enables
- **STRANS1 group** — Simply transitive group on S from standard operations
- **STRANS2 group** — Alternative simply transitive group on S
- **GIS1 octatonic** — GIS with STRANS1 as transpositions
- **GIS2 octatonic** — GIS with STRANS2 as transpositions

# Common Errors
- **Error**: Assuming the octatonic collection has only one GIS structure
  **Correction**: It supports at least two distinct GIS structures via dual simply transitive groups

# Common Confusions
- **Confusion**: Equating the octatonic collection's 8 operations with 8 of the 24 standard operations
  **Clarification**: The operations are induced from standard operations but must be understood as acting on S specifically, not on the full chromatic

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups, page 251.

# Verification Notes
- Definition source: Direct quotation from Appendix B
- Confidence rationale: Explicitly defined
- Re-extraction notes: Re-extracted from v2 card; preserved: dim7 structure, hexatonic exercise, D_4 isomorphism
