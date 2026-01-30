---
concept: Canonical Groups in Octatonic Analysis
category: technique
source: Generalized Musical Intervals and Transformations
chapter: "Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups"
chapter_number: B
pdf_page: 282
unit: null
authors: David Lewin
---

# Quick Definition
STRANS1 and STRANS2 serve as candidate CANONICAL groups for set-theoretical studies in octatonic music, determining which sets are considered equivalent under the chosen group's operations.

# Formal Definition
A CANONICAL group determines set-class equivalence: two sets are in the same class if one can be transformed into the other by a group operation. In octatonic analysis, either STRANS1 or STRANS2 can serve as the CANONICAL group, producing different notions of equivalence. STRANS1-equivalence matches standard T/I equivalence restricted to octatonic; STRANS2-equivalence represents a novel classification.

# Mathematical Formulation
CANON = STRANS1:
- Sets X and Y are equivalent iff Y = f(X) for some f in STRANS1
- Equivalence classes = T/I classes restricted to octatonic

CANON = STRANS2:
- Sets X and Y are equivalent iff Y = g(X) for some g in STRANS2
- Equivalence classes = STRANS2-forms, a novel classification

INJ properties:
- INJ(Y, Y)(f) is constant across all Y in the same STRANS2-class, for fixed f in STRANS1
- This enables set-theoretic analysis using either group as CANON

# Musical Context/Application
The choice of canonical group determines what counts as "the same" set class in analysis. Using STRANS1 gives familiar results; using STRANS2 may reveal relationships invisible to standard analysis. The dual structure suggests both perspectives are equally valid for different analytical purposes.

# Examples
**From the text:**
"STRANS2 and STRANS1, which figure as groups of interval-preserving operations in those respective GIS structures, are thereby also likely candidates for CANONical groups of operations in a variety of set-theoretical studies."

**STRANS1 as CANON:**
"The STRANS1-forms of a set within S are exactly the dodecaphonically transposed and inverted forms of the set that lie within S."

This produces familiar equivalence: two octatonic subsets are equivalent iff related by T or I operations.

**STRANS2 as CANON:**
"The STRANS2-forms of a set within S are in general a more novel sort of family."

Example with (C, E, G):
- STRANS2-forms: (C, E, G), (D#, C#, E), (F#, A#, C#), (A, G, A#), (C#, D#, F#), (A#, F#, A), (F#, C, D#), (G, A, C)
- These eight sets are STRANS2-equivalent, though not all are T/I-related

**INJ invariance:**
"If Y is any one of those eight sets, and Y' is any other one, and f is any one of the eight operations in STRANS1, then... INJ(Y, Y)(f) = INJ(Y', Y')(f)."

This demonstrates that STRANS2-equivalence preserves important set-theoretic properties with respect to STRANS1 operations.

# Related Concepts
- STRANS-Forms
- Set Class
- STRANS1 Group
- STRANS2 Group
- INJ Function
- Octatonic Analysis

# Common Confusions
The choice of canonical group is not a matter of correctness but of analytical purpose. STRANS1 gives familiar equivalences; STRANS2 reveals additional structure. Students should recognize that multiple valid canonical groups can exist for the same musical space.

# Source Reference
Appendix B: Non-Commutative Octatonic GIS Structures; More on Simply Transitive Groups
