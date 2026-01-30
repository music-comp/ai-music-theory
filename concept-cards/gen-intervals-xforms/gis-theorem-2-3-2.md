---
concept: GIS Theorem 2.3.2
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Theorem 2.3.2 proves that in any GIS, int(s, s) = e (identity) and int(t, s) = int(s, t)^(-1) follow automatically from Condition (A) and the group structure.

# Formal Definition
Theorem 2.3.2: In any GIS, int(s, s) = e and int(t, s) = int(s, t)^(-1) for every s and t in S.

These properties were intuitively expected but need not be stated separately in the GIS definition because they follow logically from Condition (A) and the group structure of IVLS.

# Mathematical Formulation
**Proof of int(s, s) = e:**
1. By Condition (A): int(s, s) * int(s, s) = int(s, s)
2. Multiply both sides by int(s, s)^(-1)
3. We obtain: int(s, s) = e

**Proof of int(t, s) = int(s, t)^(-1):**
1. By Condition (A): int(s, t) * int(t, s) = int(s, s) = e
2. Multiply both sides on the left by int(s, t)^(-1)
3. We obtain: int(t, s) = int(s, t)^(-1)

# Musical Context/Application
This theorem confirms our musical intuitions:
- The interval from any note to itself is zero (identity) - this is the unison
- The interval from t to s is the reverse (inverse) of the interval from s to t - going up a fifth inverts to going down a fifth

These properties are essential for the coherence of interval analysis and transformation theory.

# Examples
Chromatic pitch space:
- int(C4, C4) = 0 (unison)
- int(C4, G4) = 7, int(G4, C4) = -7, and 7 + (-7) = 0 = identity

Pitch-class space (mod 12):
- int(C, C) = 0
- int(C, G) = 7, int(G, C) = 5, and 7 + 5 = 12 = 0 mod 12

Just intonation:
- int(C4, C4) = 1 (ratio identity)
- int(C4, G4) = 3/2, int(G4, C4) = 2/3, and (3/2)(2/3) = 1

# Related Concepts
- GIS Condition A
- Generalized Interval System
- Identity Element
- Inverse Element
- Group

# Common Confusions
- These properties need not be stated as axioms - they are theorems
- The proof relies on Condition (A) and group structure only
- int(s, s) = e is NOT Condition (B) - that's about existence/uniqueness
- The inverse relation int(t, s) = int(s, t)^(-1) formalizes "direction reversal"

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Theorem 2.3.2 and proof
