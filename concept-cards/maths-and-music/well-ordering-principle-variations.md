---
concept: Well-Ordering Principle Variations
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
Four equivalent formulations of the Well-Ordering Principle, extending the basic statement about positive integers to cover negative integers and bounded subsets of all integers.

# Formal Definition
The four equivalent formulations are:
- **WOP.1:** Any non-empty subset of $\mathbb{Z}^+$ has a smallest element.
- **WOP.2:** Any non-empty subset of $\mathbb{Z}^-$ has a largest element.
- **WOP.3:** Any non-empty subset of $\mathbb{Z}$ which has a lower bound has a smallest element.
- **WOP.4:** Any non-empty subset of $\mathbb{Z}$ which has an upper bound has a largest element.

A lower bound for $T$ is a real number $y$ with $y \leq t$ for all $t \in T$. Upper bound is analogous.

# Mathematical Context
These formulations are taken as axioms and are easily seen to be equivalent. WOP.4 is used in the proof of the Generalized Division Algorithm: the set $S = \{\ell \in \mathbb{Z} \mid \ell m \leq x\}$ has $x/m$ as an upper bound, so by WOP.4 it has a largest element $q$. WOP.1 is used in the proof that $\mathbb{Z}$ is a principal ideal domain. These variations allow the WOP to be applied in proofs where the "natural" set may not be a subset of $\mathbb{Z}^+$.

# Musical Context
The Well-Ordering Principle variations provide the logical foundation for proofs about modular arithmetic and group generators, which in turn underpin the theory of chromatic scales, generating intervals, and unique factorization of intervals. While not directly musical, they are the axiomatic bedrock upon which the mathematical treatment of music theory rests.

# Examples
- WOP.1: $\{3, 7, 11, 15, \ldots\}$ has smallest element 3
- WOP.2: $\{-2, -5, -8, \ldots\}$ has largest element $-2$
- WOP.3: the set $\{n \in \mathbb{Z} \mid n \geq -3.5\}$ has smallest element $-3$
- WOP.4: the set $S = \{\ell \in \mathbb{Z} \mid 8\ell \leq 50\}$ has largest element 6

# Related Concepts
- Well-Ordering Principle
- Generalized Division Algorithm
- Division Algorithm

# Common Confusions
- WOP applies only to subsets of $\mathbb{Z}$, not $\mathbb{R}$; the set $(0, 1)$ in $\mathbb{R}$ has a lower bound but no smallest element
- The "bounded" versions (WOP.3 and WOP.4) require the bound to exist but it need not be an integer
- These four statements are logically equivalent; proving any one implies the others

# Source Reference
Chapter 7, "Variations On The Well-Ordering Principle" section, p. 82 (PDF)
