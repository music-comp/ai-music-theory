---
concept: Time-Span GIS Uniqueness
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (3): A Non-Commutative GIS"
chapter_number: 4
pdf_page: 91
unit: null
authors: David Lewin
---

# Quick Definition
The time-span GIS 4.1.3 is essentially unique: it is the only GIS on time spans (up to isomorphism) whose interval function is independent of both the referential time-point zero and the referential time-unit.

# Formal Definition
**Theorem 4.1.5:** Let GIS' = (TMSPS, IVLS', int') be any GIS with time spans for its objects that also enjoys Properties (A) and (B) of Theorem 4.1.4. Then the group IVLS of GIS 4.1.3 and the group IVLS' of GIS' are isomorphic via a map f such that, for all time spans s and t:

int'(s, t) = f(int(s, t))

# Mathematical Formulation
**Proof outline:**
Define f: IVLS -> IVLS' by f(i, p) = LABEL'(i, p) = int'((0, 1), (i, p))

Using the independence properties:
f(i, p)^(-1) f(j, q) = int'((i, p), (j, q))     [by 3.1.2]
                     = int'((0, p), (j-i, q))   [Property A: shift by -i]
                     = int'((0, 1), ((j-i)/p, q/p)) [Property B: scale by 1/p]
                     = f((j-i)/p, q/p)
                     = f((i, p)^(-1)(j, q))    [by Lemma 4.1.6.2]

By Lemma 4.1.6.1, f is a homomorphism. Since LABEL' is bijective, f is an isomorphism.

Finally: f(int(s, t)) = int'(s, t) by tracing through the definitions.

# Musical Context/Application
This theorem gives GIS 4.1.3 a privileged theoretical status: if you want a GIS on time spans that doesn't depend on arbitrary reference choices, you must (essentially) use this one.

The uniqueness result validates the non-commutative structure as not merely an arbitrary choice but a necessary consequence of requiring reference-independence.

# Examples
**Alternative GIS:**
Suppose someone proposes a different time-span GIS with IVLS' and int'. If their GIS also has properties (A) and (B), Theorem 4.1.5 guarantees:
1. IVLS' is isomorphic to IVLS
2. int' is just int composed with the isomorphism

**What the isomorphism means:**
Any "reference-independent" time-span interval theory will have the same algebraic structure. Different formulations may use different notation, but they all encode the same information.

**Non-example:**
GIS 4.1.2 with int((a,x),(b,y)) = (b-a, y/x) does NOT have property (B). It is a valid GIS, but not reference-independent. Theorem 4.1.5 doesn't apply.

# Related Concepts
- Time-Span Interval Independence
- Time-Span GIS
- Group Isomorphism
- LABEL Function
- Reference Independence

# Common Confusions
1. **"Essentially unique" vs. unique:** Different GIS may use different IVLS' groups, but they're all isomorphic. The structure is unique; the presentation may vary.

2. **The isomorphism f:** It's defined using the LABEL' function of the alternative GIS. The proof shows this f is automatically a group isomorphism.

3. **Significance:** This isn't just showing one nice GIS exists. It shows there's only one (up to isomorphism) with the desired properties.

4. **Why non-commutative?** The uniqueness theorem doesn't impose non-commutativity. Rather, the independence properties FORCE the resulting IVLS to be non-commutative.

# Source Reference
Chapter 4: Generalized Interval Systems (3): A Non-Commutative GIS, Theorem 4.1.5 and Lemmas 4.1.6.1-4.1.6.2, pp. 110-112
