---
concept: Radial Motion on Spiral
category: technique
source: "Tonality: An Owner's Manual"
chapter: "Rock Logic"
chapter_number: 2
pdf_page: 47
authors: "Dmitri Tymoczko"
---

# Quick Definition
Movement between chords at different radial positions on the spiral diagram, corresponding to efficient voice leadings that combine transposition along both chord and scale.

# Formal Definition
Radial motion between nearby chords on the spiral results from transposition along the chord largely counteracting transposition along the scale, leaving efficient voice leading as the result. The general algorithm: slide from one chord to the other along the spiral; the number of chords touched (not counting the first) gives the scalar transposition T_x; the number of times you revisit the initial angular position gives the chordal transposition t_{+-y} (with sign opposite to the scalar transposition). The combination T_x t_{-y} represents the most direct voice leading.

# Musical Context
Radial motion is crucial for understanding how the spiral diagram encodes voice-leading information. On the 3-in-12 major triad spiral, radial motion between chords at different rings produces the LP and PL voice leadings of neo-Riemannian theory. A musical preference for descending stepwise melodies is modeled by a geometrical tendency to move radially or clockwise on the spiral.

# Examples
- Radial motion from C to E (outward): slide counterclockwise through 4 chords (**T**_4), pass initial position once (t_{-1}). Voice leading: C down to B, E fixed, G up to G# -- the LP voice leading (Figure 2.1.4)
- Radial motion from E to C (inward): **T**_{-4} t_1, the PL voice leading (Figure 2.1.5)
- Radial motion from C to Ab and Ab to E also produces LP; from Ab to E and C to Ab also produces PL
- 90-degree counterclockwise from C to F: **T**_5 t_{-1} -- holds root, moves third up by semitone, fifth up by two semitones

# Related Concepts
- Spiral Diagrams for Chord Space
- Slide Along Spiral
- Loop on Spiral
- LP and PL Voice Leading
- Voice Leading

# Common Confusions
- For purely radial paths, both clockwise and counterclockwise routes yield the same voice leading
- The initial direction matters for non-radial paths: clockwise from C to F gives a different voice leading than counterclockwise
- Radial motion is not a separate operation from slides and loops -- it is their combination when they nearly cancel

# Source Reference
Chapter 2, Section 1, pp. 49-52, Figures 2.1.4-2.1.6. The algorithm for calculating radial paths is given on p. 50.
