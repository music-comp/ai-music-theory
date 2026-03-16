---
concept: Intervals as Transpositions
slug: intervals-as-transpositions

category: transformation-theory
subcategory: simply-transitive-groups
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (1): Intervals and Transpositions"
chapter_number: 7
pdf_page: 188
section: null

extraction_confidence: high

aliases:
  - "interval-transposition equivalence"

prerequisites:
  - simply-transitive-group
  - generalized-interval-system
  - inj-function
extends:
  - generalized-interval-system
related:
  - gis-from-simply-transitive-group
  - transformational-attitude
  - generalizing-power-of-transformational-theory
contrasts_with:
  - cartesian-versus-transformational-thinking

answers_questions:
  - "How do transpositions relate to intervals in a GIS?"
  - "Can intervals and transpositions be understood as the same phenomenon?"
---

# Quick Definition
The reconception that intervals are not merely measurements between elements but can be understood as the unique transposition operations that map one element to another, established through the relationship IFUNC(X, Y)(i) = INJ(X, Y)(T_i).

# Core Definition
Instead of thinking "i is the intervallic distance from s to t," we can think "T_i is the unique transposition operation on this space that maps s into t." The equivalence is mediated by INJ: "IFUNC(X, Y)(i) = INJ(X, Y)(T_i)" (Lewin, Ch. 7 opening, p. 157). We can shift attention from atomic "points" s and t to one-element Gestalts X and Y; then there is a unique T_i satisfying INJ(X, Y)(T_i) > 0, with label i = int(s, t).

# Prerequisites
- **Simply transitive group** — The uniqueness of T_i depends on the transposition group being simply transitive
- **Generalized interval system** — Provides the framework of intervals and the int function
- **INJ function** — Mediates the formal equivalence between IFUNC and transpositions

# Key Properties
1. For any s and t in the space of a GIS, there is a unique T_i with T_i(s) = t, namely T = T_{int(s,t)}
2. IFUNC(X, Y)(i) = INJ(X, Y)(T_i) generalizes this equivalence to sets
3. The arrow labeled "i" on a network can represent either the interval i or the transposition T_i
4. This equivalence enables replacing GIS structure entirely with simply transitive group structure
5. Intervals between individual elements and transpositional relations between Gestalts become "the same phenomenon manifested in different ways"

# Construction / Recognition
## To Construct:
1. Given elements s and t in a GIS, compute i = int(s, t)
2. The unique transposition T_i satisfies T_i(s) = t
3. The label i on an arrow can be read as interval or as transposition index
## To Recognize:
1. When the same numerical label appears on both an intervallic network and a transpositional network, the two networks manifest the same underlying phenomenon

# Context & Application
This reconception enables the subsumption of interval theory into transformation theory. Lewin emphasizes that both perspectives are equally valid: "We do not have to choose either interval-language or transposition-language; the generalizing power of transformational theory enables us to consider them as two aspects of one phenomenon" (p. 160).

# Examples
**Example 1** (Figure 7.1, pp. 159-160): In Schoenberg's op. 19, no. 6, the opening chord "rh" contains pitch-intervals -5, -9, -14. The falling-ninth motif transposes through the piece by T_{-5}, T_{-9}, T_{-14}. The symbol "-5" in both contexts points at "the same phenomenon, not at different phenomena."

**Example 2** (Figure 7.4, p. 163): In Wagner's Parsifal, the intervals of the Zauber motive and the transpositions among local tonics in the transformation music are manifestations of one phenomenon: "Intervals structure the referential sonority... transpositions make the falling-ninth motif move forward through the piece."

# Relationships
## Builds Upon
- **Generalized interval system** — Intervals are defined within a GIS
- **Simply transitive group** — Uniqueness of the transposition depends on this property
## Enables
- **Transformational attitude** — Provides the formal basis for the shift from measurement to gesture
- **Generalizing power of transformational theory** — This equivalence is the foundation for subsumption
## Related
- **INJ function** — Mediates the equivalence: IFUNC(X,Y)(i) = INJ(X,Y)(T_i)
## Contrasts With
- **Cartesian versus transformational thinking** — Two conceptual framings of this same formal equivalence

# Common Errors
- **Error**: Labeling arrows with specific transposition numbers (T_10, T_3) when TCH or RICH would be more revealing
  **Correction**: Use the transformation that reveals structural relationships; fixed transposition numbers may obscure isography

# Common Confusions
- **Confusion**: Thinking intervals and transpositions are identical
  **Clarification**: They are two manifestations of one underlying phenomenon, not the same thing in different notation
- **Confusion**: Believing this eliminates the usefulness of interval language
  **Clarification**: Lewin explicitly maintains both languages; the point is that they can be unified, not that one should replace the other

# Source Reference
Chapter 7: Transformation Graphs and Networks (1): Intervals and Transpositions, opening paragraphs and Section 7.1.2, pages 157-160.

# Verification Notes
- Definition source: Direct from Ch. 7 opening paragraphs
- Confidence rationale: Explicit formal statement with INJ connection
- Re-extraction notes: Re-extracted from v2 card; preserved: Schoenberg example, core equivalence statement
