---
concept: PAR Transformation
slug: par-transformation

category: transformation-theory
subcategory: klang-operations
tier: advanced

source: "Generalized Musical Intervals and Transformations"
source_slug: gen-intervals-xforms
authors: David Lewin
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
section: "8.1.1"

extraction_confidence: high

aliases:
  - "parallel transformation"

prerequisites:
  - klang-representation
extends: []
related:
  - rel-transformation
  - lt-transformation
  - non-intervallic-transformations
  - tarnhelm-network
contrasts_with:
  - med-transformation

answers_questions:
  - "How do I apply PAR to Klangs?"
  - "Why does PAR make Klang networks non-intervallic?"
---

# Quick Definition
A Klang transformation that takes any Klang into its parallel major/minor, changing only the mode while preserving the root: (p, sign)PAR = (p, -sign). PAR is an involution (self-inverse).

# Core Definition
"We can also define PAR, the operation that takes any Klang into its parallel minor/major. (p, sign)PAR = (p, -sign)" (Lewin, 8.1.1, p. 177). PAR preserves pitch class and reverses mode. The critical insight is that PAR is not equal to any power of MED: MED^7 on (C, +) yields (C, -) [same as PAR], but MED^7 on (C, -) yields (Cb, +), not (C, +) (8.1.2, p. 179). This means any group containing both SUBM (or MED) and PAR cannot be simply transitive.

# Prerequisites
- **Klang representation** — PAR operates on Klangs

# Key Properties
1. (p, +)PAR = (p, -)
2. (p, -)PAR = (p, +)
3. PAR^2 = identity (involution)
4. PAR preserves root, changes mode only
5. PAR is not any power of MED (proven by the (C, -) case)
6. Any group containing both MED and PAR is not simply transitive
7. This makes graphs with both PAR and SUBM genuinely non-intervallic

# Construction / Recognition
## To Construct:
1. Take any Klang (p, sign)
2. Reverse the sign (+ becomes -, - becomes +)
3. Keep the root unchanged
## To Recognize:
1. Two Klangs sharing the same root but differing in mode
2. C major to C minor, or F# minor to F# major

# Context & Application
PAR is fundamental to tonal music (parallel major/minor relationships). Its combination with SUBM in the Tarnhelm and Valhalla networks (Figure 8.2) demonstrates genuinely non-intervallic structure: "We shall not be able to find a simply transitive group on a suitable family of Klangs that enables us to consider figure 8.2(a) and (b) as formally 'intervallic' graphs" because "given elements (C, +) and (C, -), both MED^7 and PAR would do the job" (8.1.2, p. 179).

# Examples
**Example 1** (p. 177): (C, +)PAR = (C, -): C major to C minor. (F#, -)PAR = (F#, +): F# minor to F# major.

**Example 2** (p. 179): MED^7 vs. PAR: (C, +)MED^7 = (C, -) = (C, +)PAR [agree], but (C, -)MED^7 = (Cb, +) while (C, -)PAR = (C, +) [differ].

# Relationships
## Builds Upon
- **Klang representation** — PAR is defined on Klangs
## Enables
- **Non-intervallic transformations** — PAR + SUBM prevents simple transitivity
- **Tarnhelm network** — Uses PAR alongside SUBM
- **Valhalla network** — Uses PAR alongside SUBM
## Related
- **REL transformation** — Fellow involutory operation
- **LT transformation** — Fellow member of PLR group
## Contrasts With
- **MED transformation** — PAR is not MED^7 as a general operation

# Common Errors
- **Error**: Assuming PAR = MED^7 because they agree on major Klangs
  **Correction**: They are different operations; they disagree on minor Klangs

# Common Confusions
- **Confusion**: Thinking PAR is a simple operation with no theoretical significance
  **Clarification**: PAR's incompatibility with simple transitivity (when combined with MED) is the key to understanding non-intervallic Klang networks

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Sections 8.1.1-8.1.2, pages 177-179.

# Verification Notes
- Definition source: Direct quotation from 8.1.1
- Confidence rationale: Explicitly defined with proof of non-equivalence with MED^7
- Re-extraction notes: Re-extracted from v2 card; preserved: MED^7 vs. PAR proof, non-intervallic significance
