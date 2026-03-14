---
concept: Modular Equivalence on the Real Numbers
category: theory
source: "Mathematics and Music"
chapter: "Octave Identification and Modular Arithmetic"
chapter_number: 7
pdf_page: 82
authors: "David Wright"
unit: null
---

# Quick Definition
An equivalence relation on real numbers where two numbers are equivalent if they differ by an integer multiple of a fixed modulus $m$, modeling the wrapping of the number line around a circle.

# Formal Definition
For a fixed positive integer $m$, two real numbers $x$ and $y$ are equivalent, written $x \sim y$, if $x - y = qm$ for some $q \in \mathbb{Z}$. This defines an equivalence relation on $\mathbb{R}$, partitioning it into equivalence classes. The equivalence class of $x$ is denoted $\bar{x}$. The set of equivalence classes is denoted $\mathbb{R}/{\sim}$.

# Mathematical Context
The function $w: \mathbb{R} \to \mathbb{R}/{\sim}$ defined by $w(x) = \bar{x}$ wraps the number line around a circle of circumference $m$, preserving distance as arc length. The Generalized Division Algorithm guarantees that each equivalence class has exactly one representative $r$ with $0 \leq r < m$. The set $\mathbb{R}/{\sim}$ inherits addition from $\mathbb{R}$: $\bar{x} + \bar{y} = \overline{x + y}$, making $(\mathbb{R}/{\sim}, +)$ a group and $w$ a surjective group homomorphism.

# Musical Context
When $m = 12$ (measuring intervals in semitones), modular equivalence on $\mathbb{R}$ models octave equivalence for all intervals, not just chromatic ones. The wrapping of the real line around the circle is a geometric visualization of how all possible intervals map to a "circle" of equivalence classes modulo octave.

# Examples
- With $m = 8$: $\overline{13} = \overline{53} = \overline{-11}$ (since $13 - 53 = -40 = -5 \cdot 8$ and $13 - (-11) = 24 = 3 \cdot 8$)
- With $m = 8$: $\overline{6.5} = \overline{-1.5}$ (since $6.5 - (-1.5) = 8 = 1 \cdot 8$)
- Each equivalence class has a unique representative in $[0, m)$
- The origin $x = 0$ maps to the top of the circle

# Related Concepts
- Modular Equivalence on the Integers
- Modular Integers
- Generalized Division Algorithm
- Octave Equivalence Formalized
- Group of Modular Intervals

# Common Confusions
- This equivalence applies to ALL real numbers, not just integers; $3.7 \sim 11.7$ when $m = 8$
- The symbol $\sim$ depends on the choice of $m$, which must always be established in context
- The wrapping function $w$ is a homomorphism but NOT an isomorphism, since infinitely many real numbers map to each equivalence class

# Source Reference
Chapter 7, "Modular Equivalence on the Real Numbers" section, p. 82 (PDF)
