---
concept: "Developing Spectra (DVSP)"
category: analysis
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
A Developing Spectrum (DVSP) is a set of spectrum-time pairs representing how a sound's timbral content evolves over time, analyzed using a direct-product GIS combining spectral and temporal dimensions.

# Formal Definition
Let GIS1 be a timbral GIS (spectra with 8 partials). Let GIS2 be a time-point GIS. The direct product GIS3 = GIS1 x GIS2 has elements (s, a) where s is a spectrum and a is a time-point.

A DVSP is an ordered collection:
DVSP = {(s1, a1), (s2, a2), ..., (sN, aN)}

representing spectral snapshots at successive time points.

# Mathematical Formulation
**GIS3 structure:**
S3 = {(s, a) : s is spectrum, a is time-point}
IVLS3 = IVLS1 x IVLS2 (direct product)
int3((s, a), (t, b)) = (int1(s, t), int2(a, b))

**Figure 4.5 (N = 5):**
```
         partial:  1      2      3    ...    8
time a1:        s1(1)  s1(2)  s1(3)  ...  s1(8)
time a2:        s2(1)  s2(2)  s2(3)  ...  s2(8)
  ...
time a5:        s5(1)  s5(2)  s5(3)  ...  s5(8)
```

This array approximates a continuous "relief map" of spectral evolution.

# Musical Context/Application
DVSP analysis captures how timbre develops over the duration of a sound:
- Attack transients (rapid spectral change)
- Steady state (stable spectrum)
- Decay characteristics

This is the basis of modern spectral analysis and synthesis, representing instrumental sounds as evolving spectra.

# Examples
**Instrumental tone analysis:**
A violin note might have DVSP:
- a1 (attack): strong high partials
- a2-a4 (steady): characteristic partial pattern
- a5 (decay): weakening higher partials

**Relief map visualization:**
If we plot partial power as height above a plane with time on one axis and partial number on the other, we get a 3D "relief map" showing spectral evolution.

**Computer music applications:**
The Lexicon of Analyzed Tones (Moorer and Grey) uses exactly this representation for violin, clarinet, oboe, and trumpet tones.

**Interval analysis of DVSP:**
The unfolding interval vector of a DVSP (as in the Webern analysis, section 3.3.1) tracks how spectral-temporal intervals accumulate as the sound develops.

# Related Concepts
- Timbral GIS
- Direct-Product GIS
- Time-Point GIS
- Unfolding Interval Vector
- Spectral Analysis

# Common Confusions
1. **Discrete approximation:** DVSP is a finite set of snapshots, approximating continuous spectral evolution.

2. **Choice of time points:** The ai should be dense enough to capture salient spectral changes (maxima, minima of partials).

3. **Relation to additive synthesis:** DVSP is the analytic inverse of additive synthesis, which builds sounds from specified partial evolutions.

4. **Direct-product structure:** DVSP lives in GIS1 x GIS2, combining spectral intervals with temporal intervals into compound intervals.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Figure 4.5 and discussion, pp. 115-117
