---
concept: Isomorphism
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
A bijective homomorphism between two groups, establishing that they have identical algebraic structure despite possibly different elements and operations.

# Formal Definition
A homomorphism $\varphi: G \to G'$ is an isomorphism if it is bijective (one-to-one and onto). In this case, the inverse function $\varphi^{-1}: G' \to G$ is also an isomorphism. Two groups $G$ and $G'$ are called isomorphic if there exists an isomorphism between them.

# Mathematical Context
Isomorphic groups are algebraically indistinguishable: they have the same number of elements, the same structure of subgroups, and the same behavior under composition. The groups $(\mathbb{R}, +)$ and $(\mathbb{R}^+, \cdot)$ are isomorphic via $f(r) = b^r$ and $g(x) = \log_b x$. Any cyclic group of order $m$ is isomorphic to $\mathbb{Z}_m$, and any infinite cyclic group is isomorphic to $\mathbb{Z}$.

# Musical Context
The isomorphism $(\mathbb{R}, +) \cong (\mathbb{R}^+, \cdot)$ is the mathematical statement that additive interval measurement (cents, semitones) and multiplicative interval measurement (frequency ratios) are equivalent ways of describing the same algebraic structure. The exponential and logarithm functions are precisely the conversions between these two representations of the group of intervals.

# Examples
- $f(r) = 2^r$: isomorphism from $(\mathbb{R}, +)$ to $(\mathbb{R}^+, \cdot)$, converting semitone/octave measure to frequency ratio
- $g(x) = \log_2 x$: its inverse isomorphism, converting frequency ratio to additive measure
- $\varphi: \{1, -1\} \to \mathbb{Z}_2$ with $\varphi(1) = [0]$, $\varphi(-1) = [1]$: isomorphism
- The wrapping function $w: \mathbb{R} \to \mathbb{R}/{\sim}$ is a homomorphism but NOT an isomorphism (not one-to-one)

# Related Concepts
- Homomorphism
- Group
- Group of Intervals
- Cyclic Group and Generator

# Common Confusions
- Isomorphism requires bijectivity; a homomorphism that is onto but not one-to-one (like the wrapping function) is not an isomorphism
- Two isomorphic groups have identical algebraic properties, even though their elements may look completely different (e.g., real numbers under addition vs. positive reals under multiplication)
- The existence of an isomorphism is a property of the pair of groups, not of a specific function

# Source Reference
Chapter 7, "Homomorphism" section, p. 82 (PDF)
