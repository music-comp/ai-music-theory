---
concept: Partial Ordering in Serial Theory
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
A partial ordering is a subset of PROT that captures "some" precedence relations without necessarily determining a complete row - useful for modeling aggregates and incomplete serial structures.

# Formal Definition
A subset X of PROT is a (strict) partial ordering if it satisfies:
- (PO1): There is no (p, q) in PROT such that X contains both (p, q) and (q, p)
- (PO2): If (p, q) and (q, r) are in X, then so is (p, r) [transitivity]

Partial orderings that also satisfy (SIMP) - for any (p, q), either (p, q) or (q, p) is in X - are called linear orderings and correspond to twelve-tone rows.

# Mathematical Formulation
Partial ordering X subset of PROT:
- Irreflexive: (p, p) never in X (by PROT definition)
- Antisymmetric: Not both (p, q) and (q, p) in X (PO1)
- Transitive: (p, q) and (q, r) in X implies (p, r) in X (PO2)

Linear ordering L (row) adds:
- Totality (SIMP): For all distinct p, q, either (p, q) or (q, p) in L

A partial ordering represents "knowing" some precedence relations while being "uncertain" or "indifferent" about others.

# Musical Context/Application
Partial orderings model structures where complete serial ordering is not specified:
- SATB aggregates where each voice has internal ordering but cross-voice relations are unspecified
- Motivic fragments extracted from rows
- Compositional situations with local ordering but global flexibility

# Examples
From Figure 6.7:

X1 = {(E, A), (E, Bb), (A, Bb)}
- Models the linear motive E-A-Bb
- Contains 3 pairs, satisfies PO1 and PO2
- Is a partial ordering (and happens to be linear on its 3 elements)

X2 models SATB aggregate from Semi-Simple Variations:
- Soprano: B-D-Eb (3 pairs)
- Alto: G-Bb-F (3 pairs)
- Tenor: E-C#-F# (3 pairs)
- Bass: C-A-Ab (3 pairs)
- Total: 12 pairs (no cross-voice orderings)

X2 satisfies PO1 and PO2 but not SIMP (doesn't order all pairs).

INJ(L, X2)(f) measures how well the ordering X2 "fits within" row form f(L).

# Related Concepts
- Protocol Pairs (PROT)
- Linear Ordering (Rows)
- Twelve-Tone Row
- Aggregate Structure
- INJ (Injection Function)

# Common Confusions
A partial ordering is not a "partial row" - it's a set of precedence relations that may not determine a unique row completion. Many different rows might be compatible with a given partial ordering. This models compositional situations with constrained but not fully determined serial structure.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Example 6.2.4
