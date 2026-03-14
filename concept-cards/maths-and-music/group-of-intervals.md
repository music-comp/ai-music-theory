---
concept: Group of Intervals
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
The set of all musical intervals forming a group under composition, identified with $(\mathbb{R}, +)$ in additive measurement or $(\mathbb{R}^+, \cdot)$ in multiplicative measurement, with these two representations being isomorphic.

# Formal Definition
The set of musical intervals forms a group under interval composition, where: the identity element is the unison interval, the inverse of an interval is its opposite interval, and composition is associative. Using additive measurement (semitones, cents), the group is $(\mathbb{R}, +)$. Using multiplicative measurement (frequency ratios), the group is $(\mathbb{R}^+, \cdot)$. The isomorphisms $f(r) = b^r$ and $g(x) = \log_b x$ convert between these representations.

# Mathematical Context
The exponential function $f: (\mathbb{R}, +) \to (\mathbb{R}^+, \cdot)$ defined by $f(r) = b^r$ satisfies $f(r + s) = b^{r+s} = b^r \cdot b^s = f(r) \cdot f(s)$, confirming it is a homomorphism. Its inverse $g(x) = \log_b x$ satisfies $g(xy) = \log_b(xy) = \log_b x + \log_b y = g(x) + g(y)$. Since both are bijective, they are isomorphisms, proving $(\mathbb{R}, +) \cong (\mathbb{R}^+, \cdot)$.

# Musical Context
This isomorphism is exactly the conversion between cents/semitones and frequency ratios. Adding cents corresponds to multiplying frequency ratios: going up 700 cents then 500 cents (= 1200 cents = octave) corresponds to multiplying $2^{7/12} \cdot 2^{5/12} = 2$ (ratio of 2 = octave). The group structure captures the everyday musical intuition that intervals can be combined, reversed, and that the order of combination doesn't matter.

# Examples
- Unison: additive identity $0$, multiplicative identity $1$
- Octave: $1200$ cents additively, ratio $2$ multiplicatively
- Opposite of a fifth up (700 cents): a fifth down ($-700$ cents), ratio $2^{-7/12} = 1/2^{7/12}$
- Fifth + fourth = octave: $700 + 500 = 1200$ cents, or $\frac{3}{2} \cdot \frac{4}{3} = 2$ (in just intonation)

# Related Concepts
- Group
- Isomorphism
- Homomorphism
- Group of Modular Intervals
- Modular Chromatic Intervals

# Common Confusions
- The group of intervals is an abelian (commutative) group; the order of interval composition does not matter
- The additive and multiplicative representations are different descriptions of the same algebraic structure, not different groups
- This group is $(\mathbb{R}, +)$, which is NOT cyclic (unlike $\mathbb{Z}_{12}$); it contains all possible intervals, not just chromatic ones

# Source Reference
Chapter 7, "The Group of Intervals" section, p. 82 (PDF)
