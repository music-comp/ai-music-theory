---
concept: Generalized Interval System
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
A Generalized Interval System (GIS) is an ordered triple (S, IVLS, int) consisting of a musical space, a group of intervals, and a function assigning intervals between pairs of elements.

# Formal Definition
A Generalized Interval System (GIS) is an ordered triple (S, IVLS, int), where S (the space of the GIS) is a family of elements, IVLS (the group of intervals for the GIS) is a mathematical group, and int is a function mapping S x S into IVLS, all subject to two conditions:
(A) For all r, s, and t in S: int(r, s)int(s, t) = int(r, t)
(B) For every s in S and every i in IVLS, there exists a unique t in S such that int(s, t) = i

# Mathematical Formulation
- S: musical space (family of elements)
- IVLS: interval group (a mathematical group)
- int: S x S -> IVLS (interval function)
- Condition (A): int(r, s) * int(s, t) = int(r, t) [intervals compose]
- Condition (B): For all s in S and i in IVLS, exists unique t with int(s, t) = i [space is "complete"]

Derived theorems (2.3.2):
- int(s, s) = e (identity interval)
- int(t, s) = int(s, t)^(-1) (interval inversion)

# Musical Context/Application
The GIS framework unifies diverse musical concepts under a single mathematical structure. Traditional pitch intervals, pitch-class intervals, rhythmic intervals, harmonic distance, and many other musical measurements can all be formalized as GIS structures. This allows theorems proved for abstract GIS to apply across all these musical domains.

# Examples
From Chapter 2: Twelve musical spaces are presented as GIS examples:
- Chromatic pitch space (IVLS = integers)
- Diatonic pitch space (IVLS = integers)
- Pitch-class space mod 12 (IVLS = Z12)
- Just intonation pitch space (IVLS = ratios 2^a * 3^b * 5^c)
- Modular harmonic space (IVLS = Z x Z)
- Time-point space (IVLS = integers)
- Beat-class space (IVLS = ZN)
- Duration-proportion space (IVLS = multiplicative group)
- Duration-class space (IVLS = ratio-classes)

The interval from C4 to G4 in chromatic space is 7 (semitones up).
The interval from C to G in pitch-class space is 7 (mod 12).

# Related Concepts
- Musical Space S
- Interval Group IVLS
- Interval Function int
- Group
- Transposition
- Interval-Preserving Operations

# Common Confusions
- Condition (B) requires the space S to be theoretically complete (possibly extending beyond practical music)
- The interval int(s, t) is directed: from s TO t (order matters)
- int(t, s) = int(s, t)^(-1), so reversing direction inverts the interval
- Different musical dimensions require different GIS structures

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1, Theorem 2.3.2
