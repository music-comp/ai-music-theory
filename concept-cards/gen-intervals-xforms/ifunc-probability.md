---
concept: IFUNC as Probability Distribution
category: mathematical-foundation
source: Generalized Musical Intervals and Transformations
chapter: "Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions"
chapter_number: 5
pdf_page: 119
unit: null
authors: David Lewin
---

# Quick Definition
IFUNC can be interpreted as a probability distribution that models the statistical likelihood of encountering various intervals when randomly selecting elements from two sets.

# Formal Definition
Theorem 5.1.8: Let X and Y have respective cardinalities M and N. Select a member s of X at random and a member t of Y at random. Then the number IFUNC(X, Y)(i)/(MN) measures the probability that int(s, t) will be found to equal i.

# Mathematical Formulation
Given sets X with |X| = M and Y with |Y| = N:

P(int(s, t) = i) = IFUNC(X, Y)(i) / (M * N)

Properties:
- Sum over all i of IFUNC(X, Y)(i) = M * N (total number of pairs)
- Sum over all i of P(int(s, t) = i) = 1 (probabilities sum to 1)
- 0 <= IFUNC(X, Y)(i) <= M * N for all i
- IFUNC(X, Y)(i) = 0 means interval i cannot be spanned from X to Y

# Musical Context/Application
This probabilistic interpretation allows IFUNC to model statistical intervallic textures in improvisatory or aleatoric contexts. When two instruments improvise freely on different pitch collections, IFUNC predicts the statistical distribution of intervals between them. It also provides a backdrop against which to judge whether a particular interval appears "often" or "rarely."

# Examples
From Schoenberg's Violin Fantasy op.47 (Figure 5.8):
- Violin plays Y = {Bb, A, C#, B, F, G}
- Piano plays X = {Eb, E, C, D, Ab, Gb}

IFUNC(X, Y) shows:
- "Many" odd intervals (appearing frequently)
- "Few" even intervals (appearing rarely)
- IFUNC(X, Y)(0) = 0 (no common tones)
- IFUNC(X, Y)(4) = 2 and IFUNC(X, Y)(8) = 2 (scarce intervals)

The "scarce" intervals 4 and 8 are analytically significant because they appear only at structural boundary tones of the phrase.

# Related Concepts
- IFUNC (Interval Function)
- Stochastic Composition
- Statistical Texture
- Interval Vector

# Common Confusions
The probability interpretation works when random selection is appropriate. In composed music, intervals are not randomly distributed, so the probabilistic interpretation provides a "backdrop" or null hypothesis against which to measure compositional choices, not a direct model of what will occur.

# Source Reference
Chapter 5: Generalized Set Theory (1): Interval Functions; Canonical Groups and Canonical Equivalence; Embedding Functions, Theorem 5.1.8 and Figure 5.8
