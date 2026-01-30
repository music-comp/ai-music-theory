---
concept: Semi-Simple Variations INJ Analysis
category: analysis
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
Lewin's analysis of Babbitt's Semi-Simple Variations uses INJ on the protocol-pair space PROT to measure how SATB aggregates relate to row forms and to each other across variations.

# Formal Definition
Key structures:
- L2 = row of the piece (soprano voice, first 12 notes)
- X2 = SATB aggregate from opening of Variation 3, as subset of PROT
- V_n = generic SATB aggregates from variation n

INJ(L2, X2)(f) measures how well X2's ordering "fits" within row form f(L2).

# Mathematical Formulation
X2 contains 12 protocol pairs (3 per voice, no cross-voice orderings):
- Soprano: {(B, D), (B, Eb), (D, Eb)}
- Alto: {(G, Bb), (G, F), (Bb, F)}
- Tenor: {(E, C#), (E, F#), (C#, F#)}
- Bass: {(C, A), (C, Ab), (A, Ab)}

Key findings:
- INJ(L2, X2)(T1) = 11 (maximum achievable is 12)
- INJ(L2, X2)(RT1) = 11
- INJ(L2, X2)(J) = 11, where J = I_0^Bb
- No operation achieves 12

Variation 3 aggregates are "maximally compatible" with L2 forms compared to other variations.

# Musical Context/Application
The analysis reveals:
1. Variation 3 has special relationship to the row (11/12 fit)
2. Other variations have aggregates fitting at most 10/12
3. Cross-variation "ordering cross-talk" is low (INJ <= 2 between variations)
4. Exception: Variations 4 and 5 "talk with" Variation 2 at level 4-5

This structural differentiation helps analyze a piece that sounds "extremely homogeneous."

# Examples
From Figures 6.7-6.9:

Aggregate X2 fits 11/12 in rows T1(L2) and J(L2) (Figure 6.8):
- In T1(L2): only (C#, F#) of X2 is reversed (row has F#, C#)
- In J(L2): only (Bb, F) of X2 is reversed

"If only" the tenor went E-F#-C# instead of E-C#-F#, embedding would be perfect.

Cross-variation analysis:
- INJ(V_m, V_n)(T_0) <= 2 for different variations (low cross-talk)
- INJ(V_4, V_2)(T_0) = 5 (exceptional connection)
- V_4 and V_2 share 3-note linear segments from a "pivot aggregate"

Figure 6.9: The pivot aggregate {D#-B-E, Ab-C-G, C#-F#-D, Bb-F-A} controls tenor/bass of Var.2 and soprano/alto of Var.4, explaining their unique INJ connection.

# Related Concepts
- Protocol Pairs (PROT)
- Partial Ordering
- INJ (Injection Function)
- SATB Aggregate
- Signature Motive

# Common Confusions
High INJ values like 11/12 mean "almost embeds," not "embeds." The analysis uses "if-only" thinking to identify the specific pairs that prevent perfect embedding, revealing compositional structure in near-misses.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Example 6.2.4 (Semi-Simple Variations portion), Figures 6.7-6.9
