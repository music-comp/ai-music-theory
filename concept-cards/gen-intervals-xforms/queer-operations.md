---
concept: "Queer Operations (Q3 and Q9)"
slug: queer-operations

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
  - Q3
  - Q9

prerequisites:
  - octatonic-pitch-class-set
extends: []
related:
  - strans2-group
  - exchanging-operations
contrasts_with:
  - rotation-operations

answers_questions:
  - "What are the queer operations Q3 and Q9?"
---

# Quick Definition
Operations in STRANS2 that rotate each diminished-seventh chord within the octatonic collection in opposite directions simultaneously: Q3 rotates one chord forward and the other backward, while Q9 = Q3^{-1} reverses the directions.

# Core Definition
"Q3 rotates each of the diminished-seventh chords within S, but in opposite directions; it maps C to D#, D# to F#, F# to A, A to C, and also C# to A# (not to E), A# to G, G to E, and E to C#. Q9 is the inverse operation to Q3; it maps C to A,..., D# to C, and also C# to E,..., and A# to C#" (Lewin, Appendix B, p. 251).

# Prerequisites
- **Octatonic pitch-class set** — Q3 and Q9 operate on the octatonic collection

# Key Properties
1. Q3 rotates dim7 #1 (C, D#, F#, A) forward by minor third
2. Q3 simultaneously rotates dim7 #2 (C#, E, G, A#) backward by minor third
3. Q9 = Q3^{-1} (reverses both rotation directions)
4. Neither Q3 nor Q9 is a standard T or I operation
5. Members of STRANS2 (not STRANS1)
6. Commute with all STRANS1 operations

# Construction / Recognition
## To Construct:
1. Rotate one diminished-seventh chord forward by 3 semitones
2. Simultaneously rotate the other backward by 3 semitones
## To Recognize:
1. An operation that maps within each dim7 chord but in opposite directions

# Context & Application
The "queer" operations have no standard twelve-tone counterpart, representing genuinely novel transformational possibilities specific to octatonic space.

# Examples
**Example 1** (p. 251): Q3 mappings: C->D#, D#->F#, F#->A, A->C (forward); C#->A#, A#->G, G->E, E->C# (backward).

# Relationships
## Related
- **STRANS2 group** — Q3, Q9 are members
- **Exchanging operations** — Fellow STRANS2 members
## Contrasts With
- **Rotation operations** — Rotations move both dim7 chords in the same direction; queer operations move them in opposite directions

# Common Errors
- **Error**: Trying to express Q3 as a standard T or I operation
  **Correction**: Q3 has no standard twelve-tone equivalent

# Common Confusions
- **Confusion**: Thinking Q3 is the same as R3
  **Clarification**: R3 rotates both dim7 chords forward; Q3 rotates them in opposite directions

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups, page 251.

# Verification Notes
- Definition source: Direct quotation from Appendix B
- Confidence rationale: Explicitly defined with complete mapping
- Re-extraction notes: Re-extracted from v2 card; preserved: complete Q3 mapping, contrast with R3
