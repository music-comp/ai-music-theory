---
concept: PETEY Group
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Interval Systems (2): Formal Features"
chapter_number: 3
pdf_page: 62
unit: null
authors: David Lewin
---

# Quick Definition
PETEY is the group of all operations on a GIS space S that can be expressed as PT, where P is an interval-preserving operation and T is a transposition. It combines the two fundamental families of "direct" transformations.

# Formal Definition
**Theorem 3.5.11(A):** Let PETEY be the family of all operations on S that can be expressed as (functionally equal to) something of form PT, where P is some interval-preserving operation and T is some transposition. Then PETEY is a group of operations.

# Mathematical Formulation
**Closure proof:**
Let PT and P'T' be members of PETEY.
Set P'' = PP' and T'' = TT'.
Since P and T families are groups, P'' is interval-preserving and T'' is a transposition.
(PT)(P'T') = P(TP')T' = P(P'T)T' (by Theorem 3.4.10: T commutes with P)
           = (PP')(TT') = P''T''

So PETEY is closed under composition.

**Inverse proof:**
Given PT in PETEY:
P^(-1) is interval-preserving, T^(-1) is a transposition.
P^(-1)T^(-1) is in PETEY.
(PT)(P^(-1)T^(-1)) = P(TP^(-1))T^(-1) = P(P^(-1)T)T^(-1) = (PP^(-1))(TT^(-1)) = identity.

# Musical Context/Application
PETEY represents all "direct" transformations that do not involve inversion. In commutative GIS where P = T, PETEY reduces to the transposition group. In non-commutative GIS, PETEY is larger and has a richer structure.

The name "PETEY" comes from combining "P" (interval-preserving) and "T" (transposition), with the suffix making it pronounceable.

# Examples
**Commutative GIS:**
PETEY = TNSPS = PSVS (all three groups coincide)
Every element is both a transposition and interval-preserving.

**Non-commutative time-span GIS:**
PETEY contains all P(h,u) T(i,p) operations.
These form a larger group than either TNSPS or PSVS alone.
P and T operations do not coincide, but they commute with each other.

**Group structure:**
PETEY = PSVS * TNSPS (internal product)
Since P and T commute, this is actually PSVS x TNSPS as sets, but the group structure combines them.

# Related Concepts
- Interval-Preserving Operation (Pi)
- Transposition Operation (Ti)
- PETINV Group
- Commutativity of T with P
- Group of Operations

# Common Confusions
1. **PETEY vs. TNSPS:** In commutative GIS these are equal. In non-commutative GIS, PETEY is strictly larger.

2. **Order in PT:** The formula is P followed by T, but since P and T commute (Theorem 3.4.10), PT = TP as operations.

3. **PETEY is a group:** Closure requires the commutativity of P and T to rearrange (PT)(P'T') into (PP')(TT').

4. The proof uses Theorem 3.4.10 essentially--without the commutativity of T and P, PETEY might not be a group.

# Source Reference
Chapter 3: Generalized Interval Systems (2): Formal Features, Theorem 3.5.11(A), pp. 89-90
