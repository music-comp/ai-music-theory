---
concept: Serial Transformations Generalization
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Transformation Graphs and Networks (2): Non-Intervallic Transformations"
chapter_number: 8
pdf_page: 206
unit: null
authors: David Lewin
---

# Quick Definition
The extension of serial transformations (RICH, TCH, MUCH, TLAST, TFIRST, FLIPEND, FLIPSTART) to series whose elements are members of an abstract commutative GIS, with complications arising in non-commutative cases.

# Formal Definition
Serial transformations can be generalized to abstract GIS settings:
- For commutative GIS: All definitions extend naturally
- For non-commutative GIS: Multiple plausible definitions may exist

The problem with RICH in non-commutative GIS:
- Given series s with elements in a non-commutative GIS
- Three candidates for "RICH(s)": t, u, and v
- All three have the same first two elements (a, b) but may differ thereafter

# Mathematical Formulation
For abstract series s = s_1, s_2, ..., a, b in a non-commutative GIS:

Candidate t: retrograde of I(s) where I = (a/b)-inversion I^a
Candidate u: retrograde of J(s) where J = (b/a)-inversion I^b = (I^a)^(-1)
Candidate v: starts with a, proceeds by intervals i_{n-1}, i_{n-2}, ..., i_2, i_1 (reversed order)

Where i_k = int(s_k, s_{k+1}) are the serial intervals of s.

In commutative GIS: t = u = v
In non-commutative GIS: t, u, v may be three distinct series

# Musical Context/Application
This generalization shows that serial transformations are not merely computational procedures but depend on the underlying intervallic structure. The non-commutative case reveals that even "standard" serial operations require interpretive choices about inversion and interval ordering.

# Examples
The three candidates for RICH(s):
- All begin with elements a and b (last two of s, in order)
- All are retrograde-inverted forms of s
- In commutative GIS (standard pitch classes), they collapse to one series
- In non-commutative GIS (e.g., time spans with non-commutative intervals), they may differ

# Related Concepts
- RICH Transformation
- TCH Transformation
- MUCH Transformation
- Non-Commutative GIS
- Abstract GIS

# Common Confusions
- "Generalization" here means extension to abstract GIS, not looser definitions
- The complication only arises for non-commutative GIS
- Standard pitch-class applications are unaffected (Z_12 is commutative)
- The three candidates differ only in non-commutative cases

# Source Reference
Chapter 8: Transformation Graphs and Networks (2): Non-Intervallic Transformations, Section 8.4
