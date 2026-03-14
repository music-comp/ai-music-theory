---
concept: Nonstandard Chromatic Intervals
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
---

# Quick Definition
Modular intervals in an n-chromatic scale where $n \neq 12$, identified with the group $\mathbb{Z}_n$ and providing the interval vocabulary for non-standard equal temperaments.

# Formal Definition
If the octave is divided into $n$ equal intervals, intervals measured in n-chromatic units (modulo octave) form the group $\mathbb{Z}_n$. Each element $[k] \in \mathbb{Z}_n$ represents an interval of $k \cdot (1200/n)$ cents. The generating intervals are those $[m]$ with $\gcd(m, n) = 1$, and there are $\phi(n)$ such generators.

# Mathematical Context
The group $\mathbb{Z}_n$ has exactly $n$ elements and is cyclic. The number of generators, $\phi(n)$, depends on the prime factorization of $n$. For prime $n$, every non-zero element is a generator ($\phi(n) = n - 1$). For composite $n$, some intervals generate only subgroups of $\mathbb{Z}_n$.

# Musical Context
Non-standard chromatic intervals provide the building blocks for composition in alternative equal temperaments. Each choice of $n$ creates a different palette of available intervals, some of which may approximate standard intervals well while others produce entirely novel sonorities. The structure of $\mathbb{Z}_n$ determines which intervals can substitute for the role of the fifth or fourth in standard harmony.

# Examples
- In $\mathbb{Z}_5$: all non-zero elements are generators ($\phi(5) = 4$); every non-trivial interval generates all 5 note classes
- In $\mathbb{Z}_6$: generators are $[1]$ and $[5]$ only ($\phi(6) = 2$); $[2], [3], [4]$ generate proper subgroups
- In $\mathbb{Z}_{14}$: six generators $[1], [3], [5], [9], [11], [13]$
- In $\mathbb{Z}_{10}$: generators are $[1], [3], [7], [9]$ ($\phi(10) = 4$)

# Related Concepts
- N-Chromatic Scale
- Modular Chromatic Intervals
- Generating Interval
- Cyclic Group and Generator
- Non-Standard Chromatic Scales

# Common Confusions
- Nonstandard chromatic intervals are measured in n-chromatic units, not semitones; 1 unit = $1200/n$ cents, not 100 cents
- The group structure of $\mathbb{Z}_n$ varies with $n$; prime values of $n$ give the "richest" generator structure
- Converting between n-chromatic units and 12-chromatic units requires the factor $12/n$

# Source Reference
Chapter 7, "Nonstandard Chromatic Intervals" section, p. 82 (PDF)
