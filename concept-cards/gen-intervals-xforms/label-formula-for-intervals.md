---
concept: LABEL Formula for Intervals
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
The LABEL formula expresses intervals between elements in terms of their LABELs: int(s, t) = LABEL(s)^(-1) * LABEL(t). This allows interval computation using labels without direct reference to the int function.

# Formal Definition
**Theorem 3.1.2:** Given a GIS (S, IVLS, int) and reference point ref, the LABEL function satisfies:

int(s, t) = LABEL(s)^(-1) * LABEL(t)

This formula holds regardless of which ref was chosen for LABELing.

# Mathematical Formulation
**Derivation:**
LABEL(s) = int(ref, s)
LABEL(t) = int(ref, t)

int(s, t) = int(s, ref) * int(ref, t)    [GIS condition A]
          = int(ref, s)^(-1) * int(ref, t)    [inverse property]
          = LABEL(s)^(-1) * LABEL(t)

**Computational use:**
To find int(s, t):
1. Compute LABEL(s) and LABEL(t)
2. Invert LABEL(s) in the group
3. Multiply LABEL(s)^(-1) by LABEL(t)

# Musical Context/Application
This formula is the computational workhorse for GIS calculations:
- Given labels, compute intervals directly
- No need to invoke the int function definition
- Works uniformly across all GIS structures

It also justifies the familiar practice of computing pitch-class intervals by subtraction: if LABEL(E) = 4 and LABEL(G) = 7, then int(E, G) = -4 + 7 = 3 (mod 12).

# Examples
**Pitch-class example:**
ref = C, so LABEL is pitch-class integer
LABEL(E) = 4, LABEL(G) = 7
int(E, G) = LABEL(E)^(-1) * LABEL(G) = (-4) + 7 = 3 (mod 12)

**Time-span example:**
ref = (0, 1), so LABEL(a, x) = (a, x)
LABEL(2, 3) = (2, 3), LABEL(5, 6) = (5, 6)
int((2, 3), (5, 6)) = (2, 3)^(-1) * (5, 6)
                    = (-2/3, 1/3) * (5, 6)
                    = (-2/3 + (1/3)*5, (1/3)*6)
                    = (1, 2)

**Verification:**
int((2, 3), (5, 6)) = ((5-2)/3, 6/3) = (1, 2) ✓

# Related Concepts
- LABEL Function
- Reference Point (ref)
- Interval Computation
- Group Inverse
- Transposition via LABEL (Theorem 3.4.3)

# Common Confusions
1. **Order matters:** It's LABEL(s)^(-1) * LABEL(t), not LABEL(t)^(-1) * LABEL(s).

2. **In commutative groups:** LABEL(s)^(-1) * LABEL(t) = LABEL(t) * LABEL(s)^(-1), so order doesn't matter there. But the formula as stated works universally.

3. **Different refs, same interval:** Different reference points give different labels but the same interval int(s, t).

4. **The formula proves LABEL is bijective:** Since we can recover int from LABELs, and int determines the GIS, LABEL carries full information.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.1.2, pp. 62-63
