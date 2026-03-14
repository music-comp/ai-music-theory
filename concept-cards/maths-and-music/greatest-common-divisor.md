---
concept: Greatest Common Divisor
category: theory
source: "Mathematics and Music"
chapter: "Algebraic Properties of the Integers"
chapter_number: 8
pdf_page: 100
authors: "David Wright"
---

# Quick Definition
The largest positive integer that divides two given integers, fundamental to determining which intervals generate a chromatic scale and which polyrhythmic patterns are musically effective.

# Formal Definition
Given $m, n \in \mathbb{Z}$, not both zero, the greatest common divisor $\gcd(m, n)$ is the unique positive generator $d$ of the ideal $m\mathbb{Z} + n\mathbb{Z}$ in $\mathbb{Z}$. Equivalently, $d$ is the largest positive integer dividing both $m$ and $n$. There exist integers $h, k$ such that $d = hm + kn$ (Bezout's identity).

# Mathematical Context
Since $\mathbb{Z}$ is a principal ideal domain, the ideal $m\mathbb{Z} + n\mathbb{Z}$ has a unique positive generator $d = \gcd(m, n)$. The key properties are: (1) $d$ divides both $m$ and $n$; (2) if $e$ is any positive integer dividing both $m$ and $n$, then $e$ divides $d$, so $d \geq e$. The expressibility $d = hm + kn$ follows from $d$ generating the ideal $m\mathbb{Z} + n\mathbb{Z}$.

# Musical Context
The gcd determines which modular chromatic intervals are generators: $[m]$ generates $\mathbb{Z}_n$ if and only if $\gcd(m, n) = 1$. It also governs the m-on-n polyrhythmic patterns used by composers: when $\gcd(m, n) = 1$, a pattern of $m$ notes against $n$ beats runs for exactly $mn$ units before the double cycle repeats.

# Examples
- $\gcd(5, 12) = 1$: the fourth [5] is a generating interval in the 12-chromatic scale
- $\gcd(4, 12) = 4$: the major third [4] is not a generator; iterating it cycles through only 3 of 12 note classes
- $\gcd(3, 4) = 1$: explains why the 3-on-4 pattern in "In the Mood" takes $3 \times 4 = 12$ notes to complete
- $12\mathbb{Z} + 15\mathbb{Z} = 3\mathbb{Z}$, so $\gcd(12, 15) = 3$

# Related Concepts
- Relatively Prime Integers
- Generating Interval
- Euler Phi Function
- Ideals and Principal Ideals
- M on N Polyrhythmic Patterns

# Common Confusions
- $\gcd(m, n)$ is always positive, even if $m$ or $n$ is negative
- The gcd is defined via divisibility in $\mathbb{Z}$, not by comparing magnitudes; it is the largest common divisor, not the "greatest common factor" of absolute values (though these coincide)
- The existence of $h, k$ with $d = hm + kn$ is a non-trivial consequence of $\mathbb{Z}$ being a PID

# Source Reference
Chapter 8, "Greatest Common Divisor" section, p. 100 (PDF)
