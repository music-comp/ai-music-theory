---
concept: Interval Group IVLS
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
IVLS is the group of intervals in a GIS - a mathematical group whose elements represent all possible directed measurements, distances, or motions between elements of the musical space.

# Formal Definition
In a GIS (S, IVLS, int), IVLS is the group of intervals for the GIS - a mathematical group. IVLS must satisfy the four group axioms (closure, associativity, identity, inverses). The interval function int maps pairs of elements from S into IVLS, and Condition (A) ensures that interval composition in IVLS corresponds to path concatenation in S.

# Mathematical Formulation
- IVLS is a mathematical group under some binary operation (often written additively or multiplicatively)
- int: S x S -> IVLS maps pairs of space elements to intervals
- Condition (A): int(r, s) * int(s, t) = int(r, t) [intervals compose correctly]
- Identity e in IVLS: int(s, s) = e for all s
- Inverses: int(t, s) = int(s, t)^(-1)

# Musical Context/Application
IVLS captures our intuitions about musical intervals: they can be composed (go up a third, then up a fifth = up an octave), there's a "zero" interval (unison/identity), and every interval has an inverse (up a third inverts to down a third). Different musical dimensions use different interval groups: integers for pitch intervals, Z12 for pitch-class intervals, positive ratios for frequency intervals.

# Examples
From Chapter 2:
- Chromatic pitch space (2.1.2): IVLS = integers under addition. Intervals are "n semitones up."
- Pitch-class space (2.1.3): IVLS = integers mod 12. Intervals wrap around: 7 + 7 = 14 = 2 mod 12.
- Just intonation (2.1.5): IVLS = ratios 2^a * 3^b * 5^c under multiplication.
- Modular harmonic space (2.1.6): IVLS = Z x Z (direct product), with intervals (b, c) measuring "b dominants, c mediants."
- Duration proportion space (2.2.3): IVLS = multiplicative group of positive ratios.

# Related Concepts
- Generalized Interval System
- Musical Space S
- Interval Function int
- Group
- Direct Product
- Quotient Group

# Common Confusions
- IVLS is always a group (not just a set of intervals)
- The group operation need not be "addition" - it can be multiplication or other operations
- IVLS can be commutative (Z12) or non-commutative (in later chapters)
- The same musical space S can have different IVLS for different analytical purposes

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1, Section 2.4
