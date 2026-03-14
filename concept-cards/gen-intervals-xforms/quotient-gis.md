---
concept: Quotient GIS
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
A quotient GIS is a new GIS derived from an existing GIS by applying a congruence relation on the interval group, which induces an equivalence relation on the space, creating a "modularized" system.

# Formal Definition
Let (S1, IVLS1, int1) be a GIS and CONG be a congruence on the group IVLS1. The quotient GIS (S2, IVLS2, int2) = GIS1/CONG is constructed as follows:

- S2 = S1/EQUIV (equivalence classes of elements under induced equivalence)
- IVLS2 = IVLS1/CONG (congruence classes of intervals)
- int2(p, q) = congruence class containing int1(s, t), for any s in p, t in q

The induced equivalence EQUIV on S1 declares s and s' equivalent whenever int1(s, s') is congruent to the identity e in IVLS1.

# Mathematical Formulation
**Theorem/Definition 3.2.1:** Given GIS (S, IVLS, int) and congruence CONG on IVLS, an equivalence relation EQUIV is induced on S by declaring s and s' equivalent whenever int(s, s') is congruent to e.

**Lemma 3.2.2:** If s ~ s' and t ~ t' under EQUIV, then int(s, t) is congruent to int(s', t').

**Theorem 3.2.3:** The quotient structure (S2, IVLS2, int2) satisfies the GIS axioms.

**Definition 3.2.4:** GIS2 = GIS1/CONG

# Musical Context/Application
The quotient construction formalizes "modularization"--the process of creating a reduced space by identifying equivalent elements. This is ubiquitous in music theory:

- Pitch space modularized by octave equivalence yields pitch-class space
- Time-point space modularized by measure length yields beat-class space
- Just-intonation pitch space modularized by octave yields just-intonation pitch classes

# Examples
**Chromatic scale to pitch classes:**
- GIS1: S1 = infinite chromatic scale, IVLS1 = integers, int1 = semitones
- CONG: i congruent to i' when they differ by a multiple of 12
- GIS2: S2 = 12 pitch classes, IVLS2 = integers mod 12
- EQUIV: pitches are equivalent if they differ by octaves

**Just intonation modularization:**
- GIS1: pitches with ratios 2^a * 3^b * 5^c, IVLS1 = multiplicative group of such ratios
- CONG: i congruent to i' when i' = 2^n * i for some integer n
- GIS2: just-intonation pitch classes (octave equivalence), IVLS2 = pairs (b, c)

**Diatonic scale to scale degrees:**
- Seven-note diatonic scale with integer intervals
- Modularize by intervals of 7 (octave = 7 scale steps)
- Result: seven scale degrees

**Time points to beat classes:**
- Time-point space with integer intervals (beats)
- Modularize by N beats
- Result: N beat-classes

# Related Concepts
- Generalized Interval System (GIS)
- Congruence Relation
- Equivalence Relation
- Induced Equivalence
- Direct-Product GIS
- Octave Equivalence

# Common Confusions
1. The congruence is on IVLS (intervals), not directly on S (elements). The equivalence on S is *induced* by the congruence.

2. Students may forget that int2 is well-defined only because of Lemma 3.2.2: the congruence class of int1(s, t) depends only on the equivalence classes of s and t, not on the particular representatives chosen.

3. Not every equivalence on S gives rise to a quotient GIS; the equivalence must be induced by a congruence on IVLS for int2 to be well-defined.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Sections 3.2.1-3.2.4, pp. 64-68
