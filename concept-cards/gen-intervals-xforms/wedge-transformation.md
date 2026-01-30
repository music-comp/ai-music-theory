---
concept: Wedge Transformation
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
A wedge transformation systematically moves pitch classes toward a focal point, with notes on opposite sides of the focal point moving inward by semitones.

# Formal Definition
The wedge-to-E transformation, denoted w^E, maps pitch classes as follows:
- E maps to E (fixed point, focal point of wedge)
- Bb maps to Bb (antipodal fixed point)
- Every other pitch class advances one semitone toward E along the shortest path on the pitch-class clock

w^E is NOT an operation: it is neither 1-to-1 nor onto. Multiple pitch classes map to the same image; some pitch classes are not images of anything.

# Mathematical Formulation
w^E: pitch classes -> pitch classes

Explicit mapping (Figure 6.1):
- E -> E, F -> E (converge at E)
- Eb -> E, D -> Eb, C# -> D, C -> C#
- Bb -> Bb, B -> Bb (converge at Bb)
- A -> Bb, Ab -> A, G -> Ab, F# -> G

Properties:
- Not 1-to-1: w^E(E) = w^E(F) = E
- Not onto: No pitch class maps to F

For wedge-to-u (any pitch class u):
w^u = T_n * w^E * T_n^(-1) where n = u - E

# Musical Context/Application
Wedge transformations model voice-leading processes where voices converge toward a focal pitch. They capture the "directional" quality of certain progressions. The focal point often has tonic or structural significance.

In "Angst und Hoffen," the E-wedge converges toward E, the bass of the Hoffen chord, giving E a "tonic" character as a point of arrival.

# Examples
From "Angst und Hoffen" (Figures 6.2-6.3):

Chord X = {Gb, Bb, D} (Angst)
Chord Y = {Fb, Bb, Eb} (Hoffen)

Applying w^E to X:
- D -> Eb (in Y)
- Bb -> Bb (in Y)
- Gb -> G (NOT in Y; "should" go to F but Y has Fb)

INJ(X, Y)(w^E) = 2

The "missing F" theme: If Fb were F, the wedge would perfectly map X into Y.

Figure 6.3(b) shows wedging progressions Z1 -> Z2 -> Z3 -> Z4 converging toward the E of the Seufzer chord Z6.

# Related Concepts
- INJ (Injection Function)
- Focal Point
- Progressive Transformation
- System Modulation (6.7.2)
- Voice Leading

# Common Confusions
Wedge transformations are NOT operations (not invertible). They model a process of convergence, not a bijective mapping. The INJ function handles this gracefully, whereas interval-based approaches like IFUNC cannot engage wedges directly.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Example 6.2.3 and Figures 6.1-6.3
