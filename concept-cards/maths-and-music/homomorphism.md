---
concept: Homomorphism
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
unit: null
---

# Quick Definition
A function between two groups that preserves the group operation, mapping the composition of elements in one group to the composition of their images in the other.

# Formal Definition
Given two groups $(G, \cdot)$ and $(G', \circ)$, a function $\varphi: G \to G'$ is a group homomorphism if for all $x, y \in G$, $\varphi(x \cdot y) = \varphi(x) \circ \varphi(y)$. A homomorphism necessarily maps the identity of $G$ to the identity of $G'$: $\varphi(e) = e'$.

# Mathematical Context
Homomorphisms preserve algebraic structure. Key properties: $\varphi(e) = e'$ and $\varphi(x^{-1}) = \varphi(x)^{-1}$. A homomorphism can be onto (surjective) without being one-to-one (injective), or vice versa. The wrapping function $w: \mathbb{R} \to \mathbb{R}/{\sim}$ defined by $w(x) = \bar{x}$ is a homomorphism that is onto but not one-to-one.

# Musical Context
The functions converting between additive and multiplicative interval measurement are homomorphisms. The exponential $f(r) = b^r$ is a homomorphism from $(\mathbb{R}, +)$ to $(\mathbb{R}^+, \cdot)$, and the logarithm $g(x) = \log_b x$ is its inverse. These correspond to converting between additive measurement (cents, semitones) and multiplicative measurement (frequency ratios). The wrapping function models octave equivalence.

# Examples
- $f(r) = b^r$: homomorphism from $(\mathbb{R}, +)$ to $(\mathbb{R}^+, \cdot)$ since $b^{r+s} = b^r \cdot b^s$
- $g(x) = \log_b x$: homomorphism from $(\mathbb{R}^+, \cdot)$ to $(\mathbb{R}, +)$ since $\log_b(xy) = \log_b x + \log_b y$
- $w: \mathbb{R} \to \mathbb{R}/{\sim}$, $w(x) = \bar{x}$: homomorphism (onto but not one-to-one)
- $\varphi: \{1, -1\} \to \mathbb{Z}_2$ with $\varphi(1) = [0]$, $\varphi(-1) = [1]$: homomorphism and isomorphism

# Related Concepts
- Isomorphism
- Group
- Group of Intervals
- Modular Equivalence on the Real Numbers

# Common Confusions
- A homomorphism preserves structure but need not be bijective; only an isomorphism is both one-to-one and onto
- The wrapping function $w$ is a homomorphism but not an isomorphism because distinct real numbers can map to the same equivalence class
- Homomorphism is a property of the function relative to the group operations; the same function between different group structures may or may not be a homomorphism

# Source Reference
Chapter 7, "Homomorphism" section, p. 82 (PDF)
