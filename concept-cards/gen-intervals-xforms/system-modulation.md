---
concept: System Modulation
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
System modulation occurs when an entire musical context is transformed by an operation A, changing sets X and Y to A(X) and A(Y), with transformations conjugating accordingly.

# Formal Definition
When we "modulate the system by operation A":
- Set X becomes A(X)
- Set Y becomes A(Y)
- Transformation f becomes f' = AfA^-1

The conjugate transformation f' plays the same structural role in the modulated system that f played in the original. This is captured by Formula 6.7.2(C):
INJ(A(X), A(Y))(f) = INJ(X, Y)(A^-1 f A)

# Mathematical Formulation
Modulation map: (X, Y, f) -> (A(X), A(Y), AfA^-1)

Key insight: The relationship
INJ(modulated X, modulated Y)(f') = INJ(X, Y)(f)

holds when f' = AfA^-1.

For transposition A = T_n in a commutative GIS:
- T_i becomes T_n T_i T_n^-1 = T_i (transpositions commute)
- I_u becomes T_n I_u T_n^-1 = I_{n+u} (inversion center shifts by n)
- Wedge w^u becomes T_n w^u T_n^-1 = w^{T_n(u)} (focal point shifts)

# Musical Context/Application
System modulation models how an entire harmonic context can transpose while maintaining internal relationships. When music moves from one key to another, the relationships between transformations and sets are preserved up to conjugation.

This formalizes intuitions like "the dominant in the new key plays the same role the dominant played in the old key."

# Examples
From "Angst und Hoffen" (Figure 6.3):

Original system (E-centered):
- Focal point: E
- Wedge: w^E
- Inversion: I = I_E^Bb

Modulated system (F#-centered, modulation by T_2):
- Focal point: F# = T_2(E)
- Wedge: w^F# = T_2 w^E T_2^-1
- Inversion: J = I_F#^C = T_2 I T_2^-1

The bass motion E -> F# (T_2) corresponds to the modulation from E-centered structure to F#-centered structure. Chords and transformations all shift together.

Hexachord example:
- X inverts to complement via I = I_0^E
- Modulate by T_3: X' = T_3(X)
- X' inverts to complement via J = T_3 I T_3^-1 = I_3^Ab

# Related Concepts
- INJ Transformation Theorem
- Conjugation
- Transposition Operations
- Key Relationships

# Common Confusions
System modulation is not just transposition of notes - it's the coordinated transformation of sets and operations that preserves structural relationships. A single transposition T_n of pitches induces conjugation T_n ... T_n^-1 on transformations.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, discussion following Theorem 6.7.2
