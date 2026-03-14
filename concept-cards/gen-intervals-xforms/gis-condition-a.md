---
concept: GIS Condition A
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (1): Preliminary Examples and Definition"
chapter_number: 2
pdf_page: 47
unit: null
authors: David Lewin
---

# Quick Definition
Condition (A) states that intervals compose along paths: the interval from r to s, combined with the interval from s to t, equals the interval from r to t.

# Formal Definition
For all r, s, and t in S: int(r, s) * int(s, t) = int(r, t). This condition ensures that the interval function respects path concatenation. If we traverse from r to s and then from s to t, the total interval accumulated equals the direct interval from r to t.

# Mathematical Formulation
- int(r, s) * int(s, t) = int(r, t) for all r, s, t in S
- Using group operation in IVLS (written multiplicatively or additively)
- This is analogous to vector addition: displacement from A to B plus displacement from B to C equals displacement from A to C
- Condition (A) alone implies: int(s, s) = e and int(t, s) = int(s, t)^(-1)

# Musical Context/Application
Condition (A) captures the basic intuition that intervals "add up" along a path. Going up a major third (4 semitones) then up a minor third (3 semitones) equals going up a perfect fifth (7 semitones). This property must hold for any system we want to call an "interval system."

# Examples
Chromatic pitch example:
- int(C4, E4) = 4 (major third up)
- int(E4, G4) = 3 (minor third up)
- int(C4, G4) = 7 (perfect fifth up)
- Verification: 4 + 3 = 7, confirming Condition (A)

Pitch-class example:
- int(C, F#) = 6
- int(F#, C) = 6
- int(C, C) = 0
- Verification: 6 + 6 = 12 = 0 mod 12, confirming Condition (A)

Traditional interval arithmetic fails: A "3rd" plus a "3rd" gives a "5th" (3 + 3 = 5???). GIS-style numbering fixes this: 2 + 2 = 4 (two scale steps plus two scale steps equals four scale steps).

# Related Concepts
- Generalized Interval System
- Interval Function int
- GIS Condition B
- Theorem 2.3.2
- Path Composition

# Common Confusions
- Condition (A) alone doesn't fully determine a GIS - Condition (B) is also needed
- The group operation in IVLS (+ or *) determines how intervals combine
- Traditional interval names ("3rd + 3rd = 5th") violate Condition (A) - GIS uses different numbering
- Condition (A) implies but doesn't state int(s, s) = e directly

# Source Reference
Chapter 2: Generalized Interval Systems (1): Preliminary Examples and Definition, Definition 2.3.1(A), discussion following
