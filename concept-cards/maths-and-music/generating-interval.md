---
concept: Generating Interval
category: theory
source: "Mathematics and Music"
chapter: "Chromatic Scales"
chapter_number: 6
pdf_page: 74
authors: "David Wright"
unit: null
---

# Quick Definition
A modular chromatic interval whose successive iterations produce all intervals in the n-chromatic scale, corresponding to a generator of the cyclic group $\mathbb{Z}_n$.

# Formal Definition
In the n-chromatic scale, a modular interval $[m] \in \mathbb{Z}_n$ is a generating interval if every element of $\mathbb{Z}_n$ can be expressed as a multiple of $[m]$. Equivalently, $[m]$ is a generating interval if and only if $\gcd(m, n) = 1$. The number of generating intervals in the n-chromatic scale is $\phi(n)$, where $\phi$ is the Euler phi function.

# Mathematical Context
The generating intervals correspond precisely to the generators of the cyclic group $(\mathbb{Z}_n, +)$. A generator $[m]$ has order $n$ in $\mathbb{Z}_n$, meaning $n$ is the smallest positive integer $k$ such that $k \cdot [m] = [0]$. The "circle" based on a generating interval contains all $n$ chromatic intervals before returning to the starting point.

# Musical Context
In the standard 12-chromatic scale, the generating intervals are those whose iterations cycle through all 12 note classes. The circle of fifths (iterating by 7 semitones) is the most musically significant example. There are exactly $\phi(12) = 4$ generating intervals in the 12-chromatic scale.

# Examples
- In the 12-chromatic scale, the generating intervals are: semitone [1], fourth [5], fifth [7], major seventh [11], since $\gcd(1,12) = \gcd(5,12) = \gcd(7,12) = \gcd(11,12) = 1$
- In the 14-chromatic scale, $\phi(14) = 6$ generating intervals: [1], [3], [5], [9], [11], [13]
- The circle of fifths is the circle of intervals based on [7] in $\mathbb{Z}_{12}$
- Non-generating intervals (e.g., [2], [3], [4], [6] in $\mathbb{Z}_{12}$) cycle through only a subset of note classes

# Related Concepts
- Cyclic Group and Generator
- Greatest Common Divisor
- Euler Phi Function
- Relatively Prime Integers
- Circle of Intervals

# Common Confusions
- A generating interval is not the same as a "generated scale" (a scale built from stacking an interval); generating intervals specifically generate all elements of $\mathbb{Z}_n$
- The number of generating intervals depends only on $n$ through $\phi(n)$, not on which specific intervals sound "good"

# Source Reference
Chapter 6, "Generating intervals" section, p. 74 (PDF); developed further in Chapters 7 and 8
