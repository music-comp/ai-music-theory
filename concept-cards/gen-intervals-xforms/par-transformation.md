---
concept: PAR Transformation
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
unit: null
authors: David Lewin
---

# Quick Definition
A Klang transformation that takes any Klang into its parallel major/minor, changing only the mode while preserving the root: (C, +)PAR = (C, -) and (C, -)PAR = (C, +).

# Formal Definition
PAR is defined as:
- (p, sign)PAR = (p, -sign)
- Changes mode while preserving pitch class
- PAR is its own inverse: (PAR)(PAR) = identity

# Mathematical Formulation
PAR transformation:
- (p, +)PAR = (p, -)
- (p, -)PAR = (p, +)
- PAR^2 = identity (PAR is an involution)

PAR differs from MED^7:
- In some systems of intonation, MED^7 applied to a major Klang yields the parallel minor
- However, MED^7 applied to a minor Klang does NOT yield parallel major
- Example: (C, -)MED^7 = (Cb, +), not (C, +)

# Musical Context/Application
PAR captures the parallel major/minor relationship fundamental to tonal music. The fact that PAR and MED^7 are different operations (despite sometimes producing the same result) has important implications: graphs using both PAR and SUBM cannot be "formally intervallic" because no simply transitive group can contain both.

# Examples
Applications:
- (C, +)PAR = (C, -): C major to C minor
- (F#, -)PAR = (F#, +): F# minor to F# major

The difference from MED^7:
- (C, +)MED^7 = (C, -) [same as PAR in this case]
- (C, -)MED^7 = (Cb, +) [different from (C, +)]

This shows PAR is genuinely different from any power of MED.

# Related Concepts
- Klang Representation
- REL Transformation
- MED Transformation
- Simply Transitive Group (for Klangs)
- Non-Intervallic Transformations

# Common Confusions
- PAR only changes mode; the root stays the same
- PAR is not equal to MED^7 (or any power of MED) as an operation
- The impossibility of a simply transitive group containing both PAR and MED powers means some Klang networks are genuinely non-intervallic
- PAR is an involution (its own inverse)

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.1.1 and 8.1.2
