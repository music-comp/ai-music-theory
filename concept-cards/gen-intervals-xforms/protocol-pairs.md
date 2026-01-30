---
concept: Protocol Pairs (PROT)
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
PROT is the space of protocol pairs - ordered pairs (p, q) of distinct pitch classes - providing a model for twelve-tone rows as unordered sets of precedence relations.

# Formal Definition
Definition (6.2.4): A protocol pair is an ordered pair (p, q) of distinct chromatic pitch classes. The family PROT contains all 132 = 12 x 11 such pairs.

A twelve-tone row can be represented as a subset of PROT: the pair (p, q) belongs to the set if and only if p precedes q in the row.

# Mathematical Formulation
PROT = {(p, q) : p, q in pitch classes, p != q}
|PROT| = 12 * 11 = 132

Row as subset L of PROT:
(p, q) in L iff p appears before q in the row

A row L is a "linear ordering" satisfying:
- (SIMP): For any (p, q) in PROT, either (p, q) or (q, p) is in L
- (PO1): Never both (p, q) and (q, p) in L
- (PO2): If (p, q) and (q, r) in L, then (p, r) in L

Each row contains exactly 66 protocol pairs (11 + 10 + ... + 1).

Operations on PROT:
- T_i(p, q) = (T_i(p), T_i(q))
- I(p, q) = (I(p), I(q))
- R(p, q) = (q, p) [retrograde]

# Musical Context/Application
This model treats all rows as equals - no privileged "chromatic row" from which others derive. It captures the ordering information of a row without imposing any external structure. Partial orderings (subsets of PROT satisfying PO1 and PO2 but not SIMP) model incomplete orderings like aggregate structures.

# Examples
Row of Schoenberg's Fourth Quartet: D-C#-A-Bb-...
As subset of PROT, contains:
- (D, C#), (D, A), (D, Bb), ...
- (C#, A), (C#, Bb), ...
- (A, Bb), ...
- All 66 pairs where earlier note precedes later

Notation: D-C#-A-Bb-... remains convenient for quick reference.

Partial ordering example (Figure 6.7):
- X1 = E-A-Bb (small motive) contains 3 pairs: (E, A), (E, Bb), (A, Bb)
- X2 models SATB aggregate, containing 12 pairs

INJ(L, X1)(J) = 3 means the J-inverted row contains all three precedence pairs of motive X1. This identifies X1 as a "signature motive" for that row form.

# Related Concepts
- Twelve-Tone Row
- Partial Ordering
- Linear Ordering
- INJ (Injection Function)
- Row Transformations

# Common Confusions
The retrograde operation R swaps pairs: R(p, q) = (q, p). If L is a row, R(L) = complement of L in PROT. This gives a beautiful connection: row and retrograde as complement sets, like hexachord and complement in traditional theory.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Example 6.2.4
