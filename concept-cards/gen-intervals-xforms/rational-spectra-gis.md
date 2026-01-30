---
concept: Rational Spectra GIS
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
The rational spectra GIS models filter classes where elements are rational functions of frequency, representing the class of linear filters that transform sounds by frequency-dependent amplitude scaling.

# Formal Definition
**Example 4.2.2:** A "rational spectrum" is a function s(x) for frequency x in [LO, HI] satisfying:
(A) s(x) = p(x)/q(x) for polynomial functions p and q
(B) s(x) > 0 for all x in [LO, HI]

S = family of all rational spectra
IVLS = S (same family, viewed as multiplicative group)
Group operation: pointwise multiplication
int(s, t) = t/s (pointwise quotient)

(S, IVLS, int) is a commutative GIS.

# Mathematical Formulation
**Group structure of IVLS:**
- Closure: (p1/q1)(p2/q2) = (p1p2)/(q1q2) is rational
- Identity: constant function 1(x) = 1
- Inverse: (p/q)^(-1) = q/p (positive since p/q was positive)
- Associativity: inherited from function multiplication

**Filter interpretation:**
Each rational spectrum s corresponds to a class of linear filters. Filter-class s transforms input sound to output by multiplying the power at frequency x by s(x).

# Musical Context/Application
This GIS models digital audio processing:
- All-zero filters (FIR)
- All-pole filters (IIR)
- Combined linear filters

The rational function form covers most practical linear filters in computer music.

# Examples
**Simple filter:**
s(x) = x^2 / (x^2 + 1) for frequencies in [20, 20000] Hz
This boosts high frequencies relative to low.

**Interval as transformation:**
If s is current spectrum and t is desired spectrum:
int(s, t) = t/s is the filter needed to transform s to t.

**Filter concatenation:**
Filter s followed by filter t = filter st (pointwise product).
Composing filters = multiplying in the group.

**Transposition of filters:**
Ti(s) = si (the filter shifted by interval i)
Preserves the group structure of filter composition.

**Computer music reference:**
Cann's "Analysis/Synthesis Tutorial" in Computer Music Journal explains these techniques.

# Related Concepts
- Timbral GIS
- Filter Design
- Linear Time-Invariant Systems
- Computer Music
- Spectral Processing

# Common Confusions
1. **Functions as elements:** S consists of functions, not numbers. Each "point" in S is an entire frequency-response curve.

2. **Positivity constraint:** s(x) > 0 is required for s to be invertible in the multiplicative group.

3. **Frequency range:** LO and HI can be varied; the GIS depends on this choice.

4. **Rational = ratio of polynomials:** This is a technical constraint that covers most practical filters.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Example 4.2.2, pp. 117
