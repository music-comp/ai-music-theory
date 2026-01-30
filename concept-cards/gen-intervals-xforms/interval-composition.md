---
concept: Interval Composition
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Interval composition is the combining of intervals along a path: the interval from r to s composed with the interval from s to t yields the interval from r to t.

# Formal Definition
In any GIS, intervals compose according to Condition (A): int(r, s) * int(s, t) = int(r, t). This formalizes the intuition that traversing from r to s, then from s to t, accumulates to the direct interval from r to t. The composition uses the group operation of IVLS (addition, multiplication, etc.).

# Mathematical Formulation
- int(r, s) * int(s, t) = int(r, t) [Condition A]
- The operation * is the group operation in IVLS
- Additive groups: int(r, s) + int(s, t) = int(r, t)
- Multiplicative groups: int(r, s) * int(s, t) = int(r, t)
- Associativity: int(r, s) * (int(s, t) * int(t, u)) = (int(r, s) * int(s, t)) * int(t, u) = int(r, u)

# Musical Context/Application
Interval composition captures the fundamental musical intuition that intervals "add up" along a melodic or harmonic path. Going up a major third (4 semitones) then up a minor third (3 semitones) equals going up a perfect fifth (7 semitones). This property enables analysis of voice-leading, sequences, and transformational networks where paths through the space are traced.

# Examples
From Chapter 2:
- Chromatic: "If we take 2 steps up (from C4 to E4) and then take 2 more steps up (from E4 to G4), we have taken 4 steps up in all (from C4 to G4)."
- int(C4, E4) = 2, int(E4, G4) = 2, int(C4, G4) = 4, and 2 + 2 = 4

Traditional vs. GIS counting: "This obviates a defect in the traditional measurements which tell us, for example, that a '3rd' and another '3rd' compose to form a '5th.' (3 + 3 = 5???)"

Ratio example (just intonation): int(C, E) = 5/4, int(E, G#) = 5/4, int(C, G#) = 25/16, and (5/4)(5/4) = 25/16.

# Related Concepts
- GIS Condition A
- Interval Group IVLS
- Generalized Interval System
- Path in Musical Space
- Associativity

# Common Confusions
- The operation depends on IVLS: addition for Z, multiplication for ratio groups
- Traditional interval names violate this: "3rd + 3rd = 5th" is inconsistent
- GIS numbering counts steps (0, 1, 2, ...) not notes (1st, 2nd, 3rd, ...)
- Composition is associative but not necessarily commutative

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.1, Definition 2.3.1(A)
