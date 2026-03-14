---
concept: Modular Equivalence on the Integers
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
unit: null
---

# Quick Definition
The restriction of modular equivalence to the integers, where two integers are equivalent modulo $m$ when their difference is divisible by $m$, forming the foundation of chromatic interval arithmetic.

# Formal Definition
For a fixed positive integer $m$, the equivalence relation $\sim$ restricted to $\mathbb{Z}$ gives: $k \equiv \ell \pmod{m}$ if and only if $m \mid (k - \ell)$ in $\mathbb{Z}$. The equivalence class of $k$ is denoted $[k]$, and $[k] = [\ell]$ if and only if $m \mid (k - \ell)$. The set of equivalence classes $\mathbb{Z}_m = \{[0], [1], \ldots, [m-1]\}$ has exactly $m$ elements.

# Mathematical Context
$\mathbb{Z}_m$ is a subset of $\mathbb{R}/{\sim}$ and can be visualized as $m$ equally spaced points on the circle of circumference $m$ (an "m-hour clock"). The elements $[0], [1], \ldots, [m-1]$ are the complete set of distinct equivalence classes. If $x \sim y$ and $x \in \mathbb{Z}$, then $y \in \mathbb{Z}$ as well, so $\sim$ restricts consistently from $\mathbb{R}$ to $\mathbb{Z}$.

# Musical Context
When $m = 12$, the equivalence $k \equiv \ell \pmod{12}$ captures octave equivalence for chromatic intervals: two intervals measured in semitones are octave-equivalent if they differ by a multiple of 12. The 12 elements of $\mathbb{Z}_{12}$ correspond to the 12 note classes C, C$\sharp$, D, ..., B.

# Examples
- $5 \equiv 19 \pmod{7}$, so $[5] = [19]$ in $\mathbb{Z}_7$ (since $19 - 5 = 14 = 2 \cdot 7$)
- In $\mathbb{Z}_{12}$: $[14] = [2]$ (since $14 - 2 = 12$), meaning 14 semitones is octave-equivalent to 2 semitones (a whole step)
- The "clock" visualization: $\mathbb{Z}_8$ has 8 positions around a circle, with $[0]$ at the top

# Related Concepts
- Modular Equivalence on the Real Numbers
- Modular Integers
- Modular Arithmetic
- Octave Equivalence Formalized
- Modular Chromatic Intervals

# Common Confusions
- The notation $[k]$ does not reference $m$; the modulus must always be clear from context
- $[k] = [\ell]$ does NOT mean $k = \ell$; it means $k$ and $\ell$ differ by a multiple of $m$
- $\mathbb{Z}_m$ has exactly $m$ elements, regardless of how large the integer representatives are

# Source Reference
Chapter 7, "Modular Equivalence on the Integers" section, p. 82 (PDF)
