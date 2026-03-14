---
concept: INJ Complement Theorem
category: theory
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (2): The Injection Function"
chapter_number: 6
pdf_page: 154
unit: null
authors: David Lewin
---

# Quick Definition
When S is finite, INJ values for a set and its complement are related by specific formulas, generalizing Babbitt's hexachord theorem.

# Formal Definition
Theorem 6.6.1: Suppose S is finite. Given sets X and Y with complements X-bar and Y-bar; given any operation OP; then:
- (A): INJ(X, Y-bar)(OP) = cardX - INJ(X, Y)(OP)
- (B): INJ(X, Y-bar)(OP) = cardY-bar - INJ(X, Y)(OP)
- (C): INJ(X-bar, Y-bar)(OP) = cardY-bar - cardX + INJ(X, Y)(OP)
- (D): If cardY = cardX, then INJ(X-bar, Y-bar)(OP) = INJ(X, Y)(OP)
- (E): If cardX = 1/2 cardS, then INJ(X, X)(OP) = INJ(X-bar, X-bar)(OP)

Formula (E) is the Generalized Babbitt Hexachord Theorem.

# Mathematical Formulation
Proofs (following Regener's methods):

(A): OP maps each element of X either into Y or into Y-bar.
cardX = INJ(X, Y)(OP) + INJ(X, Y-bar)(OP)
Rearranging gives formula (A).

(B): Uses Corollary 6.5.2 and formula (A).
INJ(X, Y-bar)(OP) = INJ(Y-bar, X)(OP^-1)
= cardY-bar - INJ(Y-bar, X-bar)(OP^-1)
= cardY-bar - INJ(X, Y)(OP)

(C): Combine (A) applied to X-bar and algebra.

(D): Special case of (C) when cardX = cardY.

(E): Special case of (D) when Y = X.

# Musical Context/Application
Formula (E) explains why complementary hexachords have the same interval vector (when OP ranges over transpositions). More generally, it shows that half-space sets and their complements share the same INJ profile for any operation, explaining deep symmetries in twelve-tone music.

# Examples
Babbitt's Hexachord Theorem (special case):
- Let X be a hexachord, X-bar its complement
- For any T_i: INJ(X, X)(T_i) = INJ(X-bar, X-bar)(T_i)
- Since INJ(X, X)(T_i) = IFUNC(X, X)(i) (Theorem 6.7.1)
- We get: IFUNC(X, X)(i) = IFUNC(X-bar, X-bar)(i)

That is, complementary hexachords have identical interval vectors.

More general application:
- In any finite S with |S| = 2N
- Any N-element set X has INJ(X, X)(OP) = INJ(X-bar, X-bar)(OP)
- Works for any operation OP, not just transpositions

# Related Concepts
- INJ (Injection Function)
- Hexachord Theorem (Babbitt)
- Complementation
- IFUNC (Interval Function)

# Common Confusions
These formulas require OP to be an operation (not just any transformation). For non-operations, the complement relationships become more complex. Also, S must be finite for complements to be "sets" in our sense.

# Source Reference
Chapter 6: Generalized Set Theory (2): The Injection Function, Theorem 6.6.1
