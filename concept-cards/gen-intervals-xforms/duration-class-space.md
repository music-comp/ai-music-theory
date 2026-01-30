---
concept: Duration-Class Space
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Duration-class space is a GIS that reduces duration proportions by a modulus M, so durations differing by powers of M are considered equivalent.

# Formal Definition
In Example 2.2.4, we reduce the space of 2.2.3 by a durational modulus M > 1. Two durations are equivalent if one is some integral power of M times the other. This leads to a modular space whose elements are duration-classes (equivalence classes of durations). The intervals are ratio-classes (equivalence classes of ratios under the same modular reduction).

# Mathematical Formulation
- Equivalence: durations s ~ t if t = s * M^n for some integer n
- S = duration-classes (equivalence classes under M-reduction)
- IVLS = ratio-classes (quotient group of ratios mod powers of M)
- int(s-class, t-class) = ratio-class containing t/s
- This parallels pitch-class intervals: reduce pitch intervals mod 12 -> pitch-class intervals

# Musical Context/Application
With M = 2, durations differing by factors of 2 are equivalent - a quarter note and half note and whole note belong to the same duration-class. This is analogous to octave equivalence for pitches. Stockhausen used this system (M = 2) in analyzing rhythmic structures in works like Gruppen, treating "durational octave equivalence" analogously to pitch octave equivalence.

# Examples
From Example 2.2.4, with M = 2:
- Class r = {..., 5/32, 5/16, 5/8, 5/4, 5/2, 5, 10, 20, ...}
- Class s = {..., 1/12, 1/6, 1/3, 2/3, 4/3, 8/3, ...}
- Class t = {..., 7/20, 7/10, 7/5, 14/5, 28/5, ...}
- int(r, s) = ratio-class containing 16/15 = {(2^n)(16/15) : n in Z}
- int(s, t) = ratio-class containing 21/20

Irrational example: Class u = {..., sqrt(2)/4, sqrt(2)/2, sqrt(2), 2*sqrt(2), 4*sqrt(2), ...}
- int(s, u) = ratio-class containing 3*sqrt(2)/8

Stockhausen: Argued plausibility of M = 2 system, connecting rhythmic modular space to pitch modular space.

# Related Concepts
- Duration Proportion Space
- Pitch-Class Space
- Quotient Group
- Ratio-Class
- Equivalence Class

# Common Confusions
- Duration-classes collapse durations differing by powers of M (usually 2)
- The reduction from 2.2.3 to 2.2.4 parallels pitch to pitch-class reduction
- Different values of M give different duration-class systems
- Ratio-classes form the quotient group of ratios mod M^n

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Example 2.2.4, Section 2.4
