---
concept: Time-Span GIS Properties Summary
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
The time-span GIS exhibits several distinctive properties due to its non-commutativity: only the identity interval is central, transpositions don't preserve intervals, inversions are maximally rigid, and no interval-reversing operations exist.

# Formal Definition
**Notes 4.1.7 Summary:**

(E) The only central member of IVLS is (0, 1).

(F) No transposition preserves intervals, and no interval-preserving operation is a transposition, except for the identity T(0,1) = P(0,1).

(H) I_s^t = I_{s'}^{t'} iff s' = s AND t' = t.

(I) There are no interval-reversing operations on TMSPS.

# Mathematical Formulation
**Centrality (E):**
(i, p) is central iff (i, p)(j, q) = (j, q)(i, p) for all (j, q).
(i + pj, pq) = (j + qi, qp) for all j, q implies p = 1 and i = 0.

**Transposition/Interval-preserving (F):**
By Theorem 3.4.8, Ti = Pi iff i is central.
Since only (0, 1) is central, only T(0,1) = P(0,1) = identity.

**Inversion rigidity (H):**
By Theorem 3.5.3, I_s^t = I_{s'}^{t'} iff t' = I_s^t(s') and int(s', s) is central.
Since only (0, 1) is central, s' = s (int(s', s) = e implies s' = s).
Then t' = I_s^t(s) = t.

**No interval-reversal (I):**
This restates Theorem 3.6.4 for the time-span case.

# Musical Context/Application
These properties show how radically different the time-span GIS is from familiar pitch-class theory:

| Property | Pitch-class GIS | Time-span GIS |
|----------|-----------------|---------------|
| Central intervals | All | Only identity |
| T preserves intervals | Always | Never (except identity) |
| T = P | Always | Never (except identity) |
| I_u^v = I_v^u | Always | Never (except u = v) |
| # distinct inversions | n | n^2 |
| Interval-reversing | = Inversions | Don't exist |

# Examples
**Centrality failure:**
(1, 2)(0, 3) = (1, 6)
(0, 3)(1, 2) = (3, 6)
So (1, 2) is not central.

**Transposition interval distortion:**
s = (0, 1), t = (3, 1), int(s, t) = (3, 1)
T(1,2)(s) = (1, 2), T(1,2)(t) = (5, 2)
int(T(1,2)(s), T(1,2)(t)) = ((5-1)/2, 1) = (2, 1) =/= (3, 1)

**Inversion uniqueness:**
In 12-tone pitch classes: I_0 = I_6 = ... (many notations for same operation)
In time-spans: I_{(0,1)}^{(1,1)} =/= I_{(0,1)}^{(1,2)} =/= I_{(0,2)}^{(1,1)} (all different)

# Related Concepts
- Time-Span GIS
- Central Interval
- Transposition and Interval Preservation
- Interval-Reversing Operation
- Non-Commutative GIS

# Common Confusions
1. **These are consequences of non-commutativity:** The strange properties follow from IVLS being non-commutative, not from any peculiarity of time spans per se.

2. **"Transposition" means something different:** In time-span GIS, T(i,p) is still the operation moving each element by interval (i, p). But it doesn't behave like pitch transposition.

3. **Inversions are uniquely determined:** There's no "simplifying" I_{(a,x)}^{(b,y)} to a simpler notation. Each parameter pair is essential.

4. **Figure 4.4 revisited:** The figure shows equal intervals int(s1, t1) = int(s2, t2), but s1 precedes s2 while t1 follows t2. Transposition doesn't preserve order!

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Notes 4.1.7(E)-(I), pp. 113-114
