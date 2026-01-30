---
concept: Mean-Tone Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Mean-tone space is a reduction of modular harmonic space where pitch classes with the same letter name but different subscripts are declared equivalent, collapsing the two-dimensional map to a one-dimensional chain of fifths.

# Formal Definition
Starting from the two-dimensional modular harmonic space of Figure 2.2, we can declare pitch classes equivalent if they share the same letter name (differing only by subscript). This collapses the north/south (mediant) dimension, since moving one square north becomes equivalent to moving four squares east. The result is a one-dimensional space: ...Eb, Bb, F, C, G, D, A, E, ...

# Mathematical Formulation
- Equivalence: C-1 ~ C0 ~ C1 ~ ... (same letter, any subscript)
- Since one step north = 4 steps east (in terms of effect on letter name), the mediant dimension collapses
- Resulting space: infinite chain of pitch classes by fifths
- IVLS: integers under addition (counting fifths)
- This models quarter-comma mean-tone temperament where 4 fifths = major third

# Musical Context/Application
Mean-tone temperament was a common tuning system in the Renaissance and Baroque, where four perfect fifths were tuned to exactly equal a major third (hence "quarter-comma"). This GIS models that system: moving by fifths is the basic interval, and the syntonic comma (difference between just and Pythagorean thirds) is tempered out.

# Examples
From Chapter 2, following Figure 2.2:
- In modular harmonic space: C0, C1, C-1 are distinct (differ by syntonic commas)
- Declare equivalence: all C's become one pitch class C
- The chain: ...Eb, Bb, F, C, G, D, A, E, B, F#, C#, G#, D#, A#...
- int(C, G) = 1 (one fifth), int(C, D) = 2 (two fifths), int(C, E) = 4 (four fifths)

Further reduction: If we also declare enharmonic equivalence (Gb = F#, Db = C#, etc.), the chain wraps around to form the 12 pitch classes of equal temperament, measured by fifths rather than semitones.

# Related Concepts
- Modular Harmonic Space
- Just Intonation Pitch Space
- Pitch-Class Space
- Figure 2.2
- Temperament

# Common Confusions
- Mean-tone is a reduction of 2.1.6, not 2.1.3 (it comes from harmonic, not chromatic space)
- The subscripts disappear because the syntonic comma is tempered out
- Without enharmonic equivalence, the space is still infinite (all sharps/flats distinct)
- With enharmonic equivalence, we get pitch-class space measured by fifths (interval 7 in mod 12)

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, discussion following Example 2.1.6
