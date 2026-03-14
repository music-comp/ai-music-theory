---
concept: LABEL Function
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
The LABEL function maps elements of a GIS space S to elements of the interval group IVLS by measuring the interval from a fixed reference point to each element.

# Formal Definition
Given a GIS (S, IVLS, int) and a fixed referential member "ref" of S, the function LABEL, mapping S into IVLS, is defined by the equation:

LABEL(s) = int(ref, s)

The LABEL function maps S one-to-one onto IVLS, and it satisfies the formula:
int(s, t) = LABEL(s)^(-1) LABEL(t)

# Mathematical Formulation
**Definition 3.1.1:**
LABEL(s) = int(ref, s)

**Theorem 3.1.2:**
- LABEL is a bijection (1-to-1 and onto) from S to IVLS
- int(s, t) = LABEL(s)^(-1) LABEL(t)

**Proof outline:**
Given any interval i, there exists a unique s in S satisfying int(ref, s) = i by Condition (B) of the GIS definition. This establishes both the onto and 1-to-1 properties.

For the formula: LABEL(s)^(-1) LABEL(t) = int(ref, s)^(-1) int(ref, t) = int(s, ref) int(ref, t) = int(s, t)

# Musical Context/Application
In the familiar GIS of twelve pitch classes, we label pitch classes by their intervals from a referential pitch class C. Thus C, C#, D, ..., Bb, B are labeled 0, 1, 2, ..., 10, 11 (mod 12). This is the standard integer notation for pitch classes, using C as the reference point.

The LABEL function is useful for computations involving members of S. However, its use can be conceptually problematic when there are no adequate musical reasons for assigning a special referential status to the chosen ref. Why should we privilege C as reference rather than E or A?

# Examples
**Pitch-class example:** With ref = C in the twelve-tone GIS:
- LABEL(C) = 0
- LABEL(E) = 4
- LABEL(G) = 7
- int(E, G) = LABEL(E)^(-1) LABEL(G) = (-4) + 7 = 3 (mod 12)

**Just intonation example:** In the GIS of just-intonation pitch classes (example 2.1.5-2.1.6), elements are labeled by their frequency ratios from a reference pitch, reduced by powers of 2 to form congruence classes.

# Related Concepts
- Generalized Interval System (GIS)
- Reference Point (ref)
- Interval Group (IVLS)
- Transposition Operations (Ti)
- Interval-Preserving Operations (Pi)

# Common Confusions
1. The specific value of LABEL(s) depends on the choice of ref. Different reference points yield different labelings, though the intervals between elements remain the same.

2. Students often assume labeling is intrinsic to elements, but it is always relative to a chosen reference point. The pitch class E is "4" only relative to C as reference; it would be "0" in an E-labeling system.

3. Unlike intervals, which are intrinsic to pairs of elements, labels are extrinsic and depend on an arbitrary choice.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Definitions 3.1.1-3.1.2, pp. 62-63
