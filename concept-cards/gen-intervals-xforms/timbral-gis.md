---
concept: Timbral GIS
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
A timbral GIS models harmonic spectra (patterns of partial strengths) as elements of a space, with intervals measuring how one spectrum transforms into another via spectral filters or amplitude scaling.

# Formal Definition
**Example 4.2.1:** Let s = (s(1), s(3), s(5)) denote the class of harmonic sounds whose first, third, and fifth partials have respective powers s(1), s(3), s(5). Let i = (i(1), i(3), i(5)) denote a device class that multiplies these partial powers by factors i(1), i(3), i(5) respectively.

Space S: all positive-real triples s = (s(1), s(3), s(5))
IVLS: all positive-real triples i = (i(1), i(3), i(5))
Group operation: componentwise multiplication
Interval: int(s, t) = (t(1)/s(1), t(3)/s(3), t(5)/s(5))

(S, IVLS, int) is a commutative GIS.

# Mathematical Formulation
**IVLS group structure:**
(i(1), i(3), i(5)) * (j(1), j(3), j(5)) = (i(1)j(1), i(3)j(3), i(5)j(5))
Identity: (1, 1, 1)
Inverse: (i(1), i(3), i(5))^(-1) = (1/i(1), 1/i(3), 1/i(5))

This is the direct product of three copies of (R+, *).

**Interval interpretation:**
int(s, t) = i means: passing s through device-class i produces t.
Or: t has i(1), i(3), i(5) times the power of s at partials 1, 3, 5.

# Musical Context/Application
This GIS abstracts timbral relationships by focusing on spectral content:
- Fundamental frequencies are ignored (only partial ratios matter)
- The "interval" between timbres is a scaling pattern
- Devices (filters, processors) represent intervals

Variations can consider different partial sets: #1-3-5, #1-2-4, #1-8, etc.

# Examples
**Basic interval:**
s = (2, 1, 0.5) (partial powers)
t = (4, 3, 0.5)
int(s, t) = (4/2, 3/1, 0.5/0.5) = (2, 3, 1)

Meaning: t has 2x power at partial 1, 3x at partial 3, same at partial 5.

**Device interpretation:**
A filter that doubles partial 1 power and triples partial 3 power (leaving 5 unchanged) transforms s to t.

**Concatenation:**
Device i followed by device j = device ij (componentwise product).
If i = (2, 3, 1) and j = (1, 0.5, 2):
ij = (2, 1.5, 2)

**Extended version (8 partials):**
S consists of 8-tuples (s(1), ..., s(8))
This gives a finer spectral characterization.

# Related Concepts
- Direct-Product GIS
- Spectral Analysis
- Filter Classes
- Developing Spectra (DVSP)
- Rational Spectra GIS

# Common Confusions
1. **Power, not amplitude:** The partial values are powers (energy), related to amplitude squared.

2. **Fundamental frequency irrelevant:** Two sounds at different pitches can have the same spectral "position" in S.

3. **Positive values only:** Partial powers must be positive (no zero or negative).

4. **Device = interval:** A filter or processor that transforms spectra represents an interval. Concatenating devices = multiplying intervals.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Example 4.2.1, pp. 114-115
