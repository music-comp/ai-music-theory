---
concept: INJ Transformation Theorem
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
When sets X and/or Y are transformed by an operation A, the INJ function "conjugates" - INJ(A(X), A(Y))(f) = INJ(X, Y)(A^-1 f A).

# Formal Definition
Theorem 6.7.2: Given a family S of objects, given sets X and Y, given a transformation f on S and an operation A on S, then:
- (A): INJ(A(X), Y)(f) = INJ(X, Y)(fA)
- (B): INJ(X, A(Y))(f) = INJ(X, Y)(A^-1 f)
- (C): INJ(A(X), A(Y))(f) = INJ(X, Y)(A^-1 f A)

# Mathematical Formulation
Proof of (A):
INJ(A(X), Y)(f) = number of t in A(X) such that f(t) is in Y
Set t = A(s). Then t in A(X) iff s in X.
f(t) = f(A(s)) = (fA)(s)
So: number of s in X such that (fA)(s) is in Y = INJ(X, Y)(fA). Q.E.D.

Proof of (B): Similar reasoning using A^-1.

Proof of (C): Apply (A) then (B), or (B) then (A).

Corollary: The transformation f' = AfA^-1 plays the same structural role in the "modulated" system (A(X), A(Y)) that f plays in the original system (X, Y).

# Musical Context/Application
Formula (C) describes "system modulation." When we transpose or invert all sets by A, the entire INJ landscape transforms by conjugation. A transformation f that was progressive for X->Y corresponds to f' = AfA^-1 being progressive for A(X)->A(Y).

This captures how harmonic relationships "modulate" when the entire context transposes.

# Examples
From "Angst und Hoffen":
- w^E = wedge-to-E, I = inversion about E
- w^F# = wedge-to-F#, J = inversion about F#

Modulation by T_2 (from E to F#):
- w^F# = T_2 * w^E * T_2^-1
- J = T_2 * I * T_2^-1

Therefore:
- INJ(T_2(X), T_2(Y))(w^F#) = INJ(X, Y)(T_2^-1 * w^F# * T_2) = INJ(X, Y)(w^E)
- INJ(T_2(X), T_2(Y))(J) = INJ(X, Y)(I)

The F#-centered wedge and inversion in the modulated system correspond to E-centered wedge and inversion in the original system.

Hexachord combinatoriality:
- If INJ(X, X)(I) = 0 (X inverts to complement)
- Modulate by T_n: INJ(T_n(X), T_n(X))(I) != 0 in general
- But INJ(T_n(X), T_n(X))(J) = 0 where J = T_n I T_n^-1

# Related Concepts
- INJ (Injection Function)
- System Modulation
- Conjugation
- Progressive/Internal Transformations

# Common Confusions
The formula involves conjugation: A^-1 f A, not f A^-1 A = f. This is because we're asking what transformation in the original system corresponds to f in the modulated system. The order of operations matters.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Theorem 6.7.2
