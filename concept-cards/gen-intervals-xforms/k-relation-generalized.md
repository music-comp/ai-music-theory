---
concept: "K and Kh Relations (Generalized)"
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
The K and Kh relations generalize Forte's inclusion relations using INJ: K holds when INJ(X, Y)(A) takes extreme values (maximum or minimum) for some canonical operation A.

# Formal Definition
Subject to cardinality restrictions (cardX <= card(complement of X), cardX <= cardY <= card(complement of Y)):

- K1 relation: Some form of X is embedded in Y
  K'1: For some A in CANON, INJ(X, Y)(A) = cardX

- K2 relation: Some form of X is disjoint from Y
  K'2: For some B in CANON, INJ(X, Y)(B) = 0

- K relation: K1 or K2 holds
- Kh relation: Both K1 and K2 hold

# Mathematical Formulation
Using progressive/dispersive terminology:

K: INJ(X, Y)(A) takes either its theoretical maximum (cardX) or its theoretical minimum (0), as A varies over CANON.

Kh: INJ(X, Y)(A) takes both its maximum and minimum values, as A varies over CANON.

K'1 (maximally progressive): CANON contains some transformation mapping all of X into Y.

K'2 (maximally dispersive): CANON contains some transformation mapping X entirely outside Y.

# Musical Context/Application
The K and Kh relations measure the range of possible relationships between set classes. Kh means both maximal similarity (embedding) and maximal dissimilarity (disjointness) are possible under canonical operations. This reveals the full scope of how two set classes can relate.

# Examples
In pitch-class space with CANON = T_i and I_j:

Y = black-note pentatonic scale = {F#, G#, A#, C#, D#}
X1 = F#-major triad = {F#, A#, C#}
X2 = {Ab, Bb, Db}

For X1 and Y:
- K1: T_0(X1) is subset of Y (embedded)
- K2: T_1(X1) = {G, B, D} is disjoint from Y

X1 and Y are Kh-related with multiplicity (2, 6):
- 2 operations embed X1 in Y
- 6 operations make X1 disjoint from Y

For X2 = {Ab, Bb, Db} (= T_1(X1) = same set class):
X2 and Y are Kh-related with multiplicity (4, 8):
- Different specific set, same set class
- Multiplicities depend on specific sets, not just classes

# Related Concepts
- INJ (Injection Function)
- Progressive Transformation
- Dispersive Transformation
- EMB (Embedding Function)
- Set Class

# Common Confusions
The generalized K and Kh relations depend on CANON. Different canonical groups yield different K/Kh relationships. Also, multiplicities of K1-ness and K2-ness can be computed when CANON is finite, giving more refined information than just K/Kh.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, section 6.8
