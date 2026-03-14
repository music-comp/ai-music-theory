---
concept: Interval Group Isomorphisms
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
The isomorphism between $(\mathbb{R}, +)$ and $(\mathbb{R}^+, \cdot)$ established by the exponential and logarithm functions, which is precisely the mathematical statement that additive and multiplicative interval measurements are equivalent.

# Formal Definition
For any base $b \in \mathbb{R}^+$, the function $f: \mathbb{R} \to \mathbb{R}^+$ defined by $f(r) = b^r$ and its inverse $g: \mathbb{R}^+ \to \mathbb{R}$ defined by $g(x) = \log_b x$ are group isomorphisms between $(\mathbb{R}, +)$ and $(\mathbb{R}^+, \cdot)$. These satisfy $f(r + s) = f(r) \cdot f(s)$ and $g(xy) = g(x) + g(y)$.

# Mathematical Context
These isomorphisms show that $(\mathbb{R}, +)$ and $(\mathbb{R}^+, \cdot)$ are the same abstract group with different concrete representations. The exponential map converts additive structure to multiplicative, and the logarithm converts back. The homomorphism property of $f$ is the familiar law of exponents $b^{r+s} = b^r \cdot b^s$, and for $g$ it is the logarithm product rule $\log_b(xy) = \log_b x + \log_b y$.

# Musical Context
The set of musical intervals can be identified with EITHER $(\mathbb{R}, +)$ (additive measurement: cents, semitones, octaves) OR $(\mathbb{R}^+, \cdot)$ (multiplicative measurement: frequency ratios). The isomorphism $f$ converts from additive to multiplicative measurement, and $g$ converts back. The identity element is the unison (0 additively, 1 multiplicatively), and the inverse of an interval is its opposite ($-r$ additively, $1/x$ multiplicatively).

# Examples
- With $b = 2$ (octaves): $f(1) = 2$ (1 octave $\to$ ratio 2), $g(3/2) = \log_2(3/2) \approx 0.585$ (ratio 3/2 $\to$ 0.585 octaves)
- $f$ maps the identity $0 \in \mathbb{R}$ to the identity $1 \in \mathbb{R}^+$: $f(0) = b^0 = 1$
- Adding intervals (cents) corresponds to multiplying ratios: 700 cents + 500 cents = 1200 cents maps to $2^{7/12} \cdot 2^{5/12} = 2^1 = 2$
- $g$ maps ratio 2 to 1200 cents (with appropriate base): using base $2^{1/1200}$

# Related Concepts
- Isomorphism
- Homomorphism
- Group of Intervals
- Group

# Common Confusions
- The base $b$ determines the unit of additive measurement; $b = 2$ gives octaves, $b = 2^{1/12}$ gives semitones, $b = 2^{1/1200}$ gives cents
- The isomorphism is between the GROUPS, not the sets; the same sets with different operations would not be isomorphic
- This is not just a "convenient conversion"; it is a deep structural equivalence

# Source Reference
Chapter 7, "The Group of Intervals" section, p. 82 (PDF)
