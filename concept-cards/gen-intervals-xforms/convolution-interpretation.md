---
concept: IFUNC as Convolution
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
In a more abstract mathematical setting, IFUNC(X, Y)(i) can be interpreted as the convolution of characteristic functions, connecting set theory to harmonic analysis on groups.

# Formal Definition
For readers with graduate-level mathematical background (as noted in the text):

Using LABEL to identify S with IVLS, we treat S = IVLS as a locally compact group under the discrete topology. "Sets" are compact subsets. If f and g are the characteristic functions of sets X and Y respectively, then:

IFUNC(X, Y)(i) = (f * g)(i)

where * denotes convolution of functions.

# Mathematical Formulation
Characteristic function of set X:
f(s) = 1 if s in X, 0 otherwise

Convolution:
(f * g)(i) = sum over s of f(s) * g(s + i)
           = sum over s in X of g(s + i)
           = |{s in X : s + i in Y}|
           = IFUNC(X, Y)(i)

For continuous groups, convolution involves integration rather than summation.

Generalization: Questions about IFUNC can be rephrased as questions about convolutions of characteristic functions of compact subsets in locally compact groups.

# Musical Context/Application
The convolution interpretation connects music-theoretic constructions to the broader mathematical field of harmonic analysis. Techniques from Fourier analysis and representation theory can potentially be applied to study IFUNC systematically.

This connection is part of what makes Lewin's approach powerful: musical structures connect to deep mathematical structures with well-developed theory.

# Examples
In Z_12 (integers mod 12 = pitch classes):
- X = {0, 4, 7} (C major triad), characteristic function f
- Y = {0, 3, 7} (C minor triad), characteristic function g
- IFUNC(X, Y) = f * g

The convolution (f * g)(i) counts how many ways interval i appears from X to Y.

For the full interval vector:
- IFUNC(X, X) = f * f (self-convolution)
- This is the "autocorrelation" of set X

# Related Concepts
- IFUNC (Interval Function)
- Characteristic Function
- Fourier Analysis
- Harmonic Analysis on Groups
- Z-Relation

# Common Confusions
The convolution interpretation requires mathematical sophistication and is optional for understanding IFUNC musically. Lewin marks this paragraph as skippable for readers without graduate-level background. The musical applications of IFUNC do not require understanding convolution.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, paragraph following Theorem 5.1.8 (marked as for advanced readers)
