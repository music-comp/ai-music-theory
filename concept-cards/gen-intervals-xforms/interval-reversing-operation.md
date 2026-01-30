---
concept: Interval-Reversing Operation
category: transformation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
An interval-reversing transformation maps elements so that the interval between images is the reverse (inverse) of the interval between originals. In commutative GIS, inversions are interval-reversing; in non-commutative GIS, no interval-reversing transformations exist.

# Formal Definition
A transformation Y on the space S of a GIS is called "interval-reversing" if for all s and all t in S:

int(Y(s), Y(t)) = int(t, s)

Note: int(t, s) = int(s, t)^(-1), so this says the image interval is the inverse of the original interval.

# Mathematical Formulation
**Definition 3.6.1:**
Y is interval-reversing if int(Y(s), Y(t)) = int(t, s) for all s, t.

**Lemma 3.6.2:** If Y is interval-reversing, then there exists interval i such that:
LABEL(Y(t)) = i * (LABEL(t))^(-1) for all t.

**Theorem 3.6.3:** In commutative GIS, the inversion operations reverse intervals, and every interval-reversing transformation is some inversion operation.

**Theorem 3.6.4:** In non-commutative GIS, there exists no interval-reversing transformation on S.

# Musical Context/Application
The intuition that inversion "reverses intervals" is formalized here. When we invert a melody, ascending intervals become descending and vice versa. This is precisely interval-reversal.

However, this property depends crucially on commutativity. In non-commutative GIS (like time-spans), the concept of "reversing intervals" cannot be consistently implemented.

# Examples
**Pitch-class inversion is interval-reversing:**
- Let I = I_0 (inversion about C)
- int(C, E) = 4, int(I(C), I(E)) = int(C, Ab) = 8 = -4 mod 12
- int(E, C) = -4 mod 12 = 8
- So int(I(C), I(E)) = int(E, C) as required

**Proof for commutative case:**
int(I_u^v(s), I_u^v(t)) = LABEL(I_u^v(t))^(-1) LABEL(I_u^v(s))
                        = (i LABEL(t)^(-1) j)^(-1) (i LABEL(s)^(-1) j)
                        = j^(-1) LABEL(t) i^(-1) i LABEL(s)^(-1) j
                        = LABEL(s)^(-1) LABEL(t) (using commutativity!)
                        = int(t, s)

**Non-commutative failure:**
In time-span GIS, no transformation can reverse intervals. The proof derives a contradiction from the assumption that such Y exists.

# Related Concepts
- Interval-Preserving Operation
- Inversion Operation (I_u^v)
- Commutative vs. Non-commutative GIS
- LABEL Function

# Common Confusions
1. **Reversing vs. preserving:** Interval-preserving: int(X(s), X(t)) = int(s, t). Interval-reversing: int(Y(s), Y(t)) = int(t, s) = int(s, t)^(-1).

2. **Only in commutative GIS:** The theorem that inversions reverse intervals requires commutativity in its proof. This fails completely in non-commutative GIS.

3. **No interval-reversing ops in non-commutative GIS:** This is a strong negative result (Theorem 3.6.4). It means "inversion" in non-commutative GIS cannot be characterized by interval-reversal.

4. **The Lemma:** Any interval-reversing Y must have the form LABEL(Y(t)) = i * LABEL(t)^(-1), which matches the inversion formula (with j = e).

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Definitions 3.6.1-3.6.4, pp. 90-92
