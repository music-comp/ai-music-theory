---
concept: Transitivity Sets
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
Transitivity sets partition S into orbits under an operation, grouping elements that transform among themselves - useful for tracking "partnerships" under inversion or other operations.

# Formal Definition
Given an operation OP on S, a transitivity set is a minimal non-empty subset T of S such that OP maps T into itself. The operation OP permutes the elements of T among themselves.

For inversion I about axis u:
- {u} is a transitivity set (u maps to itself)
- For v != u with I(v) != v, {v, I(v)} is a transitivity set (a "partnership")

# Mathematical Formulation
The transitivity sets of OP partition S:
- Every element belongs to exactly one transitivity set
- OP(T) = T for each transitivity set T
- T is minimal: no proper non-empty subset T' of T has OP(T') = T'

For I = I_E^Bb (inversion about E and Bb) on pitch classes:
- {E} - singleton (E maps to E)
- {Bb} - singleton (Bb maps to Bb)
- {Eb, F} - partnership
- {D, F#} - partnership
- {C#, G} - partnership
- {C, Ab} - partnership
- {B, A} - partnership

# Musical Context/Application
Transitivity sets reveal the "structure" of an operation. For inversions, they show which notes are partners. When analyzing progressions, we can track how entire transitivity sets (partnerships) transform under other operations like wedges.

# Examples
From "Angst und Hoffen" (Figure 6.3):

I = I_E^Bb partitions pitch classes into transitivity sets:
- (Bb): fixed
- (A, B): partnership
- (Ab, C): partnership
- (G, C#): partnership
- (Gb, D): partnership
- (F, Eb): partnership
- (E): fixed

Analysis insight: In the chord progression, we can track whether entire I-partnerships are preserved or broken. When Fb appears instead of F, the partnership (F, Eb) is broken - Eb is "bereft of its I-partner."

The wedge w^E and inversion I commute. This means w^E maps I-partnerships to I-partnerships. Figure 6.3(b) shows entire partnerships wedging together: (Ab, C) wedges to (G, C#) in the Z1->Z2 progression.

# Related Concepts
- Inversion Operations
- INJ (Injection Function)
- Operation Orbits
- Partnership (Inversional)

# Common Confusions
Transitivity sets are specific to an operation. The same pitch class may belong to different transitivity sets for different operations. The term "transitivity" comes from group theory, where it describes how a group action partitions a set into orbits.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, discussion following Figure 6.3
