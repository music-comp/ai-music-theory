---
concept: Just Intonation Pitch Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Just intonation pitch space is a GIS where pitches are tuned by pure frequency ratios, and intervals are measured as quotients of fundamental frequencies.

# Formal Definition
In Example 2.1.5, the musical space S comprises pitches available from a given pitch using just intonation. If FQ(s) denotes the fundamental frequency of pitch s, then int(s, t) is the quotient FQ(t)/FQ(s). This quotient will be some number of the form 2^a * 3^b * 5^c, where a, b, and c are integers. The interval group IVLS is the multiplicative group of such ratios.

# Mathematical Formulation
- S = pitches available through just intonation from a given pitch
- IVLS = {2^a * 3^b * 5^c : a, b, c in Z} under multiplication
- int(s, t) = FQ(t)/FQ(s) (frequency ratio)
- Identity: int(s, s) = 1 (ratio of 1)
- Inverse: int(t, s) = 1/int(s, t)
- Composition: int(r, s) * int(s, t) = int(r, t)

# Musical Context/Application
This GIS models harmonic relationships in just intonation. The prime factorization (2^a * 3^b * 5^c) encodes the harmonic "path" - powers of 2 are octaves, powers of 3 are fifths/fourths, and powers of 5 are major thirds/sixths. Lewin argues that our intuition of these intervals is culturally conditioned and involves chains of basic harmonic moves rather than direct ratio perception.

# Examples
From Example 2.1.5:
- Octave: int(C4, C5) = 2
- Perfect fifth: int(C4, G4) = 3/2
- Major third: int(C4, E4) = 5/4
- Tritone: int(C4, F#4) = 45/32 = 2 * (5/4) * (3/4)^2

Figure 2.1: The interval 45/32 from C4 to F#4 arises as a chain: F#4 lies an octave above (x2) the major third (x5/4) of D3, which is the dominant (x3/4) of the dominant (x3/4) of C4.

The "natural" factorization 45/32 = 2 * (5/4) * (3/4) * (3/4) reflects harmonic intuition: octave, mediant, dominant, dominant.

# Related Concepts
- Modular Harmonic Space
- Chromatic Pitch Space
- Generalized Interval System
- Harmonic Distance
- Frequency Ratio

# Common Confusions
- Intervals are ratios (multiplied), not distances (added)
- The same ratio 45/32 can arise from different harmonic paths
- Cultural conditioning shapes which paths we actually intuit
- This space is infinite and extends beyond practical tuning systems

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.1.5, Section 2.4
